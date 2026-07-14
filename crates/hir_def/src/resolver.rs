use base_db::{EditionedFileId, Package, file_package, input::PackageId};
use triomphe::Arc;

use crate::{
    body::{
        BindingId,
        scope::{ExprScopes, ScopeId},
    },
    database::{
        DefDatabase, FunctionId, GlobalConstantId, GlobalVariableId, ModuleDefinitionId,
        OverrideId, StructId, TypeAliasId,
    },
    expression_store::path::Path,
    item_scope::ItemScope,
    item_tree::Name,
    mod_path::{AbsoluteModPath, ModPath, PathKind},
    name_resolution::resolve_module,
    visibility::Visibility,
};

#[derive(Clone)]
pub enum Scope {
    /// Local bindings.
    Expression(ExpressionScope),
    /// The items inside a module.
    Module(ModuleScope),
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

#[derive(Debug)]
pub enum ResolveKind {
    Local(BindingId, FunctionId),
    Struct(StructId),
    TypeAlias(TypeAliasId),
    GlobalVariable(GlobalVariableId),
    GlobalConstant(GlobalConstantId),
    Override(OverrideId),
    Function(FunctionId),
}

impl TryFrom<ModuleDefinitionId> for ResolveKind {
    type Error = ();

    fn try_from(value: ModuleDefinitionId) -> Result<Self, ()> {
        Ok(match value {
            ModuleDefinitionId::Function(id) => Self::Function(id),
            ModuleDefinitionId::GlobalVariable(id) => Self::GlobalVariable(id),
            ModuleDefinitionId::GlobalConstant(id) => Self::GlobalConstant(id),
            ModuleDefinitionId::GlobalAssertStatement(_) => return Err(()),
            ModuleDefinitionId::Override(id) => Self::Override(id),
            ModuleDefinitionId::Struct(id) => Self::Struct(id),
            ModuleDefinitionId::TypeAlias(id) => Self::TypeAlias(id),
        })
    }
}

pub enum ScopeDef {
    Local(BindingId),
    ModuleDefinition(ModuleDefinitionId),
    ModulePath(AbsoluteModPath),
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

    /// Calls the passed closure `callback` on all names in scope.
    pub fn process_all_names<Callback>(
        &self,
        mut callback: Callback,
    ) where
        Callback: FnMut(&Name, ScopeDef),
    {
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
                scope
                    .module_info
                    .import_paths
                    .iter()
                    .for_each(|(name, item)| {
                        callback(name, ScopeDef::ModulePath(item.path.clone()));
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
    ///
    /// # Panics
    /// Panics if an empty path was passed.
    /// Empty paths should not be produced, they should at most contain a "missing" name.
    pub fn resolve(
        &self,
        database: &dyn DefDatabase,
        path: &Path,
    ) -> Result<ResolveKind, ResolutionDiagnostic> {
        let path = path.mod_path();
        if path.is_empty() {
            return Err(ResolutionDiagnostic::MissingName);
        }
        match path.kind() {
            PathKind::Plain if path.len() == 1 => self.resolve_name(database, &path.segments()[0]),
            PathKind::Plain => {
                let dependency_name = &path.segments()[0];
                // The first segment is either an import or a package name
                let item_scope = ItemScope::of(database, self.file_id);
                if let Some(module_import) = item_scope.import_paths.get(dependency_name) {
                    let mut absolute_path = module_import.path.clone();
                    for segment in &path.segments()[1..] {
                        absolute_path.push_segment(segment.clone());
                    }

                    resolve_path_to_item(database, module_import.package, absolute_path.segments())
                } else {
                    let package = file_package(database, self.file_id.file_id(database))
                        .ok_or(ResolutionDiagnostic::DetachedFile)?;

                    let resolved_dependency = package
                        .data(database)
                        .dependencies
                        .iter()
                        .find(|dep| dep.name.as_str() == dependency_name.as_str())
                        .ok_or(ResolutionDiagnostic::UnresolvedPackage {
                            name: dependency_name.clone(),
                        })?;

                    let dependency_package = resolved_dependency.package(database);

                    resolve_path_to_item(database, dependency_package, &path.segments()[1..])
                }
            },
            PathKind::Super(levels) => {
                let package = file_package(database, self.file_id.file_id(database))
                    .ok_or(ResolutionDiagnostic::DetachedFile)?;

                let mut mod_path = AbsoluteModPath::for_file(database, package, self.file_id)
                    .ok_or(ResolutionDiagnostic::DetachedFile)?;

                for level in 0..levels {
                    if mod_path.pop_segment().is_none() {
                        return Err(ResolutionDiagnostic::TooManySupers);
                    }
                }

                for segment in path.segments() {
                    mod_path.push_segment(segment.clone());
                }

                resolve_path_to_item(database, package, mod_path.segments())
            },
            PathKind::Package => {
                let package = file_package(database, self.file_id.file_id(database))
                    .ok_or(ResolutionDiagnostic::DetachedFile)?;
                resolve_path_to_item(database, package, path.segments())
            },
        }
    }

    fn resolve_name(
        &self,
        database: &dyn DefDatabase,
        name: &Name,
    ) -> Result<ResolveKind, ResolutionDiagnostic> {
        self.scopes()
            .find_map(|scope| match scope {
                Scope::Expression(scope) => {
                    let entry = scope
                        .expression_scopes
                        .resolve_name_in_scope(scope.scope_id, name)?;
                    Some(ResolveKind::Local(entry.binding, scope.owner))
                },
                Scope::Module(scope) => {
                    let item = scope.module_info.items.get(name)?;
                    ResolveKind::try_from(item.definition).ok()
                },
                Scope::Builtin => {
                    // TODO: Match against the first name segment and then point at a "builtin" file
                    // See: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/559
                    None
                },
            })
            .ok_or(ResolutionDiagnostic::UnresolvedName { name: name.clone() })
    }
}

fn resolve_path_to_item(
    database: &dyn DefDatabase,
    package: Package,
    segments: &[Name],
) -> Result<ResolveKind, ResolutionDiagnostic> {
    let [mod_path_segments @ .., name] = segments else {
        return Err(ResolutionDiagnostic::MissingName);
    };

    let Some(file_id) = resolve_module(database, package, mod_path_segments) else {
        return Err(ResolutionDiagnostic::UnresolvedFile {
            package: package.package_id(database),
            path: AbsoluteModPath::from_segments(mod_path_segments),
        });
    };

    let item_scope = ItemScope::of(database, file_id);
    let Some(item) = item_scope.items.get(name) else {
        return Err(ResolutionDiagnostic::UnresolvedItem {
            file_id,
            name: name.clone(),
        });
    };
    if item.visibility == Visibility::File {
        // TODO: allow importing from self, see https://github.com/wgsl-analyzer/wgsl-analyzer/issues/1192
        return Err(ResolutionDiagnostic::PrivateItem {
            name: name.clone(),
            visibility: item.visibility,
        });
    }

    Ok(ResolveKind::try_from(item.definition)
        .expect("Item scope may only contain items that can be resolved"))
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ResolutionDiagnostic {
    /// Cannot resolve a name in the current file.
    UnresolvedName {
        name: Name,
    },
    UnresolvedFile {
        package: PackageId,
        path: AbsoluteModPath,
    },
    UnresolvedPackage {
        name: Name,
    },
    /// Cannot resolve a name in a different file.
    UnresolvedItem {
        file_id: EditionedFileId,
        name: Name,
    },
    PrivateItem {
        name: Name,
        visibility: Visibility,
    },
    TooManySupers,
    /// Cannot resolve an import statement, because the current file is not a part of a package.
    DetachedFile,
    MissingName,
}
