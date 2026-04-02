use std::{ffi::OsString, path::Path};

use expect_test::{expect, expect_file};

use crate::test_util::{check, check_tabs};

mod attributes;
mod code_indentation;
mod comments;
mod fn_body;
mod fn_signature;
mod struct_def;
mod types;

mod bevy_reference;
mod directives;
mod expressions;
mod statements;

#[test]
fn format_empty() {
    check("", expect![[""]]);
}

#[test]
fn snapshots() {
    let source_path = Path::new("./tests/snapshots/source");
    let output_path = Path::new("./tests/snapshots/output");

    let skip: &[OsString] = &[];

    for entry in
        std::fs::read_dir(source_path).expect("snapshot test source directory should exist")
    {
        let entry = entry.expect("snapshot test source directory should be traversable");
        let smoke_test_source_path = entry.path();
        if smoke_test_source_path.is_file() {
            let smoke_test_name = smoke_test_source_path
                .file_name()
                .expect("smoke_test source should be a normal file.");

            if skip.iter().any(|it| it == smoke_test_name) {
                continue;
            }

            let smoke_test_output_path = std::env::current_dir()
                .unwrap()
                .join(output_path)
                .join(smoke_test_name);

            let source = std::fs::read_to_string(&smoke_test_source_path)
                .expect("source file should be a readable text file.");

            let result = std::panic::catch_unwind(|| {
                check(&source, expect_file![smoke_test_output_path]);
            });
            result.unwrap_or_else(|err| {
                panic!(
                    "Smoke test failed: {}\n{err:?}",
                    smoke_test_source_path.display(),
                )
            });
        } else {
            panic!("Expected smoke_test directory to not have subdirectories.")
        }
    }
}
