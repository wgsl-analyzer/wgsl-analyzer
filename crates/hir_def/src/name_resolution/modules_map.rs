use base_db::{
    EditionedFileId, InternedSourceRootId, Package, SourceDatabase, SourceRoot, file_package,
};
use rustc_hash::FxHashMap;
use vfs::FileId;

use crate::{FxIndexMap, item_tree::Name};

/// Used for
/// - `import foo::|` autocompletions. When I'm typing `foo::|` then I need to know what valid names come next.
/// - Open symbol by name. `all_symbols() => for each crate: crate_symbols() => for each file (ignore visibility): file_symbols()`
/// - Unit test discovery. `unit_tests() => for each file (ignore visibility): file_unit_tests()`
/// - Check all files. `check_all() => for each file (ignore visibility): check()`
///
///
/// Ruff has this kind of info as well <https://github.com/astral-sh/ruff/blob/b409cbeea655e402152d9c0d2057e180d3b660b8/crates/ty_python_semantic/src/semantic_model.rs#L187>
/// Ruff does read children: https://github.com/astral-sh/ruff/blob/b409cbeea655e402152d9c0d2057e180d3b660b8/crates/ty_module_resolver/src/module.rs#L123
/// `#[salsa::tracked] all_submodule_names_for_package(db, module: Module<'db>) -> Option<Vec<Module<'db>>>` is what they got
///
/// Ruff does it tree shaped https://github.com/astral-sh/ruff/blob/a88b2f619c3065de00c75f44903e4d74c37b7796/crates/ty_module_resolver/src/list.rs#L12
/// for all_symbols, import completions (via the list_modules function),
///
/// We can do `import foo::bar;` and `import super::bar;` with the current API. We'd just call `SourceRoot.resolve_path` at the right time. Hmm
///
///  Any query that depends on `fn all_files()` will be re-executed whenever any file is added or removed.
/// How do I get salsa to memoize just the right bit? Like
/// - source root changes => need to recompute the entire "tree" structure
/// - children(file_id) -> Vec<FileId> needs to be one of those tracked functions. It should see that the result hasn't changed and thus skippy skip the rest
/// The only way of beating all that would be by making the Change itself touch the relevant bits. Then the parents and children would be inputs and we'd do set_whatever.
///
/// Also hmm basically no part of the language server does path lookups.
/// There's a bit in r-a that deals with moving and renaming files.
///
#[salsa_macros::tracked]
pub struct ModulesMap<'db> {
    #[tracked]
    #[returns(ref)]
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

// TODO: Look into the incrementality of this
#[salsa_macros::tracked(returns(ref))]
pub fn module_data<'db>(
    database: &'db dyn SourceDatabase,
    file_id: EditionedFileId,
) -> Option<ModuleData> {
    let raw_file_id = file_id.file_id(database);
    let package = file_package(database, raw_file_id)?;
    let modules = package_modules_map(database, package);

    // TODO: Is cloned the right thing to use here?
    modules.modules(database).get(&raw_file_id).cloned()
}

#[salsa_macros::tracked]
pub fn package_modules_map<'db>(
    database: &'db dyn SourceDatabase,
    package: Package,
) -> ModulesMap<'db> {
    let package_data = package.data(database);
    let source_root = database
        .source_root(
            database
                .file_source_root(package_data.root_file_id)
                .source_root_id(database),
        )
        .source_root(database);

    let mut modules = FxIndexMap::from_iter(source_root.iter().map(|file_id| {
        (
            file_id,
            ModuleData {
                name: None,
                origin: EditionedFileId::new(database, file_id, package_data.edition),
                parent: None,
                children: FxIndexMap::default(),
            },
        )
    }));
    for file_id in source_root.iter() {
        add_file(&mut modules, file_id, &source_root);
    }

    ModulesMap::new(database, modules)
}

fn add_file(
    modules: &mut FxIndexMap<FileId, ModuleData>,
    file_id: FileId,
    source_root: &SourceRoot,
) -> Option<()> {
    let path = source_root.path_for_file(file_id)?;
    let (name, extension) = path.name_and_extension()?;
    // TODO: in another place we're doing extension.eq_ignore_ascii_case("wesl")
    // Though I think we are assuming case sensitivity in other parts of the code???
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

/// Goes from a path like `foo/bar.wesl` to `foo.wesl`
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
