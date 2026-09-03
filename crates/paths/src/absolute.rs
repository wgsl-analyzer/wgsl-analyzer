use std::{
    borrow::Borrow,
    ffi::OsStr,
    fmt, ops,
    path::{Path, PathBuf},
};

use camino::{Utf8Components, Utf8Path, Utf8PathBuf};

use crate::{RelPath, normalize_path};

/// A [`Utf8PathBuf`] that is guaranteed to be absolute.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, Hash)]
pub struct AbsPathBuf(pub(crate) Utf8PathBuf);

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
    /// Wrap the given absolute path in [`AbsPathBuf`].
    ///
    /// # Panics
    ///
    /// Panics if `path` is not absolute.
    #[must_use]
    pub fn assert(path: Utf8PathBuf) -> Self {
        Self::try_from(path).unwrap_or_else(|path| panic!("expected absolute path, got {path}"))
    }

    /// Wrap the given absolute path in [`AbsPathBuf`].
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

    /// Coerces to an [`AbsPath`] slice.
    ///
    /// Equivalent of [`Utf8PathBuf::as_path`] for [`AbsPathBuf`].
    #[must_use]
    pub fn as_path(&self) -> &AbsPath {
        // SAFETY: the path is already known to be absolute
        unsafe { AbsPath::new_unchecked(self.0.as_path()) }
    }

    /// Equivalent of [`Utf8PathBuf::pop`] for [`AbsPathBuf`].
    ///
    /// Note that this won't remove the root component, so `self` will still be
    /// absolute.
    pub fn pop(&mut self) -> bool {
        self.0.pop()
    }

    /// Equivalent of [`PathBuf::push`] for [`AbsPathBuf`].
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
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        fmt::Display::fmt(&self.0, formatter)
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
        AbsPath::new(path)
    }
}

impl AbsPath {
    /// Creates a new [`AbsPath`] from `path`.
    pub fn new(path: &Utf8Path) -> Result<&Self, &Utf8Path> {
        if !path.is_absolute() {
            return Err(path);
        }
        // SAFETY: invariant is checked
        let new = unsafe { Self::new_unchecked(path) };
        Ok(new)
    }

    /// Creates a new [`AbsPath`] from `path` without checking whether it is absolute.
    ///
    /// # Panics
    ///
    /// Panics if `path` is not absolute.
    ///
    /// # Safety
    ///
    /// Calling this method on a relative path is *[undefined behavior]*.
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    #[must_use]
    #[expect(clippy::as_conversions, reason = "necessary here")]
    pub unsafe fn new_unchecked(path: &Utf8Path) -> &Self {
        debug_assert!(path.is_absolute(), "{path} is not absolute");
        // SAFETY: pointer is guaranteed to be valid
        unsafe { &*(std::ptr::from_ref::<Utf8Path>(path) as *const Self) }
    }

    /// Equivalent of [`Utf8Path::parent`] for [`AbsPath`].
    #[must_use]
    pub fn parent(&self) -> Option<&Self> {
        // SAFETY: the parent of an absolute path is an absolute path
        self.0
            .parent()
            .map(|parent| unsafe { Self::new_unchecked(parent) })
    }

    /// Equivalent of [`Utf8Path::join`] for [`AbsPath`] with an additional normalize step afterwards.
    pub fn absolutize<Path>(
        &self,
        path: Path,
    ) -> AbsPathBuf
    where
        Path: AsRef<Utf8Path>,
    {
        self.join(path).normalize()
    }

    /// Equivalent of [`Utf8Path::join`] for [`AbsPath`].
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

    /// Equivalent of [`Utf8Path::to_path_buf`] for [`AbsPath`].
    #[must_use]
    pub fn to_path_buf(&self) -> AbsPathBuf {
        AbsPathBuf(self.0.to_path_buf())
    }

    #[must_use]
    pub fn as_utf8_path(&self) -> &Utf8Path {
        self.as_ref()
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

    /// Equivalent of [`Utf8Path::strip_prefix`] for [`AbsPath`].
    ///
    /// Returns a relative path.
    #[must_use]
    pub fn strip_prefix(
        &self,
        base: &Self,
    ) -> Option<&RelPath> {
        // SAFETY: stripping the prefix of any path ensures that it is a relative path
        self.0
            .strip_prefix(base)
            .ok()
            .map(|stripped| unsafe { RelPath::new_unchecked(stripped) })
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
        unimplemented!("use Display instead")
    }

    #[expect(clippy::unimplemented, reason = "on purpose")]
    #[deprecated(note = "use std::fs::metadata().is_ok() instead")]
    pub fn exists(&self) -> ! {
        unimplemented!("use std::fs::metadata().is_ok() instead")
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::{ffi::OsStr, path::PathBuf};

    #[test]
    fn utf_from_absbuf() {
        let path: PathBuf = "/".into();
        let absbuf = AbsPathBuf::assert_utf8(path.clone());
        let utf8: Utf8PathBuf = absbuf.into();
        assert_eq!(path, utf8);
    }

    #[test]
    fn path_from_absbuf() {
        let path: PathBuf = "/".into();
        let absbuf = AbsPathBuf::assert_utf8(path.clone());
        let utf8: PathBuf = absbuf.into();
        assert_eq!(path, utf8);
    }

    #[test]
    fn absbuf_asref_utf8() {
        let utf8_expect: &Utf8Path = "/".into();
        let absbuf = AbsPathBuf::assert("/".into());
        let utf8: &Utf8Path = absbuf.as_ref();
        assert_eq!(utf8_expect, utf8);
    }

    #[test]
    fn absbuf_asref_abs() {
        let abs_expect: &AbsPath = AbsPath::new("/".into()).unwrap();
        let absbuf = AbsPathBuf::assert("/".into());
        let abs: &AbsPath = absbuf.as_ref();
        assert_eq!(abs_expect, abs);
    }

    #[test]
    fn absbuf_borrow_abs() {
        let owned = AbsPathBuf::assert("/".into());
        let borrowed: &AbsPath = owned.borrow();
        assert_eq!(AbsPath::new("/".into()).unwrap(), borrowed);
    }

    #[test]
    fn absbuf_partialeq() {
        let path: PathBuf = "/".into();
        let absbuf = AbsPathBuf::assert("/".into());
        assert_eq!(absbuf, path);
    }

    #[test]
    fn absbuf_try_from_utf8_fail() {
        let path: Utf8PathBuf = ".".into();
        let absbuf = AbsPathBuf::try_from(path.clone());
        assert_eq!(Err(path), absbuf);
    }

    #[test]
    fn absbuf_pop() {
        let mut absbuf = AbsPathBuf::assert("/test".into());
        let expected = AbsPathBuf::assert("/".into());
        absbuf.pop();
        assert_eq!(expected, absbuf);
    }

    #[test]
    fn absbuf_push() {
        let mut absbuf = AbsPathBuf::assert("/test".into());
        let expected = AbsPathBuf::assert("/test/push".into());
        absbuf.push("push");
        assert_eq!(expected, absbuf);
    }

    #[test]
    fn absbuf_display() {
        let absbuf = AbsPathBuf::assert("/test".into());
        let display = format!("{absbuf:#}");
        assert_eq!("/test", display);
    }

    #[test]
    fn abs_toowned() {
        let abs = AbsPath::new("/test".into()).unwrap();
        let absbuf = AbsPathBuf::assert("/test".into());
        let owned: AbsPathBuf = abs.to_owned();
        assert_eq!(absbuf, owned);
    }

    #[test]
    fn absolutize_works() {
        let abs1 = AbsPath::new("/test".into()).unwrap();
        let abs2 = abs1.absolutize("relative");
        let expect = AbsPath::new("/test/relative".into()).unwrap();
        assert_eq!(expect, &abs2);
    }

    #[test]
    #[should_panic = "We explicitly do not provide canonicalization API, as that is almost always a wrong solution, see #14430"]
    #[expect(clippy::diverging_sub_expression, deprecated, reason = "test")]
    fn canonicalize_panics() {
        let abs1 = AbsPath::new("/test".into()).unwrap();
        _ = abs1.canonicalize();
    }

    #[test]
    fn abs_starts_with() {
        let abs1 = AbsPath::new("/test/path".into()).unwrap();
        let abs2 = AbsPath::new("/test".into()).unwrap();
        let abs3 = AbsPath::new("/wrong".into()).unwrap();
        assert!(abs1.starts_with(abs2));
        assert!(!abs1.starts_with(abs3));
    }

    #[test]
    fn abs_ends_with() {
        let abs = AbsPath::new("/test/path".into()).unwrap();
        let rel1 = RelPath::new("path".into()).unwrap();
        let rel2 = RelPath::new("wrong".into()).unwrap();
        assert!(abs.ends_with(rel1));
        assert!(!abs.ends_with(rel2));
    }

    #[test]
    fn abs_asref_abs() {
        let absbuf: &AbsPath = AbsPath::new("/".into()).unwrap();
        let osstr: &OsStr = absbuf.as_ref();
        assert_eq!(OsStr::new("/"), osstr);
    }

    #[test]
    fn abs_tryfrom_utf8() {
        let expect: &AbsPath = AbsPath::new("/".into()).unwrap();
        let abs: &AbsPath = Utf8Path::new("/").try_into().unwrap();
        assert_eq!(expect, abs);
    }

    #[test]
    fn abs_tryfrom_utf8_err() {
        let utf8 = Utf8Path::new(".");
        let result: Result<&AbsPath, _> = utf8.try_into();
        assert_eq!(Err(utf8), result);
    }

    #[test]
    fn abs_name_and_extension() {
        let abs = AbsPath::new("/name.extension".into()).unwrap();
        let (name, extension) = abs.name_and_extension().unwrap();
        assert_eq!("name", name);
        assert_eq!(Some("extension"), extension);
    }

    #[test]
    #[should_panic = "not implemented: use Display instead"]
    #[expect(clippy::diverging_sub_expression, deprecated, reason = "test")]
    fn abs_display_panics() {
        let abs = AbsPath::new("/name.extension".into()).unwrap();
        _ = abs.display();
    }

    #[test]
    #[should_panic = "not implemented: use std::fs::metadata().is_ok() instead"]
    #[expect(clippy::diverging_sub_expression, deprecated, reason = "test")]
    fn abs_exists() {
        let abs = AbsPath::new("/name.extension".into()).unwrap();
        _ = abs.exists();
    }

    #[test]
    fn abs_components() {
        let abs = AbsPath::new("/name.extension".into()).unwrap();
        let components = abs.components();
        let vec: Vec<_> = components.map(|component| component.to_string()).collect();
        assert_eq!(vec!["/", "name.extension"], vec);
    }

    #[test]
    fn abs_display() {
        let abs = AbsPath::new("/test".into()).unwrap();
        let display = format!("{abs:#}");
        assert_eq!("/test", display);
    }
}
