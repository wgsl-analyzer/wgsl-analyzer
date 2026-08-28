use base_db::input::PackageOrigin;
use expect_test::expect;

use super::*;

#[test]
fn bevy() {
    check_load_project(
        "bevy/Cargo.toml",
        PackageOrigin::Local,
        expect![[r#"
            Project bevy at bevy/Cargo.toml
            edition: WESL 2025 (Unstable)
            root: bevy/assets/shaders
            dependencies:
            - bevy_core_pipeline
            - bevy_pbr
            - bevy_render
            - bevy_sprite_render
            - bevy_ui_render
        "#]],
    );

    check_load_project_files(
        "bevy/Cargo.toml",
        PackageOrigin::Local,
        expect![[r#"
            extensions: wgsl, wesl, toml
            include: bevy/assets/shaders
            file: bevy/Cargo.toml
        "#]],
    );
}

#[test]
fn bevy_anti_alias() {
    check_load_project(
        "bevy/crates/bevy_anti_alias/Cargo.toml",
        PackageOrigin::Local,
        expect![[r#"
            Project bevy_anti_alias at bevy/crates/bevy_anti_alias/Cargo.toml
            edition: WESL 2025 (Unstable)
            root: bevy/crates/bevy_anti_alias/src
            dependencies:
            - bevy_core_pipeline
        "#]],
    );

    check_load_project_files(
        "bevy/crates/bevy_anti_alias/Cargo.toml",
        PackageOrigin::Local,
        expect![[r#"
            extensions: wgsl, wesl, toml
            include: bevy/crates/bevy_anti_alias/src
            file: bevy/crates/bevy_anti_alias/Cargo.toml
        "#]],
    );
}

#[test]
fn bevy_core_pipeline() {
    check_load_project(
        "bevy/crates/bevy_core_pipeline/Cargo.toml",
        PackageOrigin::Local,
        expect![[r#"
            Project bevy_core_pipeline at bevy/crates/bevy_core_pipeline/Cargo.toml
            edition: WESL 2025 (Unstable)
            root: bevy/crates/bevy_core_pipeline/src
            dependencies:
            - bevy_pbr
            - bevy_render
            - constants
        "#]],
    );

    check_load_project_files(
        "bevy/crates/bevy_core_pipeline/Cargo.toml",
        PackageOrigin::Local,
        expect![[r#"
            extensions: wgsl, wesl, toml
            include: bevy/crates/bevy_core_pipeline/src
            file: bevy/crates/bevy_core_pipeline/Cargo.toml
        "#]],
    );
}

#[test]
fn bevy_dev_tools() {
    check_load_project(
        "bevy/crates/bevy_dev_tools/Cargo.toml",
        PackageOrigin::Local,
        expect![[r#"
            Project bevy_dev_tools at bevy/crates/bevy_dev_tools/Cargo.toml
            edition: WESL 2025 (Unstable)
            root: bevy/crates/bevy_dev_tools/src
            dependencies:
            - bevy_core_pipeline
            - bevy_pbr
            - bevy_render
            - bevy_ui_render
        "#]],
    );

    check_load_project_files(
        "bevy/crates/bevy_dev_tools/Cargo.toml",
        PackageOrigin::Local,
        expect![[r#"
            extensions: wgsl, wesl, toml
            include: bevy/crates/bevy_dev_tools/src
            file: bevy/crates/bevy_dev_tools/Cargo.toml
        "#]],
    );
}

#[test]
fn bevy_feathers() {
    check_load_project(
        "bevy/crates/bevy_feathers/Cargo.toml",
        PackageOrigin::Local,
        expect![[r#"
            Project bevy_feathers at bevy/crates/bevy_feathers/Cargo.toml
            edition: WESL 2025 (Unstable)
            root: bevy/crates/bevy_feathers/src
            dependencies:
            - bevy_render
            - bevy_ui_render
        "#]],
    );

    check_load_project_files(
        "bevy/crates/bevy_feathers/Cargo.toml",
        PackageOrigin::Local,
        expect![[r#"
            extensions: wgsl, wesl, toml
            include: bevy/crates/bevy_feathers/src
            file: bevy/crates/bevy_feathers/Cargo.toml
        "#]],
    );
}

#[test]
fn bevy_gizmos_render() {
    check_load_project(
        "bevy/crates/bevy_gizmos_render/Cargo.toml",
        PackageOrigin::Local,
        expect![[r#"
            Project bevy_gizmos_render at bevy/crates/bevy_gizmos_render/Cargo.toml
            edition: WESL 2025 (Unstable)
            root: bevy/crates/bevy_gizmos_render/src
            dependencies:
            - bevy_render
        "#]],
    );

    check_load_project_files(
        "bevy/crates/bevy_gizmos_render/Cargo.toml",
        PackageOrigin::Local,
        expect![[r#"
            extensions: wgsl, wesl, toml
            include: bevy/crates/bevy_gizmos_render/src
            file: bevy/crates/bevy_gizmos_render/Cargo.toml
        "#]],
    );
}

#[test]
fn bevy_pbr() {
    check_load_project(
        "bevy/crates/bevy_pbr/Cargo.toml",
        PackageOrigin::Local,
        expect![[r#"
            Project bevy_pbr at bevy/crates/bevy_pbr/Cargo.toml
            edition: WESL 2025 (Unstable)
            root: bevy/crates/bevy_pbr/src
            dependencies:
            - bevy_core_pipeline
            - bevy_render
            - constants
        "#]],
    );

    check_load_project_files(
        "bevy/crates/bevy_pbr/Cargo.toml",
        PackageOrigin::Local,
        expect![[r#"
            extensions: wgsl, wesl, toml
            include: bevy/crates/bevy_pbr/src
            file: bevy/crates/bevy_pbr/Cargo.toml
        "#]],
    );
}

#[test]
fn bevy_post_process() {
    check_load_project(
        "bevy/crates/bevy_post_process/Cargo.toml",
        PackageOrigin::Local,
        expect![[r#"
            Project bevy_post_process at bevy/crates/bevy_post_process/Cargo.toml
            edition: WESL 2025 (Unstable)
            root: bevy/crates/bevy_post_process/src
            dependencies:
            - bevy_core_pipeline
            - bevy_pbr
            - bevy_render
        "#]],
    );

    check_load_project_files(
        "bevy/crates/bevy_post_process/Cargo.toml",
        PackageOrigin::Local,
        expect![[r#"
            extensions: wgsl, wesl, toml
            include: bevy/crates/bevy_post_process/src
            file: bevy/crates/bevy_post_process/Cargo.toml
        "#]],
    );
}

#[test]
fn bevy_render() {
    check_load_project(
        "bevy/crates/bevy_render/Cargo.toml",
        PackageOrigin::Local,
        expect![[r#"
            Project bevy_render at bevy/crates/bevy_render/Cargo.toml
            edition: WESL 2025 (Unstable)
            root: bevy/crates/bevy_render/src
            dependencies:
            - bevy_core_pipeline
        "#]],
    );

    check_load_project_files(
        "bevy/crates/bevy_render/Cargo.toml",
        PackageOrigin::Local,
        expect![[r#"
            extensions: wgsl, wesl, toml
            include: bevy/crates/bevy_render/src
            file: bevy/crates/bevy_render/Cargo.toml
        "#]],
    );
}

#[test]
fn bevy_solari() {
    check_load_project(
        "bevy/crates/bevy_solari/Cargo.toml",
        PackageOrigin::Local,
        expect![[r#"
            Project bevy_solari at bevy/crates/bevy_solari/Cargo.toml
            edition: WESL 2025 (Unstable)
            root: bevy/crates/bevy_solari/src
            dependencies:
            - bevy_core_pipeline
            - bevy_pbr
            - bevy_render
            - constants
        "#]],
    );

    check_load_project_files(
        "bevy/crates/bevy_solari/Cargo.toml",
        PackageOrigin::Local,
        expect![[r#"
            extensions: wgsl, wesl, toml
            include: bevy/crates/bevy_solari/src
            file: bevy/crates/bevy_solari/Cargo.toml
        "#]],
    );
}

#[test]
fn bevy_sprite_render() {
    check_load_project(
        "bevy/crates/bevy_sprite_render/Cargo.toml",
        PackageOrigin::Local,
        expect![[r#"
            Project bevy_sprite_render at bevy/crates/bevy_sprite_render/Cargo.toml
            edition: WESL 2025 (Unstable)
            root: bevy/crates/bevy_sprite_render/src
            dependencies:
            - bevy_core_pipeline
            - bevy_render
        "#]],
    );

    check_load_project_files(
        "bevy/crates/bevy_sprite_render/Cargo.toml",
        PackageOrigin::Local,
        expect![[r#"
            extensions: wgsl, wesl, toml
            include: bevy/crates/bevy_sprite_render/src
            file: bevy/crates/bevy_sprite_render/Cargo.toml
        "#]],
    );
}

#[test]
fn bevy_ui_render() {
    check_load_project(
        "bevy/crates/bevy_ui_render/Cargo.toml",
        PackageOrigin::Local,
        expect![[r#"
            Project bevy_ui_render at bevy/crates/bevy_ui_render/Cargo.toml
            edition: WESL 2025 (Unstable)
            root: bevy/crates/bevy_ui_render/src
            dependencies:
            - bevy_render
            - constants
        "#]],
    );

    check_load_project_files(
        "bevy/crates/bevy_ui_render/Cargo.toml",
        PackageOrigin::Local,
        expect![[r#"
            extensions: wgsl, wesl, toml
            include: bevy/crates/bevy_ui_render/src
            file: bevy/crates/bevy_ui_render/Cargo.toml
        "#]],
    );
}
