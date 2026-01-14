pub mod definition;
pub mod diagnostics;

use base_db::FileId;
use definition::Definition;
use diagnostics::{AnyDiagnostic, DiagnosticsConfig};
use either::Either;
use hir_def::{
    HasSource as _, HirFileId, InFile,
    body::{BindingId, Body, BodySourceMap},
    database::{
        DefDatabase, DefinitionWithBodyId, FunctionId, GlobalAssertStatementId, GlobalConstantId,
        GlobalVariableId, ImportId, Location, Lookup as _, OverrideId, StructId, TypeAliasId,
    },
    expression::{ExpressionId, StatementId},
    expression_store::path::Path,
    item_tree::{self, ItemTree, ModuleItem, Name},
    resolver::{ResolveKind, Resolver},
    signature::{FieldId, ParameterId},
};
pub use hir_ty::database::HirDatabase;
use hir_ty::{infer::InferenceResult, ty::Type};
use smallvec::SmallVec;
use stdx::impl_from;
use syntax::{AstNode as _, HasName as _, SyntaxNode, ast, pointer::AstPointer};
use triomphe::Arc;

pub trait HasSource {
    type Ast;
    fn source(
        self,
        database: &dyn DefDatabase,
    ) -> Option<InFile<Self::Ast>>;
}

/// Nice API on top of the layers below
pub struct Semantics<'database> {
    pub database: &'database dyn HirDatabase,
}

impl<'database> Semantics<'database> {
    pub fn new(database: &'database dyn HirDatabase) -> Self {
        Semantics { database }
    }

    #[must_use]
    pub fn parse(
        &self,
        file_id: FileId,
    ) -> ast::SourceFile {
        self.database.parse(file_id).tree()
    }

    #[must_use]
    pub fn analyze(
        &self,
        definition: DefinitionWithBodyId,
    ) -> SourceAnalyzer<'_> {
        SourceAnalyzer::new(self.database, definition)
    }

    /// Finds the root level container for a given node
    #[must_use]
    pub fn find_container(
        &self,
        file_id: HirFileId,
        source: &SyntaxNode,
    ) -> Option<ChildContainer> {
        source
            .ancestors()
            .find_map(|syntax| -> Option<ChildContainer> {
                if let Some(item) = ast::Item::cast(syntax) {
                    let container: ChildContainer = match item {
                        ast::Item::ImportStatement(import) => {
                            let definition = self.import_to_def(&InFile::new(file_id, import))?;
                            ChildContainer::ImportId(definition)
                        },
                        ast::Item::FunctionDeclaration(function_declaration) => {
                            let child_offset = source.text_range().start();
                            let is_in_body =
                                function_declaration
                                    .body()
                                    .is_some_and(|compound_statement| {
                                        compound_statement
                                            .syntax()
                                            .text_range()
                                            .contains(child_offset)
                                    });

                            let definition =
                                self.function_to_def(&InFile::new(file_id, function_declaration))?;
                            if is_in_body {
                                DefinitionWithBodyId::Function(definition).into()
                            } else {
                                ChildContainer::FunctionId(definition)
                            }
                        },
                        ast::Item::VariableDeclaration(variable_declaration) => {
                            let child_offset = source.text_range().start();
                            let is_in_body =
                                variable_declaration.init().is_some_and(|expression| {
                                    expression.syntax().text_range().contains(child_offset)
                                });

                            let definition = self.global_variable_to_def(&InFile::new(
                                file_id,
                                variable_declaration,
                            ))?;
                            if is_in_body {
                                DefinitionWithBodyId::GlobalVariable(definition).into()
                            } else {
                                ChildContainer::GlobalVariableId(definition)
                            }
                        },
                        ast::Item::ConstantDeclaration(constant_declaration) => {
                            let child_offset = source.text_range().start();
                            let is_in_body =
                                constant_declaration.init().is_some_and(|expression| {
                                    expression.syntax().text_range().contains(child_offset)
                                });

                            let definition = self.global_constant_to_def(&InFile::new(
                                file_id,
                                constant_declaration,
                            ))?;
                            if is_in_body {
                                DefinitionWithBodyId::GlobalConstant(definition).into()
                            } else {
                                ChildContainer::GlobalConstantId(definition)
                            }
                        },
                        ast::Item::OverrideDeclaration(override_declaration) => {
                            let child_offset = source.text_range().start();
                            let is_in_body =
                                override_declaration.init().is_some_and(|expression| {
                                    expression.syntax().text_range().contains(child_offset)
                                });

                            let definition = self.global_override_to_def(&InFile::new(
                                file_id,
                                override_declaration,
                            ))?;
                            if is_in_body {
                                DefinitionWithBodyId::Override(definition).into()
                            } else {
                                ChildContainer::OverrideId(definition)
                            }
                        },
                        ast::Item::TypeAliasDeclaration(type_alias_declaration) => {
                            let definition = self.global_type_alias_to_def(&InFile::new(
                                file_id,
                                type_alias_declaration,
                            ))?;
                            ChildContainer::TypeAliasId(definition)
                        },
                        ast::Item::StructDeclaration(struct_declaration) => {
                            let definition = self
                                .global_struct_to_def(&InFile::new(file_id, struct_declaration))?;
                            ChildContainer::StructId(definition)
                        },
                        ast::Item::AssertStatement(assert_statement) => {
                            let definition = self.global_assert_statement_to_def(&InFile::new(
                                file_id,
                                assert_statement,
                            ))?;
                            ChildContainer::GlobalAssertStatementId(definition)
                        },
                    };
                    Some(container)
                } else {
                    None
                }
            })
    }

    #[must_use]
    pub fn resolver(
        &self,
        file_id: HirFileId,
        source: &SyntaxNode,
    ) -> Resolver {
        if let Some(definition) = self.find_container(file_id, source) {
            definition.resolver(self.database)
        } else {
            let module_info = self.database.item_tree(file_id);
            Resolver::default().push_module_scope(file_id, module_info)
        }
    }

    #[must_use]
    #[expect(clippy::unused_self, reason = "intentional API")]
    pub fn module(
        self,
        file_id: FileId,
    ) -> Module {
        Module {
            file_id: file_id.into(),
        }
    }

    fn resolve_path_in_container(
        &self,
        container: ChildContainer,
        expression: &ast::Expression,
        path: &Path,
    ) -> Option<Definition> {
        let mut resolver = container.resolver(self.database);

        if let ChildContainer::DefinitionWithBodyId(DefinitionWithBodyId::Function(function)) =
            container
        {
            let (_, source_map) = self
                .database
                .body_with_source_map(DefinitionWithBodyId::Function(function));
            let expression_id = source_map.lookup_expression(&AstPointer::new(expression))?;
            let expression_scopes = self
                .database
                .expression_scopes(DefinitionWithBodyId::Function(function));
            let scope_id = expression_scopes.scope_for_expression(expression_id)?;
            resolver = resolver.push_expression_scope(function, expression_scopes, scope_id);
        }

        let value = resolver.resolve(path)?;

        let definition = match value {
            ResolveKind::Local(binding) => Definition::Local(Local {
                parent: resolver.body_owner()?,
                binding,
            }),
            ResolveKind::GlobalVariable(location) => {
                let id = self.database.intern_global_variable(location);
                Definition::ModuleDef(ModuleDef::GlobalVariable(GlobalVariable { id }))
            },
            ResolveKind::GlobalConstant(location) => {
                let id = self.database.intern_global_constant(location);
                Definition::ModuleDef(ModuleDef::GlobalConstant(GlobalConstant { id }))
            },
            ResolveKind::Override(location) => {
                let id = self.database.intern_override(location);
                Definition::ModuleDef(ModuleDef::Override(Override { id }))
            },
            ResolveKind::Struct(location) => {
                let id = self.database.intern_struct(location);
                Definition::ModuleDef(ModuleDef::Struct(Struct { id }))
            },
            ResolveKind::TypeAlias(location) => {
                let id = self.database.intern_type_alias(location);
                Definition::ModuleDef(ModuleDef::TypeAlias(TypeAlias { id }))
            },
            ResolveKind::Function(location) => {
                let id = self.database.intern_function(location);
                Definition::ModuleDef(ModuleDef::Function(Function { id }))
            },
        };

        Some(definition)
    }

    fn import_to_def(
        &self,
        source: &InFile<ast::ImportStatement>,
    ) -> Option<ImportId> {
        let import = item_tree::find_item(self.database, source.file_id, &source.value)?;
        let import_id = self
            .database
            .intern_import(Location::new(source.file_id, import));
        Some(import_id)
    }

    fn function_to_def(
        &self,
        source: &InFile<ast::FunctionDeclaration>,
    ) -> Option<FunctionId> {
        let function = item_tree::find_item(self.database, source.file_id, &source.value)?;
        let function_id = self
            .database
            .intern_function(Location::new(source.file_id, function));
        Some(function_id)
    }

    fn global_constant_to_def(
        &self,
        source: &InFile<ast::ConstantDeclaration>,
    ) -> Option<GlobalConstantId> {
        let global_constant = item_tree::find_item(self.database, source.file_id, &source.value)?;
        let id = self
            .database
            .intern_global_constant(Location::new(source.file_id, global_constant));
        Some(id)
    }

    fn global_variable_to_def(
        &self,
        source: &InFile<ast::VariableDeclaration>,
    ) -> Option<GlobalVariableId> {
        let global_variable = item_tree::find_item(self.database, source.file_id, &source.value)?;
        let id = self
            .database
            .intern_global_variable(Location::new(source.file_id, global_variable));
        Some(id)
    }

    fn global_override_to_def(
        &self,
        source: &InFile<ast::OverrideDeclaration>,
    ) -> Option<OverrideId> {
        let item = item_tree::find_item(self.database, source.file_id, &source.value)?;
        let id = self
            .database
            .intern_override(Location::new(source.file_id, item));
        Some(id)
    }

    fn global_type_alias_to_def(
        &self,
        source: &InFile<ast::TypeAliasDeclaration>,
    ) -> Option<TypeAliasId> {
        let item = item_tree::find_item(self.database, source.file_id, &source.value)?;
        let id = self
            .database
            .intern_type_alias(Location::new(source.file_id, item));
        Some(id)
    }

    fn global_struct_to_def(
        &self,
        source: &InFile<ast::StructDeclaration>,
    ) -> Option<StructId> {
        let item = item_tree::find_item(self.database, source.file_id, &source.value)?;
        let id = self
            .database
            .intern_struct(Location::new(source.file_id, item));
        Some(id)
    }

    fn global_assert_statement_to_def(
        &self,
        source: &InFile<ast::AssertStatement>,
    ) -> Option<GlobalAssertStatementId> {
        let item = item_tree::find_item(self.database, source.file_id, &source.value)?;
        let id = self
            .database
            .intern_global_assert_statement(Location::new(source.file_id, item));
        Some(id)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[expect(clippy::enum_variant_names, reason = "Suffix makes sense")]
pub enum ChildContainer {
    /// This variant is for when the expression is inside the body
    DefinitionWithBodyId(DefinitionWithBodyId),
    ImportId(ImportId),
    FunctionId(FunctionId),
    GlobalVariableId(GlobalVariableId),
    GlobalConstantId(GlobalConstantId),
    OverrideId(OverrideId),
    StructId(StructId),
    TypeAliasId(TypeAliasId),
    GlobalAssertStatementId(GlobalAssertStatementId),
}

impl_from!(
    DefinitionWithBodyId,
    ImportId,
    FunctionId,
    GlobalVariableId,
    GlobalConstantId,
    OverrideId,
    StructId,
    TypeAliasId
    for ChildContainer
);

impl ChildContainer {
    pub fn file_id(
        self,
        database: &dyn DefDatabase,
    ) -> HirFileId {
        match self {
            Self::DefinitionWithBodyId(id) => id.file_id(database),
            Self::ImportId(id) => id.lookup(database).file_id,
            Self::FunctionId(id) => id.lookup(database).file_id,
            Self::GlobalVariableId(id) => id.lookup(database).file_id,
            Self::GlobalConstantId(id) => id.lookup(database).file_id,
            Self::OverrideId(id) => id.lookup(database).file_id,
            Self::StructId(id) => id.lookup(database).file_id,
            Self::TypeAliasId(id) => id.lookup(database).file_id,
            Self::GlobalAssertStatementId(id) => id.lookup(database).file_id,
        }
    }

    pub fn resolver(
        self,
        database: &dyn HirDatabase,
    ) -> Resolver {
        match self {
            Self::DefinitionWithBodyId(id) => id.resolver(database),
            Self::ImportId(_)
            | Self::FunctionId(_)
            | Self::GlobalVariableId(_)
            | Self::GlobalConstantId(_)
            | Self::OverrideId(_)
            | Self::StructId(_)
            | Self::GlobalAssertStatementId(_)
            | Self::TypeAliasId(_) => {
                let file_id = self.file_id(database);
                let module_info = database.item_tree(file_id);
                Resolver::default().push_module_scope(file_id, module_info)
            },
        }
    }

    #[must_use]
    pub const fn as_def_with_body_id(self) -> Option<DefinitionWithBodyId> {
        if let Self::DefinitionWithBodyId(id) = self {
            Some(id)
        } else {
            None
        }
    }
}

fn module_item_to_def(
    database: &dyn HirDatabase,
    file_id: HirFileId,
    module_item: ModuleItem,
) -> SmallVec<[ModuleDef; 1]> {
    let definition = match module_item {
        ModuleItem::Function(function) => {
            let location = Location::new(file_id, function);
            let id = database.intern_function(location);
            ModuleDef::Function(Function { id })
        },
        ModuleItem::Struct(r#struct) => {
            let location = Location::new(file_id, r#struct);
            let id = database.intern_struct(location);
            ModuleDef::Struct(Struct { id })
        },
        ModuleItem::GlobalVariable(variable) => {
            let location = Location::new(file_id, variable);
            let id = database.intern_global_variable(location);
            ModuleDef::GlobalVariable(GlobalVariable { id })
        },
        ModuleItem::GlobalConstant(constant) => {
            let location = Location::new(file_id, constant);
            let id = database.intern_global_constant(location);
            ModuleDef::GlobalConstant(GlobalConstant { id })
        },
        ModuleItem::Override(constant) => {
            let location = Location::new(file_id, constant);
            let id = database.intern_override(location);
            ModuleDef::Override(Override { id })
        },
        ModuleItem::TypeAlias(type_alias) => {
            let location = Location::new(file_id, type_alias);
            let id = database.intern_type_alias(location);
            ModuleDef::TypeAlias(TypeAlias { id })
        },
        ModuleItem::GlobalAssertStatement(global_assert_statement) => {
            let location = Location::new(file_id, global_assert_statement);
            let id = database.intern_global_assert_statement(location);
            ModuleDef::GlobalAssertStatement(GlobalAssertStatement { id })
        },
        ModuleItem::ImportStatement(_) => return smallvec::SmallVec::new(),
    };
    smallvec::smallvec![definition]
}

pub struct SourceAnalyzer<'database> {
    pub database: &'database dyn HirDatabase,
    pub body: Arc<Body>,
    pub body_source_map: Arc<BodySourceMap>,
    pub infer: Arc<InferenceResult>,
    pub owner: DefinitionWithBodyId,
}

impl<'database> SourceAnalyzer<'database> {
    fn new(
        database: &'database dyn HirDatabase,
        definition: DefinitionWithBodyId,
    ) -> Self {
        let (body, body_source_map) = database.body_with_source_map(definition);
        let infer = database.infer(definition);
        Self {
            database,
            body,
            body_source_map,
            infer,
            owner: definition,
        }
    }

    #[must_use]
    pub fn type_of_expression(
        &self,
        expression: &ast::Expression,
    ) -> Option<Type> {
        let id = self.expression_id(expression)?;
        Some(self.infer[id])
    }

    #[must_use]
    pub fn type_of_binding(
        &self,
        binding: &ast::Name,
    ) -> Option<Type> {
        let id = self.binding_id(binding)?;
        Some(self.infer[id])
    }

    #[must_use]
    pub fn resolve_field(
        &self,
        field: ast::FieldExpression,
    ) -> Option<Field> {
        let expression = self.expression_id(&ast::Expression::FieldExpression(field))?;
        let field = self.infer.field_resolution(expression)?;

        Some(Field { id: field })
    }

    #[must_use]
    pub fn resolver_for(
        &self,
        scope: Either<ast::Expression, ast::Statement>,
    ) -> Resolver {
        let mut resolver = self.owner.resolver(self.database);

        let expression_scopes = self.database.expression_scopes(self.owner);

        let scope_id = scope
            .map_left(|expression| {
                let id = self.expression_id(&expression)?;
                expression_scopes.scope_for_expression(id)
            })
            .map_right(|statement| {
                let id = self.statement_id(&statement)?;
                if let Some(Either::Left(root)) = self.body.root
                    && root == id
                {
                    return expression_scopes.scope_for_statement(id);
                }
                expression_scopes.scope_for_statement(id)
            })
            .into_inner();
        let Some(scope_id) = scope_id else {
            return resolver;
        };

        if let DefinitionWithBodyId::Function(function) = self.owner {
            resolver = resolver.push_expression_scope(function, expression_scopes, scope_id);
        }

        resolver
    }

    #[must_use]
    pub fn binding_id(
        &self,
        source: &ast::Name,
    ) -> Option<BindingId> {
        self.body_source_map
            .lookup_binding(&AstPointer::new(source))
    }

    #[must_use]
    pub fn expression_id(
        &self,
        source: &ast::Expression,
    ) -> Option<ExpressionId> {
        self.body_source_map
            .lookup_expression(&AstPointer::new(source))
    }

    #[must_use]
    pub fn statement_id(
        &self,
        source: &ast::Statement,
    ) -> Option<StatementId> {
        self.body_source_map
            .lookup_statement(&AstPointer::new(source))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct Local {
    pub parent: FunctionId,
    pub binding: BindingId,
}

impl HasSource for Local {
    type Ast = ast::Name;

    fn source(
        self,
        database: &dyn DefDatabase,
    ) -> Option<InFile<Self::Ast>> {
        let file_id = self.parent.lookup(database).file_id;
        let (_, source_map) =
            database.body_with_source_map(DefinitionWithBodyId::Function(self.parent));
        let binding = source_map
            .binding_to_source(self.binding)
            .map_err(drop)
            .ok()?;

        let root = database.parse_or_resolve(file_id).syntax();
        Some(InFile::new(file_id, binding.to_node(&root)))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct Parameter {
    pub id: ParameterId,
}

impl HasSource for Parameter {
    type Ast = ast::Parameter;

    fn source(
        self,
        database: &dyn DefDatabase,
    ) -> Option<InFile<Self::Ast>> {
        let function_data = database.function_data(self.id.function).0;
        let parameter_data = &function_data.parameters[self.id.param];
        let parameter_name = &parameter_data.name;

        let function = self.id.function.lookup(database).source(database);

        let parameter = function
            .value
            .parameter_list()?
            .parameters()
            .find_map(|parameter| {
                let name = parameter.name()?;
                (name.ident_token()?.text() == parameter_name.as_str()).then_some(parameter)
            })?;

        Some(InFile::new(function.file_id, parameter))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct Function {
    id: FunctionId,
}

impl HasSource for Function {
    type Ast = ast::FunctionDeclaration;

    fn source(
        self,
        database: &dyn DefDatabase,
    ) -> Option<InFile<Self::Ast>> {
        Some(self.id.lookup(database).source(database))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct GlobalVariable {
    id: GlobalVariableId,
}

impl HasSource for GlobalVariable {
    type Ast = ast::VariableDeclaration;

    fn source(
        self,
        database: &dyn DefDatabase,
    ) -> Option<InFile<Self::Ast>> {
        Some(self.id.lookup(database).source(database))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct GlobalConstant {
    id: GlobalConstantId,
}

impl HasSource for GlobalConstant {
    type Ast = ast::ConstantDeclaration;

    fn source(
        self,
        database: &dyn DefDatabase,
    ) -> Option<InFile<Self::Ast>> {
        Some(self.id.lookup(database).source(database))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct Override {
    id: OverrideId,
}

impl HasSource for Override {
    type Ast = ast::OverrideDeclaration;

    fn source(
        self,
        database: &dyn DefDatabase,
    ) -> Option<InFile<Self::Ast>> {
        Some(self.id.lookup(database).source(database))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct Struct {
    id: StructId,
}

impl HasSource for Struct {
    type Ast = ast::StructDeclaration;

    fn source(
        self,
        database: &dyn DefDatabase,
    ) -> Option<InFile<Self::Ast>> {
        Some(self.id.lookup(database).source(database))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct TypeAlias {
    id: TypeAliasId,
}

impl HasSource for TypeAlias {
    type Ast = ast::TypeAliasDeclaration;

    fn source(
        self,
        database: &dyn DefDatabase,
    ) -> Option<InFile<Self::Ast>> {
        Some(self.id.lookup(database).source(database))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct GlobalAssertStatement {
    id: GlobalAssertStatementId,
}

impl HasSource for GlobalAssertStatement {
    type Ast = ast::AssertStatement;

    fn source(
        self,
        database: &dyn DefDatabase,
    ) -> Option<InFile<Self::Ast>> {
        Some(self.id.lookup(database).source(database))
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Field {
    pub id: FieldId,
}

impl HasSource for Field {
    type Ast = ast::StructMember;

    fn source(
        self,
        database: &dyn DefDatabase,
    ) -> Option<InFile<Self::Ast>> {
        let struct_data = database.struct_data(self.id.r#struct).0;
        let field_data = &struct_data.fields()[self.id.field];
        let field_name = &field_data.name;

        let r#struct = self.id.r#struct.lookup(database).source(database);

        let field = r#struct.value.body()?.fields().find_map(|field| {
            let name = field.name()?;
            (name.ident_token()?.text() == field_name.as_str()).then_some(field)
        })?;

        Some(InFile::new(r#struct.file_id, field))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleDef {
    Function(Function),
    GlobalVariable(GlobalVariable),
    GlobalConstant(GlobalConstant),
    Override(Override),
    Struct(Struct),
    TypeAlias(TypeAlias),
    GlobalAssertStatement(GlobalAssertStatement),
}

impl ModuleDef {
    #[must_use]
    pub const fn as_def_with_body_id(&self) -> Option<DefinitionWithBodyId> {
        match *self {
            Self::Function(function) => Some(DefinitionWithBodyId::Function(function.id)),
            Self::GlobalVariable(variable) => {
                Some(DefinitionWithBodyId::GlobalVariable(variable.id))
            },
            Self::GlobalConstant(constant) => {
                Some(DefinitionWithBodyId::GlobalConstant(constant.id))
            },
            Self::Override(override_declaration) => {
                Some(DefinitionWithBodyId::Override(override_declaration.id))
            },
            Self::GlobalAssertStatement(global_assert_statement) => Some(
                DefinitionWithBodyId::GlobalAssertStatement(global_assert_statement.id),
            ),
            Self::Struct(_) | Self::TypeAlias(_) => None,
        }
    }
}

pub struct Module {
    file_id: HirFileId,
}

impl Module {
    pub fn items(
        &self,
        database: &dyn HirDatabase,
    ) -> Vec<ModuleDef> {
        let item_tree = database.item_tree(self.file_id);
        item_tree
            .items()
            .iter()
            .flat_map(|item| module_item_to_def(database, self.file_id, *item))
            .collect()
    }

    pub fn diagnostics(
        &self,
        database: &dyn HirDatabase,
        config: &DiagnosticsConfig,
        accumulator: &mut Vec<AnyDiagnostic>,
    ) {
        for item in self.items(database) {
            match item {
                ModuleDef::Function(_function) => {},
                ModuleDef::GlobalVariable(variable) => {
                    diagnostics::global_variable::collect(database, variable.id, |error| {
                        if let Some(source) = variable.source(database) {
                            let source = source.map(|declaration| AstPointer::new(&declaration));
                            accumulator.push(diagnostics::any_diag_from_global_var(error, source));
                        }
                    });
                },
                ModuleDef::GlobalConstant(_constant) => {},
                ModuleDef::Override(_constant) => {},
                ModuleDef::GlobalAssertStatement(_global_assert_statement) => {},
                ModuleDef::Struct(strukt) => {
                    let file = strukt.id.lookup(database).file_id;
                    let (_, signature_map) = database.struct_data(strukt.id);
                    let diagnostics = &database.field_types(strukt.id).1;
                    for diagnostic in diagnostics {
                        match diagnostics::any_diag_from_infer_diagnostic(
                            diagnostic,
                            &signature_map,
                            &signature_map,
                            file,
                        ) {
                            Some(diagnostic) => accumulator.push(diagnostic),
                            None => {
                                tracing::warn!("could not create diagnostic from {:?}", diagnostic);
                            },
                        }
                    }
                },
                ModuleDef::TypeAlias(type_alias) => {
                    let file = type_alias.id.lookup(database).file_id;
                    let (_, signature_map) = database.type_alias_data(type_alias.id);
                    let diagnostics = &database.type_alias_type(type_alias.id).1;
                    for diagnostic in diagnostics {
                        match diagnostics::any_diag_from_infer_diagnostic(
                            diagnostic,
                            &signature_map,
                            &signature_map,
                            file,
                        ) {
                            Some(diagnostic) => accumulator.push(diagnostic),
                            None => {
                                tracing::warn!("could not create diagnostic from {:?}", diagnostic);
                            },
                        }
                    }
                },
            }
            if let Some(definition) = item.as_def_with_body_id() {
                let file = definition.file_id(database);
                let (_, signature_map) = database.signature_with_source_map(definition);
                let (_, source_map) = database.body_with_source_map(definition);
                if config.type_errors {
                    let infer = database.infer(definition);
                    for diagnostic in infer.diagnostics() {
                        match diagnostics::any_diag_from_infer_diagnostic(
                            diagnostic,
                            &signature_map,
                            source_map.expression_source_map(),
                            file,
                        ) {
                            Some(diagnostic) => accumulator.push(diagnostic),
                            None => {
                                tracing::warn!("could not create diagnostic from {:?}", diagnostic);
                            },
                        }
                    }
                }

                diagnostics::precedence::collect(database, definition, |diagnostic| {
                    match diagnostics::any_diag_from_shift(
                        &diagnostic,
                        source_map.expression_source_map(),
                        file,
                    ) {
                        Some(diagnostic) => accumulator.push(diagnostic),
                        None => {
                            tracing::warn!("could not create diagnostic from {:?}", diagnostic);
                        },
                    }
                });
            }
        }
    }
}
