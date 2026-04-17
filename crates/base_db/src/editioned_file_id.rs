//! Defines [`EditionedFileId`], an interned wrapper around [`RawEditionedFileId`] that
//! is interned (so queries can take it) and stores only the underlying `span::EditionedFileId`.

use salsa::Database;
use syntax::{Diagnostic, Edition, ast};
use vfs::FileId;

use crate::SourceDatabase;

/// File together with an edition.
/// Simpler than Rust-Analyzer, because we do not macros.
/// We only track the editions at a file level, as opposed to tracking it per span.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct RawEditionedFileId {
    pub file_id: FileId,
    pub edition: Edition,
}

#[salsa_macros::interned(debug, constructor = from_span_file_id, no_lifetime, revisions = usize::MAX)]
#[derive(PartialOrd, Ord)]
pub struct EditionedFileId {
    field: RawEditionedFileId,
}

impl EditionedFileId {
    pub fn parse(
        self,
        database: &dyn SourceDatabase,
    ) -> syntax::Parse {
        #[salsa::tracked(lru = 128)]
        pub fn parse(
            database: &dyn SourceDatabase,
            file_id: EditionedFileId,
        ) -> syntax::Parse {
            let _p = tracing::info_span!("parse", ?file_id).entered();
            let (file_id, edition) = (file_id.file_id(database), file_id.edition(database));
            let text = database.file_text(file_id).text(database);
            syntax::parse(text, edition)
        }
        parse(database, self)
    }

    // firewall query
    pub fn parse_errors(
        self,
        database: &dyn SourceDatabase,
    ) -> Option<&[Diagnostic]> {
        #[salsa::tracked(returns(as_deref))]
        pub fn parse_errors(
            database: &dyn SourceDatabase,
            file_id: EditionedFileId,
        ) -> Option<Box<[Diagnostic]>> {
            let parse = file_id.parse(database);
            let errors = parse.errors();
            match errors {
                [] => None,
                [..] => Some(errors.into()),
            }
        }
        parse_errors(database, self)
    }
}

impl EditionedFileId {
    #[inline]
    pub fn new(
        database: &dyn Database,
        file_id: FileId,
        edition: Edition,
    ) -> Self {
        Self::from_span_file_id(database, RawEditionedFileId { file_id, edition })
    }

    pub fn from_file(
        database: &dyn SourceDatabase,
        file_id: FileId,
    ) -> Self {
        let source_root = database
            .source_root(database.file_source_root(file_id).source_root_id(database))
            .source_root(database);
        let edition = if let Some((_, Some(extension))) = source_root
            .path_for_file(file_id)
            .and_then(|file| file.name_and_extension())
        {
            if extension.eq_ignore_ascii_case("wesl") {
                Edition::LATEST
            } else if extension.eq_ignore_ascii_case("wgsl") {
                Edition::Wgsl
            } else {
                Edition::CURRENT
            }
        } else {
            Edition::CURRENT
        };

        Self::new(database, file_id, edition)
    }

    #[inline]
    pub fn file_id(
        self,
        database: &dyn Database,
    ) -> vfs::FileId {
        self.field(database).file_id
    }

    #[inline]
    pub fn edition(
        self,
        database: &dyn Database,
    ) -> Edition {
        self.field(database).edition
    }

    #[inline]
    pub fn unpack(
        self,
        database: &dyn Database,
    ) -> RawEditionedFileId {
        self.field(database)
    }
}
