use std::path::{Path, PathBuf};

const WGSL_FILE_EXTENSION: &str = "wgsl";

pub(crate) fn list_wgsl_files(directory: &Path) -> Vec<PathBuf> {
    let mut result = list_files(directory);
    result.retain(|it| {
        std::path::Path::new(
            it.file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default(),
        )
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case(WGSL_FILE_EXTENSION))
    });
    result
}

pub(crate) fn list_files(directory: &Path) -> Vec<PathBuf> {
    let mut result = Vec::new();
    let mut work = vec![directory.to_path_buf()];
    while let Some(directory) = work.pop() {
        for entry in directory.read_dir().unwrap() {
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
                    result.push(path);
                }
            }
        }
    }
    result
}
