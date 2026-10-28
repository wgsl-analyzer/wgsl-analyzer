use std::{
    fs,
    path::PathBuf,
    sync::atomic::{AtomicUsize, Ordering},
};

use paths::AbsPathBuf;

use super::*;

/// Self-deleting directory. Deletes itself when dropped.
struct Scratch(PathBuf);

impl Scratch {
    fn new(name: &str) -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        let directory = std::env::temp_dir().join(format!(
            "wgsl-analyzer-discover-{}-{}-{name}",
            std::process::id(),
            COUNTER.fetch_add(1, Ordering::Relaxed),
        ));
        drop(fs::remove_dir_all(&directory));
        fs::create_dir_all(&directory).unwrap();
        Self(directory)
    }

    fn write(
        &self,
        relative: &str,
        contents: &str,
    ) {
        let path = self.0.join(relative);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, contents).unwrap();
    }

    fn path(
        &self,
        relative: &str,
    ) -> AbsPathBuf {
        let path = self.0.join(relative);
        fs::create_dir_all(&path).unwrap();
        AbsPathBuf::assert_utf8(path)
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        drop(fs::remove_dir_all(&self.0));
    }
}

const CARGO_TOML_WORKSPACE: &str = r#"
[workspace]
members = ["crate"]
"#;

const CARGO_TOML_PACKAGE_WITHOUT: &str = r#"
[package]
name = "plain"
version = "0.1.0"
"#;

const CARGO_TOML_PACKAGE_WITH: &str = r#"
[package]
name = "shaders"
version = "0.1.0"

[package.metadata.wgsl-analyzer]
"#;

const WESL_TOML: &str = r#""#;

const WGSL: &str = "const A = 3;";

/// Mirrors the intent of `find_cargo_metadata_table(..).is_ok()` without running cargo.
fn has_metadata_table(manifest: &ManifestPath) -> bool {
    fs::read_to_string(manifest.to_string())
        .is_ok_and(|text| text.contains("[package.metadata.wgsl-analyzer]"))
}

/// Variant plus manifest path relative to `root`, with `/` separators.
fn describe(
    root: &Scratch,
    found: Option<ProjectManifest>,
) -> Option<(&'static str, String)> {
    let found = found?;
    let kind = match &found {
        ProjectManifest::CargoToml(_) => "cargo",
        ProjectManifest::WeslToml(_) => "wesl",
        ProjectManifest::ProjectJson(_) => "json",
    };
    let full = found.to_string().replace('\\', "/");
    let prefix = root.0.to_string_lossy().replace('\\', "/");
    let relative = full
        .strip_prefix(&prefix)
        .unwrap_or(&full)
        .trim_start_matches('/')
        .to_owned();
    Some((kind, relative))
}

#[test]
fn unrelated_cargo_project_is_not_wesl_manifest() {
    let scratch = Scratch::new("wgsl-next-to-cargo-workspace");
    scratch.write("Cargo.toml", CARGO_TOML_WORKSPACE);
    scratch.write("crate/Cargo.toml", CARGO_TOML_PACKAGE_WITHOUT);
    scratch.write("shaders/shader.wgsl", WGSL);

    let shaders = scratch.path("shaders");
    let found = ProjectManifest::discover(&shaders, true, has_metadata_table);
    assert_eq!(describe(&scratch, found), None);
}

#[test]
fn cargo_workspace_is_not_wesl_manifest() {
    let scratch = Scratch::new("cargo-alone");
    scratch.write("Cargo.toml", CARGO_TOML_WORKSPACE);

    let root = AbsPathBuf::assert_utf8(scratch.0.clone());
    let found = ProjectManifest::discover(&root, false, has_metadata_table);
    assert_eq!(describe(&scratch, found), None);
}

#[test]
fn cargo_package_is_not_wesl_manifest() {
    let scratch = Scratch::new("wgsl-next-to-cargo-package");
    scratch.write("Cargo.toml", CARGO_TOML_PACKAGE_WITHOUT);
    scratch.write("src/shader.wgsl", WGSL);

    let source = scratch.path("src");
    let found = ProjectManifest::discover(&source, true, has_metadata_table);
    assert_eq!(describe(&scratch, found), None);
}

#[test]
fn cargo_package_with_metadata_table_is_discovered() {
    let scratch = Scratch::new("wesl-package");
    scratch.write("Cargo.toml", CARGO_TOML_PACKAGE_WITH);
    scratch.write("src/shader.wesl", WGSL);

    let source = scratch.path("src");
    let found = ProjectManifest::discover(&source, true, has_metadata_table);
    assert_eq!(
        describe(&scratch, found),
        Some(("cargo", "Cargo.toml".to_owned()))
    );
}

#[test]
fn cargo_toml_without_metadata_keeps_walking() {
    let scratch = Scratch::new("walk-past-rejected");
    scratch.write("wesl.toml", WESL_TOML);
    scratch.write("project/Cargo.toml", CARGO_TOML_WORKSPACE);
    scratch.write("project/shaders/shader.wesl", WGSL);

    let shaders = scratch.path("project/shaders");
    let found = ProjectManifest::discover(&shaders, true, has_metadata_table);
    assert_eq!(
        describe(&scratch, found),
        Some(("wesl", "wesl.toml".to_owned()))
    );
}

#[test]
fn rejected_cargo_toml_without_parent_search_gives_none() {
    let scratch = Scratch::new("no-parent-search");
    scratch.write("wesl.toml", WESL_TOML);
    scratch.write("project/Cargo.toml", CARGO_TOML_WORKSPACE);

    let project = scratch.path("project");
    let found = ProjectManifest::discover(&project, false, has_metadata_table);
    assert_eq!(describe(&scratch, found), None);
}

#[test]
fn wesl_toml_beats_cargo_toml() {
    let scratch = Scratch::new("wesl-beats-cargo");
    scratch.write("wesl.toml", WESL_TOML);
    scratch.write("Cargo.toml", CARGO_TOML_WORKSPACE);

    let root = AbsPathBuf::assert_utf8(scratch.0.clone());
    let found = ProjectManifest::discover(&root, false, |_| {
        panic!("filter must not run for non-Cargo manifests")
    });
    assert_eq!(
        describe(&scratch, found),
        Some(("wesl", "wesl.toml".to_owned()))
    );
}

#[test]
fn nearest_accepted_manifest_wins() {
    let scratch = Scratch::new("nearest-wins");
    scratch.write("Cargo.toml", CARGO_TOML_PACKAGE_WITH);
    scratch.write("inner/Cargo.toml", CARGO_TOML_PACKAGE_WITH);
    scratch.write("inner/src/shader.wesl", WGSL);

    let source = scratch.path("inner/src");
    let found = ProjectManifest::discover(&source, true, has_metadata_table);
    assert_eq!(
        describe(&scratch, found),
        Some(("cargo", "inner/Cargo.toml".to_owned()))
    );
}
