use base_db::{EditionedFileId, Intern as _, file_package};
use syntax::ast;

use crate::{
    database::{DefDatabase, Location, ModuleDefinitionId},
    item_scope::{ItemScope, ModuleItem},
    item_tree::{FlatImport, ItemTree, ModuleItemId, Name},
    mod_path::PathKind,
    name_resolution::{ModuleData, diagnostics::DefDiagnostic},
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

/// Walks a single module, populating defs and imports.
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
                    let import_id = location.intern(self.database);
                    item_tree[id].expand(|flat_import| {
                        match self.resolve_import(self.file_id, location, &flat_import) {
                            Ok(definition) => {
                                // If we do not have a leaf name, there are a few possible cases
                                // - PathKind::Plain => Must have a leaf name, otherwise the path is completely empty
                                // - PathKind::Super => Don't need to add `super` to the scope, it is already a keyword
                                // - PathKind::Package => Don't need to add `package` to the scope, it is already a keyword
                                if let Some(name) = flat_import.leaf_name() {
                                    let previous = self.item_scope.push_item(
                                        name.clone(),
                                        ModuleItem {
                                            definition,
                                            visibility: Visibility::File,
                                            import: Some(import_id),
                                        },
                                    );

                                    if let Some(previous) = previous {
                                        self.item_scope.push_diagnostic(
                                            DefDiagnostic::name_conflict(
                                                self.file_id,
                                                Location::new(self.file_id, id.upcast()),
                                                name.clone(),
                                            ),
                                        );
                                    }
                                }
                            },
                            Err(diagnostic) => {
                                self.item_scope.push_diagnostic(diagnostic);
                            },
                        }
                    });
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

            let previous = self.item_scope.push_item(
                name.clone(),
                ModuleItem {
                    definition,
                    visibility: Visibility::Public,
                    import: None,
                },
            );

            if let Some(previous) = previous {
                self.item_scope
                    .push_diagnostic(DefDiagnostic::name_conflict(
                        self.file_id,
                        Location::new(self.file_id, item.ast_id()),
                        name.clone(),
                    ));
            }
        }
    }

    /// Resolve a part of an import statement.
    ///
    /// To avoid cycle handling, we only look at the modules and the item trees.
    /// With that, we can follow an import statement, including re-exports, to the very end.
    /// Re-exported items will cause redundant resolutions.
    fn resolve_import(
        &self,
        mut file_id: EditionedFileId,
        location: Location<ast::ImportStatement>,
        import: &FlatImport,
    ) -> Result<ModuleDefinitionId, DefDiagnostic> {
        match import.path.kind() {
            PathKind::Plain => {
                let name_start = import.path.segments().first().ok_or_else(|| {
                    DefDiagnostic::unresolved_import(file_id, location, Name::missing())
                })?;
                // Local names can shadow an import
                if let Some(resolved_def) = self.resolve_in_module(file_id, name_start) {
                    if import.path.segments().len() > 1 {
                        // Not at the last segment
                        return Err(DefDiagnostic::unresolved_import(
                            file_id,
                            location,
                            name_start.clone(),
                        ));
                    }
                    return Ok(resolved_def);
                }

                let package_data = file_package(self.database, file_id.file_id(self.database))
                    .ok_or_else(|| DefDiagnostic::detached_file(file_id, location))?
                    .data(self.database);

                if let Some(resolved_dependency) = package_data
                    .dependencies
                    .iter()
                    .find(|dep| dep.name.as_str() == name_start.as_str())
                {
                    let dependency_package = resolved_dependency.package(self.database);
                    self.resolve_submodules(
                        dependency_package
                            .data(self.database)
                            .root_file(self.database),
                        location,
                        &import.path.segments()[1..],
                    )
                } else {
                    Err(DefDiagnostic::unresolved_import(
                        file_id,
                        location,
                        name_start.clone(),
                    ))
                }
            },
            PathKind::Super(levels) => {
                for _ in 0..levels {
                    let module_data = ModuleData::of(self.database, file_id)
                        .ok_or_else(|| DefDiagnostic::detached_file(file_id, location))?;
                    if let Some(parent) = module_data.parent {
                        file_id = parent;
                    } else {
                        return Err(DefDiagnostic::super_escaping_root(file_id, location));
                    }
                }
                self.resolve_submodules(file_id, location, import.path.segments())
            },
            PathKind::Package => {
                let package_data = file_package(self.database, file_id.file_id(self.database))
                    .ok_or_else(|| DefDiagnostic::detached_file(file_id, location))?
                    .data(self.database);
                let file_id = package_data.root_file(self.database);
                self.resolve_submodules(file_id, location, import.path.segments())
            },
        }
    }

    fn resolve_submodules(
        &self,
        mut file_id: EditionedFileId,
        location: Location<ast::ImportStatement>,
        segments: &[Name],
    ) -> Result<ModuleDefinitionId, DefDiagnostic> {
        for (index, segment) in segments.iter().enumerate() {
            let is_path_done = index == segments.len() - 1;
            // Check in current module
            if let Some(resolved_def) = self.resolve_in_module(file_id, segment) {
                if is_path_done {
                    return Ok(resolved_def);
                }
                if let ModuleDefinitionId::Module(child) = resolved_def {
                    file_id = child;
                } else {
                    // Not at the last segment
                    return Err(DefDiagnostic::unresolved_import(
                        file_id,
                        location,
                        segment.clone(),
                    ));
                }
            }

            // Otherwise go to the child file
            let module_data = ModuleData::of(self.database, file_id)
                .ok_or_else(|| DefDiagnostic::detached_file(file_id, location))?;
            if let Some(child_module) = module_data.children.get(segment) {
                file_id = *child_module;
            } else {
                return Err(DefDiagnostic::unresolved_import(
                    file_id,
                    location,
                    segment.clone(),
                ));
            }
        }

        // We got to the end of the resolution
        Ok(ModuleDefinitionId::Module(file_id))
    }

    fn resolve_in_module(
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
