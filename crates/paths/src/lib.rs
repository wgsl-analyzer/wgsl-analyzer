//! Thin wrappers around [`camino::Utf8PathBuf`], distinguishing between absolute and relative paths.

#![warn(unused)]

pub use camino::{Utf8Component, Utf8Components, Utf8Path, Utf8PathBuf, Utf8Prefix};

mod absolute;
mod relative;

pub use absolute::{AbsPath, AbsPathBuf};
pub use relative::{RelPath, RelPathBuf};

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

#[cfg(test)]
mod tests {
    use super::normalize_path;

    #[test]
    fn normalize_path_normal() {
        let utf8 = "test/path".into();
        let normalized = normalize_path(utf8);
        assert_eq!("test/path", normalized);
    }

    #[test]
    #[cfg(windows)]
    fn normalize_path_prefix() {
        let verbatim = r"\\?\pictures\kittens";
        let verbatim_unc = r"\\?\UNC\server\share";
        let verbatim_disk = r"\\?\C:\";
        let device_ns = r"\\.\BrainInterface";
        let unc = r"\\server\share";
        let disk = r"C:\Users\Rust\Pictures\Ferris";
        assert_eq!(r#"\\?\pictures\kittens"#, normalize_path(verbatim.into()));
        assert_eq!(
            r#"\\?\UNC\server\share"#,
            normalize_path(verbatim_unc.into())
        );
        assert_eq!(r#"\\?\C:\"#, normalize_path(verbatim_disk.into()));
        assert_eq!(r#"\\.\BrainInterface"#, normalize_path(device_ns.into()));
        assert_eq!(r#"\\server\share"#, normalize_path(unc.into()));
        assert_eq!(
            r#"C:\Users\Rust\Pictures\Ferris"#,
            normalize_path(disk.into())
        );
    }

    #[test]
    fn normalize_path_cur_dir() {
        let utf8 = r#"./normalize/./current/./dir/."#.into();
        let normalized = normalize_path(utf8);
        assert_eq!(r#"normalize/current/dir"#, normalized);
    }

    #[test]
    fn normalize_path_root() {
        let utf8 = r#"/normalize/root"#.into();
        let normalized = normalize_path(utf8);
        assert_eq!(r#"/normalize/root"#, normalized);
    }

    #[test]
    fn normalize_path_trailing_slash() {
        let utf8 = r#"trailing/slash/"#.into();
        let normalized = normalize_path(utf8);
        assert_eq!(r#"trailing/slash"#, normalized);
    }
}
