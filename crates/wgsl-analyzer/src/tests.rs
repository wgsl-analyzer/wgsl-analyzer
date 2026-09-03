#![expect(clippy::use_debug, reason = "tests")]

use std::fmt::{self, Write as _};

use base_db::input::PackageOrigin;
use expect_test::{Expect, expect};
use itertools::Itertools as _;
use project_model::{ProjectManifest, WeslPackage};
use test_utils::project_root;
use vfs::{AbsPath, AbsPathBuf};

mod bevy;

use crate::{
    discover::{LoadPackageMessage, LoadPackageTask},
    reload::to_load_and_source_root_config,
};

fn get_test_directory() -> AbsPathBuf {
    AbsPathBuf::try_from(env!("CARGO_MANIFEST_DIR"))
        .unwrap()
        .join("src/tests")
}

fn print_path(
    path: &AbsPath,
    base: &AbsPath,
) -> String {
    let relative_path = path.strip_prefix(base).unwrap().as_utf8_path();
    relative_path
        .components()
        .map(|component| component.as_str())
        .join("/")
}

#[expect(clippy::needless_pass_by_value, reason = "matches expect! macro")]
fn check_load_project(
    manifest: &str,
    origin: PackageOrigin,
    expect: Expect,
) {
    let mut actual = String::new();

    let test_directory = get_test_directory();
    let project_manifest =
        ProjectManifest::from_manifest_file(test_directory.join(manifest)).unwrap();
    let (load_package_sender, load_package_receiver) = crossbeam_channel::unbounded();
    let task = LoadPackageTask::new(project_manifest, origin, load_package_sender);
    task.run();
    task.join();
    std::mem::drop(task);

    // Either a finished or an error message should happen
    for message in &load_package_receiver {
        match message {
            LoadPackageMessage::Finished { manifest, project } => {
                let project_name = match project.display_name {
                    Some(name) => format!("Project {name}"),
                    None => "Unnamed project".to_owned(),
                };
                writeln!(
                    actual,
                    "{project_name} at {}",
                    print_path(&manifest, &test_directory)
                )
                .unwrap();
                writeln!(actual, "edition: {}", project.edition).unwrap();
                writeln!(
                    actual,
                    "root: {}",
                    print_path(project.root.as_path().unwrap(), &test_directory),
                );
                writeln!(actual, "dependencies:").unwrap();
                for dependency in project.dependencies {
                    writeln!(actual, "- {}", dependency.name());
                }
            },
            LoadPackageMessage::Error { error, source } => {
                writeln!(actual, "{error} - {source:?}");
            },
            LoadPackageMessage::Dependency { task } => {},
            LoadPackageMessage::Progress { message } => {},
        }
    }

    expect.assert_eq(&actual);
}

#[expect(clippy::needless_pass_by_value, reason = "matches expect! macro")]
fn check_load_project_files(
    manifest: &str,
    origin: PackageOrigin,
    expect: Expect,
) {
    let mut actual = String::new();

    let test_directory = get_test_directory();
    let project_manifest =
        ProjectManifest::from_manifest_file(test_directory.join(manifest)).unwrap();
    let (load_package_sender, load_package_receiver) = crossbeam_channel::unbounded();
    let task = LoadPackageTask::new(project_manifest, origin, load_package_sender);
    task.run();
    task.join();
    std::mem::drop(task);

    let project = load_package_receiver
        .iter()
        .filter_map(|message| match message {
            LoadPackageMessage::Finished { project, .. } => Some(project),
            LoadPackageMessage::Error { .. }
            | LoadPackageMessage::Dependency { .. }
            | LoadPackageMessage::Progress { .. } => None,
        })
        .exactly_one()
        .unwrap();

    let (load, _) = to_load_and_source_root_config([project.to_root().unwrap()].to_vec());

    for entry in load {
        match entry {
            vfs::loader::Entry::Files(paths) => {
                for file_path in paths {
                    writeln!(actual, "file: {}", print_path(&file_path, &test_directory));
                }
            },
            vfs::loader::Entry::Directories(directories) => {
                writeln!(actual, "extensions: {}", directories.extensions.join(", "));
                for directory_path in directories.include {
                    writeln!(
                        actual,
                        "include: {}",
                        print_path(&directory_path, &test_directory)
                    );
                }
                for directory_path in directories.exclude {
                    writeln!(
                        actual,
                        "exclude: {}",
                        print_path(&directory_path, &test_directory)
                    );
                }
            },
        }
    }
    expect.assert_eq(&actual);
}

#[test]
fn simple_wesl() {
    check_load_project(
        "simple_wesl/wesl.toml",
        PackageOrigin::Local,
        expect![[r#"
            Project simple_wesl at simple_wesl/wesl.toml
            edition: WESL 2025 (Unstable)
            root: simple_wesl/shaders
            dependencies:
        "#]],
    );

    check_load_project_files(
        "simple_wesl/wesl.toml",
        PackageOrigin::Local,
        expect![[r#"
            extensions: wgsl, wesl, toml
            include: simple_wesl/shaders
            file: simple_wesl/wesl.toml
        "#]],
    );
}

#[test]
fn flat_wesl() {
    check_load_project(
        "flat_wesl/wesl.toml",
        PackageOrigin::Local,
        expect![[r#"
            Project flat_wesl at flat_wesl/wesl.toml
            edition: WESL 2025 (Unstable)
            root: flat_wesl
            dependencies:
        "#]],
    );

    check_load_project_files(
        "flat_wesl/wesl.toml",
        PackageOrigin::Local,
        expect![[r#"
            extensions: wgsl, wesl, toml
            include: flat_wesl
            file: flat_wesl/wesl.toml
        "#]],
    );
}

#[test]
fn wesl_with_dependencies() {
    check_load_project(
        "wesl_with_dependencies/wesl.toml",
        PackageOrigin::Local,
        expect![[r#"
            Project wesl_with_dependencies at wesl_with_dependencies/wesl.toml
            edition: WESL 2025 (Unstable)
            root: wesl_with_dependencies
            dependencies:
            - nested
            - simple_wesl
        "#]],
    );

    check_load_project_files(
        "wesl_with_dependencies/wesl.toml",
        PackageOrigin::Local,
        expect![[r#"
            extensions: wgsl, wesl, toml
            include: wesl_with_dependencies
            file: wesl_with_dependencies/wesl.toml
        "#]],
    );
}

#[test]
fn simple_cargo() {
    check_load_project(
        "simple_cargo/Cargo.toml",
        PackageOrigin::Local,
        expect![[r#"
            Project simple_cargo at simple_cargo/Cargo.toml
            edition: WESL 2025 (Unstable)
            root: simple_cargo/src
            dependencies:
        "#]],
    );

    check_load_project_files(
        "simple_cargo/Cargo.toml",
        PackageOrigin::Local,
        expect![[r#"
            extensions: wgsl, wesl, toml
            include: simple_cargo/src
            file: simple_cargo/Cargo.toml
        "#]],
    );
}

#[test]
fn wesl_with_dependencies_cargo() {
    check_load_project(
        "wesl_with_dependencies_cargo/Cargo.toml",
        PackageOrigin::Local,
        expect![[r#"
            Project wesl_with_dependencies_cargo at wesl_with_dependencies_cargo/Cargo.toml
            edition: WESL 2025 (Unstable)
            root: wesl_with_dependencies_cargo
            dependencies:
            - nested
            - simple_wesl
        "#]],
    );

    check_load_project_files(
        "wesl_with_dependencies_cargo/Cargo.toml",
        PackageOrigin::Local,
        expect![[r#"
            extensions: wgsl, wesl, toml
            include: wesl_with_dependencies_cargo
            file: wesl_with_dependencies_cargo/Cargo.toml
        "#]],
    );
}
