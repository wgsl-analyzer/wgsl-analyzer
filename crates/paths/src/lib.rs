//! Thin wrappers around [`camino::Utf8PathBuf`], distinguishing between absolute and relative paths.

use std::{
    borrow::Borrow,
    ffi::OsStr,
    fmt, ops,
    path::{Path, PathBuf},
};

pub use camino::{Utf8Component, Utf8Components, Utf8Path, Utf8PathBuf, Utf8Prefix};

/// A [`Utf8PathBuf`] that is guaranteed to be absolute.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, Hash)]
pub struct AbsPathBuf(Utf8PathBuf);

impl From<AbsPathBuf> for Utf8PathBuf {
    fn from(AbsPathBuf(path_buf): AbsPathBuf) -> Self {
        path_buf
    }
}

impl From<AbsPathBuf> for PathBuf {
    fn from(AbsPathBuf(path_buf): AbsPathBuf) -> Self {
        path_buf.into()
    }
}

impl ops::Deref for AbsPathBuf {
    type Target = AbsPath;
    fn deref(&self) -> &AbsPath {
        self.as_path()
    }
}

impl AsRef<Utf8Path> for AbsPathBuf {
    fn as_ref(&self) -> &Utf8Path {
        self.0.as_path()
    }
}

impl AsRef<OsStr> for AbsPathBuf {
    fn as_ref(&self) -> &OsStr {
        self.0.as_ref()
    }
}

impl AsRef<Path> for AbsPathBuf {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

impl AsRef<AbsPath> for AbsPathBuf {
    fn as_ref(&self) -> &AbsPath {
        self.as_path()
    }
}

impl Borrow<AbsPath> for AbsPathBuf {
    fn borrow(&self) -> &AbsPath {
        self.as_path()
    }
}

impl TryFrom<Utf8PathBuf> for AbsPathBuf {
    type Error = Utf8PathBuf;
    fn try_from(path_buf: Utf8PathBuf) -> Result<Self, Utf8PathBuf> {
        if !path_buf.is_absolute() {
            return Err(path_buf);
        }
        Ok(Self(path_buf))
    }
}

impl TryFrom<&str> for AbsPathBuf {
    type Error = Utf8PathBuf;
    fn try_from(path: &str) -> Result<Self, Utf8PathBuf> {
        Self::try_from(Utf8PathBuf::from(path))
    }
}

impl<P: AsRef<Path> + ?Sized> PartialEq<P> for AbsPathBuf {
    fn eq(
        &self,
        other: &P,
    ) -> bool {
        self.0.as_std_path() == other.as_ref()
    }
}

impl AbsPathBuf {
    /// Wrap the given absolute path in `AbsPathBuf`.
    ///
    /// # Panics
    ///
    /// Panics if `path` is not absolute.
    #[must_use]
    pub fn assert(path: Utf8PathBuf) -> Self {
        Self::try_from(path).unwrap_or_else(|path| panic!("expected absolute path, got {path}"))
    }

    /// Wrap the given absolute path in `AbsPathBuf`.
    ///
    /// # Panics
    ///
    /// Panics if `path` is not absolute.
    #[must_use]
    pub fn assert_utf8(path: PathBuf) -> Self {
        Self::assert(
            Utf8PathBuf::from_path_buf(path)
                .unwrap_or_else(|path| panic!("expected utf8 path, got {}", path.display())),
        )
    }

    /// Coerces to an `AbsPath` slice.
    ///
    /// Equivalent of [`Utf8PathBuf::as_path`] for `AbsPathBuf`.
    #[must_use]
    pub fn as_path(&self) -> &AbsPath {
        AbsPath::assert(self.0.as_path())
    }

    /// Equivalent of [`Utf8PathBuf::pop`] for `AbsPathBuf`.
    ///
    /// Note that this won't remove the root component, so `self` will still be
    /// absolute.
    pub fn pop(&mut self) -> bool {
        self.0.pop()
    }

    /// Equivalent of [`PathBuf::push`] for `AbsPathBuf`.
    ///
    /// Extends `self` with `path`.
    ///
    /// If `path` is absolute, it replaces the current path.
    ///
    /// On Windows:
    ///
    /// * if `path` has a root but no prefix (e.g., `\windows`), it
    ///   replaces everything except for the prefix (if any) of `self`.
    /// * if `path` has a prefix but no root, it replaces `self`.
    /// * if `self` has a verbatim prefix (e.g. `\\?\C:\windows`)
    ///   and `path` is not empty, the new path is normalized: all references
    ///   to `.` and `..` are removed.
    pub fn push<Path>(
        &mut self,
        suffix: Path,
    ) where
        Path: AsRef<Utf8Path>,
    {
        self.0.push(suffix);
    }

    #[must_use]
    pub fn join<Path>(
        &self,
        path: Path,
    ) -> Self
    where
        Path: AsRef<Utf8Path>,
    {
        Self(self.0.join(path))
    }
}

impl fmt::Display for AbsPathBuf {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

/// Wrapper around an absolute [`Utf8Path`].
#[derive(Debug, Ord, PartialOrd, Eq, Hash)]
#[repr(transparent)]
pub struct AbsPath(Utf8Path);

impl<P: AsRef<Path> + ?Sized> PartialEq<P> for AbsPath {
    fn eq(
        &self,
        other: &P,
    ) -> bool {
        self.0.as_std_path() == other.as_ref()
    }
}

impl AsRef<Utf8Path> for AbsPath {
    fn as_ref(&self) -> &Utf8Path {
        &self.0
    }
}

impl AsRef<Path> for AbsPath {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

impl AsRef<OsStr> for AbsPath {
    fn as_ref(&self) -> &OsStr {
        self.0.as_ref()
    }
}

impl ToOwned for AbsPath {
    type Owned = AbsPathBuf;

    fn to_owned(&self) -> Self::Owned {
        AbsPathBuf(self.0.to_owned())
    }
}

impl<'path> TryFrom<&'path Utf8Path> for &'path AbsPath {
    type Error = &'path Utf8Path;
    fn try_from(path: &'path Utf8Path) -> Result<&'path AbsPath, &'path Utf8Path> {
        if !path.is_absolute() {
            return Err(path);
        }
        Ok(AbsPath::assert(path))
    }
}

impl AbsPath {
    /// Wrap the given absolute path in `AbsPath`.
    ///
    /// # Panics
    ///
    /// Panics if `path` is not absolute.
    #[must_use]
    #[expect(clippy::as_conversions, reason = "necessary here")]
    pub fn assert(path: &Utf8Path) -> &Self {
        assert!(path.is_absolute(), "{path} is not absolute");
        // SAFETY: pointer is guaranteed to be valid
        unsafe { &*(std::ptr::from_ref::<Utf8Path>(path) as *const Self) }
    }

    /// Equivalent of [`Utf8Path::parent`] for `AbsPath`.
    pub fn parent(&self) -> Option<&Self> {
        self.0.parent().map(Self::assert)
    }

    /// Equivalent of [`Utf8Path::join`] for `AbsPath` with an additional normalize step afterwards.
    pub fn absolutize<Path>(
        &self,
        path: Path,
    ) -> AbsPathBuf
    where
        Path: AsRef<Utf8Path>,
    {
        self.join(path).normalize()
    }

    /// Equivalent of [`Utf8Path::join`] for `AbsPath`.
    pub fn join<Path>(
        &self,
        path: Path,
    ) -> AbsPathBuf
    where
        Path: AsRef<Utf8Path>,
    {
        AbsPathBuf(Utf8Path::join(self.as_ref(), path))
    }

    /// Normalize the given path:
    /// - Removes repeated separators: `/a//b` becomes `/a/b`
    /// - Removes occurrences of `.` and resolves `..`.
    /// - Removes trailing slashes: `/a/b/` becomes `/a/b`.
    ///
    /// # Example
    /// ```ignore
    /// # use paths::AbsPathBuf;
    /// let abs_path_buf = AbsPathBuf::assert("/a/../../b/.//c//".into());
    /// let normalized = abs_path_buf.normalize();
    /// assert_eq!(normalized, AbsPathBuf::assert("/b/c".into()));
    /// ```
    #[must_use]
    pub fn normalize(&self) -> AbsPathBuf {
        AbsPathBuf(normalize_path(&self.0))
    }

    /// Equivalent of [`Utf8Path::to_path_buf`] for `AbsPath`.
    #[must_use]
    pub fn to_path_buf(&self) -> AbsPathBuf {
        AbsPathBuf(self.0.to_path_buf())
    }

    /// # Panics
    ///
    /// Do not use.
    #[deprecated]
    #[expect(clippy::unused_self, reason = "intentional API")]
    pub fn canonicalize(&self) -> ! {
        panic!(
            "We explicitly do not provide canonicalization API, as that is almost always a wrong solution, see #14430"
        )
    }

    /// Equivalent of [`Utf8Path::strip_prefix`] for `AbsPath`.
    ///
    /// Returns a relative path.
    pub fn strip_prefix(
        &self,
        base: &Self,
    ) -> Option<&RelPath> {
        self.0.strip_prefix(base).ok().map(RelPath::new_unchecked)
    }

    #[must_use]
    pub fn starts_with(
        &self,
        base: &Self,
    ) -> bool {
        self.0.starts_with(&base.0)
    }

    #[must_use]
    pub fn ends_with(
        &self,
        suffix: &RelPath,
    ) -> bool {
        self.0.ends_with(&suffix.0)
    }

    #[must_use]
    pub fn name_and_extension(&self) -> Option<(&str, Option<&str>)> {
        Some((self.file_stem()?, self.extension()))
    }

    // region:delegate-methods

    // Note that we deliberately don't implement `Deref<Target = Utf8Path>` here.
    //
    // The problem with `Utf8Path` is that it directly exposes convenience IO-ing
    // methods. For example, `Utf8Path::exists` delegates to `fs::metadata`.
    //
    // For `AbsPath`, we want to make sure that this is a POD type, and that all
    // IO goes via `fs`. That way, it becomes easier to mock IO when we need it.
    #[must_use]
    pub fn file_name(&self) -> Option<&str> {
        self.0.file_name()
    }

    #[must_use]
    pub fn extension(&self) -> Option<&str> {
        self.0.extension()
    }

    #[must_use]
    pub fn file_stem(&self) -> Option<&str> {
        self.0.file_stem()
    }

    #[must_use]
    pub fn as_os_str(&self) -> &OsStr {
        self.0.as_os_str()
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    #[expect(clippy::unimplemented, reason = "on purpose")]
    #[deprecated(note = "use Display instead")]
    pub fn display(&self) -> ! {
        unimplemented!()
    }

    #[expect(clippy::unimplemented, reason = "on purpose")]
    #[deprecated(note = "use std::fs::metadata().is_ok() instead")]
    pub fn exists(&self) -> ! {
        unimplemented!()
    }

    pub fn components(&self) -> Utf8Components<'_> {
        self.0.components()
    }
    // endregion:delegate-methods
}

impl fmt::Display for AbsPath {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        fmt::Display::fmt(&self.0, formatter)
    }
}

/// Wrapper around a relative [`Utf8PathBuf`].
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct RelPathBuf(Utf8PathBuf);

impl From<RelPathBuf> for Utf8PathBuf {
    fn from(RelPathBuf(path_buf): RelPathBuf) -> Self {
        path_buf
    }
}

impl ops::Deref for RelPathBuf {
    type Target = RelPath;
    fn deref(&self) -> &RelPath {
        self.as_path()
    }
}

impl AsRef<Utf8Path> for RelPathBuf {
    fn as_ref(&self) -> &Utf8Path {
        self.0.as_path()
    }
}

impl AsRef<Path> for RelPathBuf {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

impl TryFrom<Utf8PathBuf> for RelPathBuf {
    type Error = Utf8PathBuf;
    fn try_from(path_buf: Utf8PathBuf) -> Result<Self, Utf8PathBuf> {
        if !path_buf.is_relative() {
            return Err(path_buf);
        }
        Ok(Self(path_buf))
    }
}

impl TryFrom<&str> for RelPathBuf {
    type Error = Utf8PathBuf;
    fn try_from(path: &str) -> Result<Self, Utf8PathBuf> {
        Self::try_from(Utf8PathBuf::from(path))
    }
}

impl RelPathBuf {
    /// Coerces to a `RelPath` slice.
    ///
    /// Equivalent of [`Utf8PathBuf::as_path`] for `RelPathBuf`.
    #[must_use]
    pub fn as_path(&self) -> &RelPath {
        RelPath::new_unchecked(self.0.as_path())
    }
}

/// Wrapper around a relative [`Utf8Path`].
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct RelPath(Utf8Path);

impl AsRef<Utf8Path> for RelPath {
    fn as_ref(&self) -> &Utf8Path {
        &self.0
    }
}

impl AsRef<Path> for RelPath {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

impl RelPath {
    /// Creates a new `RelPath` from `path`, without checking if it is relative.
    #[must_use]
    #[expect(clippy::as_conversions, reason = "necessary here")]
    pub const fn new_unchecked(path: &Utf8Path) -> &Self {
        // SAFETY: pointer is guaranteed to be valid
        unsafe { &*(std::ptr::from_ref::<Utf8Path>(path) as *const Self) }
    }

    /// Equivalent of [`Utf8Path::to_path_buf`] for `RelPath`.
    #[must_use]
    pub fn to_path_buf(&self) -> RelPathBuf {
        // SAFETY: invariant of the type that this is already relative
        unsafe { RelPathBuf::try_from(self.0.to_path_buf()).unwrap_unchecked() }
    }

    #[must_use]
    pub fn as_utf8_path(&self) -> &Utf8Path {
        self.as_ref()
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// Taken from <https://github.com/rust-lang/cargo/blob/79c769c3d7b4c2cf6a93781575b7f592ef974255/src/cargo/util/paths.rs#L60-L85>.
fn normalize_path(path: &Utf8Path) -> Utf8PathBuf {
    let mut components = path.components().peekable();
    let mut return_value =
        if let Some(prefix @ Utf8Component::Prefix(..)) = components.peek().copied() {
            components.next();
            Utf8PathBuf::from(prefix.as_str())
        } else {
            Utf8PathBuf::new()
        };

    #[expect(clippy::unreachable, reason = "best way to write it")]
    for component in components {
        match component {
            Utf8Component::Prefix(..) => {
                unreachable!("prefixes may only be at the beginning and it was already consumed")
            },
            Utf8Component::RootDir => {
                return_value.push(component.as_str());
            },
            Utf8Component::CurDir => {},
            Utf8Component::ParentDir => {
                return_value.pop();
            },
            Utf8Component::Normal(component) => {
                return_value.push(component);
            },
        }
    }
    return_value
}
