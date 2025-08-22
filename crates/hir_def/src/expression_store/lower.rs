use la_arena::Arena;
use syntax::{HasGenerics, HasName, ast, pointer::AstPointer};

use crate::{
    HirFileId, InFile,
    data::{
        FieldData, FunctionData, GlobalConstantData, GlobalVariableData, OverrideData, StructData,
        TypeAliasData,
    },
    database::DefDatabase,
    expression::{Expression, ExpressionId, parse_literal},
    expression_store::{
        ExpressionSourceMap, ExpressionStore, ExpressionStoreBuilder, SyntheticSyntax,
    },
    module_data::Name,
    type_specifier::TypeSpecifier,
};

pub struct ExprCollector<'database> {
    database: &'database dyn DefDatabase,
    pub store: ExpressionStoreBuilder,
}

impl ExprCollector<'_> {
    pub fn new<'a>(database: &'a dyn DefDatabase) -> ExprCollector<'a> {
        ExprCollector {
            database,
            store: ExpressionStoreBuilder::default(),
        }
    }

    #[expect(clippy::too_many_lines, reason = "TODO")]
    pub fn collect_expression(
        &mut self,
        expression: ast::Expression,
    ) -> ExpressionId {
        let syntax_pointer = AstPointer::new(&expression);
        let expression = match expression {
            ast::Expression::InfixExpression(expression) => {
                let left_side = self.collect_expression_opt(expression.left_side());
                let right_side = self.collect_expression_opt(expression.right_side());

                expression
                    .op_kind()
                    .map_or(Expression::Missing, |op| Expression::BinaryOperation {
                        left_side,
                        right_side,
                        operation: op,
                    })
            },
            ast::Expression::PrefixExpression(prefix_expression) => {
                let expression = self.collect_expression_opt(prefix_expression.expression());
                prefix_expression
                    .op_kind()
                    .map_or(Expression::Missing, |op| Expression::UnaryOperator {
                        expression,
                        op,
                    })
            },
            ast::Expression::Literal(literal) => {
                let literal = literal.kind();
                Expression::Literal(parse_literal(literal))
            },
            ast::Expression::ParenthesisExpression(expression) => {
                let inner = self.collect_expression_opt(expression.inner());
                // make the paren expression point to the inner expression as well
                self.store.expression_map.insert(syntax_pointer, inner);
                self.store.parenthesis_expressions.insert(inner);
                return inner;
            },
            ast::Expression::FieldExpression(field) => {
                let expression = self.collect_expression_opt(field.expression());
                let name = field
                    .field()
                    .map_or_else(Name::missing, |field| Name::from(field.text()));

                Expression::Field { expression, name }
            },
            ast::Expression::FunctionCall(call) => {
                let arguments = call
                    .parameters()
                    .into_iter()
                    .flat_map(|parameters| parameters.arguments())
                    .map(|expression| self.collect_expression(expression))
                    .collect();
                let name = as_name_opt(
                    call.ident_expression()
                        .and_then(|identifier| identifier.name_ref()),
                );
                let generics = self.collect_generics(
                    call.ident_expression()
                        .and_then(|identifier| (identifier.generic_arg_list())),
                );

                Expression::Call {
                    type_specifier: TypeSpecifier {
                        path: name,
                        generics,
                    },
                    arguments,
                }
            },
            ast::Expression::IdentExpression(identifier) => {
                let name = as_name_opt(identifier.name_ref());
                let generics = self.collect_generics(identifier.generic_arg_list());

                Expression::TypeSpecifier { name, generics }
            },
            ast::Expression::IndexExpression(index) => {
                let left_side = self.collect_expression_opt(index.expression());
                let index = self.collect_expression_opt(index.index());
                Expression::Index { left_side, index }
            },
        };

        self.alloc_expression(expression, syntax_pointer)
    }

    pub fn collect_type_specifier(
        &mut self,
        expression: ast::TypeSpecifier,
    ) -> TypeSpecifier {
        TypeSpecifier {
            path: as_name_opt(expression.name_ref()),
            generics: self.collect_generics(expression.generic_arg_list()),
        }
    }
    pub fn collect_generics(
        &mut self,
        generics: Option<ast::GenericArgumentList>,
    ) -> Vec<ExpressionId> {
        generics.map_or_else(Vec::new, |generics| {
            generics
                .generics()
                .map(|g| self.collect_expression(g))
                .collect()
        })
    }

    pub fn collect_type_specifier_opt(
        &mut self,
        expression: Option<ast::TypeSpecifier>,
    ) -> TypeSpecifier {
        match expression {
            Some(v) => self.collect_type_specifier(v),
            None => TypeSpecifier {
                path: Name::missing(),
                generics: vec![],
            },
        }
    }

    pub fn collect_function_param_list(
        &mut self,
        function_param_list: &ast::FunctionParameters,
    ) -> Vec<(TypeSpecifier, Name)> {
        function_param_list
            .parameters()
            .map(|parameter| {
                let r#type = self.collect_type_specifier_opt(parameter.ty());
                let name = parameter.name().map_or_else(Name::missing, Name::from);
                (r#type, name)
            })
            .collect()
    }

    fn alloc_expression(
        &mut self,
        expression: Expression,
        source: AstPointer<ast::Expression>,
    ) -> ExpressionId {
        let id = self.make_expression(expression, Ok(source.clone()));
        self.store.expression_map.insert(source, id);
        id
    }

    fn make_expression(
        &mut self,
        expression: Expression,
        source: Result<AstPointer<ast::Expression>, SyntheticSyntax>,
    ) -> ExpressionId {
        let id = self.store.exprs.alloc(expression);
        self.store.expression_map_back.insert(id, source);
        id
    }

    fn missing_expression(&mut self) -> ExpressionId {
        self.make_expression(Expression::Missing, Err(SyntheticSyntax))
    }

    pub fn collect_expression_opt(
        &mut self,
        expression: Option<ast::Expression>,
    ) -> ExpressionId {
        match expression {
            Some(expression) => self.collect_expression(expression),
            None => self.missing_expression(),
        }
    }
}

pub(crate) fn lower_function(
    database: &dyn DefDatabase,
    function: InFile<ast::FunctionDeclaration>,
) -> (FunctionData, ExpressionSourceMap) {
    let name = as_name_opt(function.value.name());

    let mut collector = ExprCollector::new(database);
    let parameters = function.value.parameter_list().map_or_else(
        || Vec::new(),
        |parameters| collector.collect_function_param_list(&parameters),
    );
    let return_type = function
        .value
        .return_type()
        .and_then(|r#type| r#type.ty())
        .map(|r#type| collector.collect_type_specifier(r#type));

    let (store, source_map) = collector.store.finish();
    let specifier = FunctionData {
        name,
        store,
        parameters,
        return_type,
    };
    (specifier, source_map)
}

pub(crate) fn lower_struct(
    database: &dyn DefDatabase,
    struct_declaration: InFile<ast::StructDeclaration>,
) -> (StructData, ExpressionSourceMap) {
    let name = as_name_opt(struct_declaration.value.name());

    let mut collector = ExprCollector::new(database);
    let mut fields = Arena::new();
    if let Some(body) = struct_declaration.value.body() {
        fields.alloc_many(body.fields().map(|field| FieldData {
            name: as_name_opt(field.name()),
            r#type: collector.collect_type_specifier_opt(field.ty()),
        }));
    }

    let (store, source_map) = collector.store.finish();
    let specifier = StructData {
        name,
        store,
        fields,
    };
    (specifier, source_map)
}

pub(crate) fn lower_type_alias(
    database: &dyn DefDatabase,
    type_alias: InFile<ast::TypeAliasDeclaration>,
) -> (TypeAliasData, ExpressionSourceMap) {
    let name = as_name_opt(type_alias.value.name());

    let mut collector = ExprCollector::new(database);
    let r#type = collector.collect_type_specifier_opt(type_alias.value.type_declaration());

    let (store, source_map) = collector.store.finish();
    let specifier = TypeAliasData {
        name,
        store,
        r#type,
    };
    (specifier, source_map)
}

pub(crate) fn lower_variable(
    database: &dyn DefDatabase,
    global_variable: InFile<ast::VariableDeclaration>,
) -> (GlobalVariableData, ExpressionSourceMap) {
    let name = as_name_opt(global_variable.value.name());

    let mut collector = ExprCollector::new(database);
    let r#type = global_variable
        .value
        .ty()
        .map(|ty| collector.collect_type_specifier(ty));

    let generics = if let Some(generics) = global_variable.value.generic_arg_list() {
        generics
            .generics()
            .map(|expression| collector.collect_expression(expression))
            .collect()
    } else {
        Vec::new()
    };

    let (store, source_map) = collector.store.finish();
    let specifier = GlobalVariableData {
        name,
        store,
        r#type,
        generics,
    };
    (specifier, source_map)
}

pub(crate) fn lower_constant(
    database: &dyn DefDatabase,
    global_constant: InFile<ast::ConstantDeclaration>,
) -> (GlobalConstantData, ExpressionSourceMap) {
    let name = as_name_opt(global_constant.value.name());

    let mut collector = ExprCollector::new(database);
    let r#type = global_constant
        .value
        .ty()
        .map(|ty| collector.collect_type_specifier(ty));

    let (store, source_map) = collector.store.finish();
    let specifier = GlobalConstantData {
        name,
        store,
        r#type,
    };
    (specifier, source_map)
}
pub(crate) fn lower_override(
    database: &dyn DefDatabase,
    global_override: InFile<ast::OverrideDeclaration>,
) -> (OverrideData, ExpressionSourceMap) {
    let name = as_name_opt(global_override.value.name());

    let mut collector = ExprCollector::new(database);
    let r#type = global_override
        .value
        .ty()
        .map(|ty| collector.collect_type_specifier(ty));

    let (store, source_map) = collector.store.finish();
    let specifier = OverrideData {
        name,
        store,
        r#type,
    };
    (specifier, source_map)
}

fn as_name_opt<N>(name: Option<N>) -> Name
where
    Name: From<N>,
{
    name.map_or_else(Name::missing, Name::from)
}
