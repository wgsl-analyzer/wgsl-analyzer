use la_arena::Arena;
use syntax::{HasName as _, HasTemplateParameters as _, ast, pointer::AstPointer};
use triomphe::Arc;

use crate::{
    InFile,
    data::{
        FieldData, FunctionData, GlobalConstantData, GlobalVariableData, OverrideData, ParamData,
        StructData, TypeAliasData,
    },
    database::DefDatabase,
    expression::{Expression, ExpressionId, parse_literal},
    expression_store::{
        ExpressionSourceMap, ExpressionStoreBuilder, ExpressionStoreSource, SyntheticSyntax,
    },
    module_data::Name,
    type_specifier::{IdentExpression, TypeSpecifier, TypeSpecifierId},
};

pub struct ExprCollector<'database> {
    database: &'database dyn DefDatabase,
    store: ExpressionStoreBuilder,
}

impl ExprCollector<'_> {
    pub fn new(
        database: &dyn DefDatabase,
        store_source: ExpressionStoreSource,
    ) -> ExprCollector<'_> {
        ExprCollector {
            database,
            store: ExpressionStoreBuilder {
                store_source,
                ..Default::default()
            },
        }
    }

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
                    .map_or(Expression::Missing, |operator| {
                        Expression::BinaryOperation {
                            left_side,
                            right_side,
                            operation: operator,
                        }
                    })
            },
            ast::Expression::PrefixExpression(prefix_expression) => {
                let expression = self.collect_expression_opt(prefix_expression.expression());
                prefix_expression
                    .operator_kind()
                    .map_or(Expression::Missing, |operator| Expression::UnaryOperator {
                        expression,
                        operator,
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
                let template_parameters = self.collect_template_parameters(
                    call.ident_expression()
                        .and_then(|identifier| identifier.template_parameters()),
                );

                Expression::Call {
                    ident_expression: IdentExpression {
                        path: name,
                        template_parameters,
                    },
                    arguments,
                }
            },
            ast::Expression::IdentExpression(identifier) => {
                let name = as_name_opt(identifier.name_ref());
                let template_parameters =
                    self.collect_template_parameters(identifier.template_parameters());

                Expression::IdentExpression(IdentExpression {
                    path: name,
                    template_parameters,
                })
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
        type_specifier: &ast::TypeSpecifier,
    ) -> TypeSpecifierId {
        let syntax_pointer = AstPointer::new(type_specifier);
        let type_specifier = TypeSpecifier {
            path: as_name_opt(type_specifier.name_ref()),
            template_parameters: self
                .collect_template_parameters(type_specifier.template_parameters()),
        };
        self.alloc_type_specifier(type_specifier, syntax_pointer)
    }

    pub fn collect_template_parameters(
        &mut self,
        template_parameters: Option<ast::TemplateList>,
    ) -> Vec<ExpressionId> {
        template_parameters.map_or_else(Vec::new, |template_parameters| {
            template_parameters
                .parameters()
                .map(|expression| self.collect_expression(expression))
                .collect()
        })
    }

    pub fn collect_function_param_list(
        &mut self,
        function_param_list: &ast::FunctionParameters,
    ) -> Arena<ParamData> {
        function_param_list
            .parameters()
            .map(|parameter| {
                let r#type = self.collect_type_specifier_opt(parameter.r#type());
                let name = parameter.name().map_or_else(Name::missing, Name::from);
                ParamData { name, r#type }
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

    fn alloc_type_specifier(
        &mut self,
        type_specifier: TypeSpecifier,
        source: AstPointer<ast::TypeSpecifier>,
    ) -> TypeSpecifierId {
        let id = self.make_type_specifier(type_specifier, Ok(source.clone()));
        self.store.type_map.insert(source, id);
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

    fn make_type_specifier(
        &mut self,
        type_specifier: TypeSpecifier,
        source: Result<AstPointer<ast::TypeSpecifier>, SyntheticSyntax>,
    ) -> TypeSpecifierId {
        let id = self.store.types.alloc(type_specifier);
        self.store.type_map_back.insert(id, source);
        id
    }

    fn missing_expression(&mut self) -> ExpressionId {
        self.make_expression(Expression::Missing, Err(SyntheticSyntax))
    }

    fn missing_type_specifier(&mut self) -> TypeSpecifierId {
        self.make_type_specifier(
            TypeSpecifier {
                path: Name::missing(),
                template_parameters: vec![],
            },
            Err(SyntheticSyntax),
        )
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

    pub fn collect_type_specifier_opt(
        &mut self,
        type_specifier: Option<ast::TypeSpecifier>,
    ) -> TypeSpecifierId {
        match type_specifier {
            Some(type_specifier) => self.collect_type_specifier(&type_specifier),
            None => self.missing_type_specifier(),
        }
    }

    #[must_use]
    pub fn finish(self) -> (super::ExpressionStore, ExpressionSourceMap) {
        self.store.finish()
    }
}

pub(crate) fn lower_function(
    database: &dyn DefDatabase,
    function: &InFile<ast::FunctionDeclaration>,
) -> (FunctionData, ExpressionSourceMap) {
    let name = as_name_opt(function.value.name());

    let mut collector = ExprCollector::new(database, ExpressionStoreSource::Signature);
    let parameters = function
        .value
        .parameter_list()
        .map_or_else(Arena::new, |parameters| {
            collector.collect_function_param_list(&parameters)
        });
    let return_type = function
        .value
        .return_type()
        .and_then(|r#type| r#type.r#type())
        .map(|r#type| collector.collect_type_specifier(&r#type));

    let (store, source_map) = collector.finish();
    let specifier = FunctionData {
        name,
        store: Arc::new(store),
        parameters,
        return_type,
    };
    (specifier, source_map)
}

pub(crate) fn lower_struct(
    database: &dyn DefDatabase,
    struct_declaration: &InFile<ast::StructDeclaration>,
) -> (StructData, ExpressionSourceMap) {
    let name = as_name_opt(struct_declaration.value.name());

    let mut collector = ExprCollector::new(database, ExpressionStoreSource::Signature);
    let mut fields = Arena::new();
    if let Some(body) = struct_declaration.value.body() {
        fields.alloc_many(body.fields().map(|field| FieldData {
            name: as_name_opt(field.name()),
            r#type: collector.collect_type_specifier_opt(field.r#type()),
        }));
    }

    let (store, source_map) = collector.finish();
    let specifier = StructData {
        name,
        store: Arc::new(store),
        fields,
    };
    (specifier, source_map)
}

pub(crate) fn lower_type_alias(
    database: &dyn DefDatabase,
    type_alias: &InFile<ast::TypeAliasDeclaration>,
) -> (TypeAliasData, ExpressionSourceMap) {
    let name = as_name_opt(type_alias.value.name());

    let mut collector = ExprCollector::new(database, ExpressionStoreSource::Signature);
    let r#type = collector.collect_type_specifier_opt(type_alias.value.type_declaration());

    let (store, source_map) = collector.finish();
    let specifier = TypeAliasData {
        name,
        store: Arc::new(store),
        r#type,
    };
    (specifier, source_map)
}

pub(crate) fn lower_variable(
    database: &dyn DefDatabase,
    global_variable: &InFile<ast::VariableDeclaration>,
) -> (GlobalVariableData, ExpressionSourceMap) {
    let name = as_name_opt(global_variable.value.name());

    let mut collector = ExprCollector::new(database, ExpressionStoreSource::Signature);
    let r#type = global_variable
        .value
        .r#type()
        .map(|r#type| collector.collect_type_specifier(&r#type));

    let template_parameters =
        if let Some(template_parameters) = global_variable.value.template_parameters() {
            template_parameters
                .parameters()
                .map(|expression| collector.collect_expression(expression))
                .collect()
        } else {
            Vec::new()
        };

    let (store, source_map) = collector.finish();
    let specifier = GlobalVariableData {
        name,
        store: Arc::new(store),
        r#type,
        template_parameters,
    };
    (specifier, source_map)
}

pub(crate) fn lower_constant(
    database: &dyn DefDatabase,
    global_constant: &InFile<ast::ConstantDeclaration>,
) -> (GlobalConstantData, ExpressionSourceMap) {
    let name = as_name_opt(global_constant.value.name());

    let mut collector = ExprCollector::new(database, ExpressionStoreSource::Signature);
    let r#type = global_constant
        .value
        .r#type()
        .map(|r#type| collector.collect_type_specifier(&r#type));

    let (store, source_map) = collector.finish();
    let specifier = GlobalConstantData {
        name,
        store: Arc::new(store),
        r#type,
    };
    (specifier, source_map)
}
pub(crate) fn lower_override(
    database: &dyn DefDatabase,
    global_override: &InFile<ast::OverrideDeclaration>,
) -> (OverrideData, ExpressionSourceMap) {
    let name = as_name_opt(global_override.value.name());

    let mut collector = ExprCollector::new(database, ExpressionStoreSource::Signature);
    let r#type = global_override
        .value
        .r#type()
        .map(|r#type| collector.collect_type_specifier(&r#type));

    let (store, source_map) = collector.finish();
    let specifier = OverrideData {
        name,
        store: Arc::new(store),
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
