use std::path::{Path, PathBuf};
use xshell::{Shell, cmd};

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

pub(crate) fn detect_target(shell: &Shell) -> String {
    match std::env::var("WA_TARGET") {
        Ok(target) => target,
        _ => match cmd!(shell, "rustc --print=host-tuple").read() {
            Ok(target) => target,
            Err(error) => {
                panic!("Failed to detect target: {error}\nPlease set WA_TARGET explicitly")
            },
        },
    }
}
