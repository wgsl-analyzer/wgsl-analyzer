//! Abstract-ish representation of paths for VFS.

use std::fmt;

use paths::{AbsPath, AbsPathBuf, RelPath};

/// Path in [`Vfs`].
///
/// Long-term, we want to support files which do not reside in the file-system,
/// so we treat `VfsPath`s as opaque identifiers.
///
/// [`Vfs`]: crate::Vfs
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct VfsPath(VfsPathRepr);

impl VfsPath {
    /// Creates an "in-memory" path from `/`-separated string.
    ///
    /// This is most useful for testing, to avoid windows/linux differences.
    #[must_use]
    pub fn new_virtual_path(path: String) -> Self {
        Self(VfsPathRepr::VirtualPath(VirtualPath::new(path)))
    }

    /// Create a path from string. Input should be a string representation of
    /// an absolute path inside filesystem.
    #[must_use]
    pub fn new_real_path(path: String) -> Self {
        Self::from(AbsPathBuf::assert(path.into()))
    }

    /// Returns the `AbsPath` representation of `self` if `self` is on the file system.
    #[must_use]
    pub fn as_path(&self) -> Option<&AbsPath> {
        match &self.0 {
            VfsPathRepr::PathBuf(it) => Some(it.as_path()),
            VfsPathRepr::VirtualPath(_) => None,
        }
    }

    /// Returns the `VirtualPath` representation of `self` if `self` is a virtual path.
    #[must_use]
    pub const fn as_virtual_path(&self) -> Option<&VirtualPath> {
        match &self.0 {
            VfsPathRepr::PathBuf(_) => None,
            VfsPathRepr::VirtualPath(path) => Some(path),
        }
    }

    #[must_use]
    pub fn into_abs_path(self) -> Option<AbsPathBuf> {
        match self.0 {
            VfsPathRepr::PathBuf(it) => Some(it),
            VfsPathRepr::VirtualPath(_) => None,
        }
    }

    /// Creates a new `VfsPath` with `path` adjoined to `self`.
    #[must_use]
    pub fn join(
        &self,
        path: &str,
    ) -> Option<Self> {
        match &self.0 {
            VfsPathRepr::PathBuf(it) => {
                let result = it.join(path).normalize();
                Some(Self(VfsPathRepr::PathBuf(result)))
            },
            VfsPathRepr::VirtualPath(it) => {
                let result = it.join(path)?;
                Some(Self(VfsPathRepr::VirtualPath(result)))
            },
        }
    }

    /// Remove the last component of `self` if there is one.
    ///
    /// If `self` has no component, returns `false`; else returns `true`.
    ///
    /// # Example
    ///
    /// ```ignore
    /// # use vfs::{AbsPathBuf, VfsPath};
    /// let mut path = VfsPath::from(AbsPathBuf::assert("/foo/bar".into()));
    /// assert!(path.pop());
    /// assert_eq!(path, VfsPath::from(AbsPathBuf::assert("/foo".into())));
    /// assert!(path.pop());
    /// assert_eq!(path, VfsPath::from(AbsPathBuf::assert("/".into())));
    /// assert!(!path.pop());
    /// ```
    pub fn pop(&mut self) -> bool {
        match &mut self.0 {
            VfsPathRepr::PathBuf(it) => it.pop(),
            VfsPathRepr::VirtualPath(it) => it.pop(),
        }
    }

    /// Returns `true` if `other` is a prefix of `self`.
    #[must_use]
    pub fn starts_with(
        &self,
        other: &Self,
    ) -> bool {
        match (&self.0, &other.0) {
            (VfsPathRepr::PathBuf(lhs), VfsPathRepr::PathBuf(rhs)) => lhs.starts_with(rhs),
            (VfsPathRepr::VirtualPath(lhs), VfsPathRepr::VirtualPath(rhs)) => lhs.starts_with(rhs),
            (VfsPathRepr::PathBuf(_) | VfsPathRepr::VirtualPath(_), _) => false,
        }
    }

    #[must_use]
    pub fn strip_prefix(
        &self,
        other: &Self,
    ) -> Option<&RelPath> {
        match (&self.0, &other.0) {
            (VfsPathRepr::PathBuf(lhs), VfsPathRepr::PathBuf(rhs)) => lhs.strip_prefix(rhs),
            (VfsPathRepr::VirtualPath(lhs), VfsPathRepr::VirtualPath(rhs)) => lhs.strip_prefix(rhs),
            (VfsPathRepr::PathBuf(_) | VfsPathRepr::VirtualPath(_), _) => None,
        }
    }

    /// Returns the `VfsPath` without its final component, if there is one.
    ///
    /// Returns [`None`] if the path is a root or prefix.
    #[must_use]
    pub fn parent(&self) -> Option<Self> {
        let mut parent = self.clone();
        parent.pop().then_some(parent)
    }

    /// Returns `self`'s base name and file extension.
    #[must_use]
    pub fn name_and_extension(&self) -> Option<(&str, Option<&str>)> {
        match &self.0 {
            VfsPathRepr::PathBuf(p) => p.name_and_extension(),
            VfsPathRepr::VirtualPath(p) => p.name_and_extension(),
        }
    }

    /// **Don't make this `pub`**.
    ///
    /// Encode the path in the given buffer.
    ///
    /// The encoding will be `0` if [`AbsPathBuf`], `1` if [`VirtualPath`], followed
    /// by `self`'s representation.
    ///
    /// Note that this encoding is dependent on the operating system.
    pub(crate) fn encode(
        &self,
        buffer: &mut Vec<u8>,
    ) {
        let tag = match &self.0 {
            VfsPathRepr::PathBuf(_) => 0,
            VfsPathRepr::VirtualPath(_) => 1,
        };
        buffer.push(tag);
        match &self.0 {
            VfsPathRepr::PathBuf(path) => {
                buffer.extend(path.as_str().as_bytes());
            },
            VfsPathRepr::VirtualPath(VirtualPath(s)) => buffer.extend(s.as_bytes()),
        }
    }
}

/// Internal, private representation of [`VfsPath`].
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum VfsPathRepr {
    /// Path on the file system.
    PathBuf(AbsPathBuf),
    /// Virtual paths.
    VirtualPath(VirtualPath),
}

impl From<AbsPathBuf> for VfsPath {
    fn from(value: AbsPathBuf) -> Self {
        Self(VfsPathRepr::PathBuf(value.normalize()))
    }
}

impl From<VirtualPath> for VfsPath {
    fn from(value: VirtualPath) -> Self {
        Self(VfsPathRepr::VirtualPath(value))
    }
}

impl fmt::Display for VfsPath {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match &self.0 {
            VfsPathRepr::PathBuf(it) => it.fmt(f),
            VfsPathRepr::VirtualPath(VirtualPath(it)) => it.fmt(f),
        }
    }
}

impl fmt::Debug for VfsPath {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Debug for VfsPathRepr {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match &self {
            Self::PathBuf(it) => it.fmt(f),
            Self::VirtualPath(VirtualPath(it)) => it.fmt(f),
        }
    }
}

impl PartialEq<AbsPath> for VfsPath {
    fn eq(
        &self,
        other: &AbsPath,
    ) -> bool {
        match &self.0 {
            VfsPathRepr::PathBuf(lhs) => lhs == other,
            VfsPathRepr::VirtualPath(_) => false,
        }
    }
}
impl PartialEq<VfsPath> for AbsPath {
    fn eq(
        &self,
        other: &VfsPath,
    ) -> bool {
        other == self
    }
}

/// `/`-separated virtual path.
///
/// This is used to describe files that do not reside on the file system.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct VirtualPath(String);

impl VirtualPath {
    pub const SCHEME: &str = "wgsl";

    /// Creates a new virtual path.
    /// The root path is an empty string, every other path starts with `/`.
    ///
    /// # Panics
    /// Panics if `path` is invalid.
    #[must_use]
    pub fn new(path: String) -> Self {
        assert!(path.is_empty() || path.starts_with('/'));
        assert!(!path.ends_with('/'));
        Self(path)
    }
    /// Returns `true` if `other` is a prefix of `self` (as strings).
    fn starts_with(
        &self,
        other: &Self,
    ) -> bool {
        self.0.starts_with(&other.0)
    }

    fn strip_prefix(
        &self,
        base: &Self,
    ) -> Option<&RelPath> {
        <_ as AsRef<paths::Utf8Path>>::as_ref(&self.0)
            .strip_prefix(&base.0)
            .ok()
            .map(RelPath::new_unchecked)
    }

    /// Remove the last component of `self`.
    ///
    /// This will find the last `'/'` in `self`, and remove everything after it,
    /// including the `'/'`.
    ///
    /// If `self` contains no `'/'`, returns `false`; else returns `true`.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let mut path = VirtualPath("/foo/bar".to_string());
    /// path.pop();
    /// assert_eq!(path.0, "/foo");
    /// path.pop();
    /// assert_eq!(path.0, "");
    /// ```
    fn pop(&mut self) -> bool {
        let Some(pos) = self.0.rfind('/') else {
            return false;
        };
        self.0 = self.0[..pos].to_string();
        true
    }

    /// Append the given *relative* path `path` to `self`.
    ///
    /// This will resolve any leading `"../"` in `path` before appending it.
    ///
    /// Returns [`None`] if `path` has more leading `"../"` than the number of
    /// components in `self`.
    ///
    /// # Notes
    ///
    /// In practice, appending here means `self/path` as strings.
    fn join(
        &self,
        mut path: &str,
    ) -> Option<Self> {
        let mut result = self.clone();
        while path.starts_with("../") {
            if !result.pop() {
                return None;
            }
            path = &path["../".len()..];
        }
        path = path.trim_start_matches("./");
        result.0 = format!("{}/{path}", result.0);
        Some(result)
    }

    /// Returns `self`'s base name and file extension.
    ///
    /// # Returns
    /// - `None` if `self` ends with `"//"`.
    /// - `Some((name, None))` if `self`'s base contains no `.`, or only one `.` at the start.
    /// - `Some((name, Some(extension))` else.
    ///
    /// # Note
    /// The extension will not contains `.`. This means `"/foo/bar.baz.rs"` will
    /// return `Some(("bar.baz", Some("rs"))`.
    fn name_and_extension(&self) -> Option<(&str, Option<&str>)> {
        let file_path = if self.0.ends_with('/') {
            &self.0[..&self.0.len() - 1]
        } else {
            &self.0
        };
        let file_name = match file_path.rfind('/') {
            Some(position) => &file_path[position + 1..],
            None => file_path,
        };

        if file_name.is_empty() {
            None
        } else {
            let mut file_stem_and_extension = file_name.rsplitn(2, '.');
            let extension = file_stem_and_extension.next();
            let file_stem = file_stem_and_extension.next();

            match (file_stem, extension) {
                (None, None) => None,
                (None | Some(""), Some(_)) => Some((file_name, None)),
                (Some(file_stem), extension) => Some((file_stem, extension)),
            }
        }
    }

    #[must_use]
    pub fn components(&self) -> Components<'_> {
        Components::new(self)
    }
}

pub struct Components<'path> {
    /// Invariant: Always points at the next segment, without the leading `/`.
    path: &'path str,
}

impl<'path> Components<'path> {
    fn new(path: &'path VirtualPath) -> Self {
        assert!(path.0.starts_with('/'));
        Self { path: &path.0[1..] }
    }
}

impl<'path> Iterator for Components<'path> {
    type Item = &'path str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.path.is_empty() {
            return None;
        }

        if let Some((value, new_path)) = self.path.split_once('/') {
            self.path = new_path;
            Some(value)
        } else {
            let value = self.path;
            self.path = "";
            Some(value)
        }
    }
}

#[cfg(test)]
mod tests;
