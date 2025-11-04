use std::{
    collections::HashSet,
    path::Path,
};

use xshell::Shell;


use crate::{flags::Tidy, project_root};

impl Tidy {
    #[expect(clippy::unused_self, reason = "better API")]
    #[expect(
        clippy::unnecessary_wraps,
        reason = "command handlers have a specific signature"
    )]
    pub(crate) fn run(
        &self,
        shell: &Shell,
    ) -> anyhow::Result<()> {
        check_lsp_extensions_docs(shell);
        Ok(())
    }
}

fn check_lsp_extensions_docs(shell: &Shell) {
    let expected_hash = {
        let lsp_ext_rs = shell
            .read_file(project_root().join("crates/wgsl-analyzer/src/lsp/extensions.rs"))
            .unwrap();
        stable_hash(lsp_ext_rs.as_str())
    };

    let actual_hash = {
        let lsp_extensions_md = shell
            .read_file(project_root().join("docs/book/src/contributing/lsp-extensions.md"))
            .unwrap();
        let text = lsp_extensions_md
            .lines()
            .find_map(|line| line.strip_prefix("crates/wgsl-analyzer/src/lsp/extensions.rs hash:"))
            .unwrap()
            .trim();
        u64::from_str_radix(text, 16).unwrap()
    };

    assert!(
        (actual_hash == expected_hash),
        "
crates/wgsl-analyzer/src/lsp/extensions.rs was changed without touching lsp-extensions.md.

Expected hash: {expected_hash:x}
Actual hash:   {actual_hash:x}

Please adjust docs/book/src/contributing/lsp-extensions.md.
"
    );
}

fn check_test_attrs(
    path: &Path,
    text: &str,
) {
    let panic_rule =
        "https://github.com/wgsl-analyzer/wgsl-analyzer/blob/master/docs/dev/style.md#should_panic";
    let need_panic: &[&str] = &[
        // This file.
        "slow-tests/tidy.rs",
        "test-utils/src/fixture.rs",
        // Generated code from lints contains doc tests in string literals.
        "ide-db/src/generated/lints.rs",
    ];
    assert!(
        !text.contains("#[should_panic")
            || need_panic
                .iter()
                .any(|path_segment| path.ends_with(path_segment)),
        "\ndo not add `#[should_panic]` tests, see:\n\n    {panic_rule}\n\n   {}\n",
        path.display(),
    );
}

fn is_exclude_directory(
    path: &Path,
    directories_to_exclude: &[&str],
) -> bool {
    path.strip_prefix(project_root())
        .unwrap()
        .components()
        .rev()
        .skip(1)
        .filter_map(|component| component.as_os_str().to_str())
        .any(|directory| directories_to_exclude.contains(&directory))
}

#[derive(Default)]
struct TidyMarks {
    hits: HashSet<String>,
    checks: HashSet<String>,
}

impl TidyMarks {
    fn visit(
        &mut self,
        _path: &Path,
        text: &str,
    ) {
        find_marks(&mut self.hits, text, "hit");
        find_marks(&mut self.checks, text, "check");
        find_marks(&mut self.checks, text, "check_count");
    }

    fn finish(self) {
        assert!(!self.hits.is_empty());

        let diff: Vec<_> = self
            .hits
            .symmetric_difference(&self.checks)
            .map(std::string::String::as_str)
            .collect();

        assert!(diff.is_empty(), "unpaired marks: {diff:?}");
    }
}

#[expect(deprecated, reason = "stable")]
fn stable_hash(text: &str) -> u64 {
    use std::hash::{Hash as _, Hasher as _, SipHasher};

    let mut hasher = SipHasher::default();
    text.hash(&mut hasher);
    hasher.finish()
}

fn find_marks(
    set: &mut HashSet<String>,
    mut text: &str,
    mark: &str,
) {
    let mut previous_text = "";
    while text != previous_text {
        previous_text = text;
        if let Some(index) = text.find(mark) {
            text = &text[index + mark.len()..];
            if let Some(stripped_text) = text.strip_prefix("!(") {
                text = stripped_text.trim_start();
                if let Some(index2) =
                    text.find(|character: char| !(character.is_alphanumeric() || character == '_'))
                {
                    let mark_text = &text[..index2];
                    set.insert(mark_text.to_owned());
                    text = &text[index2..];
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "not implemented yet"]
    fn test() {
        Tidy {}.run(&Shell::new().unwrap()).unwrap();
    }
}
