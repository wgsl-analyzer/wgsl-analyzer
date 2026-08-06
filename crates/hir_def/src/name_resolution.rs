mod collector;
mod diagnostics;

mod modules_map;
#[cfg(test)]
mod tests;

use base_db::{EditionedFileId, Package, SourceDatabase};
pub use collector::collect_module;
pub use diagnostics::{DefDiagnostic, DefDiagnosticKind};
use itertools::Itertools as _;
pub use modules_map::{ModuleData, ModulesMap};

use crate::item_tree::Name;

#[expect(
    clippy::missing_panics_doc,
    reason = "The path manipulation should be infallible"
)]
pub fn resolve_module(
    database: &dyn SourceDatabase,
    package: Package,
    segments: &[Name],
) -> Option<EditionedFileId> {
    let source_root = package.data(database).source_root(database);
    let root_path = &package.data(database).root;

    // package.wesl special case
    if segments.is_empty() {
        return source_root
            .file_for_path(&root_path.join("package.wesl").unwrap())
            .map(|module| EditionedFileId::from_file(database, module));
    }

    let mut module_path: String = segments.iter().map(Name::as_str).join("/");
    module_path.push_str(".wesl");

    source_root
        .file_for_path(&root_path.join(&module_path).unwrap())
        .or_else(|| {
            module_path.replace_range((module_path.len() - 4).., "wgsl");
            source_root.file_for_path(&root_path.join(&module_path).unwrap())
        })
        .map(|module| EditionedFileId::from_file(database, module))
}
