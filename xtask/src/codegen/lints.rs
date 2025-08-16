use std::fs;

use crate::{codegen::add_preamble, project_root};

const DESTINATION: &str = "crates/ide-db/src/generated/lints.rs";
pub(crate) fn generate(check: bool) {
    // Do not generate assists manual when run with `--check`
    if check {
        return;
    }
    let contents = String::new();
    let contents = add_preamble(crate::flags::CodegenType::LintDefinitions, contents);
    let destination = project_root().join(DESTINATION);
    fs::write(destination, contents).unwrap();
}
