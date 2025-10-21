use std::fs;

use crate::{codegen::add_preamble, project_root};

pub(crate) fn generate(check: bool) {
    // Do not generate assists manual when run with `--check`
    if check {
        return;
    }
    let contents = String::new();
    let contents = add_preamble(crate::flags::CodegenType::DiagnosticsDocs, contents);
    let destination = project_root().join("docs/book/src/diagnostics_generated.md");
    fs::write(destination, contents).unwrap();
}
