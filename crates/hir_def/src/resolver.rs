use base_db::{EditionedFileId, Package, SourceDatabase, file_package, input::PackageId};
use triomphe::Arc;

use crate::{
    body::{
        BindingId,
        scope::{ExprScopes, ScopeId},
    },
    db::{
        FunctionId, GlobalConstantId, GlobalVariableId, ModuleDefinitionId, OverrideId, StructId,
        TypeAliasId,
    },
    expression::ExpressionId,
    expression_store::path::Path,
    item_scope::ItemScope,
    item_tree::Name,
    mod_path::{AbsoluteModPath, PathKind},
    name_resolution::resolve_module,
    visibility::Visibility,
};

#[derive(Clone)]
pub enum Scope<'db> {
    /// Local bindings.
    Expression(ExpressionScope<'db>),
    /// The items inside a module.
    Module(ModuleScope),
    /// Predeclared WGSL items.
    Builtin,
}

#[derive(Clone)]
pub struct ExpressionScope<'db> {
    owner: FunctionId,
    expression_scopes: &'db ExprScopes,
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
    BuiltinFunction(Name),
    BuiltinType(Name),
    BuiltinTypeGenerator(Name),
    // BuiltinTypeConstructor(Name),
    BuiltinEnumerant(Name),
    BuiltinDeclaration(Name),
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
    BuiltIn(BuiltInKind),
    Module,
}

pub enum BuiltInKind {
    Alias(Name),
    Constructor(Name),
    Declaration(Name),
    Enumerant(Name),
    Function(Name),
    Struct(Name),
    TypeGenerator(Name),
    Type(Name),
}

#[derive(Clone)]
pub struct Resolver<'db> {
    file_id: EditionedFileId,
    scopes: Vec<Scope<'db>>,
}

impl<'db> Resolver<'db> {
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
        scope: Scope<'db>,
    ) -> Self {
        self.scopes.push(scope);
        self
    }

    #[must_use]
    pub fn push_expression_scope(
        mut self,
        owner: FunctionId,
        expression_scopes: &'db ExprScopes,
        scope_id: ScopeId,
    ) -> Self {
        self.scopes.push(Scope::Expression(ExpressionScope {
            owner,
            expression_scopes,
            scope_id,
        }));
        self
    }

    pub fn scopes(&self) -> impl Iterator<Item = &Scope<'db>> {
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
                    .for_each(|(name, _item)| {
                        callback(name, ScopeDef::Module);
                    });
            },
            Scope::Builtin => {
                for name in wgsl_types::idents::BUILTIN_ALIAS_NAMES {
                    callback(
                        &(*name).into(),
                        ScopeDef::BuiltIn(BuiltInKind::Alias((*name).into())),
                    );
                }
                for name in wgsl_types::idents::BUILTIN_CONSTRUCTOR_NAMES {
                    callback(
                        &(*name).into(),
                        ScopeDef::BuiltIn(BuiltInKind::Constructor((*name).into())),
                    );
                }
                for name in wgsl_types::idents::BUILTIN_DECLARATION_NAMES {
                    callback(
                        &(*name).into(),
                        ScopeDef::BuiltIn(BuiltInKind::Declaration((*name).into())),
                    );
                }
                for name in wgsl_types::idents::BUILTIN_ENUMERANT_NAMES {
                    callback(
                        &(*name).into(),
                        ScopeDef::BuiltIn(BuiltInKind::Enumerant((*name).into())),
                    );
                }
                for name in wgsl_types::idents::BUILTIN_FUNCTION_NAMES {
                    callback(
                        &(*name).into(),
                        ScopeDef::BuiltIn(BuiltInKind::Function((*name).into())),
                    );
                }
                // builtin struct names are "unmentionable" in user source code and
                // never need to be processed by name or completed.
                // for name in wgsl_types::idents::BUILTIN_STRUCT_NAMES {
                //     callback(&(*name).into(), ScopeDef::BuiltIn(BuiltInKind::Struct((*name).into())));
                // }
                for name in wgsl_types::idents::BUILTIN_TYPE_GENERATOR_NAMES {
                    callback(
                        &(*name).into(),
                        ScopeDef::BuiltIn(BuiltInKind::TypeGenerator((*name).into())),
                    );
                }
                for name in wgsl_types::idents::BUILTIN_TYPE_NAMES
                    .iter()
                    .filter(|type_name| !type_name.starts_with("__"))
                {
                    callback(
                        &(*name).into(),
                        ScopeDef::BuiltIn(BuiltInKind::Type((*name).into())),
                    );
                }
            },
        });
    }

    /// Resolve an *inline* path. Import statements are already resolved.
    /// Corresponds to `resolve_path_in_type_ns` in rust-analyzer.
    pub fn resolve(
        &self,
        db: &dyn SourceDatabase,
        path: &Path,
    ) -> Result<ResolveKind, ResolutionDiagnostic> {
        let path = path.mod_path();
        if path.is_empty() {
            return Err(ResolutionDiagnostic::MissingName);
        }
        match path.kind() {
            PathKind::Plain if path.len() == 1 => self.resolve_name(db, &path.segments()[0]),
            PathKind::Plain => {
                let dependency_name = &path.segments()[0];
                // The first segment is either an import or a package name
                let item_scope = ItemScope::of(db, self.file_id);
                if let Some(module_import) = item_scope.import_paths.get(dependency_name) {
                    let mut absolute_path = module_import.path.clone();
                    for segment in &path.segments()[1..] {
                        absolute_path.push_segment(segment.clone());
                    }

                    resolve_path_to_item(db, module_import.package, absolute_path.segments())
                } else {
                    let package = file_package(db, self.file_id.file_id(db))
                        .ok_or(ResolutionDiagnostic::DetachedFile)?;

                    let resolved_dependency = package
                        .data(db)
                        .dependencies
                        .iter()
                        .find(|dep| dep.name.as_str() == dependency_name.as_str())
                        .ok_or(ResolutionDiagnostic::UnresolvedPackage {
                            name: dependency_name.clone(),
                        })?;

                    let dependency_package = resolved_dependency.package(db);

                    resolve_path_to_item(db, dependency_package, &path.segments()[1..])
                }
            },
            PathKind::Super(levels) => {
                let package = file_package(db, self.file_id.file_id(db))
                    .ok_or(ResolutionDiagnostic::DetachedFile)?;

                let mut mod_path = AbsoluteModPath::for_file(db, package, self.file_id)
                    .ok_or(ResolutionDiagnostic::DetachedFile)?;

                for _ in 0..levels {
                    if mod_path.pop_segment().is_none() {
                        return Err(ResolutionDiagnostic::TooManySupers);
                    }
                }

                for segment in path.segments() {
                    mod_path.push_segment(segment.clone());
                }

                resolve_path_to_item(db, package, mod_path.segments())
            },
            PathKind::Package => {
                let package = file_package(db, self.file_id.file_id(db))
                    .ok_or(ResolutionDiagnostic::DetachedFile)?;
                resolve_path_to_item(db, package, path.segments())
            },
        }
    }

    fn resolve_name(
        &self,
        db: &dyn SourceDatabase,
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
                    if wgsl_types::idents::BUILTIN_FUNCTION_NAMES.contains(&name.as_str()) {
                        Some(ResolveKind::BuiltinFunction(name.clone()))
                    } else if wgsl_types::idents::BUILTIN_TYPE_NAMES.contains(&name.as_str())
                        || wgsl_types::idents::BUILTIN_ALIAS_NAMES.contains(&name.as_str())
                    {
                        Some(ResolveKind::BuiltinType(name.clone()))
                    } else if wgsl_types::idents::BUILTIN_TYPE_GENERATOR_NAMES
                        .contains(&name.as_str())
                    {
                        Some(ResolveKind::BuiltinTypeGenerator(name.clone()))
                    } else if wgsl_types::idents::BUILTIN_CONSTRUCTOR_NAMES.contains(&name.as_str())
                    {
                        debug_assert!(
                            false,
                            "builtin constructor `{}` is unimplemented in wgsl-analyzer",
                            name.as_str()
                        );
                        None
                        // Some(ResolveKind::BuiltinTypeConstructor(name.clone()))
                    } else if wgsl_types::idents::BUILTIN_ENUMERANT_NAMES.contains(&name.as_str()) {
                        Some(ResolveKind::BuiltinEnumerant(name.clone()))
                    } else if wgsl_types::idents::BUILTIN_DECLARATION_NAMES.contains(&name.as_str())
                    {
                        Some(ResolveKind::BuiltinDeclaration(name.clone()))
                    } else {
                        None
                    }
                },
            })
            .ok_or(ResolutionDiagnostic::UnresolvedName { name: name.clone() })
    }
}

fn resolve_path_to_item(
    db: &dyn SourceDatabase,
    package: Package,
    segments: &[Name],
) -> Result<ResolveKind, ResolutionDiagnostic> {
    let [mod_path_segments @ .., name] = segments else {
        return Err(ResolutionDiagnostic::MissingName);
    };

    let Some(file_id) = resolve_module(db, package, mod_path_segments) else {
        return Err(ResolutionDiagnostic::UnresolvedFile {
            package: package.package_id(db),
            path: AbsoluteModPath::from_segments(mod_path_segments),
        });
    };

    let item_scope = ItemScope::of(db, file_id);
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
