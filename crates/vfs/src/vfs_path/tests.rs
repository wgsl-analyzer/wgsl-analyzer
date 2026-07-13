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
