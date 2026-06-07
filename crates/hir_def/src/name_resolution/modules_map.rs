use base_db::{EditionedFileId, Package, SourceDatabase, SourceRoot, file_package};
use std::fmt::Write as _;
use vfs::FileId;

use crate::{FxIndexMap, item_tree::Name};

/// A map of all modules and their children in a package.
///
/// Used for name resolution.
/// Can also be used to iterate over all modules in a package to discover all symbols or all unit tests.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ModulesMap {
    pub root: FileId,
    /// All modules in the project, including unreachable modules.
    pub modules: FxIndexMap<FileId, ModuleData>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ModuleData {
    /// What is the name of this module, when looking at the absolute module path.
    /// Is empty when it is the root module.
    pub name: Option<Name>,
    /// Where does this module come from?
    pub origin: EditionedFileId,
    /// Declared visibility of this module.
    // pub visibility: Visibility,
    /// Parent module in the same `DefMap`.
    pub parent: Option<FileId>,
    pub children: FxIndexMap<Name, FileId>,
}

impl ModuleData {
    fn new(origin: EditionedFileId) -> Self {
        Self {
            name: None,
            origin,
            parent: None,
            children: FxIndexMap::default(),
        }
    }
}

// TODO: Look into the incrementality of this
#[salsa_macros::tracked(returns(ref))]
pub fn module_data<'db>(
    database: &'db dyn SourceDatabase,
    file_id: EditionedFileId,
) -> Option<ModuleData> {
    let raw_file_id = file_id.file_id(database);
    let package = file_package(database, raw_file_id)?;
    let modules = modules_map_query(database, package);

    // TODO: Is cloned the right thing to use here?
    modules.modules.get(&raw_file_id).cloned()
}

#[salsa_macros::tracked(returns(ref))]
pub fn modules_map_query(
    database: &dyn SourceDatabase,
    package: Package,
) -> ModulesMap {
    let package_data = package.data(database);
    let source_root = database
        .source_root(
            database
                .file_source_root(package_data.root_file_id)
                .source_root_id(database),
        )
        .source_root(database);

    let mut modules: FxIndexMap<_, _> = source_root
        .iter()
        .map(|file_id| {
            let origin = EditionedFileId::new(database, file_id, package_data.edition);
            (file_id, ModuleData::new(origin))
        })
        .collect();

    for file_id in source_root.iter() {
        add_file(&mut modules, file_id, &source_root);
    }

    ModulesMap {
        root: package_data.root_file_id,
        modules,
    }
}

fn add_file(
    modules: &mut FxIndexMap<FileId, ModuleData>,
    file_id: FileId,
    source_root: &SourceRoot,
) -> Option<()> {
    let path = source_root.path_for_file(file_id)?;
    let (name, extension) = path.name_and_extension()?;
    if !matches!(extension, Some("wesl" | "wgsl")) {
        return None;
    }
    let name = Name::from(name);

    let file_node = &mut modules[&file_id];
    file_node.name = Some(name.clone());

    // TODO: This cannot account for the case where a module is missing. After all, missing modules do not have a file id.
    // > File not found: We assume an empty module as the current module, and continue with that.
    // > https://github.com/wgsl-tooling-wg/wesl-spec/blob/main/Imports.md#import-resolution-algorithm
    let parent_path = get_parent_path(path)?;
    if let Some(parent_id) = source_root.file_for_path(&parent_path) {
        file_node.parent = Some(*parent_id);
        modules[parent_id].children.insert(name, file_id);
    }
    Some(())
}

/// Goes from a path like `foo/bar.wesl` to `foo.wesl`.
fn get_parent_path(path: &vfs::VfsPath) -> Option<vfs::VfsPath> {
    let mut parent_path = path.parent()?;
    let (name, extension) = parent_path.name_and_extension()?;
    if extension.is_some() {
        return None;
    }
    // Only wesl files can have child modules
    let file_name = format!("{name}.wesl");
    parent_path.pop();
    parent_path.join(&file_name)
}

impl ModulesMap {
    #[must_use]
    pub fn dump(&self) -> String {
        let mut buffer = String::new();
        go(&mut buffer, self, "package", self.root);
        return buffer;

        fn go(
            buffer: &mut String,
            modules: &ModulesMap,
            path: &str,
            module: FileId,
        ) {
            _ = writeln!(buffer, "{path}");

            let mut children: Vec<_> = modules.modules[&module].children.iter().collect();
            children.sort_by(|(name_a, _), (name_b, _)| Ord::cmp(name_a, name_b));
            for (name, child) in children {
                let path = format!("{path}::{}", name.as_str());
                go(buffer, modules, &path, *child);
            }
        }
    }
}
