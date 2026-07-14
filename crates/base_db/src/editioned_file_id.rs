//! Defines [`EditionedFileId`], an interned wrapper around [`RawEditionedFileId`] that
//! is interned (so queries can take it) and stores only the underlying `span::EditionedFileId`.

use salsa::Database;
use syntax::{Diagnostic, ast};
pub use syntax::{Edition, ExtensionsConfig};
use vfs::FileId;

use crate::{SourceDatabase, SourceRoot, file_package};

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
    /// Warning: Prefer [`from_file`] to get the correct edition for WGSL and WESL files.
    #[inline]
    pub fn new_unchecked(
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
        Self::try_from_file(database, file_id).unwrap_or_else(|error| {
            tracing::error!("{error}");
            Self::new_unchecked(database, file_id, Edition::DEFAULT)
        })
    }

    pub fn try_from_file(
        database: &dyn SourceDatabase,
        file_id: FileId,
    ) -> Result<Self, InvalidFileError> {
        let source_root = database
            .source_root(database.file_source_root(file_id).source_root_id(database))
            .source_root(database);

        let extension = source_root
            .path_for_file(file_id)
            .ok_or(InvalidFileError::MissingPath)?
            .name_and_extension()
            .ok_or(InvalidFileError::MissingName)?
            .1
            .ok_or(InvalidFileError::MissingExtension)?;

        Self::try_with_extension(database, file_id, extension)
            .ok_or_else(|| InvalidFileError::InvalidExtension(extension.to_owned()))
    }

    pub fn try_with_extension(
        database: &dyn SourceDatabase,
        file_id: FileId,
        extension: &str,
    ) -> Option<Self> {
        match extension {
            "wgsl" => Some(Self::new_unchecked(database, file_id, Edition::DEFAULT)),
            "wesl" => {
                if let Some(package) = file_package(database, file_id) {
                    Some(Self::new_unchecked(
                        database,
                        file_id,
                        package.data(database).edition,
                    ))
                } else {
                    // Assume latest WESL for standalone files
                    Some(Self::new_unchecked(database, file_id, Edition::LATEST))
                }
            },
            _ => None,
        }
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

#[derive(Debug, PartialEq, Eq)]
pub enum InvalidFileError {
    MissingPath,
    MissingName,
    MissingExtension,
    InvalidExtension(String),
}

impl std::fmt::Display for InvalidFileError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::MissingPath => write!(f, "File is missing a path."),
            Self::MissingName => write!(f, "File is missing a name."),
            Self::MissingExtension => write!(f, "File is missing an extension."),
            Self::InvalidExtension(extension) => {
                write!(f, "File extension {extension} is invalid.")
            },
        }
    }
}
