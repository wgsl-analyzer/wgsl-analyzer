//! The core of the module-level name resolution algorithm.

use base_db::Dependency;
use rustc_hash::{FxHashMap, FxHashSet};
use triomphe::Arc;

use crate::{
    FxIndexMap, HirFileId,
    database::{DefDatabase, ModuleDefinitionId},
    item_tree::{ItemTree, ModuleItem, Name},
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
        // unresolved_imports: Vec::new(),
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
    // unresolved_imports: Vec<ImportDirective>,
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
            module_id: file_id,
            item_tree: &item_tree,
            // mod_dir: ModDir::root(),
        }
        .collect(item_tree.top_level_items());
        Arc::get_mut(&mut self.def_map.data)
            .unwrap()
            .shrink_to_fit();
    }

    fn inject_prelude(&mut self) {
        // Not yet implemented. This is where the wesl standard library could be injected
    }

    fn collect(&mut self) {
        todo!()
    }

    fn finish(mut self) -> DefMap {
        // Emit diagnostics for all remaining unresolved imports.
        /*   for import in &self.unresolved_imports {
                   let &ImportDirective {
                       module_id,
                       import:
                           Import {
                               ref path,
                               source:
                                   ImportSource {
                                       use_tree,
                                       id,
                                       is_prelude: _,
                                       kind: _,
                                   },
                               ..
                           },
                       ..
                   } = import;
                   if matches!(
                       (path.segments().first(), &path.kind),
                       (Some(krate), PathKind::Plain | PathKind::Abs) if self.unresolved_extern_crates.contains(krate)
                   ) {
                       continue;
                   }
                   let item_tree_id = id.lookup(self.db).id;
                   self.def_map
                       .diagnostics
                       .push(DefDiagnostic::unresolved_import(
                           module_id,
                           item_tree_id,
                           use_tree,
                       ));
               }
        */
        self.def_map
    }
}

/// Walks a single module, populating defs and imports
struct ModCollector<'a, 'db> {
    def_collector: &'a mut DefCollector<'db>,
    module_id: HirFileId,
    item_tree: &'a ItemTree,
    // mod_dir: ModDir,
}

impl ModCollector<'_, '_> {
    fn collect(
        &mut self,
        items: &[ModuleItem],
    ) {
        todo!()
    }
}
