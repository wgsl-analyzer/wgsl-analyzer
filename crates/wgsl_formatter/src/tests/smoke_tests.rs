use std::{ffi::OsString, path::Path};

use expect_test::expect_file;

use crate::test_util::check;

#[test]
fn smoke_tests() {
    let source_path = Path::new("./tests/smoke_tests/source");
    let output_path = Path::new("./tests/smoke_tests/output");

    //At this stage, the old formatter is not idempotent on all the tests.
    // But we want to achieve feature parity first, so we focus on the cases
    // where the old formatter works first, and generate tests that the new formatter works with.
    let skip = [OsString::from("old_formatter_idempotence.wgsl")];

    for entry in std::fs::read_dir(source_path).expect("smoke test source directory should exist") {
        let entry = entry.expect("smoke test source directory should be traversable");
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

            let source = std::fs::read_to_string(smoke_test_source_path)
                .expect("source file should be a readable text file.");

            check(&source, expect_file![smoke_test_output_path]);
        } else {
            panic!("Expected smoke_test directory to not have subdirectories.")
        }
    }
}
