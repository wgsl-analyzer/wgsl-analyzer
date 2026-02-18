use std::ops::ControlFlow;

use base_db::EditionedFileId;
use triomphe::Arc;

use crate::{
    HirFileId, InFile,
    body::{
        BindingId,
        scope::{ExprScopes, ScopeId},
    },
    database::{DefDatabase, FunctionId, Location, ModuleDefinitionId},
    expression_store::path::Path,
    item_tree::{
        Function, GlobalConstant, GlobalVariable, ItemTree, ModuleItem, Name, Override, Struct,
        TypeAlias,
    },
    nameres::DefMap,
};

#[derive(Clone)]
pub enum Scope {
    /// The items inside a module.
    Module(ModuleScope),
    /// Local bindings.
    Expression(ExpressionScope),

    Builtin,
}

#[derive(Clone)]
pub struct ModuleScope {
    def_map: Arc<DefMap>,
    file_id: HirFileId,
}

#[derive(Clone)]
pub struct ExpressionScope {
    owner: FunctionId,
    expression_scopes: Arc<ExprScopes>,
    scope_id: ScopeId,
}

#[derive(Debug)]
pub enum ResolveKind {
    Local(BindingId),
    Struct(Location<Struct>),
    TypeAlias(Location<TypeAlias>),
    GlobalVariable(Location<GlobalVariable>),
    GlobalConstant(Location<GlobalConstant>),
    Override(Location<Override>),
    Function(Location<Function>),
    Module(HirFileId),
}

pub enum ScopeDef {
    Local(BindingId),
    ModuleDefinition(ModuleDefinitionId),
}

#[derive(Clone)]
pub struct Resolver {
    scopes: Vec<Scope>,
}

impl Resolver {
    pub fn new(
        file_id: HirFileId,
        def_map: Arc<DefMap>,
    ) -> Self {
        Self {
            scopes: vec![
                Scope::Builtin,
                Scope::Module(ModuleScope { def_map, file_id }),
            ],
        }
    }

    pub fn unsafe_new_without_module() -> Self {
        Self {
            scopes: vec![Scope::Builtin],
        }
    }

    #[must_use]
    pub fn push_scope(
        mut self,
        scope: Scope,
    ) -> Self {
        self.scopes.push(scope);
        self
    }

    #[must_use]
    pub fn push_module_scope(
        mut self,
        file_id: HirFileId,
        def_map: Option<Arc<DefMap>>,
    ) -> Self {
        if let Some(def_map) = def_map {
            self.scopes
                .push(Scope::Module(ModuleScope { def_map, file_id }));
        } else {
            tracing::warn!("missing def map");
        }
        self
    }

    #[must_use]
    pub fn push_expression_scope(
        mut self,
        owner: FunctionId,
        expression_scopes: Arc<ExprScopes>,
        scope_id: ScopeId,
    ) -> Self {
        self.scopes.push(Scope::Expression(ExpressionScope {
            owner,
            expression_scopes,
            scope_id,
        }));
        self
    }

    pub fn scopes(&self) -> impl Iterator<Item = &Scope> {
        self.scopes.iter().rev()
    }

    #[must_use]
    pub fn body_owner(&self) -> Option<FunctionId> {
        self.scopes().find_map(|scope| match scope {
            Scope::Expression(scope) => Some(scope.owner),
            Scope::Module(_) | Scope::Builtin => None,
        })
    }

    /// Calls the passed closure `function` on all names in scope.
    pub fn process_all_names<Function: FnMut(&Name, ScopeDef)>(
        &self,
        database: &dyn DefDatabase,
        mut function: Function,
    ) {
        self.scopes().for_each(|scope| match scope {
            Scope::Module(scope) => {
                scope.def_map[scope.file_id.original_file(database).file_id]
                    .scope
                    .items()
                    .for_each(|(name, item)| function(name, ScopeDef::ModuleDefinition(*item)));
            },
            Scope::Expression(expression_scope) => {
                expression_scope
                    .expression_scopes
                    .scope_chain(Some(expression_scope.scope_id))
                    .for_each(|id| {
                        let data = &expression_scope.expression_scopes[id];
                        data.entries.iter().for_each(|entry| {
                            function(&entry.name, ScopeDef::Local(entry.binding));
                        });
                    });
            },
            Scope::Builtin => {
                // TODO: Match against "name.as_str()" and then point at a "builtin" file
                // See: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/559
            },
        });
    }

    // Corresponds to `resolve_path_in_type_ns` in rust-analyzer.
    #[must_use]
    pub fn resolve(
        &self,
        database: &dyn DefDatabase,
        path: &Path,
    ) -> Option<ResolveKind> {
        self.scopes().find_map(|scope| match scope {
            Scope::Expression(scope) => {
                let plain_name = path.mod_path().plain_name()?;
                let entry = scope
                    .expression_scopes
                    .resolve_name_in_scope(scope.scope_id, plain_name)?;
                Some(ResolveKind::Local(entry.binding))
            },
            Scope::Module(scope) => {
                let resolved = scope.def_map.resolve_path(
                    scope.file_id.original_file(database).file_id,
                    path.mod_path(),
                )?;

                Some(match resolved.resolved_def {
                    ModuleDefinitionId::Module(id) => ResolveKind::Module(id),
                    ModuleDefinitionId::Function(id) => {
                        ResolveKind::Function(database.lookup_intern_function(id))
                    },
                    ModuleDefinitionId::GlobalVariable(id) => {
                        ResolveKind::GlobalVariable(database.lookup_intern_global_variable(id))
                    },
                    ModuleDefinitionId::GlobalConstant(id) => {
                        ResolveKind::GlobalConstant(database.lookup_intern_global_constant(id))
                    },
                    ModuleDefinitionId::GlobalAssertStatement(_) => return None,
                    ModuleDefinitionId::Override(id) => {
                        ResolveKind::Override(database.lookup_intern_override(id))
                    },
                    ModuleDefinitionId::Struct(id) => {
                        ResolveKind::Struct(database.lookup_intern_struct(id))
                    },
                    ModuleDefinitionId::TypeAlias(id) => {
                        ResolveKind::TypeAlias(database.lookup_intern_type_alias(id))
                    },
                })
            },
            Scope::Builtin => {
                // TODO: Match against the plain name and then point at a "builtin" file
                // See: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/559
                None
            },
        })
    }
}

