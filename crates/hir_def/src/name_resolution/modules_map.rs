use base_db::{EditionedFileId, Package, SourceDatabase as _};

use crate::{
    FxIndexMap, database::DefDatabase, item_scope::ItemScope, item_tree::Name,
    mod_path::AbsoluteModPath,
};

/// A map of all modules and their children in a package.
///
/// Used for name resolution.
/// Can also be used to iterate over all modules in a package to discover all symbols or all unit tests.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ModulesMap {
    /// All folders and modules in the project.
    pub modules: FxIndexMap<AbsoluteModPath, ModuleData>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ModuleData {
    /// The file of the module.
    pub file: Option<EditionedFileId>,
}

#[salsa_macros::tracked(returns(ref))]
pub fn modules_map_query(
    database: &dyn DefDatabase,
    package: Package,
) -> ModulesMap {
    let package_data = package.data(database);
    let source_root = package_data.source_root(database);

    let base_modules: Vec<_> = source_root
        .iter()
        .filter_map(|file_id| {
            let (name, extension) = source_root.path_for_file(file_id)?.name_and_extension()?;
            let file_id = EditionedFileId::try_with_extension(database, file_id, extension?)?;
            let mod_path = AbsoluteModPath::for_file(database, package, file_id)?;
            Some((
                mod_path,
                ModuleData {
                    file: Some(file_id),
                },
            ))
        })
        .collect();

    // Invariant: Given a ModPath, the parent ModPath also exists
    let mut modules = FxIndexMap::default();
    modules.insert(AbsoluteModPath::new_root(), ModuleData { file: None });

    for (module_path, module) in base_modules {
        if modules.contains_key(&module_path) {
            continue;
        }
        modules.insert(module_path.clone(), module);

        let mut parent_path = module_path;
        while let Some(_) = parent_path.pop_segment()
            && !modules.contains_key(&parent_path)
        {
            modules.insert(parent_path.clone(), ModuleData { file: None });
        }
    }

    ModulesMap { modules }
}
