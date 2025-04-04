pub mod definition;
pub mod diagnostics;

use std::sync::Arc;

use base_db::FileId;
use definition::Definition;
use diagnostics::{AnyDiagnostic, DiagnosticsConfig};
use either::Either;
use hir_def::{
    HasSource as _, HirFileId, InFile,
    body::{BindingId, Body, BodySourceMap},
    data::FieldId,
    db::{
        DefDatabase, DefinitionWithBodyId, FunctionId, GlobalConstantId, GlobalVariableId,
        ImportId, Location, Lookup, OverrideId, StructId, TypeAliasId,
    },
    expression::{ExpressionId, StatementId},
    hir_file_id::{ImportFile, relative_file},
    module_data::{self, ImportValue, ModuleInfo, ModuleItem, Name},
    resolver::{ResolveValue, Resolver},
};
pub use hir_ty::db::HirDatabase;
use hir_ty::{infer::InferenceResult, ty::Type};
use smallvec::SmallVec;
use syntax::{AstNode, HasName, SyntaxNode, ast, match_ast, pointer::AstPointer};

pub trait HasSource {
    type Ast;
    fn source(
        self,
        db: &dyn DefDatabase,
    ) -> Option<InFile<Self::Ast>>;
}

/// Nice API on top of the layers below
pub struct Semantics<'db> {
    pub db: &'db dyn HirDatabase,
}

impl<'db> Semantics<'db> {
    pub fn new(db: &'db dyn HirDatabase) -> Semantics<'db> {
        Semantics { db }
    }

    pub fn parse(
        &self,
        file_id: FileId,
    ) -> ast::SourceFile {
        self.db.parse(file_id).tree()
    }

    pub fn analyze(
        &self,
        def: DefinitionWithBodyId,
    ) -> SourceAnalyzer {
        SourceAnalyzer::new(self.db, def)
    }

    pub fn find_container(
        &self,
        file_id: HirFileId,
        source: &SyntaxNode,
    ) -> Option<DefinitionWithBodyId> {
        source.ancestors().find_map(|syntax| {
            match_ast! {
                match syntax {
                    ast::Function(function) => self.function_to_def(InFile::new(file_id, function)).map(DefinitionWithBodyId::Function),
                    ast::GlobalVariableDeclaration(var) => self.global_variable_to_def(InFile::new(file_id, var)).map(DefinitionWithBodyId::GlobalVariable),
                    ast::GlobalConstantDeclaration(constant) => self.global_constant_to_def(InFile::new(file_id, constant)).map(DefinitionWithBodyId::GlobalConstant),
                    _ => None,
                }
            }
        })
    }

    pub fn resolver(
        &self,
        file_id: HirFileId,
        source: &SyntaxNode,
    ) -> Resolver {
        match self.find_container(file_id, source) {
            Some(def) => def.resolver(self.db.upcast()),
            None => {
                let module_info = self.db.module_info(file_id);
                Resolver::default().push_module_scope(self.db.upcast(), file_id, module_info)
            },
        }
    }

    pub fn module(
        &self,
        file_id: FileId,
    ) -> Module {
        Module {
            file_id: file_id.into(),
        }
    }

    fn resolve_name_in_expression_scope(
        &self,
        def: DefinitionWithBodyId,
        expression: &SyntaxNode,
        name: Name,
    ) -> Option<Definition> {
        let file_id = def.file_id(self.db.upcast());
        let module_info = self.db.module_info(file_id);
        let expression_scopes = self.db.expression_scopes(def);
        let (_, source_map) = self.db.body_with_source_map(def);
        let expression_id = source_map.lookup_expression(&AstPointer::new(
            &ast::Expression::cast(expression.clone())?,
        ))?;
        let scope_id = expression_scopes.scope_for_expression(expression_id)?;
        let mut resolver =
            Resolver::default().push_module_scope(self.db.upcast(), file_id, module_info);

        if let DefinitionWithBodyId::Function(function) = def {
            resolver = resolver.push_expression_scope(function, expression_scopes, scope_id);
        };

        let value = resolver.resolve_value(&name)?;

        let def = match value {
            ResolveValue::Local(binding) => Definition::Local(Local {
                parent: resolver.body_owner()?,
                binding,
            }),
            ResolveValue::GlobalVariable(loc) => {
                let id = self.db.intern_global_variable(loc);
                Definition::ModuleDef(ModuleDef::GlobalVariable(GlobalVariable { id }))
            },
            ResolveValue::GlobalConstant(loc) => {
                let id = self.db.intern_global_constant(loc);
                Definition::ModuleDef(ModuleDef::GlobalConstant(GlobalConstant { id }))
            },
            ResolveValue::Override(loc) => {
                let id = self.db.intern_override(loc);
                Definition::ModuleDef(ModuleDef::Override(Override { id }))
            },
        };

        Some(def)
    }

    fn function_to_def(
        &self,
        source: InFile<ast::Function>,
    ) -> Option<FunctionId> {
        let function = module_data::find_item(self.db.upcast(), source.file_id, &source.value)?;
        let function_id = self
            .db
            .intern_function(Location::new(source.file_id, function));
        Some(function_id)
    }

    fn global_constant_to_def(
        &self,
        source: InFile<ast::GlobalConstantDeclaration>,
    ) -> Option<GlobalConstantId> {
        let global_constant =
            module_data::find_item(self.db.upcast(), source.file_id, &source.value)?;
        let id = self
            .db
            .intern_global_constant(Location::new(source.file_id, global_constant));
        Some(id)
    }

    fn global_variable_to_def(
        &self,
        source: InFile<ast::GlobalVariableDeclaration>,
    ) -> Option<GlobalVariableId> {
        let global_variable =
            module_data::find_item(self.db.upcast(), source.file_id, &source.value)?;
        let id = self
            .db
            .intern_global_variable(Location::new(source.file_id, global_variable));
        Some(id)
    }

    pub fn import_to_def(
        &self,
        source: InFile<ast::Import>,
    ) -> Option<ImportId> {
        let import = module_data::find_import(self.db.upcast(), source.file_id, &source.value)?;

        let import_id = self.db.intern_import(Location::new(source.file_id, import));
        Some(import_id)
    }

    pub fn resolve_import(
        &self,
        source: InFile<ast::Import>,
    ) -> Option<Import> {
        let id = self.import_to_def(source)?;
        Some(Import { id })
    }
}

fn module_item_to_def(
    db: &dyn HirDatabase,
    file_id: HirFileId,
    module_item: &ModuleItem,
) -> SmallVec<[ModuleDef; 1]> {
    let def = match *module_item {
        ModuleItem::Function(func) => {
            let loc = Location::new(file_id, func);
            let id = db.intern_function(loc);
            ModuleDef::Function(Function { id })
        },
        ModuleItem::Struct(r#struct) => {
            let loc = Location::new(file_id, r#struct);
            let id = db.intern_struct(loc);
            ModuleDef::Struct(Struct { id })
        },
        ModuleItem::GlobalVariable(var) => {
            let loc = Location::new(file_id, var);
            let id = db.intern_global_variable(loc);
            ModuleDef::GlobalVariable(GlobalVariable { id })
        },
        ModuleItem::GlobalConstant(constant) => {
            let loc = Location::new(file_id, constant);
            let id = db.intern_global_constant(loc);
            ModuleDef::GlobalConstant(GlobalConstant { id })
        },
        ModuleItem::Override(constant) => {
            let loc = Location::new(file_id, constant);
            let id = db.intern_override(loc);
            ModuleDef::Override(Override { id })
        },
        ModuleItem::Import(import) => {
            let loc = Location::new(file_id, import);
            let import_id = db.intern_import(loc);

            let import_file = HirFileId::from(ImportFile { import_id });
            // Process imported definitions from the original file if available
            if let Some(original_file) = import_file.original_file(db.upcast()) {
                let original_file = original_file.into();
                let module_info = db.module_info(original_file);
                return module_info
                    .items()
                    .iter()
                    .flat_map(|item| module_item_to_def(db, original_file, item))
                    .collect();
            } else {
                // For custom imports without a direct file, use the import file's module info
                let module_info = db.module_info(import_file);
                return module_info
                    .items()
                    .iter()
                    .flat_map(|item| module_item_to_def(db, import_file, item))
                    .collect();
            }
        },
        ModuleItem::TypeAlias(type_alias) => {
            let loc = Location::new(file_id, type_alias);
            let id = db.intern_type_alias(loc);
            ModuleDef::TypeAlias(TypeAlias { id })
        },
    };
    smallvec::smallvec![def]
}

pub struct SourceAnalyzer<'db> {
    pub db: &'db dyn HirDatabase,
    pub body: Arc<Body>,
    pub body_source_map: Arc<BodySourceMap>,
    pub infer: Arc<InferenceResult>,
    pub owner: DefinitionWithBodyId,
}

impl<'db> SourceAnalyzer<'db> {
    fn new(
        db: &'db dyn HirDatabase,
        def: DefinitionWithBodyId,
    ) -> Self {
        let (body, body_source_map) = db.body_with_source_map(def);
        let infer = db.infer(def);
        Self {
            db,
            body,
            body_source_map,
            infer,
            owner: def,
        }
    }

    pub fn type_of_expression(
        &self,
        expression: &ast::Expression,
    ) -> Option<Type> {
        let id = self.expression_id(expression)?;
        let r#type = *self.infer.type_of_expression.get(id)?;
        Some(r#type)
    }

    pub fn type_of_binding(
        &self,
        binding: &ast::Binding,
    ) -> Option<Type> {
        let id = self.binding_id(binding)?;
        let r#type = *self.infer.type_of_binding.get(id)?;
        Some(r#type)
    }

    pub fn resolve_field(
        &self,
        field: ast::FieldExpression,
    ) -> Option<Field> {
        let expression = self.expression_id(&ast::Expression::FieldExpression(field))?;
        let field = self.infer.field_resolution(expression)?;

        Some(Field { id: field })
    }

    pub fn resolver_for(
        &self,
        scope: Either<ast::Expression, ast::Statement>,
    ) -> Resolver {
        let mut resolver = self.owner.resolver(self.db.upcast());

        let expression_scopes = self.db.expression_scopes(self.owner);

        let scope_id = scope
            .map_left(|expression| {
                let id = self.expression_id(&expression)?;
                expression_scopes.scope_for_expression(id)
            })
            .map_right(|statement| {
                let id = self.statement_id(&statement)?;
                if let Some(Either::Left(root)) = self.body.root {
                    if root == id {
                        return expression_scopes.scope_for_statement(id);
                    }
                }
                expression_scopes.scope_for_statement(id)
            })
            .into_inner();
        let scope_id = match scope_id {
            Some(scope_id) => scope_id,
            None => return resolver,
        };

        if let DefinitionWithBodyId::Function(function) = self.owner {
            resolver = resolver.push_expression_scope(function, expression_scopes, scope_id);
        }

        resolver
    }

    pub fn binding_id(
        &self,
        source: &ast::Binding,
    ) -> Option<BindingId> {
        self.body_source_map
            .lookup_binding(&AstPointer::new(source))
    }

    pub fn expression_id(
        &self,
        source: &ast::Expression,
    ) -> Option<ExpressionId> {
        self.body_source_map
            .lookup_expression(&AstPointer::new(source))
    }

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
    type Ast = ast::Binding;

    fn source(
        self,
        db: &dyn DefDatabase,
    ) -> Option<InFile<Self::Ast>> {
        let file_id = self.parent.lookup(db).file_id;
        let (_, source_map) = db.body_with_source_map(DefinitionWithBodyId::Function(self.parent));
        let binding = source_map
            .binding_to_source(self.binding)
            .map_err(drop)
            .ok()?;

        let root = db.parse_or_resolve(file_id).unwrap().syntax();
        Some(InFile::new(file_id, binding.to_node(&root)))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct Function {
    id: FunctionId,
}

impl HasSource for Function {
    type Ast = ast::Function;

    fn source(
        self,
        db: &dyn DefDatabase,
    ) -> Option<InFile<Self::Ast>> {
        Some(self.id.lookup(db).source(db))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct GlobalVariable {
    id: GlobalVariableId,
}

impl HasSource for GlobalVariable {
    type Ast = ast::GlobalVariableDeclaration;

    fn source(
        self,
        db: &dyn DefDatabase,
    ) -> Option<InFile<Self::Ast>> {
        Some(self.id.lookup(db).source(db))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct GlobalConstant {
    id: GlobalConstantId,
}

impl HasSource for GlobalConstant {
    type Ast = ast::GlobalConstantDeclaration;

    fn source(
        self,
        db: &dyn DefDatabase,
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
        db: &dyn DefDatabase,
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
        db: &dyn DefDatabase,
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
        db: &dyn DefDatabase,
    ) -> Option<InFile<Self::Ast>> {
        Some(self.id.lookup(db).source(db))
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Field {
    pub id: FieldId,
}

impl HasSource for Field {
    type Ast = ast::StructDeclarationField;

    fn source(
        self,
        db: &dyn DefDatabase,
    ) -> Option<InFile<Self::Ast>> {
        let struct_data = db.struct_data(self.id.r#struct);
        let field_data = &struct_data.fields()[self.id.field];
        let field_name = &field_data.name;

        let r#struct = self.id.r#struct.lookup(db).source(db);

        let field = r#struct.value.body()?.fields().find_map(|field| {
            let name = field.variable_ident_declaration()?.binding()?.name()?;
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
}

impl ModuleDef {
    pub fn as_def_with_body_id(&self) -> Option<DefinitionWithBodyId> {
        match *self {
            ModuleDef::Function(function) => Some(DefinitionWithBodyId::Function(function.id)),
            ModuleDef::GlobalVariable(var) => Some(DefinitionWithBodyId::GlobalVariable(var.id)),
            ModuleDef::GlobalConstant(constant) => {
                Some(DefinitionWithBodyId::GlobalConstant(constant.id))
            },
            ModuleDef::Override(override_declaration) => {
                Some(DefinitionWithBodyId::Override(override_declaration.id))
            },
            ModuleDef::Struct(_) => None,
            ModuleDef::TypeAlias(_) => None, // TODO: ?
        }
    }
}

pub struct Module {
    file_id: HirFileId,
}

impl Module {
    pub fn items(
        &self,
        db: &dyn HirDatabase,
    ) -> Vec<ModuleDef> {
        let module_info = db.module_info(self.file_id);
        module_info
            .items()
            .iter()
            .flat_map(|item| module_item_to_def(db, self.file_id, item))
            .collect()
    }

    pub fn imports(
        &self,
        db: &dyn HirDatabase,
    ) -> Vec<Import> {
        let module_info = db.module_info(self.file_id);
        module_info
            .items()
            .iter()
            .flat_map(|item| match item {
                ModuleItem::Import(import) => Some(import),
                _ => None,
            })
            .map(|id| {
                let id = db.intern_import(Location::new(self.file_id, *id));
                Import { id }
            })
            .collect()
    }

    pub fn module_info(
        &self,
        db: &dyn HirDatabase,
    ) -> Arc<ModuleInfo> {
        db.module_info(self.file_id)
    }

    pub fn diagnostics(
        &self,
        db: &dyn HirDatabase,
        config: &DiagnosticsConfig,
        accumulator: &mut Vec<AnyDiagnostic>,
    ) {
        for import in self.imports(db) {
            if import.resolve(db).is_err() {
                let import_loc = import.id.lookup(db.upcast());

                let module_info = self.module_info(db);
                let def_map = db.ast_id_map(import_loc.file_id);

                let source = import_loc.map(|id| def_map.get(module_info.get(id).ast_id));

                accumulator.push(AnyDiagnostic::UnresolvedImport { import: source })
            }
        }
        for item in self.items(db) {
            match item {
                ModuleDef::Function(_function) => {},
                ModuleDef::GlobalVariable(var) => {
                    diagnostics::global_variable::collect(db, var.id, |error| {
                        if let Some(source) = var.source(db.upcast()) {
                            let source = source.map(|declaration| AstPointer::new(&declaration));
                            accumulator.push(diagnostics::any_diag_from_global_var(error, source));
                        }
                    });
                },
                ModuleDef::GlobalConstant(_constant) => {},
                ModuleDef::Override(_constant) => {},
                ModuleDef::Struct(_strukt) => {},
                ModuleDef::TypeAlias(_type_alias) => {},
            }
            if let Some(def) = item.as_def_with_body_id() {
                let file = def.file_id(db.upcast());
                let (_, source_map) = db.body_with_source_map(def);
                if config.type_errors {
                    let infer = db.infer(def);
                    for diagnostic in &infer.diagnostics {
                        match diagnostics::any_diag_from_infer_diagnostic(
                            db,
                            diagnostic,
                            &source_map,
                            file,
                        ) {
                            Some(diagnostic) => accumulator.push(diagnostic),
                            None => {
                                tracing::warn!("could not create diagnostic from {:?}", diagnostic)
                            },
                        }
                    }
                }

                diagnostics::precedence::collect(db, def, |diagnostic| {
                    match diagnostics::any_diag_from_shift(&diagnostic, &source_map, file) {
                        Some(diagnostic) => accumulator.push(diagnostic),
                        None => {
                            tracing::warn!("could not create diagnostic from {:?}", diagnostic)
                        },
                    }
                });
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct Import {
    id: ImportId,
}

impl Import {
    pub fn is_path(
        &self,
        db: &dyn HirDatabase,
    ) -> bool {
        let import_loc = self.id.lookup(db.upcast());
        let module_info = db.module_info(import_loc.file_id);
        let import = module_info.get(import_loc.value);
        matches!(import.value, ImportValue::Path(_))
    }

    pub fn file_text(
        &self,
        db: &dyn HirDatabase,
    ) -> Option<String> {
        let import_loc = self.id.lookup(db.upcast());

        let module_info = db.module_info(import_loc.file_id);
        let import = module_info.get(import_loc.value);

        match &import.value {
            ImportValue::Path(path) => {
                let file_id = relative_file(db.upcast(), import_loc.file_id, path)?;
                Some(db.file_text(file_id).to_string())
            },
            ImportValue::Custom(key) => {
                let imports = db.custom_imports();
                let source = imports.get(key)?;
                Some(source.clone())
            },
        }
    }

    #[allow(clippy::result_unit_err)]
    pub fn resolve(
        &self,
        db: &dyn HirDatabase,
    ) -> Result<(), ()> {
        let import_loc = self.id.lookup(db.upcast());

        let module_info = db.module_info(import_loc.file_id);
        let import = module_info.get(import_loc.value);

        tracing::info!("resolving import {:?}", import);

        match &import.value {
            ImportValue::Path(path) => {
                relative_file(db.upcast(), import_loc.file_id, path).ok_or(())?;
                Ok(())
            },
            ImportValue::Custom(key) => {
                let imports = db.custom_imports();
                if imports.contains_key(key) {
                    Ok(())
                } else {
                    Err(())
                }
            },
        }
    }
}
