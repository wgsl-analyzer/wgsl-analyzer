pub mod input;
mod shader_processor;

pub mod change;

mod util_types;
use input::{SourceRoot, SourceRootId};
use line_index::LineIndex;
pub use util_types::*;
use vfs::{AnchoredPath, VfsPath};

use std::sync::Arc;

use rustc_hash::{FxHashMap, FxHashSet};
use syntax::{Parse, ParseEntryPoint};
pub use vfs::FileId;

pub trait Upcast<T: ?Sized> {
    fn upcast(&self) -> &T;
}

pub trait FileLoader {
    fn resolve_path(
        &self,
        path: AnchoredPath<'_>,
    ) -> Option<FileId>;
}

#[salsa::query_group(SourceDatabaseStorage)]
pub trait SourceDatabase: FileLoader {
    #[salsa::input]
    fn file_text(
        &self,
        file_id: FileId,
    ) -> Arc<String>;

    #[salsa::input]
    fn file_path(
        &self,
        file_id: FileId,
    ) -> VfsPath;

    #[salsa::input]
    fn file_id(
        &self,
        path: VfsPath,
    ) -> FileId;

    #[salsa::input]
    fn custom_imports(&self) -> Arc<FxHashMap<String, String>>;

    #[salsa::input]
    fn shader_defs(&self) -> Arc<FxHashSet<String>>;

    /// Path to a file, relative to the root of its source root.
    /// Source root of the file.
    #[salsa::input]
    fn file_source_root(
        &self,
        file_id: FileId,
    ) -> SourceRootId;
    /// Contents of the source root.
    #[salsa::input]
    fn source_root(
        &self,
        id: SourceRootId,
    ) -> Arc<SourceRoot>;

    #[salsa::invoke(parse_no_preprocessor_query)]
    fn parse_no_preprocessor(
        &self,
        file_id: FileId,
    ) -> syntax::Parse;

    #[salsa::invoke(parse_with_unconfigured_query)]
    fn parse_with_unconfigured(
        &self,
        file_id: FileId,
    ) -> (Parse, Arc<Vec<UnconfiguredCode>>);

    #[salsa::invoke(parse_query)]
    fn parse(
        &self,
        file_id: FileId,
    ) -> Parse;

    #[salsa::invoke(parse_import_no_preprocessor_query)]
    fn parse_import_no_preprocessor(
        &self,
        key: String,
    ) -> Result<syntax::Parse, ()>;

    #[salsa::invoke(parse_import_query)]
    fn parse_import(
        &self,
        key: String,
        parse_entrypoint: ParseEntryPoint,
    ) -> Result<Parse, ()>;

    fn line_index(
        &self,
        file_id: FileId,
    ) -> Arc<LineIndex>;
}

fn line_index(
    db: &dyn SourceDatabase,
    file_id: FileId,
) -> Arc<LineIndex> {
    let text = db.file_text(file_id);
    Arc::new(LineIndex::new(&text))
}

fn parse_no_preprocessor_query(
    db: &dyn SourceDatabase,
    file_id: FileId,
) -> syntax::Parse {
    let source = db.file_text(file_id);
    syntax::parse(&source)
}

fn parse_import_no_preprocessor_query(
    db: &dyn SourceDatabase,
    key: String,
) -> Result<syntax::Parse, ()> {
    let imports = db.custom_imports();
    let source = imports.get(&key).ok_or(())?;
    Ok(syntax::parse(source))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnconfiguredCode {
    pub range: TextRange,
    pub def: String,
}

fn parse_with_unconfigured_query(
    db: &dyn SourceDatabase,
    file_id: FileId,
) -> (Parse, Arc<Vec<UnconfiguredCode>>) {
    let shader_defs = db.shader_defs();
    let source = db.file_text(file_id);

    let mut unconfigured = Vec::new();

    let processed_source =
        shader_processor::get_shader_processor().process(&source, &shader_defs, |range, def| {
            let range = TextRange::new(
                TextSize::from(range.start as u32),
                TextSize::from(range.end as u32),
            );
            unconfigured.push(UnconfiguredCode {
                range,
                def: def.to_string(),
            })
        });
    let parse = syntax::parse(&processed_source);
    (parse, Arc::new(unconfigured))
}

fn parse_query(
    db: &dyn SourceDatabase,
    file_id: FileId,
) -> Parse {
    db.parse_with_unconfigured(file_id).0
}

fn parse_import_query(
    db: &dyn SourceDatabase,
    key: String,
    parse_entrypoint: ParseEntryPoint,
) -> Result<Parse, ()> {
    let imports = db.custom_imports();
    let shader_defs = db.shader_defs();
    let source = imports.get(&key).ok_or(())?;

    let processed_source =
        shader_processor::get_shader_processor().process(source, &shader_defs, |_, _| {});
    Ok(syntax::parse_entrypoint(
        &processed_source,
        parse_entrypoint,
    ))
}

/// Silly workaround for cyclic deps between the traits
pub struct FileLoaderDelegate<T>(pub T);

impl<T: SourceDatabase> FileLoader for FileLoaderDelegate<&'_ T> {
    fn resolve_path(
        &self,
        path: AnchoredPath<'_>,
    ) -> Option<FileId> {
        // FIXME: this *somehow* should be platform agnostic...
        let source_root = self.0.file_source_root(path.anchor);
        let source_root = self.0.source_root(source_root);
        source_root.resolve_path(path)
    }
}
