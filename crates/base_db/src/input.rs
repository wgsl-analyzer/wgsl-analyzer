//! This module specifies the input to wgsl-analyzer. In some sense, this is
//! **the** most important module, because all other fancy stuff is strictly
//! derived from this input.
//!
//! Note that neither this module, nor any other part of the analyzer's core do
//! actual IO. See `vfs` and `project_model` in the `wgsl-analyzer` package for how
//! actual IO is done and lowered to input.

use std::{fmt, ops};

use edition::Edition;
use vfs::{AnchoredPath, FileId, VfsPath, file_set::FileSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SourceRootId(pub u32);

/// The root of the source code.
///
/// Files are grouped into source roots. A source root is a directory on the
/// file systems which is watched for changes.
/// Typically it corresponds to a WESL package.
/// Source roots *might* be nested: in this case, a file belongs to
/// the nearest enclosing source root. Paths to files are always relative to a
/// source root, and the analyzer does not know the root path of the source root at
/// all. So, a file from one source root can't refer to a file in another source
/// root by path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceRoot {
    /// Libraries are considered mostly immutable, this assumption is used to
    /// optimize salsa's query structure.
    is_library: bool,
    file_set: FileSet,
}

impl SourceRoot {
    #[must_use]
    pub const fn new_local(file_set: FileSet) -> Self {
        Self {
            is_library: false,
            file_set,
        }
    }

    #[must_use]
    pub const fn new_library(file_set: FileSet) -> Self {
        Self {
            is_library: true,
            file_set,
        }
    }

    #[must_use]
    pub fn path_for_file(
        &self,
        file: FileId,
    ) -> Option<&VfsPath> {
        self.file_set.path_for_file(&file)
    }

    #[must_use]
    pub fn file_for_path(
        &self,
        path: &VfsPath,
    ) -> Option<&FileId> {
        self.file_set.file_for_path(path)
    }

    #[must_use]
    pub fn resolve_path(
        &self,
        path: AnchoredPath<'_>,
    ) -> Option<FileId> {
        self.file_set.resolve_path(path)
    }

    pub fn iter(&self) -> impl Iterator<Item = FileId> + '_ {
        self.file_set.iter()
    }

    #[must_use]
    pub const fn is_library(&self) -> bool {
        self.is_library
    }
}

/// A small and stable ID for a given package.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct PackageId(u32);

#[expect(
    clippy::absurd_extreme_comparisons,
    reason = "Keeping the code similar to Rust-Analyzer"
)]
impl PackageId {
    pub(crate) const MAX: u32 = u32::MAX;

    /// # Panics
    ///
    /// Panics if the value is larger than [`PackageId::MAX`].
    #[inline]
    #[must_use]
    pub const fn from_raw(raw: u32) -> Self {
        assert!(raw <= Self::MAX);
        Self(raw)
    }

    /// # Panics
    ///
    /// Panics if the value is larger than [`PackageId::MAX`].
    #[inline]
    #[must_use]
    pub fn from_raw_usize(raw: usize) -> Self {
        let raw = u32::try_from(raw).unwrap();
        assert!(raw <= Self::MAX);
        Self(raw)
    }

    #[inline]
    #[must_use]
    pub const fn index(self) -> u32 {
        self.0
    }

    /// # Panics
    ///
    /// Panics if the package ID is too large for a 16 bit system.
    #[inline]
    #[must_use]
    pub fn to_raw_usize(self) -> usize {
        usize::try_from(self.0).unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PackageName(String);

impl PackageName {
    /// Creates a package name, checking for dashes in the string provided.
    /// Dashes are not allowed in the package names,
    /// hence the input string is returned as `Err` for those cases.
    pub fn new(name: &str) -> Result<Self, &str> {
        // TODO: Verify that the package name is a valid WESL ident
        if name.contains('-') {
            Err(name)
        } else {
            Ok(Self(name.to_owned()))
        }
    }

    /// Creates a package name, unconditionally replacing the dashes with underscores.
    #[must_use]
    pub fn normalize_dashes(name: &str) -> Self {
        Self(name.replace('-', "_"))
    }

    #[must_use]
    pub const fn symbol(&self) -> &String {
        &self.0
    }
}

impl fmt::Display for PackageName {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl ops::Deref for PackageName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Origin of the packages.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PackageOrigin {
    /// Packages that are workspace members.
    Local,
    /// Packages that are non-member libraries.
    Library,
    /// Packages that are provided by the language, like builtins, ...
    Language,
}

impl PackageOrigin {
    #[must_use]
    pub fn is_local(self) -> bool {
        self == Self::Local
    }

    #[must_use]
    pub fn is_lib(self) -> bool {
        self == Self::Library
    }

    #[must_use]
    pub fn is_lang(self) -> bool {
        self == Self::Language
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageData {
    /// Root `.wesl` file. If `root = ./some-dir`, we create a virtual file.
    pub root_file_id: FileId,
    pub edition: Edition,
    /// A name used for UI. For purposes of analysis, packages are anonymous.
    /// (only names in `Dependency` matters).
    pub display_name: Option<String>,
    /// The dependencies of this package.
    ///
    /// Note that this may contain more dependencies than the package actually uses.
    /// A common example is the test package which is included but only actually is active when
    /// declared in source via `extern package test`.
    pub dependencies: Vec<Dependency>,
    /// Dependencies that would cause a cycle.
    pub cyclic_dependencies: Vec<Dependency>,
    pub origin: PackageOrigin,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Dependency {
    pub package_id: PackageId,
    pub name: PackageName,
}
