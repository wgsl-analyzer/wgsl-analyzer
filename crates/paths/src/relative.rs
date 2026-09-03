use std::{
    borrow::Borrow,
    ffi::OsStr,
    fmt, ops,
    path::{Path, PathBuf},
};

use camino::{Utf8Components, Utf8Path, Utf8PathBuf};

use crate::normalize_path;

/// Wrapper around a relative [`Utf8PathBuf`].
#[derive(Debug, Clone, Ord, PartialOrd, Eq, Hash)]
pub struct RelPathBuf(Utf8PathBuf);

impl From<RelPathBuf> for Utf8PathBuf {
    fn from(RelPathBuf(path_buf): RelPathBuf) -> Self {
        path_buf
    }
}

impl From<RelPathBuf> for PathBuf {
    fn from(RelPathBuf(path_buf): RelPathBuf) -> Self {
        path_buf.into()
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

impl AsRef<OsStr> for RelPathBuf {
    fn as_ref(&self) -> &OsStr {
        self.0.as_ref()
    }
}

impl AsRef<Path> for RelPathBuf {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

impl AsRef<RelPath> for RelPathBuf {
    fn as_ref(&self) -> &RelPath {
        self.as_path()
    }
}

impl Borrow<RelPath> for RelPathBuf {
    fn borrow(&self) -> &RelPath {
        self.as_path()
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

impl<P: AsRef<Path> + ?Sized> PartialEq<P> for RelPathBuf {
    fn eq(
        &self,
        other: &P,
    ) -> bool {
        self.0.as_std_path() == other.as_ref()
    }
}

impl RelPathBuf {
    /// Wrap the given relative path in [`RelPathBuf`].
    ///
    /// # Panics
    ///
    /// Panics if `path` is not relative.
    #[must_use]
    pub fn assert(path: Utf8PathBuf) -> Self {
        Self::try_from(path).unwrap_or_else(|path| panic!("expected relative path, got {path}"))
    }

    /// Wrap the given relative path in [`RelPathBuf`].
    ///
    /// # Panics
    ///
    /// Panics if `path` is not relative.
    #[must_use]
    pub fn assert_utf8(path: PathBuf) -> Self {
        Self::assert(
            Utf8PathBuf::from_path_buf(path)
                .unwrap_or_else(|path| panic!("expected utf8 path, got {}", path.display())),
        )
    }

    /// Coerces to a [`RelPath`] slice.
    ///
    /// Equivalent of [`Utf8PathBuf::as_path`] for [`RelPathBuf`].
    #[must_use]
    pub fn as_path(&self) -> &RelPath {
        // SAFETY: The path is already known to be a relative path
        unsafe { RelPath::new_unchecked(self.0.as_path()) }
    }

    /// Equivalent of [`Utf8PathBuf::pop`] for [`RelPathBuf`].
    ///
    /// Note that this won't remove the root component, so `self` will still be
    /// relative.
    pub fn pop(&mut self) -> bool {
        self.0.pop()
    }

    /// Equivalent of [`PathBuf::push`] for [`RelPathBuf`].
    ///
    /// Extends `self` with `path`.
    ///
    /// If `path` is relative, it replaces the current path.
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

impl fmt::Display for RelPathBuf {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        fmt::Display::fmt(&self.0, formatter)
    }
}

/// Wrapper around a relative [`Utf8Path`].
#[derive(Debug, Ord, PartialOrd, Eq, Hash)]
#[repr(transparent)]
pub struct RelPath(pub(crate) Utf8Path);

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

impl<P: AsRef<Path> + ?Sized> PartialEq<P> for RelPath {
    fn eq(
        &self,
        other: &P,
    ) -> bool {
        self.0.as_std_path() == other.as_ref()
    }
}

impl AsRef<OsStr> for RelPath {
    fn as_ref(&self) -> &OsStr {
        self.0.as_ref()
    }
}

impl ToOwned for RelPath {
    type Owned = RelPathBuf;

    fn to_owned(&self) -> Self::Owned {
        RelPathBuf(self.0.to_owned())
    }
}

impl<'path> TryFrom<&'path Utf8Path> for &'path RelPath {
    type Error = &'path Utf8Path;

    fn try_from(path: &'path Utf8Path) -> Result<&'path RelPath, &'path Utf8Path> {
        RelPath::new(path)
    }
}

impl RelPath {
    /// Creates a new [`RelPath`] from `path`.
    pub fn new(path: &Utf8Path) -> Result<&Self, &Utf8Path> {
        if !path.is_relative() {
            return Err(path);
        }
        // SAFETY: invariant is checked
        let new = unsafe { Self::new_unchecked(path) };
        Ok(new)
    }

    /// Creates a new [`RelPath`] from `path` without checking whether it is relative.
    ///
    /// # Panics
    ///
    /// Panics if `path` is not relative.
    ///
    /// # Safety
    ///
    /// Calling this method on an absolute path is *[undefined behavior]*.
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    #[expect(clippy::as_conversions, reason = "necessary here")]
    #[must_use]
    pub const unsafe fn new_unchecked(path: &Utf8Path) -> &Self {
        //
        // debug_assert!(path.is_relative(), "{path} is not relative");
        // SAFETY: pointer is guaranteed to be valid
        unsafe { &*(std::ptr::from_ref::<Utf8Path>(path) as *const Self) }
    }

    /// Normalize the given path:
    /// - Removes repeated separators: `a//b` becomes `a/b`
    /// - Removes occurrences of `.` and resolves `..`.
    /// - Removes trailing slashes: `a/b/` becomes `a/b`.
    ///
    /// # Example
    /// ```ignore
    /// # use paths::RelPathBuf;
    /// let rel_path_buf = RelPathBuf::assert("a/../../b/.//c//".into());
    /// let normalized = rel_path_buf.normalize();
    /// assert_eq!(normalized, RelPathBuf::assert("b/c".into()));
    /// ```
    #[must_use]
    pub fn normalize(&self) -> RelPathBuf {
        RelPathBuf(normalize_path(&self.0))
    }

    /// Equivalent of [`Utf8Path::to_path_buf`] for [`RelPath`].
    #[must_use]
    pub fn to_path_buf(&self) -> RelPathBuf {
        RelPathBuf(self.0.to_path_buf())
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

    /// Equivalent of [`Utf8Path::strip_prefix`] for [`RelPath`].
    ///
    /// Returns a relative path.
    pub fn strip_prefix<Pathy>(
        &self,
        base: Pathy,
    ) -> Option<&Self>
    where
        Pathy: AsRef<Path>,
    {
        // SAFETY: stripping the prefix of any path ensures that it is a relative path
        self.0
            .strip_prefix(base)
            .ok()
            .map(|stripped| unsafe { Self::new_unchecked(stripped) })
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
        suffix: &Self,
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
    // For `RelPath`, we want to make sure that this is a POD type, and that all
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

impl fmt::Display for RelPath {
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
    fn utf_from_relbuf() {
        let path: PathBuf = "test".into();
        let relbuf = RelPathBuf::assert_utf8(path.clone());
        let utf8: Utf8PathBuf = relbuf.into();
        assert_eq!(path, utf8);
    }

    #[test]
    fn path_from_relbuf() {
        let path: PathBuf = "test".into();
        let relbuf = RelPathBuf::assert_utf8(path.clone());
        let utf8: PathBuf = relbuf.into();
        assert_eq!(path, utf8);
    }

    #[test]
    fn relbuf_asref_utf8() {
        let utf8_expect: &Utf8Path = "test".into();
        let relbuf = RelPathBuf::assert("test".into());
        let utf8: &Utf8Path = relbuf.as_ref();
        assert_eq!(utf8_expect, utf8);
    }

    #[test]
    fn relbuf_asref_rel() {
        let rel_expect: &RelPath = RelPath::new("test".into()).unwrap();
        let relbuf = RelPathBuf::assert("test".into());
        let rel: &RelPath = relbuf.as_ref();
        assert_eq!(rel_expect, rel);
    }

    #[test]
    fn relbuf_borrow_rel() {
        let owned = RelPathBuf::assert("test".into());
        let borrowed: &RelPath = owned.borrow();
        assert_eq!(RelPath::new("test".into()).unwrap(), borrowed);
    }

    #[test]
    fn relbuf_partialeq() {
        let path: PathBuf = "test".into();
        let relbuf = RelPathBuf::assert("test".into());
        assert_eq!(relbuf, path);
    }

    #[test]
    fn relbuf_try_from_utf8_fail() {
        let path: Utf8PathBuf = "/".into();
        let relbuf = RelPathBuf::try_from(path.clone());
        assert_eq!(Err(path), relbuf);
    }

    #[test]
    fn relbuf_pop() {
        let mut relbuf = RelPathBuf::assert("test/test".into());
        let expected = RelPathBuf::assert("test".into());
        relbuf.pop();
        assert_eq!(expected, relbuf);
    }

    #[test]
    fn relbuf_push() {
        let mut relbuf = RelPathBuf::assert("test".into());
        let expected = RelPathBuf::assert("test/push".into());
        relbuf.push("push");
        assert_eq!(expected, relbuf);
    }

    #[test]
    fn relbuf_display() {
        let relbuf = RelPathBuf::assert("test".into());
        let display = format!("{relbuf:#}");
        assert_eq!("test", display);
    }

    #[test]
    fn rel_toowned() {
        let rel = RelPath::new("test".into()).unwrap();
        let relbuf = RelPathBuf::assert("test".into());
        let owned: RelPathBuf = rel.to_owned();
        assert_eq!(relbuf, owned);
    }

    #[test]
    #[should_panic = "We explicitly do not provide canonicalization API, as that is almost always a wrong solution, see #14430"]
    #[expect(clippy::diverging_sub_expression, deprecated, reason = "test")]
    fn canonicalize_panics() {
        let rel1 = RelPath::new("test".into()).unwrap();
        _ = rel1.canonicalize();
    }

    #[test]
    fn rel_starts_with() {
        let rel1 = RelPath::new("test/path".into()).unwrap();
        let rel2 = RelPath::new("test".into()).unwrap();
        let rel3 = RelPath::new("wrong".into()).unwrap();
        assert!(rel1.starts_with(rel2));
        assert!(!rel1.starts_with(rel3));
    }

    #[test]
    fn rel_ends_with() {
        let rel = RelPath::new("test/path".into()).unwrap();
        let rel1 = RelPath::new("path".into()).unwrap();
        let rel2 = RelPath::new("wrong".into()).unwrap();
        assert!(rel.ends_with(rel1));
        assert!(!rel.ends_with(rel2));
    }

    #[test]
    fn rel_asref_rel() {
        let relbuf: &RelPath = RelPath::new("test".into()).unwrap();
        let osstr: &OsStr = relbuf.as_ref();
        assert_eq!(OsStr::new("test"), osstr);
    }

    #[test]
    fn rel_tryfrom_utf8() {
        let expect: &RelPath = RelPath::new("test".into()).unwrap();
        let rel: &RelPath = Utf8Path::new("test").try_into().unwrap();
        assert_eq!(expect, rel);
    }

    #[test]
    fn rel_tryfrom_utf8_err() {
        let utf8 = Utf8Path::new("/");
        let result: Result<&RelPath, _> = utf8.try_into();
        assert_eq!(Err(utf8), result);
    }

    #[test]
    fn rel_name_and_extension() {
        let rel = RelPath::new("name.extension".into()).unwrap();
        let (name, extension) = rel.name_and_extension().unwrap();
        assert_eq!("name", name);
        assert_eq!(Some("extension"), extension);
    }

    #[test]
    #[should_panic = "not implemented: use Display instead"]
    #[expect(clippy::diverging_sub_expression, deprecated, reason = "test")]
    fn rel_display_panics() {
        let rel = RelPath::new("name.extension".into()).unwrap();
        _ = rel.display();
    }

    #[test]
    #[should_panic = "not implemented: use std::fs::metadata().is_ok() instead"]
    #[expect(clippy::diverging_sub_expression, deprecated, reason = "test")]
    fn rel_exists() {
        let rel = RelPath::new("name.extension".into()).unwrap();
        _ = rel.exists();
    }

    #[test]
    fn rel_components() {
        let rel = RelPath::new("name.extension".into()).unwrap();
        let components = rel.components();
        let vec: Vec<_> = components.map(|component| component.to_string()).collect();
        assert_eq!(vec!["name.extension"], vec);
    }

    #[test]
    fn rel_display() {
        let rel = RelPath::new("test".into()).unwrap();
        let display = format!("{rel:#}");
        assert_eq!("test", display);
    }
}
