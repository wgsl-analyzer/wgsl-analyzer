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
    /// Local bindings.
    Expression(ExpressionScope),
    /// The items inside a module.
    Module(ModuleScope),
    /// Children of a module.
    SubModules(SubModulesScope),
    /// Predeclared WGSL items.
    Builtin,
}

#[derive(Clone)]
pub struct ExpressionScope {
    owner: FunctionId,
    expression_scopes: Arc<ExprScopes>,
    scope_id: ScopeId,
}

#[derive(Clone)]
pub struct ModuleScope {
    module_info: Arc<ItemScope>,
    file_id: EditionedFileId,
}

#[derive(Clone)]
pub struct SubModulesScope {
    module_info: Arc<ModuleData>,
    file_id: EditionedFileId,
}

#[derive(Debug)]
pub enum ResolveKind {
    Local(BindingId, FunctionId),
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
    pub(crate) fn new_for_submodules(
        file_id: EditionedFileId,
        item_scope: Arc<ItemScope>,
        module_data: Option<Arc<ModuleData>>,
    ) -> Self {
        let module_scope = ModuleScope {
            module_info: item_scope,
            file_id,
        };
        let scopes = if let Some(module_info) = module_data {
            let sub_modules = SubModulesScope {
                module_info,
                file_id,
            };
            vec![Scope::SubModules(sub_modules), Scope::Module(module_scope)]
        } else {
            vec![Scope::Module(module_scope)]
        };
        Self { file_id, scopes }
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
            Scope::Module(_) | Scope::SubModules(_) | Scope::Builtin => None,
        })
    }

    /// Calls the passed closure `function` on all names in scope.
    pub fn process_all_names<Function: FnMut(&Name, ScopeDef)>(
        &self,
        database: &dyn DefDatabase,
        mut callback: Function,
    ) {
        self.scopes().for_each(|scope| match scope {
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
            Scope::Module(scope) => {
                scope.module_info.items.iter().for_each(|(name, item)| {
                    callback(name, ScopeDef::ModuleDefinition(item.definition));
                });
            },
            Scope::SubModules(scope) => {
                scope.module_info.children.iter().for_each(|(name, item)| {
                    callback(
                        name,
                        ScopeDef::ModuleDefinition(ModuleDefinitionId::Module(*item)),
                    );
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
    pub fn resolve(
        &self,
        database: &dyn DefDatabase,
        path: &Path,
    ) -> Result<ResolveKind, ResolutionDiagnostic> {
        let path = path.mod_path();
        match path.kind() {
            PathKind::Plain => self.resolve_plain(database, path.segments()),
            PathKind::Super(levels) => {
                let mut file_id = self.file_id;
                for level in 0..levels {
                    let parent = ModuleData::of(database, file_id)
                        .and_then(|module_data| module_data.parent)
                        .ok_or_else(|| ResolutionDiagnostic {
                            failed_segment: usize::from(level),
                        })?;
                    file_id = parent;
                }
                if path.is_empty() {
                    Ok(ResolveKind::Module(file_id))
                } else {
                    // Continue resolution with the remaining path
                    Self::new_for_submodules(
                        file_id,
                        ItemScope::of(database, file_id),
                        ModuleData::of(database, file_id),
                    )
                    .resolve_plain(database, path.segments())
                    .map_err(|mut diagnostic| {
                        diagnostic.failed_segment += usize::from(levels);
                        diagnostic
                    })
                }
            },
            PathKind::Package => {
                let package_data = base_db::file_package(database, self.file_id.file_id(database))
                    .ok_or(ResolutionDiagnostic { failed_segment: 0 })?
                    .data(database);
                let file_id =
                    EditionedFileId::new(database, package_data.root_file_id, package_data.edition);
                if path.is_empty() {
                    Ok(ResolveKind::Module(file_id))
                } else {
                    // Continue resolution with the remaining path
                    Self::new_for_submodules(
                        file_id,
                        ItemScope::of(database, file_id),
                        ModuleData::of(database, file_id),
                    )
                    .resolve_plain(database, path.segments())
                    .map_err(|mut diagnostic| {
                        diagnostic.failed_segment += 1;
                        diagnostic
                    })
                }
            },
        }
    }

    fn resolve_plain(
        &self,
        database: &dyn DefDatabase,
        segments: &[Name],
    ) -> Result<ResolveKind, ResolutionDiagnostic> {
        let name_start = segments
            .first()
            .ok_or(ResolutionDiagnostic { failed_segment: 0 })?;
        let is_path_done = segments.len() == 1;

        for scope in self.scopes() {
            match scope {
                Scope::Expression(scope) => {
                    let Some(entry) = scope
                        .expression_scopes
                        .resolve_name_in_scope(scope.scope_id, name_start)
                    else {
                        continue;
                    };

                    if is_path_done {
                        return Ok(ResolveKind::Local(entry.binding, scope.owner));
                    }
                    return Err(ResolutionDiagnostic { failed_segment: 1 });
                },
                Scope::Module(scope) => {
                    let Some(item) = scope.module_info.items.get(name_start) else {
                        continue;
                    };
                    let resolved = match item.definition {
                        ModuleDefinitionId::Module(file_id) => {
                            if is_path_done {
                                ResolveKind::Module(file_id)
                            } else {
                                return Self::new_for_submodules(
                                    file_id,
                                    ItemScope::of(database, file_id),
                                    ModuleData::of(database, file_id),
                                )
                                .resolve_plain(database, &segments[1..])
                                .map_err(|mut diagnostic| {
                                    diagnostic.failed_segment += 1;
                                    diagnostic
                                });
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

                    if is_path_done {
                        return Ok(resolved);
                    }
                    return Err(ResolutionDiagnostic { failed_segment: 1 });
                },
                Scope::SubModules(scope) => {
                    let Some(file_id) = scope.module_info.children.get(name_start) else {
                        continue;
                    };
                    if is_path_done {
                        return Ok(ResolveKind::Module(*file_id));
                    }
                    return Self::new_for_submodules(
                        *file_id,
                        ItemScope::of(database, *file_id),
                        ModuleData::of(database, *file_id),
                    )
                    .resolve_plain(database, &segments[1..])
                    .map_err(|mut diagnostic| {
                        diagnostic.failed_segment += 1;
                        diagnostic
                    });
                },
                Scope::Builtin => {
                    // TODO: Match against the first name segment and then point at a "builtin" file
                    // See: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/559
                },
            }
        }

        Err(ResolutionDiagnostic { failed_segment: 0 })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ResolutionDiagnostic {
    /// The index of the last segment where resolution failed.
    pub failed_segment: usize,
}
