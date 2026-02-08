//! The core of the module-level name resolution algorithm.
use crate::{
    HasSource, InFile,
    database::Lookup,
    mod_path::{ModPath, PathKind},
    nameres::{ModuleData, diagnostics::DefDiagnostic, path_resolution::ResolvePathResult},
};
use base_db::{Dependency, EditionedFileId};
use rustc_hash::{FxHashMap, FxHashSet};
use triomphe::Arc;
use vfs::{AnchoredPath, FileId};

use crate::{
    FxIndexMap, HirFileId,
    database::{DefDatabase, Location, ModuleDefinitionId},
    item_tree::{FlatImport, ImportStatement, ItemTree, ModuleItem, Name},
    nameres::DefMap,
};

pub(super) fn collect_defs(
    database: &dyn DefDatabase,
    def_map: DefMap,
    file_id: HirFileId,
) -> DefMap {
    let package_graph = database.package_graph();
    let package = &package_graph[def_map.package];

    // populate dependency list
    let mut deps =
        FxIndexMap::with_capacity_and_hasher(package.dependencies.len(), Default::default());
    for dep in &package.dependencies {
        deps.insert(Name::from(dep.name.as_str()), dep.clone());
    }

    let mut collector = DefCollector {
        database,
        def_map,
        deps,
        // glob_imports: FxHashMap::default(),
        unresolved_imports: Vec::new(),
        unresolved_extern_crates: Default::default(),
        // from_glob_import: Default::default(),
    };
    collector.seed(file_id);
    collector.collect();
    let mut def_map = collector.finish();
    def_map.shrink_to_fit();
    def_map
}

/// Walks the tree of modules recursively
struct DefCollector<'db> {
    database: &'db dyn DefDatabase,
    def_map: DefMap,
    // The dependencies of the current crate, including optional deps like `test`.
    deps: FxIndexMap<Name, Dependency>,
    // glob_imports: FxHashMap<LocalModuleId, Vec<(LocalModuleId, Visibility, GlobId)>>,
    unresolved_imports: Vec<(Location<ImportStatement>, FlatImport)>,
    // We'd like to avoid emitting a diagnostics avalanche when some `extern crate` doesn't
    // resolve. When we emit diagnostics for unresolved imports, we only do so if the import
    // doesn't start with an unresolved crate's name.
    unresolved_extern_crates: FxHashSet<Name>,
    // from_glob_import: PerNsGlobImports,
}

impl<'db> DefCollector<'db> {
    fn seed(
        &mut self,
        file_id: HirFileId,
    ) {
        let item_tree = self.database.item_tree(file_id);
        self.inject_prelude();
        ModCollector {
            def_collector: self,
            file_id,
            item_tree: &item_tree,
        }
        .collect(item_tree.top_level_items());
    }

    fn inject_prelude(&mut self) {
        // Not yet implemented. This is where the wesl standard library could be injected
    }

    fn collect(&mut self) {
        // To read a file, look in mod_resolution.rs (fn resolve_declaration)
        while let Some((location, unresolved_import)) = self.unresolved_imports.pop() {
            self.database.unwind_if_cancelled();

            let file_id = location.file_id.original_file(self.database).file_id;

            // The file has already been collected
            // Now resolve the imports
            let resolved_import =
                self.resolve_import_with_modules(file_id, location, &unresolved_import);

            if let Ok(resolved) = resolved_import {
                // If we do not have a leaf name, there are a few possible cases
                // - PathKind::Plain => Must have a leaf name, otherwise the path is completely empty
                // - PathKind::Super => Don't need to add `super` to the scope, it is already a keyword
                // - PathKind::Package => Don't need to add `package` to the scope, it is already a keyword
                if let Some(name) = unresolved_import.leaf_name() {
                    let module_data = &mut self.def_map.modules[file_id];
                    module_data
                        .scope
                        .push_item(name.clone(), resolved.resolved_def);
                }
            }
        }
    }

    fn collect_child_module(
        &mut self,
        module_id: EditionedFileId,
        name: Name,
    ) {
        if self.def_map.modules.contains_key(&module_id.file_id) {
            return;
        }
        let file_id = HirFileId::from(module_id);
        let item_tree = self.database.item_tree(file_id);
        self.def_map
            .modules
            .insert(module_id.file_id, ModuleData::new(module_id, Some(name)));
        ModCollector {
            def_collector: self,
            file_id,
            item_tree: &item_tree,
        }
        .collect(item_tree.top_level_items());
    }

    // Resolves an import while also force-loading modules
    fn resolve_import_with_modules(
        &mut self,
        mut file_id: FileId,
        location: Location<ImportStatement>,
        import: &FlatImport,
    ) -> Result<ResolvePathResult, ()> {
        file_id = match import.path.kind() {
            PathKind::Plain => {
                // TODO:
                tracing::warn!("importing libraries is not yet implemented");
                return Err(());
            },
            PathKind::Super(levels) => {
                // Parent modules are guaranteed to exist and be loaded all the way until the root.
                for _ in 0..levels {
                    if let Some(parent) = self.def_map.modules[file_id].parent {
                        file_id = parent;
                    } else {
                        self.def_map
                            .diagnostics
                            .push(DefDiagnostic::super_escaping_root(file_id, location));
                        return Err(());
                    }
                }
                file_id
            },
            PathKind::Package => self.def_map.crate_root(),
        };

        for (index, segment) in import.path.segments().iter().enumerate() {
            self.database.unwind_if_cancelled();

            // Check in current module
            let module_data = &self.def_map.modules[file_id];
            if let Some(resolved_def) = module_data.scope.get(segment) {
                if index < import.path.segments().len() - 1 {
                    // Not at the last segment
                    self.def_map
                        .diagnostics
                        .push(DefDiagnostic::unresolved_import(file_id, location));
                    return Err(());
                }
                return Ok(ResolvePathResult {
                    resolved_def,
                    segment_index: Some(index),
                });
            }
            // Otherwise go to the child file
            if let Some(child_module) = module_data.children.get(segment) {
                file_id = *child_module;
            } else {
                match self.resolve_child_module(file_id, segment) {
                    Ok(resolved) => {
                        self.collect_child_module(
                            EditionedFileId {
                                file_id: resolved,
                                edition: self.def_map.edition(),
                            },
                            segment.clone(),
                        );
                        file_id = resolved;
                    },
                    Err(candidate_files) => {
                        self.def_map
                            .diagnostics
                            .push(DefDiagnostic::unresolved_module(
                                file_id,
                                location,
                                candidate_files,
                            ));
                    },
                }
            }
        }
        // We got to the end of the resolution
        Ok(ResolvePathResult {
            resolved_def: ModuleDefinitionId::Module(EditionedFileId {
                file_id,
                edition: self.def_map.edition(),
            }),
            segment_index: None,
        })
    }

    fn resolve_child_module(
        &self,
        file_id: FileId,
        child_name: &Name,
    ) -> Result<FileId, Vec<String>> {
        let module_data = &self.def_map.modules[file_id];
        let dir_path = module_data
            .name
            .as_ref()
            .map(|name| name.as_str())
            .unwrap_or_default();

        let candidate_files = [
            format!("{}{}.wesl", dir_path, child_name.as_str()),
            format!("{}{}.wgsl", dir_path, child_name.as_str()),
        ]
        .to_vec();

        // Load the file
        for candidate in &candidate_files {
            let path = AnchoredPath {
                anchor: file_id,
                path: candidate.as_str(),
            };
            if let Some(file_id) = self.database.resolve_path(path) {
                return Ok((file_id));
            }
        }

        Err(candidate_files)
    }

    fn finish(mut self) -> DefMap {
        // Emit diagnostics for all remaining unresolved imports.
        for (location, import) in self.unresolved_imports {
            if matches!(
                (import.path.segments().first(), &import.path.kind()),
                (Some(krate), PathKind::Plain) if self.unresolved_extern_crates.contains(krate)
            ) {
                continue;
            }
            self.def_map
                .diagnostics
                .push(DefDiagnostic::unresolved_import(
                    location.file_id.original_file(self.database).file_id,
                    location,
                ));
        }

        self.def_map
    }
}

/// Walks a single module, populating defs and imports
struct ModCollector<'a, 'db> {
    def_collector: &'a mut DefCollector<'db>,
    file_id: HirFileId,
    item_tree: &'a ItemTree,
}

impl ModCollector<'_, '_> {
    fn collect(
        &mut self,
        items: &[ModuleItem],
    ) {
        let database = self.def_collector.database;
        let hir_file_id = self.file_id;
        let item_scope =
            &mut self.def_collector.def_map[hir_file_id.original_file(database).file_id].scope;
        for item in items {
            match *item {
                ModuleItem::ImportStatement(id) => {
                    self.item_tree.get(id).expand(|flat_import| {
                        self.def_collector
                            .unresolved_imports
                            .push((InFile::new(hir_file_id, id), flat_import));
                    });
                },
                ModuleItem::Function(id) => {
                    let def = ModuleDefinitionId::Function(
                        database.intern_function(Location::new(hir_file_id, id)),
                    );
                    item_scope.declare(def);
                    item_scope.push_item(self.item_tree.get(id).name.clone(), def);
                },
                ModuleItem::Struct(id) => {
                    let def = ModuleDefinitionId::Struct(
                        database.intern_struct(Location::new(hir_file_id, id)),
                    );
                    item_scope.declare(def);
                    item_scope.push_item(self.item_tree.get(id).name.clone(), def);
                },
                ModuleItem::GlobalVariable(id) => {
                    let def = ModuleDefinitionId::GlobalVariable(
                        database.intern_global_variable(Location::new(hir_file_id, id)),
                    );
                    item_scope.declare(def);
                    item_scope.push_item(self.item_tree.get(id).name.clone(), def);
                },
                ModuleItem::GlobalConstant(id) => {
                    let def = ModuleDefinitionId::GlobalConstant(
                        database.intern_global_constant(Location::new(hir_file_id, id)),
                    );
                    item_scope.declare(def);
                    item_scope.push_item(self.item_tree.get(id).name.clone(), def);
                },
                ModuleItem::Override(id) => {
                    let def = ModuleDefinitionId::Override(
                        database.intern_override(Location::new(hir_file_id, id)),
                    );
                    item_scope.declare(def);
                    item_scope.push_item(self.item_tree.get(id).name.clone(), def);
                },
                ModuleItem::TypeAlias(id) => {
                    let def = ModuleDefinitionId::TypeAlias(
                        database.intern_type_alias(Location::new(hir_file_id, id)),
                    );
                    item_scope.declare(def);
                    item_scope.push_item(self.item_tree.get(id).name.clone(), def);
                },
                ModuleItem::GlobalAssertStatement(_) => (),
            }
        }
    }
}
