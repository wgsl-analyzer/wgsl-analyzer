//! A high-level object-oriented access to code.
#![warn(unused)]

pub mod db;
pub mod definition;
pub mod diagnostics;

use base_db::{EditionedFileId, Intern as _, Lookup as _, SourceDatabase};
use diagnostics::AnyDiagnostic;
use either::Either;
use hir_def::{
    AstIdMap, HasSource as _, InFile,
    body::{BindingId, Body, BodySourceMap, scope::ExprScopes},
    db::{
        DefinitionWithBodyId, FunctionId, GlobalAssertStatementId, GlobalConstantId,
        GlobalVariableId, ImportId, Location, OverrideId, StructId, TypeAliasId,
    },
    expression::{ExpressionId, StatementId},
    expression_store::{ExpressionStore, ExpressionStoreOwnerId, ExpressionStoreSource},
    item_scope::ItemScope,
    item_tree::{ItemTree, ModuleItemId},
    resolver::Resolver,
    signature::{FieldId, FunctionSignature, ParameterId, StructSignature, TypeAliasSignature},
};
use hir_ty::{infer::InferenceResult, ty::Type};
use smallvec::SmallVec;
use stdx::impl_from;
use syntax::{AstNode as _, HasName as _, SyntaxNode, ast, pointer::AstPointer};

pub use hir_ty::{AddressSpace, db::HirDatabase};

pub trait HasSource {
    type Ast;
    fn source(
        self,
        db: &dyn SourceDatabase,
    ) -> Option<InFile<Self::Ast>>;
}

type ExprOrStatement = Either<ast::Expression, ast::Statement>;

/// Nice API on top of the layers below.
pub struct Semantics<'db> {
    pub db: &'db dyn HirDatabase,
}

impl<'db> Semantics<'db> {
    pub fn new(db: &'db dyn HirDatabase) -> Self {
        Semantics { db }
    }

    #[must_use]
    pub fn parse(
        &self,
        file_id: EditionedFileId,
    ) -> ast::SourceFile {
        file_id.parse(self.db).tree()
    }

    #[must_use]
    pub fn analyze(
        &self,
        definition: DefinitionWithBodyId,
    ) -> SourceAnalyzer<'db> {
        SourceAnalyzer::new(self.db, definition)
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
                let container = self.item_to_container(file_id, item, is_in_body)?;
                Some(container)
            })
    }

    fn item_to_container(
        &self,
        file_id: EditionedFileId,
        item: ast::Item,
        is_in_body: bool,
    ) -> Option<ChildContainer> {
        let child_container = match item {
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
                let definition =
                    self.global_variable_to_def(&InFile::new(file_id, variable_declaration))?;
                if is_in_body {
                    DefinitionWithBodyId::GlobalVariable(definition).into()
                } else {
                    ChildContainer::GlobalVariableId(definition)
                }
            },
            ast::Item::ConstantDeclaration(constant_declaration) => {
                let definition =
                    self.global_constant_to_def(&InFile::new(file_id, constant_declaration))?;
                if is_in_body {
                    DefinitionWithBodyId::GlobalConstant(definition).into()
                } else {
                    ChildContainer::GlobalConstantId(definition)
                }
            },
            ast::Item::OverrideDeclaration(override_declaration) => {
                let definition =
                    self.global_override_to_def(&InFile::new(file_id, override_declaration))?;
                if is_in_body {
                    DefinitionWithBodyId::Override(definition).into()
                } else {
                    ChildContainer::OverrideId(definition)
                }
            },
            ast::Item::TypeAliasDeclaration(type_alias_declaration) => {
                let definition =
                    self.global_type_alias_to_def(&InFile::new(file_id, type_alias_declaration))?;
                ChildContainer::TypeAliasId(definition)
            },
            ast::Item::StructDeclaration(struct_declaration) => {
                let definition =
                    self.global_struct_to_def(&InFile::new(file_id, struct_declaration))?;
                ChildContainer::StructId(definition)
            },
            ast::Item::AssertStatement(assert_statement) => {
                let definition =
                    self.global_assert_statement_to_def(&InFile::new(file_id, assert_statement))?;
                if is_in_body {
                    DefinitionWithBodyId::GlobalAssertStatement(definition).into()
                } else {
                    ChildContainer::GlobalAssertStatementId(definition)
                }
            },
            ast::Item::GlobalCompoundDeclaration(global_compound_declaration) => {
                global_compound_declaration
                    .items()
                    .find_map(|item| self.item_to_container(file_id, item, is_in_body))?
            },
        };
        Some(child_container)
    }

    #[must_use]
    pub fn resolver(
        &self,
        file_id: EditionedFileId,
        source: &SyntaxNode,
    ) -> Resolver<'db> {
        if let Some(definition) = self.find_container(file_id, source) {
            match definition {
                ChildContainer::DefinitionWithBodyId(id @ DefinitionWithBodyId::Function(_)) => {
                    if let Some(nearest_scope) = nearest_scope(source) {
                        self.analyze(id).resolver_for(nearest_scope)
                    } else {
                        id.resolver(self.db)
                    }
                },
                ChildContainer::DefinitionWithBodyId(id) => id.resolver(self.db),
                ChildContainer::ImportId(_)
                | ChildContainer::FunctionId(_)
                | ChildContainer::GlobalVariableId(_)
                | ChildContainer::GlobalConstantId(_)
                | ChildContainer::OverrideId(_)
                | ChildContainer::StructId(_)
                | ChildContainer::GlobalAssertStatementId(_)
                | ChildContainer::TypeAliasId(_) => {
                    let file_id = definition.file_id(self.db);
                    Resolver::new(self.db, file_id)
                },
            }
        } else {
            Resolver::new(self.db, file_id)
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
        let ast_id_map = AstIdMap::of(self.db, source.file_id);
        let id = ast_id_map.try_ast_id(&source.value)?;
        Some(Location::new(source.file_id, id).intern(self.db))
    }

    fn function_to_def(
        &self,
        source: &InFile<ast::FunctionDeclaration>,
    ) -> Option<FunctionId> {
        let ast_id_map = AstIdMap::of(self.db, source.file_id);
        let id = ast_id_map.try_ast_id(&source.value)?;
        Some(Location::new(source.file_id, id).intern(self.db))
    }

    fn global_constant_to_def(
        &self,
        source: &InFile<ast::ConstantDeclaration>,
    ) -> Option<GlobalConstantId> {
        let ast_id_map = AstIdMap::of(self.db, source.file_id);
        let id = ast_id_map.try_ast_id(&source.value)?;
        Some(Location::new(source.file_id, id).intern(self.db))
    }

    fn global_variable_to_def(
        &self,
        source: &InFile<ast::VariableDeclaration>,
    ) -> Option<GlobalVariableId> {
        let ast_id_map = AstIdMap::of(self.db, source.file_id);
        let id = ast_id_map.try_ast_id(&source.value)?;
        Some(Location::new(source.file_id, id).intern(self.db))
    }

    fn global_override_to_def(
        &self,
        source: &InFile<ast::OverrideDeclaration>,
    ) -> Option<OverrideId> {
        let ast_id_map = AstIdMap::of(self.db, source.file_id);
        let id = ast_id_map.try_ast_id(&source.value)?;
        Some(Location::new(source.file_id, id).intern(self.db))
    }

    fn global_type_alias_to_def(
        &self,
        source: &InFile<ast::TypeAliasDeclaration>,
    ) -> Option<TypeAliasId> {
        let ast_id_map = AstIdMap::of(self.db, source.file_id);
        let id = ast_id_map.try_ast_id(&source.value)?;
        Some(Location::new(source.file_id, id).intern(self.db))
    }

    fn global_struct_to_def(
        &self,
        source: &InFile<ast::StructDeclaration>,
    ) -> Option<StructId> {
        let ast_id_map = AstIdMap::of(self.db, source.file_id);
        let id = ast_id_map.try_ast_id(&source.value)?;
        Some(Location::new(source.file_id, id).intern(self.db))
    }

    fn global_assert_statement_to_def(
        &self,
        source: &InFile<ast::AssertStatement>,
    ) -> Option<GlobalAssertStatementId> {
        let ast_id_map = AstIdMap::of(self.db, source.file_id);
        let id = ast_id_map.try_ast_id(&source.value)?;
        Some(Location::new(source.file_id, id).intern(self.db))
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
    let child_offset = node.text_range().start();
    match item {
        ast::Item::FunctionDeclaration(function_declaration) => function_declaration
            .body()
            .is_some_and(|compound_statement| {
                compound_statement
                    .syntax()
                    .text_range()
                    .contains(child_offset)
            }),
        ast::Item::VariableDeclaration(variable_declaration) => variable_declaration
            .init()
            .is_some_and(|expression| expression.syntax().text_range().contains(child_offset)),
        ast::Item::ConstantDeclaration(constant_declaration) => constant_declaration
            .init()
            .is_some_and(|expression| expression.syntax().text_range().contains(child_offset)),
        ast::Item::OverrideDeclaration(override_declaration) => override_declaration
            .init()
            .is_some_and(|expression| expression.syntax().text_range().contains(child_offset)),
        ast::Item::AssertStatement(assert_statement) => assert_statement
            .expression()
            .is_some_and(|expression| expression.syntax().text_range().contains(child_offset)),
        ast::Item::ImportStatement(_)
        | ast::Item::TypeAliasDeclaration(_)
        | ast::Item::StructDeclaration(_) => false,
        ast::Item::GlobalCompoundDeclaration(global_compound_declaration) => {
            global_compound_declaration
                .items()
                .any(|item| is_node_in_body(node, &item))
        },
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
        db: &dyn SourceDatabase,
    ) -> EditionedFileId {
        match self {
            Self::DefinitionWithBodyId(id) => id.file_id(db),
            Self::ImportId(id) => id.lookup(db).file_id,
            Self::FunctionId(id) => id.lookup(db).file_id,
            Self::GlobalVariableId(id) => id.lookup(db).file_id,
            Self::GlobalConstantId(id) => id.lookup(db).file_id,
            Self::OverrideId(id) => id.lookup(db).file_id,
            Self::StructId(id) => id.lookup(db).file_id,
            Self::TypeAliasId(id) => id.lookup(db).file_id,
            Self::GlobalAssertStatementId(id) => id.lookup(db).file_id,
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
    db: &dyn HirDatabase,
    file_id: EditionedFileId,
    module_item: ModuleItemId,
) -> SmallVec<[ModuleDef; 1]> {
    let definition = match module_item {
        ModuleItemId::Function(function) => {
            let id = Location::new(file_id, function).intern(db);
            ModuleDef::Function(Function { id })
        },
        ModuleItemId::Struct(r#struct) => {
            let id = Location::new(file_id, r#struct).intern(db);
            ModuleDef::Struct(Struct { id })
        },
        ModuleItemId::GlobalVariable(variable) => {
            let id = Location::new(file_id, variable).intern(db);
            ModuleDef::GlobalVariable(GlobalVariable { id })
        },
        ModuleItemId::GlobalConstant(constant) => {
            let id = Location::new(file_id, constant).intern(db);
            ModuleDef::GlobalConstant(GlobalConstant { id })
        },
        ModuleItemId::Override(constant) => {
            let id = Location::new(file_id, constant).intern(db);
            ModuleDef::Override(Override { id })
        },
        ModuleItemId::TypeAlias(type_alias) => {
            let id = Location::new(file_id, type_alias).intern(db);
            ModuleDef::TypeAlias(TypeAlias { id })
        },
        ModuleItemId::GlobalAssertStatement(global_assert_statement) => {
            let id = Location::new(file_id, global_assert_statement).intern(db);
            ModuleDef::GlobalAssertStatement(GlobalAssertStatement { id })
        },
        ModuleItemId::ImportStatement(_) => return smallvec::SmallVec::new(),
    };
    smallvec::smallvec![definition]
}

pub struct SourceAnalyzer<'db> {
    pub db: &'db dyn HirDatabase,
    pub body: &'db Body,
    pub body_source_map: &'db BodySourceMap,
    pub infer: &'db InferenceResult,
    pub owner: DefinitionWithBodyId,
}

impl<'db> SourceAnalyzer<'db> {
    fn new(
        db: &'db dyn HirDatabase,
        definition: DefinitionWithBodyId,
    ) -> Self {
        let (body, body_source_map) = Body::with_source_map(db, definition);
        let infer = InferenceResult::of(db, definition);
        Self {
            db,
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
    ) -> Resolver<'db> {
        let mut resolver = self.owner.resolver(self.db);

        let expression_scopes = ExprScopes::of(self.db, self.owner);

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
        db: &dyn SourceDatabase,
    ) -> Option<InFile<Self::Ast>> {
        let file_id = self.parent.lookup(db).file_id;
        let (_, source_map) =
            Body::with_source_map(db, DefinitionWithBodyId::Function(self.parent));
        let binding = source_map.binding_to_source(self.binding).ok()?;
        let root = file_id.parse(db).syntax();
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
        db: &dyn SourceDatabase,
    ) -> Option<InFile<Self::Ast>> {
        let function_data = FunctionSignature::of(db, self.id.function);
        let parameter_data = &function_data.parameters[self.id.param];
        let parameter_name = &parameter_data.name;

        let function = self.id.function.lookup(db).source(db);

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
        db: &dyn SourceDatabase,
    ) -> Option<InFile<Self::Ast>> {
        Some(self.id.lookup(db).source(db))
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
        db: &dyn SourceDatabase,
    ) -> Option<InFile<Self::Ast>> {
        Some(self.id.lookup(db).source(db))
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
        db: &dyn SourceDatabase,
    ) -> Option<InFile<Self::Ast>> {
        Some(self.id.lookup(db).source(db))
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
        db: &dyn SourceDatabase,
    ) -> Option<InFile<Self::Ast>> {
        Some(self.id.lookup(db).source(db))
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
        db: &dyn SourceDatabase,
    ) -> Option<InFile<Self::Ast>> {
        Some(self.id.lookup(db).source(db))
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
        db: &dyn SourceDatabase,
    ) -> Option<InFile<Self::Ast>> {
        Some(self.id.lookup(db).source(db))
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
        db: &dyn SourceDatabase,
    ) -> Option<InFile<Self::Ast>> {
        Some(self.id.lookup(db).source(db))
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
        db: &dyn SourceDatabase,
    ) -> Option<InFile<Self::Ast>> {
        let struct_data = StructSignature::of(db, self.id.r#struct);
        let field_data = &struct_data.fields()[self.id.field];
        let field_name = &field_data.name;

        let r#struct = self.id.r#struct.lookup(db).source(db);

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

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct Module {
    pub file_id: EditionedFileId,
}

impl HasSource for Module {
    type Ast = ast::SourceFile;

    fn source(
        self,
        db: &dyn SourceDatabase,
    ) -> Option<InFile<Self::Ast>> {
        let source_file = self.file_id.parse(db).tree();
        Some(InFile::new(self.file_id, source_file))
    }
}

impl Module {
    pub fn items(
        self,
        db: &dyn HirDatabase,
    ) -> Vec<ModuleDef> {
        let item_tree = ItemTree::of(db, self.file_id);
        item_tree
            .top_level_items()
            .iter()
            .flat_map(|item| module_item_to_def(db, self.file_id, *item))
            .collect()
    }

    pub fn semantic_diagnostics(
        self,
        db: &dyn HirDatabase,
        accumulator: &mut Vec<AnyDiagnostic>,
    ) {
        validate_identifiers(self.file_id, db, accumulator);

        for item in self.items(db) {
            match item {
                ModuleDef::Function(_) => {},
                ModuleDef::GlobalVariable(variable) => {
                    diagnostics::global_variable::collect(db, variable.id, |error| {
                        if let Some(source) = variable.source(db) {
                            let source = source.map(|declaration| AstPointer::new(&declaration));
                            accumulator.push(diagnostics::any_diag_from_global_var(error, source));
                        }
                    });
                },
                ModuleDef::GlobalConstant(_constant) => {},
                ModuleDef::Override(_constant) => {},
                ModuleDef::GlobalAssertStatement(_global_assert_statement) => {},
                ModuleDef::Struct(r#struct) => {
                    let file = r#struct.id.lookup(db).file_id;
                    let (_, signature_map) = StructSignature::with_source_map(db, r#struct.id);
                    let (_, diagnostics) = &*db.field_types(r#struct.id);
                    for diagnostic in diagnostics {
                        if diagnostic.source != ExpressionStoreSource::Signature {
                            tracing::warn!(
                                "struct diagnostic with an invalid source {:?}",
                                diagnostic
                            );
                            continue;
                        }
                        match diagnostics::to_any_diagnostic(&diagnostic.kind, signature_map, file)
                        {
                            Some(diagnostic) => accumulator.push(diagnostic),
                            None => {
                                tracing::warn!("could not create diagnostic from {:?}", diagnostic);
                            },
                        }
                    }
                },
                ModuleDef::TypeAlias(type_alias) => {
                    let file = type_alias.id.lookup(db).file_id;
                    let (_, signature_map) = TypeAliasSignature::with_source_map(db, type_alias.id);
                    let diagnostics = &db.type_alias_type(type_alias.id).1;
                    for diagnostic in diagnostics {
                        if diagnostic.source != ExpressionStoreSource::Signature {
                            tracing::warn!(
                                "type alias diagnostic with an invalid source {:?}",
                                diagnostic
                            );
                            continue;
                        }
                        match diagnostics::to_any_diagnostic(&diagnostic.kind, signature_map, file)
                        {
                            Some(diagnostic) => accumulator.push(diagnostic),
                            None => {
                                tracing::warn!("could not create diagnostic from {:?}", diagnostic);
                            },
                        }
                    }
                },
            }
            check_type_errors(db, accumulator, &item);
        }

        for diagnostic in &ItemScope::of(db, self.file_id).diagnostics {
            accumulator.push(diagnostics::any_diag_from_def_diagnostic(db, diagnostic));
        }
    }
}

#[expect(clippy::doc_paragraphs_missing_punctuation, reason = "clippy bug")]
/// Check for identifiers starting with "__". These are invalid according the WGSL specification.
///
/// See: <https://www.w3.org/TR/WGSL/#identifiers>
fn validate_identifiers(
    file_id: EditionedFileId,
    db: &dyn HirDatabase,
    accumulator: &mut Vec<AnyDiagnostic>,
) {
    let item_tree = ItemTree::of(db, file_id);
    let ast_id_map = AstIdMap::of(db, file_id);
    let root = file_id.parse(db).syntax();

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
    db: &dyn HirDatabase,
    accumulator: &mut Vec<AnyDiagnostic>,
    item: &ModuleDef,
) {
    if let Some(definition) = item.as_def_with_body_id() {
        let file = definition.file_id(db);
        let (_, signature_map) =
            ExpressionStore::with_source_map(db, ExpressionStoreOwnerId::Signature(definition));
        let (_, source_map) = Body::with_source_map(db, definition);
        let infer = InferenceResult::of(db, definition);
        for diagnostic in infer.diagnostics() {
            match diagnostics::to_any_diagnostic(
                &diagnostic.kind,
                match diagnostic.source {
                    ExpressionStoreSource::Body => source_map.expression_source_map(),
                    ExpressionStoreSource::Signature => signature_map,
                },
                file,
            ) {
                Some(diagnostic) => accumulator.push(diagnostic),
                None => {
                    tracing::warn!("could not create diagnostic from {:?}", diagnostic);
                },
            }
        }

        diagnostics::precedence::collect(db, definition, |diagnostic| {
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

pub use hir_ty::setup_tracing;
