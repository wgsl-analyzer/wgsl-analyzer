use base_db::{FileRange, RangeInfo};
use hir::{HirDatabase, Semantics};
use hir_def::InFile;
use syntax::{ast, AstNode};

pub enum HoverResult {
    SourceCode(String),
    Text(String),
}

pub fn hover(db: &dyn HirDatabase, file_range: FileRange) -> Option<RangeInfo<HoverResult>> {
    let sema = &Semantics::new(db);

    let file = db.parse(file_range.file_id).tree();

    let import = file
        .syntax()
        .token_at_offset(file_range.range.start())
        .right_biased()?
        .parent()?
        .ancestors()
        .find_map(ast::Import::cast);

    if let Some(import) = import {
        let import = sema.resolve_import(InFile::new(file_range.file_id.into(), import))?;

        if !import.is_path(db) {
            return Some(RangeInfo {
                range: file_range.range,
                info: HoverResult::SourceCode(import.file_text(db)?),
            });
        }
    }

    None
}
