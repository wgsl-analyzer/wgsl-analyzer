use std::path::PathBuf;

use anyhow::Context as _;
use ignore::Walk;

use crate::FormattingSource;

/// Resolves a list of patterns into concrete file paths.
///
/// Each pattern is interpreted as:
///
/// - `"-"` → stdin
/// - A directory path → recursively walk for `.wgsl` files
/// - A glob pattern (contains `*`, `?`, or `[`) → expand using [`glob::glob`]
/// - Otherwise → a literal file path
pub fn resolve_patterns(patterns: &[String]) -> Result<Vec<FormattingSource>, anyhow::Error> {
    let mut files = Vec::new();
    for pattern in patterns {
        if pattern == "-" {
            files.push(FormattingSource::Stdin);
        } else if PathBuf::from(pattern).is_dir() {
            collect_wgsl_files(&PathBuf::from(pattern), &mut files)?;
        } else if pattern.contains('*') || pattern.contains('?') || pattern.contains('[') {
            let paths =
                glob::glob(pattern).with_context(|| format!("invalid glob pattern: {pattern}"))?;
            for entry in paths {
                let path =
                    entry.with_context(|| format!("error reading glob match for: {pattern}"))?;
                if path.is_dir() {
                    collect_wgsl_files(&path, &mut files)?;
                } else {
                    files.push(FormattingSource::File(path));
                }
            }
        } else {
            files.push(FormattingSource::File(PathBuf::from(pattern)));
        }
    }
    Ok(files)
}

/// Recursively collects all `.wgsl` and `.wesl` files under `directory`.
fn collect_wgsl_files(
    directory: &PathBuf,
    out: &mut Vec<FormattingSource>,
) -> Result<(), anyhow::Error> {
    for result in Walk::new(directory) {
        // TODO Do the stuff
    }
    Ok(())
}
