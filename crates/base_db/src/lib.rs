//! Basic database traits.

pub mod change;
pub mod input;

mod editioned_file_id;
mod package;
mod util_types;

use std::{
    cell::RefCell,
    fmt,
    hash::{self, BuildHasherDefault},
    ops, panic,
    sync::{Once, atomic::AtomicUsize},
};

use crate::input::{PackageData, PackageId, PackageName};
use dashmap::{DashMap, Entry};
use rustc_hash::FxHasher;
use salsa::{Durability, Setter as _};
use triomphe::Arc;

pub use crate::editioned_file_id::{
    Capabilities, EditionedFileId, FileExtension, RawEditionedFileId,
};
pub use input::{SourceRoot, SourceRootId};
pub use package::{
    ExtraPackageData, Package, PackageDisplayName, all_packages, builtin_package, file_package,
    set_all_packages_with_durability,
};
pub use salsa;
pub use salsa_macros;
pub use util_types::*;
pub use vfs::{AnchoredPath, AnchoredPathBuf, FileId, VfsPath, VirtualPath, file_set::FileSet};

#[macro_export]
macro_rules! impl_intern_key {
    ($id:ident, $loc:ty) => {
        #[salsa::interned(unsafe(no_lifetime), revisions = usize::MAX)]
        #[derive(PartialOrd, Ord)]
        pub struct $id {
            #[returns(ref)]
            pub location: $loc,
        }

        // If we derive this salsa prints the values recursively, and this causes us to blow.
        impl ::std::fmt::Debug for $id {
            fn fmt(
                &self,
                f: &mut ::std::fmt::Formatter<'_>,
            ) -> ::std::fmt::Result {
                f.debug_tuple(stringify!($id))
                    .field(&format_args!("{:04x}", self.0.index()))
                    .finish()
            }
        }
    };
}

#[macro_export]
macro_rules! impl_intern_lookup {
    ($id:ident, $loc:ty) => {
        impl base_db::Intern for $loc {
            type ID = $id;
            fn intern(
                self,
                db: &dyn ::base_db::SourceDatabase,
            ) -> Self::ID {
                $id::new(db, self)
            }
        }

        impl base_db::Lookup for $id {
            type Data = $loc;

            fn lookup<'db>(
                &self,
                db: &'db dyn ::base_db::SourceDatabase,
            ) -> &'db Self::Data {
                self.location(db)
            }
        }
    };
}

pub trait Intern {
    type ID;
    fn intern(
        self,
        db: &dyn SourceDatabase,
    ) -> Self::ID;
}

pub trait Lookup {
    type Data;
    fn lookup<'db>(
        &self,
        db: &'db dyn SourceDatabase,
    ) -> &'db Self::Data;
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
    /// If called with a file id that has not been added by the [`change::Change`]s.
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
        db: &mut dyn SourceDatabase,
        file_id: vfs::FileId,
        text: &str,
    ) {
        match self.files.entry(file_id) {
            Entry::Occupied(mut occupied) => {
                occupied.get_mut().set_text(db).to(Arc::from(text));
            },
            Entry::Vacant(vacant) => {
                let text = FileText::new(db, Arc::from(text), file_id);
                vacant.insert(text);
            },
        }
    }

    pub fn set_file_text_with_durability(
        &self,
        db: &mut dyn SourceDatabase,
        file_id: vfs::FileId,
        text: &str,
        durability: Durability,
    ) {
        match self.files.entry(file_id) {
            Entry::Occupied(mut occupied) => {
                occupied
                    .get_mut()
                    .set_text(db)
                    .with_durability(durability)
                    .to(Arc::from(text));
            },
            Entry::Vacant(vacant) => {
                let text = FileText::builder(Arc::from(text), file_id)
                    .durability(durability)
                    .new(db);
                vacant.insert(text);
            },
        }
    }

    /// Source root of the file.
    ///
    /// # Panics
    /// If the source root has not been set. This can only happen if there were some incorrect [`change::Change`]s.
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
        db: &mut dyn SourceDatabase,
        source_root_id: SourceRootId,
        source_root: Arc<SourceRoot>,
        durability: Durability,
    ) {
        match self.source_roots.entry(source_root_id) {
            Entry::Occupied(mut occupied) => {
                occupied
                    .get_mut()
                    .set_source_root(db)
                    .with_durability(durability)
                    .to(source_root);
            },
            Entry::Vacant(vacant) => {
                let source_root = SourceRootInput::builder(source_root)
                    .durability(durability)
                    .new(db);
                vacant.insert(source_root);
            },
        }
    }

    /// Gets the source root for a file.
    ///
    /// # Panics
    /// If the source root has not been set. This can only happen if there were some incorrect [`change::Change`]s.
    #[must_use]
    pub fn file_source_root(
        &self,
        id: vfs::FileId,
    ) -> FileSourceRootInput {
        let Some(file_source_root) = self.file_source_roots.get(&id) else {
            panic!("unable to get `FileSourceRootInput` with `vfs::FileId` ({id:?}); this is a bug")
        };
        *file_source_root
    }

    pub fn set_file_source_root_with_durability(
        &self,
        db: &mut dyn SourceDatabase,
        id: vfs::FileId,
        source_root_id: SourceRootId,
        durability: Durability,
    ) {
        match self.file_source_roots.entry(id) {
            Entry::Occupied(mut occupied) => {
                occupied
                    .get_mut()
                    .set_source_root_id(db)
                    .with_durability(durability)
                    .to(source_root_id);
            },
            Entry::Vacant(vacant) => {
                let file_source_root = FileSourceRootInput::builder(source_root_id)
                    .durability(durability)
                    .new(db);
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
    #[returns(copy)]
    pub source_root_id: SourceRootId,
}

#[salsa::input(debug)]
pub struct SourceRootInput {
    #[returns(clone)]
    pub source_root: Arc<SourceRoot>,
}

#[salsa::input(singleton, debug)]
pub struct CapabilitiesInput {
    #[returns(ref)]
    pub capabilities: Capabilities,
}

impl CapabilitiesInput {
    #[must_use]
    pub fn get_capabilities(db: &dyn SourceDatabase) -> &Capabilities {
        Self::get(db).capabilities(db)
    }

    pub fn update_capabilities(
        db: &mut dyn SourceDatabase,
        capabilities: Capabilities,
    ) {
        Self::try_get(db)
            .unwrap_or_else(|| Self::new(db, Capabilities::default()))
            .set_capabilities(db)
            .with_durability(Durability::MEDIUM)
            .to(capabilities);
    }
}

#[salsa::db]
pub trait SourceDatabase: salsa::Database + std::fmt::Debug {
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

    fn nonce_and_revision(&self) -> (Nonce, salsa::Revision);
}

static NEXT_NONCE: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Nonce(usize);

impl Default for Nonce {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Nonce {
    #[inline]
    pub fn new() -> Self {
        Self(NEXT_NONCE.fetch_add(1, std::sync::atomic::Ordering::SeqCst))
    }
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
    pub fn enter(frame: String) -> Self {
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
        Self
    }

    fn with_ctx<Function>(function: Function)
    where
        Function: FnOnce(&mut Vec<String>),
    {
        thread_local! {
            static CTX: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
        }
        CTX.with(|ctx| function(&mut ctx.borrow_mut()));
    }
}
