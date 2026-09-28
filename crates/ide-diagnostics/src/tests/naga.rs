use super::check_diagnostics_with_config;
use crate::DiagnosticsConfig;
use expect_test::expect;

#[test]
fn ambiguous_clamp_call() {
    check_diagnostics_with_config(
        &DiagnosticsConfig {
            naga_parsing_enabled: true,
            ..DiagnosticsConfig::NONE
        },
        "fn foo() { let ambiguous_clamp = clamp(1u, 0, 1i); }",
        expect![[r#"
            33..38 naga Error 15: inconsistent type passed as argument #3 to `clamp`
        "#]],
    );
}

#[test]
fn invalid_workgroup_size() {
    check_diagnostics_with_config(
        &DiagnosticsConfig {
            naga_validation_enabled: true,
            ..DiagnosticsConfig::NONE
        },
        "@compute @workgroup_size(0, 0, 0) fn foo() { }",
        expect![[r#"
            0..46 naga Error 15: Entry point foo at Compute is invalid: Workgroup size is out of range
        "#]],
    );
}

#[test]
fn ignore_wesl_files() {
    check_diagnostics_with_config(
        &DiagnosticsConfig {
            naga_parsing_enabled: true,
            naga_validation_enabled: true,
            ..DiagnosticsConfig::NONE
        },
        "
//- /package.wesl edition:2026_pre
@compute @workgroup_size(0, 0, 0) fn foo() { 
    let ambiguous_clamp = clamp(1u, 0, 1i);
}",
        expect![""],
    );
}
