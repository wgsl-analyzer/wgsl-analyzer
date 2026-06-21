//! A high-level object-oriented access to code.

pub mod database;
pub mod definition;
pub mod diagnostics;

use base_db::{EditionedFileId, Intern as _, Lookup as _};
use diagnostics::{AnyDiagnostic, DiagnosticsConfig};
use either::Either;
use hir_def::{
    HasSource as _, InFile,
    body::{BindingId, Body, BodySourceMap},
    database::{
        DefDatabase, DefinitionWithBodyId, FunctionId, GlobalAssertStatementId, GlobalConstantId,
        GlobalVariableId, ImportId, Location, OverrideId, StructId, TypeAliasId,
    },
    expression::{ExpressionId, StatementId},
    expression_store::{ExpressionStoreSource, path::Path},
    item_scope::ItemScope,
    item_tree::{self, ItemTree, ModuleItemId, Name},
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

type ExprOrStatement = Either<ast::Expression, ast::Statement>;

/// Nice API on top of the layers below.
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
        file_id: EditionedFileId,
    ) -> ast::SourceFile {
        file_id.parse(self.database).tree()
    }

    #[must_use]
    pub fn analyze(
        &self,
        definition: DefinitionWithBodyId,
    ) -> SourceAnalyzer<'_> {
        SourceAnalyzer::new(self.database, definition)
    }

    /// Finds the root level container for a given node.
    #[must_use]
    pub fn find_container(
        &self,
        file_id: EditionedFileId,
        source: &SyntaxNode,
    ) -> Option<ChildContainer> {
        source
            .ancestors()
            .find_map(|syntax| -> Option<ChildContainer> {
                let item = ast::Item::cast(syntax)?;
                let is_in_body = is_node_in_body(source, &item);

                let container: ChildContainer = match item {
                    ast::Item::ImportStatement(import) => {
                        let definition = self.import_to_def(&InFile::new(file_id, import))?;
                        ChildContainer::ImportId(definition)
                    },
                    ast::Item::FunctionDeclaration(function_declaration) => {
                        let definition =
                            self.function_to_def(&InFile::new(file_id, function_declaration))?;
                        if is_in_body {
                            DefinitionWithBodyId::Function(definition).into()
                        } else {
                            ChildContainer::FunctionId(definition)
                        }
                    },
                    ast::Item::VariableDeclaration(variable_declaration) => {
                        let definition = self
                            .global_variable_to_def(&InFile::new(file_id, variable_declaration))?;
                        if is_in_body {
                            DefinitionWithBodyId::GlobalVariable(definition).into()
                        } else {
                            ChildContainer::GlobalVariableId(definition)
                        }
                    },
                    ast::Item::ConstantDeclaration(constant_declaration) => {
                        let definition = self
                            .global_constant_to_def(&InFile::new(file_id, constant_declaration))?;
                        if is_in_body {
                            DefinitionWithBodyId::GlobalConstant(definition).into()
                        } else {
                            ChildContainer::GlobalConstantId(definition)
                        }
                    },
                    ast::Item::OverrideDeclaration(override_declaration) => {
                        let definition = self
                            .global_override_to_def(&InFile::new(file_id, override_declaration))?;
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
                        let definition =
                            self.global_struct_to_def(&InFile::new(file_id, struct_declaration))?;
                        ChildContainer::StructId(definition)
                    },
                    ast::Item::AssertStatement(assert_statement) => {
                        let definition = self.global_assert_statement_to_def(&InFile::new(
                            file_id,
                            assert_statement,
                        ))?;
                        if is_in_body {
                            DefinitionWithBodyId::GlobalAssertStatement(definition).into()
                        } else {
                            ChildContainer::GlobalAssertStatementId(definition)
                        }
                    },
                };
                Some(container)
            })
    }

    #[must_use]
    pub fn resolver(
        &self,
        file_id: EditionedFileId,
        source: &SyntaxNode,
    ) -> Resolver {
        if let Some(definition) = self.find_container(file_id, source) {
            match definition {
                ChildContainer::DefinitionWithBodyId(
                    id @ DefinitionWithBodyId::Function(function_id),
                ) => {
                    if let Some(nearest_scope) = nearest_scope(source) {
                        self.analyze(id).resolver_for(nearest_scope)
                    } else {
                        id.resolver(self.database)
                    }
                },
                ChildContainer::DefinitionWithBodyId(id) => id.resolver(self.database),
                ChildContainer::ImportId(_)
                | ChildContainer::FunctionId(_)
                | ChildContainer::GlobalVariableId(_)
                | ChildContainer::GlobalConstantId(_)
                | ChildContainer::OverrideId(_)
                | ChildContainer::StructId(_)
                | ChildContainer::GlobalAssertStatementId(_)
                | ChildContainer::TypeAliasId(_) => {
                    let file_id = definition.file_id(self.database);
                    let module_info = ItemScope::of(self.database, file_id);
                    Resolver::new(file_id, module_info)
                },
            }
        } else {
            let module_info = ItemScope::of(self.database, file_id);
            Resolver::new(file_id, module_info)
        }
    }

    #[must_use]
    #[expect(clippy::unused_self, reason = "intentional API")]
    pub const fn module(
        self,
        file_id: EditionedFileId,
    ) -> Module {
        Module { file_id }
    }

    fn import_to_def(
        &self,
        source: &InFile<ast::ImportStatement>,
    ) -> Option<ImportId> {
        let ast_id_map = self.database.ast_id_map(source.file_id);
        let id = ast_id_map.try_ast_id(&source.value)?;
        Some(Location::new(source.file_id, id).intern(self.database))
    }

    fn function_to_def(
        &self,
        source: &InFile<ast::FunctionDeclaration>,
    ) -> Option<FunctionId> {
        let ast_id_map = self.database.ast_id_map(source.file_id);
        let id = ast_id_map.try_ast_id(&source.value)?;
        Some(Location::new(source.file_id, id).intern(self.database))
    }

    fn global_constant_to_def(
        &self,
        source: &InFile<ast::ConstantDeclaration>,
    ) -> Option<GlobalConstantId> {
        let ast_id_map = self.database.ast_id_map(source.file_id);
        let id = ast_id_map.try_ast_id(&source.value)?;
        Some(Location::new(source.file_id, id).intern(self.database))
    }

    fn global_variable_to_def(
        &self,
        source: &InFile<ast::VariableDeclaration>,
    ) -> Option<GlobalVariableId> {
        let ast_id_map = self.database.ast_id_map(source.file_id);
        let id = ast_id_map.try_ast_id(&source.value)?;
        Some(Location::new(source.file_id, id).intern(self.database))
    }

    fn global_override_to_def(
        &self,
        source: &InFile<ast::OverrideDeclaration>,
    ) -> Option<OverrideId> {
        let ast_id_map = self.database.ast_id_map(source.file_id);
        let id = ast_id_map.try_ast_id(&source.value)?;
        Some(Location::new(source.file_id, id).intern(self.database))
    }

    fn global_type_alias_to_def(
        &self,
        source: &InFile<ast::TypeAliasDeclaration>,
    ) -> Option<TypeAliasId> {
        let ast_id_map = self.database.ast_id_map(source.file_id);
        let id = ast_id_map.try_ast_id(&source.value)?;
        Some(Location::new(source.file_id, id).intern(self.database))
    }

    fn global_struct_to_def(
        &self,
        source: &InFile<ast::StructDeclaration>,
    ) -> Option<StructId> {
        let ast_id_map = self.database.ast_id_map(source.file_id);
        let id = ast_id_map.try_ast_id(&source.value)?;
        Some(Location::new(source.file_id, id).intern(self.database))
    }

    fn global_assert_statement_to_def(
        &self,
        source: &InFile<ast::AssertStatement>,
    ) -> Option<GlobalAssertStatementId> {
        let ast_id_map = self.database.ast_id_map(source.file_id);
        let id = ast_id_map.try_ast_id(&source.value)?;
        Some(Location::new(source.file_id, id).intern(self.database))
    }
}

#[must_use]
pub fn nearest_scope(node: &SyntaxNode) -> Option<ExprOrStatement> {
    node.siblings(syntax::Direction::Prev)
        .find_map(|sib| {
            if ExprOrStatement::can_cast(sib.kind()) {
                ExprOrStatement::cast(sib)
            } else {
                None
            }
        })
        .or_else(|| node.ancestors().find_map(ExprOrStatement::cast))
}

fn is_node_in_body(
    node: &SyntaxNode,
    item: &ast::Item,
) -> bool {
    match item {
        ast::Item::FunctionDeclaration(function_declaration) => {
            let child_offset = node.text_range().start();

            function_declaration
                .body()
                .is_some_and(|compound_statement| {
                    compound_statement
                        .syntax()
                        .text_range()
                        .contains(child_offset)
                })
        },
        ast::Item::VariableDeclaration(variable_declaration) => {
            let child_offset = node.text_range().start();

            variable_declaration
                .init()
                .is_some_and(|expression| expression.syntax().text_range().contains(child_offset))
        },
        ast::Item::ConstantDeclaration(constant_declaration) => {
            let child_offset = node.text_range().start();

            constant_declaration
                .init()
                .is_some_and(|expression| expression.syntax().text_range().contains(child_offset))
        },
        ast::Item::OverrideDeclaration(override_declaration) => {
            let child_offset = node.text_range().start();

            override_declaration
                .init()
                .is_some_and(|expression| expression.syntax().text_range().contains(child_offset))
        },
        ast::Item::AssertStatement(assert_statement) => {
            let child_offset = node.text_range().start();
            assert_statement
                .expression()
                .is_some_and(|expression| expression.syntax().text_range().contains(child_offset))
        },
        ast::Item::ImportStatement(_)
        | ast::Item::TypeAliasDeclaration(_)
        | ast::Item::StructDeclaration(_) => false,
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[expect(clippy::enum_variant_names, reason = "Suffix makes sense")]
pub enum ChildContainer {
    /// This variant is for when the expression is inside the body.
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
    ) -> EditionedFileId {
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
    file_id: EditionedFileId,
    module_item: ModuleItemId,
) -> SmallVec<[ModuleDef; 1]> {
    let definition = match module_item {
        ModuleItemId::Function(function) => {
            let location = Location::new(file_id, function);
            let id = database.intern_function(location);
            ModuleDef::Function(Function { id })
        },
        ModuleItemId::Struct(r#struct) => {
            let location = Location::new(file_id, r#struct);
            let id = database.intern_struct(location);
            ModuleDef::Struct(Struct { id })
        },
        ModuleItemId::GlobalVariable(variable) => {
            let location = Location::new(file_id, variable);
            let id = database.intern_global_variable(location);
            ModuleDef::GlobalVariable(GlobalVariable { id })
        },
        ModuleItemId::GlobalConstant(constant) => {
            let location = Location::new(file_id, constant);
            let id = database.intern_global_constant(location);
            ModuleDef::GlobalConstant(GlobalConstant { id })
        },
        ModuleItemId::Override(constant) => {
            let location = Location::new(file_id, constant);
            let id = database.intern_override(location);
            ModuleDef::Override(Override { id })
        },
        ModuleItemId::TypeAlias(type_alias) => {
            let location = Location::new(file_id, type_alias);
            let id = database.intern_type_alias(location);
            ModuleDef::TypeAlias(TypeAlias { id })
        },
        ModuleItemId::GlobalAssertStatement(global_assert_statement) => {
            let location = Location::new(file_id, global_assert_statement);
            let id = database.intern_global_assert_statement(location);
            ModuleDef::GlobalAssertStatement(GlobalAssertStatement { id })
        },
        ModuleItemId::ImportStatement(_) => return smallvec::SmallVec::new(),
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
        scope: ExprOrStatement,
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
        let binding = source_map.binding_to_source(self.binding).ok()?;
        let root = file_id.parse(database).syntax();
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

/// The defs which can be visible in the module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleDef {
    Module(Module),
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
            Self::Module(_) | Self::Struct(_) | Self::TypeAlias(_) => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct Module {
    pub file_id: EditionedFileId,
}

impl HasSource for Module {
    type Ast = ast::SourceFile;

    fn source(
        self,
        database: &dyn DefDatabase,
    ) -> Option<InFile<Self::Ast>> {
        let source_file = self.file_id.parse(database).tree();
        Some(InFile::new(self.file_id, source_file))
    }
}

impl Module {
    pub fn items(
        self,
        database: &dyn HirDatabase,
    ) -> Vec<ModuleDef> {
        let item_tree = database.item_tree(self.file_id);
        item_tree
            .top_level_items()
            .iter()
            .flat_map(|item| module_item_to_def(database, self.file_id, *item))
            .collect()
    }

    pub fn diagnostics(
        self,
        database: &dyn HirDatabase,
        config: &DiagnosticsConfig,
        accumulator: &mut Vec<AnyDiagnostic>,
    ) {
        validate_identifiers(self.file_id, database, accumulator);

        for item in self.items(database) {
            match item {
                ModuleDef::Module(_) | ModuleDef::Function(_) => {},
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
                ModuleDef::Struct(r#struct) => {
                    let file = r#struct.id.lookup(database).file_id;
                    let (_, signature_map) = database.struct_data(r#struct.id);
                    let diagnostics = &database.field_types(r#struct.id).1;
                    for diagnostic in diagnostics {
                        if diagnostic.source != ExpressionStoreSource::Signature {
                            tracing::warn!(
                                "struct diagnostic with an invalid source {:?}",
                                diagnostic
                            );
                            continue;
                        }
                        match diagnostics::any_diag_from_infer_diagnostic(
                            &diagnostic.kind,
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
                        if diagnostic.source != ExpressionStoreSource::Signature {
                            tracing::warn!(
                                "type alias diagnostic with an invalid source {:?}",
                                diagnostic
                            );
                            continue;
                        }
                        match diagnostics::any_diag_from_infer_diagnostic(
                            &diagnostic.kind,
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
            if config.type_errors {
                check_type_errors(database, accumulator, &item);
            }
        }

        for diagnostic in &ItemScope::of(database, self.file_id).diagnostics {
            accumulator.push(diagnostics::any_diag_from_def_diagnostic(
                database,
                diagnostic,
                self.file_id,
            ));
        }
    }
}

#[expect(clippy::doc_paragraphs_missing_punctuation, reason = "clippy bug")]
/// Check for identifiers starting with "__". These are invalid according the WGSL specification.
///
/// See: <https://www.w3.org/TR/WGSL/#identifiers>
fn validate_identifiers(
    file_id: EditionedFileId,
    database: &dyn HirDatabase,
    accumulator: &mut Vec<AnyDiagnostic>,
) {
    let item_tree = database.item_tree(file_id);
    let ast_id_map = database.ast_id_map(file_id);
    let root = file_id.parse(database).syntax();

    macro_rules! validate {
        (
            $id:expr,
            $item_tree:expr,
            $ast_id_map:expr,
            $root:expr,
            $accumulator:expr,
            $file_id:expr
        ) => {{
            let data = &$item_tree[*$id];
            if data.name.as_str().starts_with("__") {
                let ast_ptr = $ast_id_map.get(*$id);
                let node = ast_ptr.to_node(&$root);
                if let Some(name_node) = node.name() {
                    $accumulator.push(AnyDiagnostic::InvalidIdentifier {
                        file_id: $file_id,
                        name: data.name.clone(),
                        range: name_node.syntax().text_range(),
                    });
                }
            }
        }};
    }

    for item in item_tree.top_level_items() {
        match item {
            ModuleItemId::Function(id) => {
                validate!(id, item_tree, ast_id_map, root, accumulator, file_id);
            },
            ModuleItemId::GlobalVariable(id) => {
                validate!(id, item_tree, ast_id_map, root, accumulator, file_id);
            },
            ModuleItemId::GlobalConstant(id) => {
                validate!(id, item_tree, ast_id_map, root, accumulator, file_id);
            },
            ModuleItemId::Override(id) => {
                validate!(id, item_tree, ast_id_map, root, accumulator, file_id);
            },
            ModuleItemId::Struct(id) => {
                validate!(id, item_tree, ast_id_map, root, accumulator, file_id);
            },
            ModuleItemId::TypeAlias(id) => {
                validate!(id, item_tree, ast_id_map, root, accumulator, file_id);
            },
            ModuleItemId::ImportStatement(_) | ModuleItemId::GlobalAssertStatement(_) => {},
        }
    }
}

fn check_type_errors(
    database: &dyn HirDatabase,
    accumulator: &mut Vec<AnyDiagnostic>,
    item: &ModuleDef,
) {
    if let Some(definition) = item.as_def_with_body_id() {
        let file = definition.file_id(database);
        let (_, signature_map) = database.signature_with_source_map(definition);
        let (_, source_map) = database.body_with_source_map(definition);
        let infer = database.infer(definition);
        for diagnostic in infer.diagnostics() {
            match diagnostics::any_diag_from_infer_diagnostic(
                &diagnostic.kind,
                match diagnostic.source {
                    ExpressionStoreSource::Body => source_map.expression_source_map(),
                    ExpressionStoreSource::Signature => &signature_map,
                },
                file,
            ) {
                Some(diagnostic) => accumulator.push(diagnostic),
                None => {
                    tracing::warn!("could not create diagnostic from {:?}", diagnostic);
                },
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
