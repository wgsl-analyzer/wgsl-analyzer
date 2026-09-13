use std::{
    path::{self, PathBuf},
    string::ToString,
};

use crate::{FormattingSource, patterns::resolve_patterns};

fn get_project_directory() -> PathBuf {
    path::absolute(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/tests/project")).unwrap()
}

fn resolve(paths: &[&'static str]) -> Vec<FormattingSource> {
    let paths: Vec<_> = paths.iter().map(ToString::to_string).collect();
    let mut result = resolve_patterns(&paths).expect("Reading Test files should not error.");
    result.sort();
    result
}

#[test]
fn specify_files_simple() {
    let dir = get_project_directory();
    std::env::set_current_dir(&dir);
    let result = resolve(&["aaaa.wgsl", "bbbb.wgsl", "aaaa/file_a.wgsl"]);

    assert_eq!(
        result,
        vec![
            FormattingSource::File(dir.join("aaaa/file_a.wgsl")),
            FormattingSource::File(dir.join("aaaa.wgsl")),
            FormattingSource::File(dir.join("bbbb.wgsl")),
        ]
    );
}

#[test]
fn specify_file_that_is_ignored() {
    let dir = get_project_directory();
    std::env::set_current_dir(&dir);
    let mut result = resolve(&["generated/gen_a.wgsl"]);

    assert_eq!(
        result,
        vec![FormattingSource::File(dir.join("generated/gen_a.wgsl")),]
    );

    let mut result = resolve(&["generated"]);

    assert_eq!(result, vec![]);
}

#[test]
fn specify_whole_dir() {
    let dir = get_project_directory();
    std::env::set_current_dir(&dir);
    let mut result = resolve(&["aaaa/"]);

    assert_eq!(
        result,
        vec![FormattingSource::File(dir.join("generated/gen_a.wgsl")),]
    );

    let mut result = resolve(&["generated"]);

    assert_eq!(result, vec![]);
}
