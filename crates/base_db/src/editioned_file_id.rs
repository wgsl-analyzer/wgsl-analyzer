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

#[salsa_macros::interned(debug, constructor = from_span_file_id, unsafe(no_lifetime), revisions = usize::MAX)]
#[derive(PartialOrd, Ord)]
pub struct EditionedFileId {
    field: RawEditionedFileId,
}

#[salsa::tracked]
impl EditionedFileId {
    #[salsa::tracked(lru = 128, returns(clone))]
    pub fn parse(
        self,
        db: &dyn SourceDatabase,
    ) -> syntax::Parse {
        let _p = tracing::info_span!("parse", ?self).entered();
        let RawEditionedFileId { file_id, edition } = self.unpack(db);
        let text = db.file_text(file_id).text(db);
        syntax::parse(text, edition)
    }

    // firewall query
    #[salsa::tracked(returns(as_deref))]
    pub fn parse_errors(
        self,
        db: &dyn SourceDatabase,
    ) -> Option<Box<[Diagnostic]>> {
        let parse = self.parse(db);
        let errors = parse.errors();
        match errors {
            [] => None,
            [..] => Some(errors.into()),
        }
    }
}

impl EditionedFileId {
    /// Warning: Prefer [`from_file`] to get the correct edition for WGSL and WESL files.
    #[inline]
    pub fn new_unchecked(
        db: &dyn Database,
        file_id: FileId,
        edition: Edition,
    ) -> Self {
        Self::from_span_file_id(db, RawEditionedFileId { file_id, edition })
    }

    pub fn from_file(
        db: &dyn SourceDatabase,
        file_id: FileId,
    ) -> Self {
        let source_root = db
            .source_root(db.file_source_root(file_id).source_root_id(db))
            .source_root(db);

        let extension = match FileExtension::from_file(&source_root, file_id) {
            Ok(extension) => extension,
            Err(error) => {
                tracing::error!("{error}");
                return Self::new_unchecked(db, file_id, Edition::DEFAULT);
            },
        };

        Self::from_file_with_extension(db, file_id, extension)
    }

    pub fn from_file_with_extension(
        db: &dyn SourceDatabase,
        file_id: FileId,
        extension: FileExtension,
    ) -> Self {
        match extension {
            FileExtension::Wgsl => Self::new_unchecked(db, file_id, Edition::DEFAULT),
            FileExtension::Wesl => {
                if let Some(package) = file_package(db, file_id) {
                    Self::new_unchecked(db, file_id, package.data(db).edition)
                } else {
                    // Assume latest WESL for standalone files
                    Self::new_unchecked(db, file_id, Edition::LATEST)
                }
            },
        }
    }

    #[inline]
    pub fn file_id(
        self,
        db: &dyn Database,
    ) -> vfs::FileId {
        self.field(db).file_id
    }

    #[inline]
    pub fn edition(
        self,
        db: &dyn Database,
    ) -> Edition {
        self.field(db).edition
    }

    #[inline]
    pub fn unpack(
        self,
        db: &dyn Database,
    ) -> RawEditionedFileId {
        *self.field(db)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FileExtension {
    Wgsl,
    Wesl,
}

impl FileExtension {
    pub fn from_file(
        source_root: &SourceRoot,
        file_id: FileId,
    ) -> Result<Self, InvalidFileError> {
        let extension = source_root
            .path_for_file(file_id)
            .ok_or(InvalidFileError::MissingPath)?
            .name_and_extension()
            .ok_or(InvalidFileError::MissingName)?
            .1
            .ok_or(InvalidFileError::MissingExtension)?;

        match extension {
            "wgsl" => Ok(Self::Wgsl),
            "wesl" => Ok(Self::Wesl),
            other => Err(InvalidFileError::InvalidExtension(other.to_owned())),
        }
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
