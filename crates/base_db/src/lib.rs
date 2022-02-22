mod shader_processor;

pub mod change;
pub mod line_index;

mod util_types;
pub use util_types::*;

use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use line_index::LineIndex;
use syntax::Parse;
pub use vfs::FileId;

pub trait Upcast<T: ?Sized> {
    fn upcast(&self) -> &T;
}

#[salsa::query_group(SourceDatabaseStorage)]
pub trait SourceDatabase {
    #[salsa::input]
    fn file_text(&self, file_id: FileId) -> Arc<String>;

    #[salsa::input]
    fn custom_imports(&self) -> Arc<HashMap<String, String>>;

    #[salsa::input]
    fn shader_defs(&self) -> Arc<HashSet<String>>;

    #[salsa::invoke(parse_no_preprocessor_query)]
    fn parse_no_preprocessor(&self, file_id: FileId) -> syntax::Parse;

    #[salsa::invoke(parse_query)]
    fn parse(&self, file_id: FileId) -> Parse;

    #[salsa::invoke(parse_import_no_preprocessor_query)]
    fn parse_import_no_preprocessor(&self, key: String) -> Result<syntax::Parse, ()>;

    #[salsa::invoke(parse_import_query)]
    fn parse_import(&self, key: String) -> Result<Parse, ()>;

    fn line_index(&self, file_id: FileId) -> Arc<LineIndex>;
}

fn line_index(db: &dyn SourceDatabase, file_id: FileId) -> Arc<LineIndex> {
    let text = db.file_text(file_id);
    Arc::new(LineIndex::new(&*text))
}

fn parse_no_preprocessor_query(db: &dyn SourceDatabase, file_id: FileId) -> syntax::Parse {
    let source = db.file_text(file_id);
    syntax::parse(&*source)
}

fn parse_import_no_preprocessor_query(
    db: &dyn SourceDatabase,
    key: String,
) -> Result<syntax::Parse, ()> {
    let imports = db.custom_imports();
    let source = imports.get(&key).ok_or(())?;
    Ok(syntax::parse(&*source))
}

fn parse_query(db: &dyn SourceDatabase, file_id: FileId) -> Parse {
    let shader_defs = db.shader_defs();
    let source = db.file_text(file_id);

    let processed_source = shader_processor::SHADER_PROCESSOR.process(&source, &shader_defs);
    syntax::parse(&processed_source)
}

fn parse_import_query(db: &dyn SourceDatabase, key: String) -> Result<Parse, ()> {
    let imports = db.custom_imports();
    let shader_defs = db.shader_defs();
    let source = imports.get(&key).ok_or(())?;

    let processed_source = shader_processor::SHADER_PROCESSOR.process(source, &shader_defs);
    Ok(syntax::parse(&processed_source))
}
