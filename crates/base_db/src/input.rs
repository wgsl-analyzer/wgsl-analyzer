use vfs::{AnchoredPath, FileId, VfsPath, file_set::FileSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SourceRootId(pub u32);

/// The root of the source code.
///
/// Files are grouped into source roots. A source root is a directory on the
/// file systems which is watched for changes. Typically it corresponds to a
/// Rust crate. Source roots *might* be nested: in this case, a file belongs to
/// the nearest enclosing source root. Paths to files are always relative to a
/// source root, and the analyzer does not know the root path of the source root at
/// all. So, a file from one source root can't refer to a file in another source
/// root by path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceRoot {
    /// Sysroot or crates.io library.
    ///
    /// Libraries are considered mostly immutable, this assumption is used to
    /// optimize salsa's query structure
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
