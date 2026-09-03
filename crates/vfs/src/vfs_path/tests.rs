use super::*;

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
    let path = VirtualPath::new("".to_owned());
    let mut components = path.components();
    assert_eq!(components.next(), None);
}
