use base_db::{EditionedFileId, Package, SourceDatabase, SourceRoot, file_package};
use std::fmt::Write as _;
use syntax::Edition;
use vfs::FileId;

use crate::{FxIndexMap, database::DefDatabase, item_scope::ItemScope, item_tree::Name};

/// A map of all modules and their children in a package.
///
/// Used for name resolution.
/// Can also be used to iterate over all modules in a package to discover all symbols or all unit tests.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ModulesMap {
    pub root: EditionedFileId,
    /// All reachable modules in the project.
    pub modules: FxIndexMap<EditionedFileId, ModuleData>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ModuleData {
    /// What is the name of this module, when looking at the absolute module path.
    /// Is empty when it is the root module.
    pub name: Option<Name>,
    /// The file of the module.
    pub origin: EditionedFileId,
    /// Declared visibility of this module.
    // pub visibility: Visibility,
    /// Parent module in the same [`ModulesMap`].
    pub parent: Option<EditionedFileId>,
    pub children: FxIndexMap<Name, EditionedFileId>,
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

#[salsa::tracked]
impl ModuleData {
    #[salsa::tracked(returns(ref))]
    // TODO: Look into the incrementality of this
    /// Returns the parent and children of the module.
    /// Will return `None` for modules that are not a part of any package.
    pub fn of(
        database: &dyn SourceDatabase,
        file_id: EditionedFileId,
    ) -> Option<ModuleData> {
        let raw_file_id = file_id.file_id(database);
        let package = file_package(database, raw_file_id)?;
        let modules = modules_map_query(database, package);

        // TODO: Is cloned the right thing to use here?
        modules.modules.get(&file_id).cloned()
    }
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

    let root = EditionedFileId::new(database, package_data.root_file_id, package_data.edition);

    // package.wesl at the root gets special treatment. It assumes that the children are adjacent to it.
    let package_wesl = source_root
        .path_for_file(package_data.root_file_id)
        .filter(|path| path.name_and_extension() == Some(("package", Some("wesl"))))
        .map(|_| package_data.root_file_id);

    let modules: FxIndexMap<_, _> = source_root
        .iter()
        .map(|file_id| {
            let file_id = EditionedFileId::new(database, file_id, package_data.edition);
            (file_id, ModuleData::new(file_id))
        })
        .collect();

    let mut modules_map = ModulesMap { root, modules };
    for file_id in source_root.iter() {
        modules_map.add_file(
            database,
            file_id,
            package_data.edition,
            &source_root,
            package_wesl,
        );
    }
    // Clear the name of the root module, since we only want names that can be used in shader code
    modules_map.modules[&root].name = None;

    modules_map.keep_reachable();

    modules_map
}

impl ModulesMap {
    fn add_file(
        &mut self,
        database: &dyn SourceDatabase,
        raw_file_id: FileId,
        edition: Edition,
        source_root: &SourceRoot,
        package_wesl: Option<FileId>,
    ) -> Option<()> {
        if Some(raw_file_id) == package_wesl {
            return Some(());
        }

        let path = source_root.path_for_file(raw_file_id)?;
        let (name, extension) = path.name_and_extension()?;
        if !matches!(extension, Some("wesl" | "wgsl")) {
            return None;
        }
        let name = Name::from(name);

        let file_id = EditionedFileId::new(database, raw_file_id, edition);
        self.modules[&file_id].name = Some(name.clone());

        // TODO: This cannot account for the case where a module is missing. After all, missing modules do not have a file id.
        // > File not found: We assume an empty module as the current module, and continue with that.
        // > https://github.com/wgsl-tooling-wg/wesl-spec/blob/main/Imports.md#import-resolution-algorithm

        let parent = package_wesl
            .filter(|id| get_package_parent(path, source_root) == Some(*id))
            .or_else(|| get_parent(path, source_root));

        if let Some(parent_id) = parent {
            let parent_id = EditionedFileId::new(database, parent_id, edition);
            // .wesl files will shadow .wgsl files
            let parent_module = &mut self.modules[&parent_id];
            let is_slot_empty = !parent_module.children.contains_key(&name);
            if extension == Some("wesl") || is_slot_empty {
                parent_module.children.insert(name, file_id);
                self.modules[&file_id].parent = Some(parent_id);
            }
        }
        Some(())
    }

    fn keep_reachable(&mut self) {
        fn keep_children(
            root: EditionedFileId,
            old_modules: &mut FxIndexMap<EditionedFileId, ModuleData>,
            new_modules: &mut FxIndexMap<EditionedFileId, ModuleData>,
        ) {
            let module = old_modules.swap_remove(&root).unwrap();
            for child_id in module.children.values() {
                keep_children(*child_id, old_modules, new_modules);
            }
            new_modules.insert(root, module);
        }
        let mut new_modules = FxIndexMap::default();
        keep_children(self.root, &mut self.modules, &mut new_modules);

        self.modules = new_modules;
        self.modules.shrink_to_fit();
    }

    #[must_use]
    pub fn dump(&self) -> String {
        fn go(
            buffer: &mut String,
            modules: &ModulesMap,
            path: &str,
            module: EditionedFileId,
        ) {
            _ = writeln!(buffer, "{path}");

            let mut children: Vec<_> = modules.modules[&module].children.iter().collect();
            children.sort_by(|(name_a, _), (name_b, _)| Ord::cmp(name_a, name_b));
            for (name, child) in children {
                let path = format!("{path}::{}", name.as_str());
                go(buffer, modules, &path, *child);
            }
        }

        let mut buffer = String::new();
        go(&mut buffer, self, "package", self.root);
        return buffer;
    }

    #[must_use]
    pub fn dump_with_items(
        &self,
        database: &dyn DefDatabase,
    ) -> String {
        fn go(
            buffer: &mut String,
            modules: &ModulesMap,
            path: &str,
            module: EditionedFileId,
            database: &dyn DefDatabase,
        ) {
            _ = writeln!(buffer, "{path}");
            ItemScope::of(database, module).dump(buffer);

            let mut children: Vec<_> = modules.modules[&module].children.iter().collect();
            children.sort_by(|(name_a, _), (name_b, _)| Ord::cmp(name_a, name_b));
            for (name, child) in children {
                let path = format!("{path}::{}", name.as_str());
                go(buffer, modules, &path, *child, database);
            }
        }

        let mut buffer = String::new();
        go(&mut buffer, self, "package", self.root, database);
        return buffer;
    }
}

/// Goes from a path like `foo/bar.wesl` to `foo.wesl`.
fn get_parent(
    path: &vfs::VfsPath,
    source_root: &SourceRoot,
) -> Option<FileId> {
    let mut parent_path = path.parent()?;
    let (name, extension) = parent_path.name_and_extension()?;
    if extension.is_some() {
        return None;
    }
    // Only wesl files can have child modules
    let file_name = format!("{name}.wesl");
    parent_path.pop();
    source_root.file_for_path(&parent_path.join(&file_name)?)
}

/// Goes from a path like `foo.wesl` to `package.wesl`.
fn get_package_parent(
    path: &vfs::VfsPath,
    source_root: &SourceRoot,
) -> Option<FileId> {
    let mut parent_path = path.parent()?;
    source_root.file_for_path(&parent_path.join("package.wesl")?)
}
