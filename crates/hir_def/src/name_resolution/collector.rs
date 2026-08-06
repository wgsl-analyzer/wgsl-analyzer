use base_db::{EditionedFileId, Intern as _, Package, file_package, input::PackageData};
use itertools::Itertools as _;
use syntax::ast;
use vfs::VfsPath;

use crate::{
    database::{DefDatabase, Location, ModuleDefinitionId},
    item_scope::{ItemScope, ModuleImportPath, ModuleItem},
    item_tree::{FlatImport, ImportStatement, ItemTree, ModuleItemId, Name},
    mod_path::{AbsoluteModPath, ModPath, PathKind},
    name_resolution::{
        ModulesMap,
        diagnostics::{self, DefDiagnostic},
        resolve_module,
    },
    visibility::Visibility,
};

pub fn collect_module(
    database: &dyn DefDatabase,
    file_id: EditionedFileId,
) -> ItemScope {
    let item_tree = database.item_tree(file_id);

    let mut collector = ModCollector {
        database,
        file_id,
        item_scope: ItemScope::default(),
    };
    collector.collect(&item_tree);
    collector.item_scope.shrink_to_fit();
    collector.item_scope
}

/// Walks over the defs and imports of a single module.
///
/// This is a precomputation step to speed up name resolution.
/// It saves us the effort of repeatedly going over all import
/// statements during normal name resolution.
/// It also eagerly verifies that names, including imported ones,
/// do not clash.
pub(super) struct ModCollector<'db> {
    database: &'db dyn DefDatabase,
    file_id: EditionedFileId,
    item_scope: ItemScope,
}

impl ModCollector<'_> {
    fn collect(
        &mut self,
        item_tree: &ItemTree,
    ) {
        for item in item_tree.top_level_items() {
            let (name, definition) = match *item {
                ModuleItemId::ImportStatement(id) => {
                    let location = Location::new(self.file_id, id);
                    self.collect_import_statement(&item_tree[id], location, *item);
                    continue;
                },
                ModuleItemId::Function(id) => (
                    &item_tree[id].name,
                    ModuleDefinitionId::Function(
                        Location::new(self.file_id, id).intern(self.database),
                    ),
                ),
                ModuleItemId::Struct(id) => (
                    &item_tree[id].name,
                    ModuleDefinitionId::Struct(
                        Location::new(self.file_id, id).intern(self.database),
                    ),
                ),
                ModuleItemId::GlobalVariable(id) => (
                    &item_tree[id].name,
                    ModuleDefinitionId::GlobalVariable(
                        Location::new(self.file_id, id).intern(self.database),
                    ),
                ),
                ModuleItemId::GlobalConstant(id) => (
                    &item_tree[id].name,
                    ModuleDefinitionId::GlobalConstant(
                        Location::new(self.file_id, id).intern(self.database),
                    ),
                ),
                ModuleItemId::Override(id) => (
                    &item_tree[id].name,
                    ModuleDefinitionId::Override(
                        Location::new(self.file_id, id).intern(self.database),
                    ),
                ),
                ModuleItemId::TypeAlias(id) => (
                    &item_tree[id].name,
                    ModuleDefinitionId::TypeAlias(
                        Location::new(self.file_id, id).intern(self.database),
                    ),
                ),
                ModuleItemId::GlobalAssertStatement(_) => continue,
            };

            self.push_item(
                name,
                ModuleItem {
                    definition,
                    visibility: Visibility::Public,
                    import: None,
                },
                *item,
            );
        }
    }

    fn collect_import_statement(
        &mut self,
        import_statement: &ImportStatement,
        location: Location<ast::ImportStatement>,
        item_id: ModuleItemId,
    ) {
        let import_id = location.intern(self.database);
        import_statement.expand(|flat_import| {
            let Some(name) = flat_import.leaf_name().cloned() else {
                // If we do not have a leaf name, there are a few possible cases
                // - PathKind::Plain => Must have a leaf name, otherwise the path is completely empty
                // - PathKind::Super => Don't need to add `super` to the scope, it is already a keyword
                // - PathKind::Package => Don't need to add `package` to the scope, it is already a keyword
                self.item_scope
                    .push_diagnostic(DefDiagnostic::unnamed_import(self.file_id, location));
                return;
            };
            let (package, path) = match self.absolutize_import(location, &flat_import) {
                Ok(value) => value,
                Err(diagnostic) => {
                    self.item_scope.push_diagnostic(diagnostic);
                    return;
                },
            };
            self.push_import_path(
                &name,
                ModuleImportPath {
                    package,
                    path: path.clone(),
                    import: import_id,
                },
                item_id,
            );

            let definition = match self.resolve_import(package, &path, location) {
                Ok(value) => value,
                Err(diagnostic) => {
                    self.item_scope.push_diagnostic(diagnostic);
                    return;
                },
            };

            if let Some(definition) = definition {
                self.push_item(
                    &name,
                    ModuleItem {
                        definition,
                        visibility: Visibility::File,
                        import: Some(import_id),
                    },
                    item_id,
                );
            }
        });
    }

    fn push_item(
        &mut self,
        name: &Name,
        item: ModuleItem,
        item_id: ModuleItemId,
    ) {
        let previous = self.item_scope.push_item(name.clone(), item);

        if let Some(previous) = previous {
            self.item_scope
                .push_diagnostic(DefDiagnostic::name_conflict(
                    self.file_id,
                    Location::new(self.file_id, item_id.ast_id()),
                    name.clone(),
                ));
        }
    }

    fn push_import_path(
        &mut self,
        name: &Name,
        path: ModuleImportPath,
        item_id: ModuleItemId,
    ) {
        let previous = self.item_scope.push_import_path(name.clone(), path);

        if let Some(previous) = previous {
            self.item_scope
                .push_diagnostic(DefDiagnostic::name_conflict(
                    self.file_id,
                    Location::new(self.file_id, item_id.ast_id()),
                    name.clone(),
                ));
        }
    }

    fn absolutize_import(
        &self,
        location: Location<ast::ImportStatement>,
        import: &FlatImport,
    ) -> Result<(Package, AbsoluteModPath), DefDiagnostic> {
        let package = file_package(self.database, self.file_id.file_id(self.database))
            .ok_or_else(|| DefDiagnostic::detached_file(self.file_id, location))?;

        match import.path.kind() {
            PathKind::Plain => {
                let dependency_name = import
                    .path
                    .segments()
                    .first()
                    .ok_or_else(|| DefDiagnostic::unnamed_import(self.file_id, location))?;

                let resolved_dependency = package
                    .data(self.database)
                    .dependencies
                    .iter()
                    .find(|dep| dep.name.as_str() == dependency_name.as_str())
                    .ok_or_else(|| {
                        DefDiagnostic::unresolved_package(
                            self.file_id,
                            location,
                            dependency_name.clone(),
                        )
                    })?;

                let dependency_package = resolved_dependency.package(self.database);
                Ok((
                    dependency_package,
                    AbsoluteModPath::from_segments(&import.path.segments()[1..]),
                ))
            },
            PathKind::Super(levels) => {
                let mut mod_path = AbsoluteModPath::for_file(self.database, package, self.file_id)
                    .ok_or_else(|| DefDiagnostic::detached_file(self.file_id, location))?;

                for _ in 0..levels {
                    if mod_path.pop_segment().is_none() {
                        return Err(DefDiagnostic::super_escaping_root(self.file_id, location));
                    }
                }

                for segment in import.path.segments() {
                    mod_path.push_segment(segment.clone());
                }

                Ok((package, AbsoluteModPath::from_segments(mod_path.segments())))
            },
            PathKind::Package => Ok((
                package,
                AbsoluteModPath::from_segments(import.path.segments()),
            )),
        }
    }

    /// Given a path `foo::bar`, we need to check for `foo/bar.wesl` and for item `bar` in `foo.wesl`.
    fn resolve_import(
        &self,
        package: Package,
        path: &AbsoluteModPath,
        location: Location<ast::ImportStatement>,
    ) -> Result<Option<ModuleDefinitionId>, DefDiagnostic> {
        let [head_segments @ .., name] = path.segments() else {
            return Err(DefDiagnostic::unnamed_import(self.file_id, location));
        };
        let module = resolve_module(self.database, package, head_segments);
        let item = module.and_then(|module| self.resolve_item(module, name));
        if item.is_some() {
            Ok(item)
        } else {
            let modules_map = ModulesMap::of(self.database, package);
            if modules_map.modules.contains_key(path) {
                Ok(None)
            } else {
                Err(DefDiagnostic::unresolved_import(self.file_id, location))
            }
        }
    }

    fn resolve_item(
        &self,
        file_id: EditionedFileId,
        name: &Name,
    ) -> Option<ModuleDefinitionId> {
        let item_tree = self.database.item_tree(file_id);
        item_tree
            .top_level_items()
            .iter()
            .find_map(|item| match item {
                ModuleItemId::Struct(id) => {
                    let r#struct = &item_tree[*id];
                    (&r#struct.name == name).then(|| {
                        ModuleDefinitionId::Struct(
                            Location::new(file_id, *id).intern(self.database),
                        )
                    })
                },
                ModuleItemId::TypeAlias(id) => {
                    let type_alias = &item_tree[*id];
                    (&type_alias.name == name).then(|| {
                        ModuleDefinitionId::TypeAlias(
                            Location::new(file_id, *id).intern(self.database),
                        )
                    })
                },
                ModuleItemId::GlobalVariable(id) => {
                    let variable = &item_tree[*id];
                    (&variable.name == name).then(|| {
                        ModuleDefinitionId::GlobalVariable(
                            Location::new(file_id, *id).intern(self.database),
                        )
                    })
                },
                ModuleItemId::GlobalConstant(id) => {
                    let constant = &item_tree[*id];
                    (&constant.name == name).then(|| {
                        ModuleDefinitionId::GlobalConstant(
                            Location::new(file_id, *id).intern(self.database),
                        )
                    })
                },
                ModuleItemId::Override(id) => {
                    let r#override = &item_tree[*id];
                    (&r#override.name == name).then(|| {
                        ModuleDefinitionId::Override(
                            Location::new(file_id, *id).intern(self.database),
                        )
                    })
                },
                ModuleItemId::Function(id) => {
                    let function = &item_tree[*id];
                    (&function.name == name).then(|| {
                        ModuleDefinitionId::Function(
                            Location::new(file_id, *id).intern(self.database),
                        )
                    })
                },
                // TODO: for re-exports, look through the `public import` statements. See https://github.com/wgsl-analyzer/wgsl-analyzer/issues/632
                ModuleItemId::GlobalAssertStatement(_) | ModuleItemId::ImportStatement(_) => None,
            })
    }
}
