use std::ops::ControlFlow;

use base_db::{EditionedFileId, Intern as _};
use triomphe::Arc;

use crate::{
    InFile,
    body::{
        BindingId,
        scope::{ExprScopes, ScopeId},
    },
    database::{
        self, DefDatabase, FunctionId, GlobalConstantId, GlobalVariableId, Location,
        ModuleDefinitionId, OverrideId, StructId, TypeAliasId,
    },
    expression_store::path::Path,
    item_scope::ItemScope,
    item_tree::{
        Function, GlobalConstant, GlobalVariable, ItemTree, ModuleItemId, Name, Override, Struct,
        TypeAlias,
    },
    mod_path::{ModPath, PathKind},
    name_resolution::ModuleData,
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
    module_info: Arc<ItemScope>,
    file_id: EditionedFileId,
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
    Struct(StructId),
    TypeAlias(TypeAliasId),
    GlobalVariable(GlobalVariableId),
    GlobalConstant(GlobalConstantId),
    Override(OverrideId),
    Function(FunctionId),
    Module(EditionedFileId),
}

pub enum ScopeDef {
    Local(BindingId),
    ModuleDefinition(ModuleDefinitionId),
}

#[derive(Clone)]
pub struct Resolver {
    file_id: EditionedFileId,
    scopes: Vec<Scope>,
}

impl Resolver {
    #[must_use]
    pub fn new(
        file_id: EditionedFileId,
        module_info: Arc<ItemScope>,
    ) -> Self {
        let module_scope = ModuleScope {
            module_info,
            file_id,
        };
        Self {
            file_id,
            scopes: vec![Scope::Builtin, Scope::Module(module_scope)],
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
        mut callback: Function,
    ) {
        self.scopes().for_each(|scope| match scope {
            Scope::Module(scope) => {
                scope.module_info.items.iter().for_each(|(name, item)| {
                    callback(name, ScopeDef::ModuleDefinition(item.definition))
                });
            },
            Scope::Expression(expression_scope) => {
                expression_scope
                    .expression_scopes
                    .scope_chain(Some(expression_scope.scope_id))
                    .for_each(|id| {
                        let data = &expression_scope.expression_scopes[id];
                        data.entries.iter().for_each(|entry| {
                            callback(&entry.name, ScopeDef::Local(entry.binding));
                        });
                    });
            },
            Scope::Builtin => {
                // TODO: Match against "name.as_str()" and then point at a "builtin" file
                // See: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/559
            },
        });
    }

    /// Resolve an *inline* path. Import statements are already resolved.
    /// Corresponds to `resolve_path_in_type_ns` in rust-analyzer.
    #[must_use]
    pub fn resolve(
        &self,
        database: &dyn DefDatabase,
        path: &Path,
    ) -> Option<ResolveKind> {
        let mut path = path.mod_path().clone();
        match path.kind() {
            PathKind::Plain => {
                if path.is_empty() {
                    return None;
                }
            },
            PathKind::Super(levels) => {
                let mut file_id = self.file_id;
                for _ in 0..levels {
                    let module_data = ModuleData::of(database, file_id).as_ref()?;
                    if let Some(parent) = module_data.parent {
                        file_id = parent;
                    } else {
                        return None;
                    }
                }
                if path.is_empty() {
                    return Some(ResolveKind::Module(file_id));
                }
                // Continue resolution with the remaining path
                path.set_kind(PathKind::Plain);
                return Resolver::new(file_id, ItemScope::of(database, file_id))
                    .resolve(database, &Path(path));
            },
            PathKind::Package => {
                let package_data =
                    base_db::file_package(database, self.file_id.file_id(database))?.data(database);
                let file_id =
                    EditionedFileId::new(database, package_data.root_file_id, package_data.edition);
                if path.is_empty() {
                    return Some(ResolveKind::Module(file_id));
                }
                // Continue resolution with the remaining path
                path.set_kind(PathKind::Plain);
                return Resolver::new(file_id, ItemScope::of(database, file_id))
                    .resolve(database, &Path(path));
            },
        };
        assert_eq!(
            path.kind(),
            PathKind::Plain,
            "Only plain paths should exist here"
        );

        let name_start = path.pop_segment()?;

        self.scopes().find_map(|scope| match scope {
            Scope::Expression(scope) => {
                let entry = scope
                    .expression_scopes
                    .resolve_name_in_scope(scope.scope_id, &name_start)?;

                if !path.is_empty() {
                    // TODO: Report an error!
                    return None;
                }

                Some(ResolveKind::Local(entry.binding))
            },
            Scope::Module(scope) => {
                let item = scope.module_info.items.get(&name_start)?;
                let resolved = match item.definition {
                    ModuleDefinitionId::Module(file_id) => {
                        if !path.is_empty() {
                            return Resolver::new(file_id, ItemScope::of(database, file_id))
                                .resolve(database, &Path(path.clone()));
                        } else {
                            ResolveKind::Module(file_id)
                        }
                    },
                    ModuleDefinitionId::Function(id) => ResolveKind::Function(id),
                    ModuleDefinitionId::GlobalVariable(id) => ResolveKind::GlobalVariable(id),
                    ModuleDefinitionId::GlobalConstant(id) => ResolveKind::GlobalConstant(id),
                    ModuleDefinitionId::GlobalAssertStatement(_) => {
                        panic!("Item resolution should never return an assert statement")
                    },
                    ModuleDefinitionId::Override(id) => ResolveKind::Override(id),
                    ModuleDefinitionId::Struct(id) => ResolveKind::Struct(id),
                    ModuleDefinitionId::TypeAlias(id) => ResolveKind::TypeAlias(id),
                };

                if !path.is_empty() {
                    // TODO: Report an error!
                    return None;
                }

                Some(resolved)
            },
            Scope::Builtin => {
                // TODO: Match against the first name segment and then point at a "builtin" file
                // See: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/559
                None
            },
        })
    }
}
