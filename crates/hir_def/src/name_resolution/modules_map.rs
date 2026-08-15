use base_db::{EditionedFileId, FileExtension, Package, SourceDatabase};

use crate::{FxIndexMap, mod_path::AbsoluteModPath};

/// A map of all modules and their children in a package.
///
/// Used for name resolution.
/// Can also be used to iterate over all modules in a package to discover all symbols or all unit tests.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ModulesMap {
    /// All folders and modules in the project.
    /// Invariant: If a module path exists, then the parent module path exists.
    pub modules: FxIndexMap<AbsoluteModPath, ModuleData>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ModuleData {
    /// The file of the module.
    pub file: Option<EditionedFileId>,
}

#[salsa::tracked]
impl ModulesMap {
    #[salsa::tracked(returns(ref))]
    pub fn of(
        db: &dyn SourceDatabase,
        package: Package,
    ) -> ModulesMap {
        modules_map_query(db, package)
    }
}

fn modules_map_query(
    db: &dyn SourceDatabase,
    package: Package,
) -> ModulesMap {
    let package_data = package.data(db);
    let source_root = package_data.source_root(db);

    let base_modules: Vec<_> = source_root
        .iter()
        .filter_map(|file_id| {
            let extension = FileExtension::from_file(&source_root, file_id).ok()?;
            let file_id = EditionedFileId::from_file_with_extension(db, file_id, extension);
            let mod_path = AbsoluteModPath::for_file(db, package, file_id)?;
            Some((
                mod_path,
                ModuleData {
                    file: Some(file_id),
                },
                extension,
            ))
        })
        .collect();

    // Invariant: If a module path exists, then the parent module path exists.
    let mut modules = FxIndexMap::default();
    modules.insert(AbsoluteModPath::new_root(), ModuleData { file: None });

    for (module_path, module, extension) in base_modules {
        // Insert modules, making sure to shadow WGSL files
        let is_empty = modules
            .get(&module_path)
            .is_none_or(|module| module.file.is_none());
        if is_empty || extension == FileExtension::Wesl {
            modules.insert(module_path.clone(), module);
        }

        let mut parent_path = module_path;
        while let Some(_) = parent_path.pop_segment()
            && !modules.contains_key(&parent_path)
        {
            modules.insert(parent_path.clone(), ModuleData { file: None });
        }
    }

    ModulesMap { modules }
}
