use std::fs;

use crate::{codegen::add_preamble, project_root};

pub(crate) fn generate(check: bool) {
    if check {
        return;
    }
    // Generate assists manual.
    // Note that we do *not* commit manual to the git repository.
    // Instead, `cargo xtask release` runs this test before making a release.
    let contents = String::new();
    let contents = add_preamble(crate::flags::CodegenType::AssistsDocTests, contents);
    let destination = project_root().join("docs/book/src/assists_generated.md");
    fs::write(destination, contents).unwrap();
}
