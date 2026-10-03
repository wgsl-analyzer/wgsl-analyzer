use super::*;

#[must_use]
pub fn root() -> &'static AbsPath {
    #[cfg(windows)]
    {
        // SAFETY: static value known to be safe
        unsafe { AbsPath::new_unchecked(r#"C:\"#.into()) }
    }
    #[cfg(not(windows))]
    {
        // SAFETY: static value known to be safe
        unsafe { AbsPath::new_unchecked("/".into()) }
    }
}

#[test]
fn virtual_path_extensions() {
    assert_eq!(VirtualPath("/".to_owned()).name_and_extension(), None);
    assert_eq!(
        VirtualPath("/directory".to_owned()).name_and_extension(),
        Some(("directory", None))
    );
    assert_eq!(
        VirtualPath("/directory/".to_owned()).name_and_extension(),
        Some(("directory", None))
    );
    assert_eq!(
        VirtualPath("/directory/file".to_owned()).name_and_extension(),
        Some(("file", None))
    );
    assert_eq!(
        VirtualPath("/directory/.file".to_owned()).name_and_extension(),
        Some((".file", None))
    );
    assert_eq!(
        VirtualPath("/directory/.file.rs".to_owned()).name_and_extension(),
        Some((".file", Some("rs")))
    );
    assert_eq!(
        VirtualPath("/directory/file.rs".to_owned()).name_and_extension(),
        Some(("file", Some("rs")))
    );
}

#[test]
fn root_virtual_path() {
    assert_eq!(
        VfsPath::new_virtual_path(String::new()),
        VfsPath(VfsPathRepr::VirtualPath(VirtualPath(String::new())))
    );

    assert_eq!(
        VfsPath::new_virtual_path(String::new()).join("./foo/bar"),
        Some(VfsPath(VfsPathRepr::VirtualPath(VirtualPath(
            "/foo/bar".to_owned()
        ))))
    );
}

#[test]
fn virtual_path_components() {
    let path = VirtualPath::new("/foo/bar/cat.wesl".to_owned());
    let mut components = path.components();
    assert_eq!(components.next(), Some("foo"));
    assert_eq!(components.next(), Some("bar"));
    assert_eq!(components.next(), Some("cat.wesl"));
}

#[test]
fn empty_virtual_path_components() {
    let path = VirtualPath::empty();
    let mut components = path.components();
    assert_eq!(components.next(), None);
}

#[test]
fn as_path_virtual_path() {
    let vfs_virtual_path = VfsPath::new_virtual_path(String::new());
    let path = vfs_virtual_path.as_path();
    assert_eq!(path, None);
}

#[test]
fn as_path_path() {
    let vfs_path = VfsPath::new_real_path(root().to_string());
    let path = vfs_path.as_path();
    assert_eq!(path, Some(root()));
}

#[test]
fn as_virtual_path_virtual_path() {
    let vfs_virtual_path = VfsPath::new_virtual_path(String::new());
    let path = vfs_virtual_path.as_virtual_path();
    assert_eq!(path, Some(&VirtualPath(String::new())));
}

#[test]
fn as_virtual_path_path() {
    let vfs_path = VfsPath::new_real_path(root().to_string());
    let path = vfs_path.as_virtual_path();
    assert_eq!(path, None);
}

#[test]
fn path_as_inner() {
    let vfs_path = VfsPath::new_real_path(root().to_string());
    let path = vfs_path.as_inner();
    assert_eq!(path, Either::Left(root()));
}

#[test]
fn virtual_path_as_inner() {
    let vfs_virtual_path = VfsPath::new_virtual_path(String::new());
    let path = vfs_virtual_path.as_inner();
    assert_eq!(path, Either::Right(&VirtualPath(String::new())));
}

#[test]
fn into_abs_path_virtual_path() {
    let vfs_virtual_path = VfsPath::new_virtual_path(String::new());
    let path = vfs_virtual_path.into_abs_path();
    assert_eq!(path, None);
}

#[test]
fn into_abs_path_path() {
    let vfs_path = VfsPath::new_real_path(root().to_string());
    let path = vfs_path.into_abs_path();
    assert_eq!(path, Some(root().to_owned()));
}
