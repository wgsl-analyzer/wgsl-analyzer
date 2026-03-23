#![expect(
    clippy::trailing_empty_array,
    reason = "Clippy has a false positive for the query_group macro, see: https://github.com/rust-lang/rust-clippy/issues/16754"
)]

pub mod change;
pub mod input;

mod editioned_file_id;
mod util_types;
use std::{cell::RefCell, hash::BuildHasherDefault, panic, sync::Once};

pub use crate::editioned_file_id::{EditionedFileId, RawEditionedFileId};
use dashmap::{DashMap, Entry};
pub use input::{SourceRoot, SourceRootId};
use rustc_hash::FxHasher;
pub use salsa;
use salsa::{Durability, Setter as _};
pub use salsa_macros;
use syntax::Parse;
use triomphe::Arc;
pub use util_types::*;
pub use vfs::{AnchoredPath, AnchoredPathBuf, FileId, VfsPath, file_set::FileSet};

#[macro_export]
macro_rules! impl_intern_key {
    ($id:ident, $loc:ty) => {
        #[salsa_macros::interned(no_lifetime)]
        #[derive(PartialOrd, Ord)]
        pub struct $id {
            pub loc: $loc,
        }

        // If we derive this salsa prints the values recursively, and this causes us to blow.
        impl ::std::fmt::Debug for $id {
            fn fmt(
                &self,
                f: &mut ::std::fmt::Formatter<'_>,
            ) -> ::std::fmt::Result {
                f.debug_tuple(stringify!($id))
                    .field(&format_args!("{:04x}", self.0.as_u32()))
                    .finish()
            }
        }
    };
}

#[macro_export]
macro_rules! impl_intern_lookup {
    ($db:ident, $id:ident, $loc:ty, $intern:ident, $lookup:ident) => {
        impl base_db::Intern for $loc {
            type Database = dyn $db;
            type ID = $id;

            fn intern(
                self,
                database: &Self::Database,
            ) -> Self::ID {
                database.$intern(self)
            }
        }

        impl base_db::Lookup for $id {
            type Database = dyn $db;
            type Data = $loc;

            fn lookup(
                &self,
                database: &Self::Database,
            ) -> $loc {
                database.$lookup(*self)
            }
        }
    };
}

pub trait Intern {
    type Database: ?Sized;
    type ID;
    fn intern(
        self,
        database: &Self::Database,
    ) -> Self::ID;
}

pub trait Lookup {
    type Database: ?Sized;
    type Data;
    fn lookup(
        &self,
        database: &Self::Database,
    ) -> Self::Data;
}

#[expect(
    clippy::struct_field_names,
    reason = "Keeping it similar to rust-analyzer"
)]
#[derive(Debug, Default)]
pub struct Files {
    files: Arc<DashMap<vfs::FileId, FileText, BuildHasherDefault<FxHasher>>>,
    source_roots: Arc<DashMap<SourceRootId, SourceRootInput, BuildHasherDefault<FxHasher>>>,
    file_source_roots: Arc<DashMap<vfs::FileId, FileSourceRootInput, BuildHasherDefault<FxHasher>>>,
}

impl Files {
    /// Contents of a file.
    ///
    /// # Panics
    /// If called with a file id that has not been added by the [`Change`]s.
    #[must_use]
    pub fn file_text(
        &self,
        file_id: vfs::FileId,
    ) -> FileText {
        match self.files.get(&file_id) {
            Some(text) => *text,
            None => {
                panic!("Unable to fetch file text for `vfs::FileId`: {file_id:?}; this is a bug")
            },
        }
    }

    pub fn set_file_text(
        &self,
        database: &mut dyn SourceDatabase,
        file_id: vfs::FileId,
        text: &str,
    ) {
        match self.files.entry(file_id) {
            Entry::Occupied(mut occupied) => {
                occupied.get_mut().set_text(database).to(Arc::from(text));
            },
            Entry::Vacant(vacant) => {
                let text = FileText::new(database, Arc::from(text), file_id);
                vacant.insert(text);
            },
        }
    }

    pub fn set_file_text_with_durability(
        &self,
        database: &mut dyn SourceDatabase,
        file_id: vfs::FileId,
        text: &str,
        durability: Durability,
    ) {
        match self.files.entry(file_id) {
            Entry::Occupied(mut occupied) => {
                occupied
                    .get_mut()
                    .set_text(database)
                    .with_durability(durability)
                    .to(Arc::from(text));
            },
            Entry::Vacant(vacant) => {
                let text = FileText::builder(Arc::from(text), file_id)
                    .durability(durability)
                    .new(database);
                vacant.insert(text);
            },
        }
    }

    /// Source root of the file.
    ///
    /// # Panics
    /// If the source root has not been set. This can only happen if there were some incorrect [`Change`]s.
    #[must_use]
    pub fn source_root(
        &self,
        source_root_id: SourceRootId,
    ) -> SourceRootInput {
        let Some(source_root) = self.source_roots.get(&source_root_id) else {
            panic!(
                "Unable to fetch `SourceRootInput` with `SourceRootId` ({source_root_id:?}); this is a bug"
            )
        };
        *source_root
    }

    pub fn set_source_root_with_durability(
        &self,
        database: &mut dyn SourceDatabase,
        source_root_id: SourceRootId,
        source_root: Arc<SourceRoot>,
        durability: Durability,
    ) {
        match self.source_roots.entry(source_root_id) {
            Entry::Occupied(mut occupied) => {
                occupied
                    .get_mut()
                    .set_source_root(database)
                    .with_durability(durability)
                    .to(source_root);
            },
            Entry::Vacant(vacant) => {
                let source_root = SourceRootInput::builder(source_root)
                    .durability(durability)
                    .new(database);
                vacant.insert(source_root);
            },
        }
    }

    /// Gets the source root for a file.
    ///
    /// # Panics
    /// If the source root has not been set. This can only happen if there were some incorrect [`Change`]s.
    #[must_use]
    pub fn file_source_root(
        &self,
        id: vfs::FileId,
    ) -> FileSourceRootInput {
        let Some(file_source_root) = self.file_source_roots.get(&id) else {
            panic!("Unable to get `FileSourceRootInput` with `vfs::FileId` ({id:?}); this is a bug",)
        };
        *file_source_root
    }

    pub fn set_file_source_root_with_durability(
        &self,
        database: &mut dyn SourceDatabase,
        id: vfs::FileId,
        source_root_id: SourceRootId,
        durability: Durability,
    ) {
        match self.file_source_roots.entry(id) {
            Entry::Occupied(mut occupied) => {
                occupied
                    .get_mut()
                    .set_source_root_id(database)
                    .with_durability(durability)
                    .to(source_root_id);
            },
            Entry::Vacant(vacant) => {
                let file_source_root = FileSourceRootInput::builder(source_root_id)
                    .durability(durability)
                    .new(database);
                vacant.insert(file_source_root);
            },
        }
    }
}

#[salsa::input(debug)]
pub struct FileText {
    #[returns(ref)]
    pub text: Arc<str>,
    pub file_id: vfs::FileId,
}

#[salsa::input(debug)]
pub struct FileSourceRootInput {
    pub source_root_id: SourceRootId,
}

#[salsa::input(debug)]
pub struct SourceRootInput {
    pub source_root: Arc<SourceRoot>,
}

/// Database which stores all significant input facts: source code and project
/// model. Everything else in rust-analyzer is derived from these queries.
#[query_group::query_group]
pub trait RootQueryDb: SourceDatabase + salsa::Database {
    #[salsa::invoke(parse)]
    #[salsa::lru(128)]
    fn parse(
        &self,
        key: EditionedFileId,
    ) -> Parse;
}

#[salsa::db]
pub trait SourceDatabase: salsa::Database {
    /// Text of the file.
    fn file_text(
        &self,
        file_id: vfs::FileId,
    ) -> FileText;

    fn set_file_text(
        &mut self,
        file_id: vfs::FileId,
        text: &str,
    );

    fn set_file_text_with_durability(
        &mut self,
        file_id: vfs::FileId,
        text: &str,
        durability: Durability,
    );

    /// Contents of the source root.
    fn source_root(
        &self,
        id: SourceRootId,
    ) -> SourceRootInput;

    fn file_source_root(
        &self,
        id: vfs::FileId,
    ) -> FileSourceRootInput;

    fn set_file_source_root_with_durability(
        &mut self,
        id: vfs::FileId,
        source_root_id: SourceRootId,
        durability: Durability,
    );

    /// Source root of the file.
    fn set_source_root_with_durability(
        &mut self,
        source_root_id: SourceRootId,
        source_root: Arc<SourceRoot>,
        durability: Durability,
    );

    fn resolve_path(
        &self,
        path: AnchoredPath<'_>,
    ) -> Option<FileId> {
        // FIXME: this *somehow* should be platform agnostic...
        let source_root = self.file_source_root(path.anchor);
        let source_root = self.source_root(source_root.source_root_id(self));
        source_root.source_root(self).resolve_path(path)
    }
}

fn parse(
    database: &dyn RootQueryDb,
    file_id: EditionedFileId,
) -> Parse {
    let RawEditionedFileId { file_id, edition } = file_id.unpack(database);
    let source = database.file_text(file_id).text(database);
    syntax::parse(source, edition)
}

#[must_use]
#[non_exhaustive]
pub struct DbPanicContext;

impl Drop for DbPanicContext {
    fn drop(&mut self) {
        Self::with_ctx(|ctx| assert!(ctx.pop().is_some()));
    }
}

impl DbPanicContext {
    pub fn enter(frame: String) -> DbPanicContext {
        #[expect(clippy::print_stderr, reason = "already panicking anyway")]
        fn set_hook() {
            let default_hook = panic::take_hook();
            panic::set_hook(Box::new(move |panic_info| {
                default_hook(panic_info);
                if let Some(backtrace) = salsa::Backtrace::capture() {
                    eprintln!("{backtrace:#}");
                }
                DbPanicContext::with_ctx(|ctx| {
                    if !ctx.is_empty() {
                        eprintln!("additional context:");
                        for (index, frame) in ctx.iter().enumerate() {
                            eprintln!("{index:>4}: {frame}\n");
                        }
                    }
                });
            }));
        }

        static SET_HOOK: Once = Once::new();
        SET_HOOK.call_once(set_hook);

        Self::with_ctx(|ctx| ctx.push(frame));
        DbPanicContext
    }

    fn with_ctx(f: impl FnOnce(&mut Vec<String>)) {
        thread_local! {
            static CTX: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
        }
        CTX.with(|ctx| f(&mut ctx.borrow_mut()));
    }
}
