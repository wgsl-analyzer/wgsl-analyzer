use std::path::{Path, PathBuf};

pub(crate) fn list_rust_files(dir: &Path) -> Vec<PathBuf> {
    let mut res = list_files(dir);
    res.retain(|it| {
        std::path::Path::new(
            it.file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default(),
        )
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("rs"))
    });
    res
}

pub(crate) fn list_files(dir: &Path) -> Vec<PathBuf> {
    let mut res = Vec::new();
    let mut work = vec![dir.to_path_buf()];
    while let Some(dir) = work.pop() {
        for entry in dir.read_dir().unwrap() {
            let entry = entry.unwrap();
            let file_type = entry.file_type().unwrap();
            let path = entry.path();
            let is_hidden = path
                .file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
                .starts_with('.');
            if !is_hidden {
                #[expect(clippy::filetype_is_file, reason = "overzealous")]
                if file_type.is_dir() {
                    work.push(path);
                } else if file_type.is_file() {
                    res.push(path);
                }
            }
        }
    }
    res
}
