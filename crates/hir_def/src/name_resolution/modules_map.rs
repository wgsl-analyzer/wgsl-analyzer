use base_db::{EditionedFileId, Package, SourceDatabase, SourceRoot, file_package};
use std::fmt::Write as _;
use syntax::Edition;
use triomphe::Arc;
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
    pub modules: FxIndexMap<EditionedFileId, Arc<ModuleData>>,
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
    #[salsa::tracked]
    // TODO: Look into the incrementality of this
    /// Returns the parent and children of the module.
    /// Will return `None` for modules that are not a part of any package.
    pub fn of(
        db: &dyn SourceDatabase,
        file_id: EditionedFileId,
    ) -> Option<Arc<ModuleData>> {
        let raw_file_id = file_id.file_id(db);
        let package = file_package(db, raw_file_id)?;
        let modules = modules_map_query(db, package);

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

    let mut builder =
        ModulesMapBuilder::new(database, package_data.root_file(database), &source_root);
    for file_id in source_root.iter() {
        builder.add_file(file_id);
    }
    builder.build()
}

struct ModulesMapBuilder<'db> {
    root: EditionedFileId,
    edition: Edition,
    modules: FxIndexMap<EditionedFileId, ModuleData>,
    source_root: &'db SourceRoot,
    database: &'db dyn SourceDatabase,
    /// package.wesl at the root gets special treatment. It assumes that the children are adjacent to it.
    package_wesl: Option<FileId>,
}

impl<'db> ModulesMapBuilder<'db> {
    fn new(
        database: &'db dyn SourceDatabase,
        root: EditionedFileId,
        source_root: &'db SourceRoot,
    ) -> Self {
        let edition = root.edition(database);
        let modules: FxIndexMap<_, _> = source_root
            .iter()
            .map(|file_id| {
                let file_id =
                    EditionedFileId::from_file_in_source_root(database, file_id, source_root);
                (file_id, ModuleData::new(file_id))
            })
            .collect();

        let root_file_id = root.file_id(database);
        let package_wesl = source_root
            .path_for_file(root_file_id)
            .filter(|path| path.name_and_extension() == Some(("package", Some("wesl"))))
            .map(|_| root_file_id);

        Self {
            root,
            edition,
            modules,
            source_root,
            database,
            package_wesl,
        }
    }

    fn add_file(
        &mut self,
        raw_file_id: FileId,
    ) -> Option<()> {
        if Some(raw_file_id) == self.package_wesl {
            return Some(());
        }

        let path = self.source_root.path_for_file(raw_file_id)?;
        let (name, extension) = path.name_and_extension()?;
        if !matches!(extension, Some("wesl" | "wgsl")) {
            return None;
        }
        let name = Name::from(name);

        let file_id =
            EditionedFileId::from_file_in_source_root(self.database, raw_file_id, self.source_root);
        self.modules[&file_id].name = Some(name.clone());

        // TODO: This cannot account for the case where a module is missing. After all, missing modules do not have a file id.
        // > File not found: We assume an empty module as the current module, and continue with that.
        // > https://github.com/wgsl-tooling-wg/wesl-spec/blob/main/Imports.md#import-resolution-algorithm

        let parent = self
            .package_wesl
            .filter(|id| get_package_parent(path, self.source_root) == Some(*id))
            .or_else(|| get_parent(path, self.source_root));

        if let Some(parent_id) = parent {
            let parent_id = EditionedFileId::new_unchecked(self.database, parent_id, self.edition);
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

    fn build(mut self) -> ModulesMap {
        // Clear the name of the root module, since we only want names that can be used in shader code
        self.modules[&self.root].name = None;

        fn insert_reachable(
            root: EditionedFileId,
            old_modules: &mut FxIndexMap<EditionedFileId, ModuleData>,
            new_modules: &mut FxIndexMap<EditionedFileId, Arc<ModuleData>>,
        ) {
            let module = old_modules.swap_remove(&root).unwrap();
            for child_id in module.children.values() {
                insert_reachable(*child_id, old_modules, new_modules);
            }
            new_modules.insert(root, Arc::new(module));
        }
        let mut new_modules = FxIndexMap::default();
        new_modules.reserve_exact(self.modules.len());
        insert_reachable(self.root, &mut self.modules, &mut new_modules);

        new_modules.shrink_to_fit();

        ModulesMap {
            root: self.root,
            modules: new_modules,
        }
    }
}

impl ModulesMap {
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
        buffer
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
        buffer
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
