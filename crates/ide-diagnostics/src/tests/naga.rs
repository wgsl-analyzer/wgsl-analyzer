use super::check_diagnostics_with_config;
use crate::DiagnosticsConfig;
use expect_test::expect;

#[test]
fn float_atomics_require_native_features() {
    fn check<N: crate::naga::Naga>() {
        let module =
            N::parse("@group(0) @binding(0) var<storage, read_write> value: atomic<f32>;").unwrap();
        assert!(N::validate(&module, false).is_err());
        assert!(N::validate(&module, true).is_ok());

        let module =
            N::parse("@group(0) @binding(0) var<storage, read_write> value: atomic<u32>;").unwrap();
        assert!(N::validate(&module, false).is_ok());
        assert!(N::validate(&module, true).is_ok());
    }

    check::<crate::naga::Naga27>();
    check::<crate::naga::Naga28>();
    check::<crate::naga::Naga29>();
    check::<crate::naga::NagaMain>();
}

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
