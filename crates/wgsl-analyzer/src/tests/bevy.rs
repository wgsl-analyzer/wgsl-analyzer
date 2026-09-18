use base_db::input::PackageOrigin;
use expect_test::expect;

use super::*;

#[test]
fn bevy() {
    check_load_project(
        "bevy/Cargo.toml",
        PackageOrigin::Local,
        expect![[r#"
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_animation/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_asset/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_core_pipeline/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_dylib/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_ecs/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_extract/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_gizmos/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_image/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_internal/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_pbr/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_reflect/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_render/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_scene/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_settings/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_sprite_render/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_state/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_ui_render/Cargo.toml
            Project bevy at bevy/Cargo.toml
            edition: WESL 2025 (Unstable)
            root: bevy/assets/shaders
            dependencies:
            - accesskit
            - anyhow
            - argh
            - bevy_animation
            - bevy_asset
            - bevy_core_pipeline
            - bevy_dylib
            - bevy_ecs
            - bevy_extract
            - bevy_gizmos
            - bevy_image
            - bevy_internal
            - bevy_pbr
            - bevy_reflect
            - bevy_render
            - bevy_scene
            - bevy_settings
            - bevy_sprite_render
            - bevy_state
            - bevy_ui_render
            - bytemuck
            - chacha20
            - crossbeam_channel
            - event_listener
            - flate2
            - futures_lite
            - futures_timer
            - getrandom
            - gltf
            - indexmap
            - nonmax
            - rand
            - ron
            - serde
            - serde_json
            - thiserror
            - tracing
            - ureq
            - wasm_bindgen
            - web_sys
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
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_app/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_asset/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_camera/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_core_pipeline/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_derive/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_diagnostic/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_ecs/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_extract/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_image/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_math/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_reflect/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_render/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_shader/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_utils/Cargo.toml
            Project bevy_anti_alias at bevy/crates/bevy_anti_alias/Cargo.toml
            edition: WESL 2025 (Unstable)
            root: bevy/crates/bevy_anti_alias/src
            dependencies:
            - bevy_app
            - bevy_asset
            - bevy_camera
            - bevy_core_pipeline
            - bevy_derive
            - bevy_diagnostic
            - bevy_ecs
            - bevy_extract
            - bevy_image
            - bevy_math
            - bevy_reflect
            - bevy_render
            - bevy_shader
            - bevy_utils
            - dlss_wgpu
            - tracing
            - uuid
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
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_app/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_asset/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_camera/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_color/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_derive/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_diagnostic/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_ecs/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_extract/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_image/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_light/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_log/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_math/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_pbr/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_platform/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_reflect/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_render/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_shader/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_transform/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_utils/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_window/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/constants/wesl.toml
            Project bevy_core_pipeline at bevy/crates/bevy_core_pipeline/Cargo.toml
            edition: WESL 2025 (Unstable)
            root: bevy/crates/bevy_core_pipeline/src
            dependencies:
            - bevy_app
            - bevy_asset
            - bevy_camera
            - bevy_color
            - bevy_derive
            - bevy_diagnostic
            - bevy_ecs
            - bevy_extract
            - bevy_image
            - bevy_light
            - bevy_log
            - bevy_math
            - bevy_pbr
            - bevy_platform
            - bevy_reflect
            - bevy_render
            - bevy_shader
            - bevy_transform
            - bevy_utils
            - bevy_window
            - bitflags
            - constants
            - indexmap
            - nonmax
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
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_animation/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_app/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_asset/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_audio/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_camera/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_color/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_core_pipeline/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_diagnostic/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_ecs/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_extract/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_image/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_input/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_light/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_log/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_math/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_mesh/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_pbr/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_picking/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_platform/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_reflect/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_render/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_shader/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_sprite/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_state/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_text/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_time/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_transform/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_ui/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_ui_render/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_ui_widgets/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_utils/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_window/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_world_serialization/Cargo.toml
            Project bevy_dev_tools at bevy/crates/bevy_dev_tools/Cargo.toml
            edition: WESL 2025 (Unstable)
            root: bevy/crates/bevy_dev_tools/src
            dependencies:
            - bevy_animation
            - bevy_app
            - bevy_asset
            - bevy_audio
            - bevy_camera
            - bevy_color
            - bevy_core_pipeline
            - bevy_diagnostic
            - bevy_ecs
            - bevy_extract
            - bevy_image
            - bevy_input
            - bevy_light
            - bevy_log
            - bevy_math
            - bevy_mesh
            - bevy_pbr
            - bevy_picking
            - bevy_platform
            - bevy_reflect
            - bevy_render
            - bevy_shader
            - bevy_sprite
            - bevy_state
            - bevy_text
            - bevy_time
            - bevy_transform
            - bevy_ui
            - bevy_ui_render
            - bevy_ui_widgets
            - bevy_utils
            - bevy_window
            - bevy_world_serialization
            - ron
            - serde
            - thiserror
            - tracing
            - x264
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
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_a11y/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_app/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_asset/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_camera/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_color/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_derive/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_ecs/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_input/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_input_focus/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_log/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_math/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_picking/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_platform/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_reflect/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_render/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_scene/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_shader/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_text/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_ui/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_ui_render/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_ui_widgets/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_window/Cargo.toml
            Project bevy_feathers at bevy/crates/bevy_feathers/Cargo.toml
            edition: WESL 2025 (Unstable)
            root: bevy/crates/bevy_feathers/src
            dependencies:
            - accesskit
            - bevy_a11y
            - bevy_app
            - bevy_asset
            - bevy_camera
            - bevy_color
            - bevy_derive
            - bevy_ecs
            - bevy_input
            - bevy_input_focus
            - bevy_log
            - bevy_math
            - bevy_picking
            - bevy_platform
            - bevy_reflect
            - bevy_render
            - bevy_scene
            - bevy_shader
            - bevy_text
            - bevy_ui
            - bevy_ui_render
            - bevy_ui_widgets
            - bevy_window
            - smol_str
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
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_app/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_asset/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_camera/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_color/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_core_pipeline/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_ecs/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_extract/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_gizmos/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_image/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_log/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_material/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_math/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_mesh/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_pbr/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_reflect/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_render/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_shader/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_sprite_render/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_transform/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_utils/Cargo.toml
            Project bevy_gizmos_render at bevy/crates/bevy_gizmos_render/Cargo.toml
            edition: WESL 2025 (Unstable)
            root: bevy/crates/bevy_gizmos_render/src
            dependencies:
            - bevy_app
            - bevy_asset
            - bevy_camera
            - bevy_color
            - bevy_core_pipeline
            - bevy_ecs
            - bevy_extract
            - bevy_gizmos
            - bevy_image
            - bevy_log
            - bevy_material
            - bevy_math
            - bevy_mesh
            - bevy_pbr
            - bevy_reflect
            - bevy_render
            - bevy_shader
            - bevy_sprite_render
            - bevy_transform
            - bevy_utils
            - bytemuck
            - tracing
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
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_app/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_asset/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_camera/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_color/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_core_pipeline/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_derive/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_diagnostic/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_ecs/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_extract/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_gltf/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_image/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_light/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_log/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_material/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_math/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_mesh/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_platform/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_reflect/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_render/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_shader/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_tasks/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_transform/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_utils/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/constants/wesl.toml
            Project bevy_pbr at bevy/crates/bevy_pbr/Cargo.toml
            edition: WESL 2025 (Unstable)
            root: bevy/crates/bevy_pbr/src
            dependencies:
            - arrayvec
            - bevy_app
            - bevy_asset
            - bevy_camera
            - bevy_color
            - bevy_core_pipeline
            - bevy_derive
            - bevy_diagnostic
            - bevy_ecs
            - bevy_extract
            - bevy_gltf
            - bevy_image
            - bevy_light
            - bevy_log
            - bevy_material
            - bevy_math
            - bevy_mesh
            - bevy_platform
            - bevy_reflect
            - bevy_render
            - bevy_shader
            - bevy_tasks
            - bevy_transform
            - bevy_utils
            - bitflags
            - bitvec
            - bytemuck
            - constants
            - derive_more
            - fixedbitset
            - indexmap
            - itertools
            - lz4_flex
            - meshopt
            - metis
            - nonmax
            - offset_allocator
            - range_alloc
            - smallvec
            - static_assertions
            - thiserror
            - tracing
            - wgpu_types
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
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_app/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_asset/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_camera/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_color/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_core_pipeline/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_derive/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_ecs/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_extract/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_image/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_math/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_pbr/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_platform/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_reflect/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_render/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_shader/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_utils/Cargo.toml
            Project bevy_post_process at bevy/crates/bevy_post_process/Cargo.toml
            edition: WESL 2025 (Unstable)
            root: bevy/crates/bevy_post_process/src
            dependencies:
            - bevy_app
            - bevy_asset
            - bevy_camera
            - bevy_color
            - bevy_core_pipeline
            - bevy_derive
            - bevy_ecs
            - bevy_extract
            - bevy_image
            - bevy_math
            - bevy_pbr
            - bevy_platform
            - bevy_reflect
            - bevy_render
            - bevy_shader
            - bevy_utils
            - smallvec
            - thiserror
            - tracing
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
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_app/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_asset/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_camera/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_color/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_core_pipeline/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_derive/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_diagnostic/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_ecs/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_encase_derive/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_extract/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_extract/macros/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_image/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_log/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_material/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_material/macros/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_math/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_mesh/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_platform/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_reflect/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_render/macros/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_shader/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_tasks/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_time/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_transform/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_utils/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_window/Cargo.toml
            Project bevy_render at bevy/crates/bevy_render/Cargo.toml
            edition: WESL 2025 (Unstable)
            root: bevy/crates/bevy_render/src
            dependencies:
            - async_channel
            - bevy_app
            - bevy_asset
            - bevy_camera
            - bevy_color
            - bevy_core_pipeline
            - bevy_derive
            - bevy_diagnostic
            - bevy_ecs
            - bevy_encase_derive
            - bevy_extract
            - bevy_extract_macros
            - bevy_image
            - bevy_log
            - bevy_material
            - bevy_material_macros
            - bevy_math
            - bevy_mesh
            - bevy_platform
            - bevy_reflect
            - bevy_render_macros
            - bevy_shader
            - bevy_tasks
            - bevy_time
            - bevy_transform
            - bevy_utils
            - bevy_window
            - bitflags
            - bytemuck
            - derive_more
            - downcast_rs
            - encase
            - glam
            - image
            - indexmap
            - itertools
            - js_sys
            - nonmax
            - offset_allocator
            - profiling
            - proptest
            - proptest_derive
            - send_wrapper
            - smallvec
            - static_assertions
            - thiserror
            - tracing
            - tracy_client
            - variadics_please
            - wasm_bindgen
            - weak_table
            - web_sys
            - wgpu
            - wgpu_types
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
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_anti_alias/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_app/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_asset/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_camera/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_color/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_core_pipeline/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_derive/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_diagnostic/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_ecs/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_image/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_math/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_mesh/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_pbr/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_platform/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_reflect/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_render/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_shader/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_transform/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_utils/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/constants/wesl.toml
            Project bevy_solari at bevy/crates/bevy_solari/Cargo.toml
            edition: WESL 2025 (Unstable)
            root: bevy/crates/bevy_solari/src
            dependencies:
            - bevy_anti_alias
            - bevy_app
            - bevy_asset
            - bevy_camera
            - bevy_color
            - bevy_core_pipeline
            - bevy_derive
            - bevy_diagnostic
            - bevy_ecs
            - bevy_image
            - bevy_math
            - bevy_mesh
            - bevy_pbr
            - bevy_platform
            - bevy_reflect
            - bevy_render
            - bevy_shader
            - bevy_transform
            - bevy_utils
            - bytemuck
            - constants
            - derive_more
            - tracing
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
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_app/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_asset/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_camera/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_color/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_core_pipeline/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_derive/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_ecs/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_extract/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_image/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_material/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_math/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_mesh/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_platform/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_reflect/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_render/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_shader/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_sprite/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_text/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_transform/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_utils/Cargo.toml
            Project bevy_sprite_render at bevy/crates/bevy_sprite_render/Cargo.toml
            edition: WESL 2025 (Unstable)
            root: bevy/crates/bevy_sprite_render/src
            dependencies:
            - bevy_app
            - bevy_asset
            - bevy_camera
            - bevy_color
            - bevy_core_pipeline
            - bevy_derive
            - bevy_ecs
            - bevy_extract
            - bevy_image
            - bevy_material
            - bevy_math
            - bevy_mesh
            - bevy_platform
            - bevy_reflect
            - bevy_render
            - bevy_shader
            - bevy_sprite
            - bevy_text
            - bevy_transform
            - bevy_utils
            - bitflags
            - bytemuck
            - derive_more
            - fixedbitset
            - nonmax
            - smallvec
            - static_assertions
            - tracing
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
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_a11y/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_app/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_asset/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_camera/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_color/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_core_pipeline/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_derive/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_ecs/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_extract/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_image/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_input_focus/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_math/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_mesh/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_platform/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_reflect/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_render/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_shader/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_sprite/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_sprite_render/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_text/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_transform/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_ui/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/crates/bevy_utils/Cargo.toml
            /home/benjamin/source/wgsl-analyzer/crates/wgsl-analyzer/src/tests/bevy/constants/wesl.toml
            Project bevy_ui_render at bevy/crates/bevy_ui_render/Cargo.toml
            edition: WESL 2025 (Unstable)
            root: bevy/crates/bevy_ui_render/src
            dependencies:
            - bevy_a11y
            - bevy_app
            - bevy_asset
            - bevy_camera
            - bevy_color
            - bevy_core_pipeline
            - bevy_derive
            - bevy_ecs
            - bevy_extract
            - bevy_image
            - bevy_input_focus
            - bevy_math
            - bevy_mesh
            - bevy_platform
            - bevy_reflect
            - bevy_render
            - bevy_shader
            - bevy_sprite
            - bevy_sprite_render
            - bevy_text
            - bevy_transform
            - bevy_ui
            - bevy_utils
            - bytemuck
            - constants
            - derive_more
            - indexmap
            - smallvec
            - tracing
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
