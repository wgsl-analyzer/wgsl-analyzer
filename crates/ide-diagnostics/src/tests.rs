use std::fmt::Write as _;

use expect_test::{Expect, expect};
use ide_db::RootDatabase;
use itertools::Itertools;
use test_fixture::WithFixture as _;

use crate::{Diagnostic, DiagnosticsConfig, Severity};

mod tint;

#[expect(clippy::needless_pass_by_value, reason = "Matches expect! macro")]
#[expect(clippy::use_debug, reason = "useful in tests")]
fn check_diagnostics(
    source: &str,
    expect: Expect,
) {
    let config = DiagnosticsConfig {
        enabled: true,
        semantic_enabled: true,
        naga_parsing_enabled: false,
        naga_validation_enabled: false,
        ..Default::default()
    };
    check_diagnostics_wth_config(&config, source, expect);
}

#[expect(clippy::needless_pass_by_value, reason = "Matches expect! macro")]
#[expect(clippy::use_debug, reason = "useful in tests")]
fn check_diagnostics_wth_config(
    config: &DiagnosticsConfig,
    source: &str,
    expect: Expect,
) {
    let (database, file_id) = RootDatabase::with_single_file(source);
    let diagnostics = crate::diagnostics(&database, config, file_id.file_id(&database));
    let mut actual = String::new();
    for Diagnostic {
        code,
        message,
        range,
        severity,
        source,
        ..
    } in diagnostics
    {
        let severity_text = match severity {
            Severity::Error => "Error",
            Severity::Warning => "Warning",
            Severity::Information => "Information",
            Severity::Hint => "Hint",
        };
        writeln!(
            actual,
            "{range:?} {source} {severity_text} {}: {message}",
            code.as_str()
        );
    }

    expect.assert_eq(&actual);
}

#[test]
fn store_type_must_be_storable() {
    check_diagnostics(
        "fn foo() { var x = 1; var y = &x; }",
        expect![[r#"
            30..32 wgsl-analyzer Error 29: store type must be storable, found ptr<i32>
        "#]],
    );
}

#[test]
fn unexpected_return_value() {
    check_diagnostics(
        "fn foo() { return 0; }",
        expect![[r#"
            18..19 wgsl-analyzer Error 30: unexpected return value of type `integer` in function with no return type
        "#]],
    );
}

#[test]
fn no_builtin_overload() {
    check_diagnostics(
        "fn foo() { var x = 1f + mat2x2f(); }",
        expect![[r#"
            11..34 wgsl-analyzer Error 8: no overload of `+` found for given arguments.Found (f32, mat2x2<f32>), expected one of:
            fn op_binary_number(vecN<U>, vecN<U>) -> vecN<U>
            fn op_binary_number(vecN<U>, U) -> vecN<U>
            fn op_binary_number(T, vecM<T>) -> vecM<T>
            fn op_binary_number(matNxM<f32>, matNxM<f32>) -> matNxM<f32>
            fn op_binary_number(T, T) -> T
        "#]],
    );
}

#[test]
fn deref_not_a_pointer() {
    check_diagnostics(
        "fn foo() { var x = *1f; }",
        expect![[r#"
            20..22 wgsl-analyzer Error 10: cannot dereference expression of type f32
        "#]],
    );
}

#[test]
fn no_constructor() {
    check_diagnostics(
        "fn foo() { var x = vec2f(1, 2, 3); }",
        expect![[r#"
            19..33 wgsl-analyzer Error 18: no overload of constructor `vec2<f32>` found for given arguments. Found (integer, integer, integer), expected one of:
            fn op_vec2_constructor(vec2<T>) -> vec2<T>
            fn op_vec2_constructor(T) -> vec2<T>
            fn op_vec2_constructor(T, T) -> vec2<T>
        "#]],
    );
}

#[test]
fn precedence_sequence_allowed() {
    check_diagnostics(
        "fn foo() { let x = true == true & true; }",
        expect![[r#"
            19..31 wgsl-analyzer Error 19: & sequences may only have unary operands. More complex operands must be this with parenthesized `()`
        "#]],
    );
}

#[test]
fn precedence_sequence_disallowed() {
    check_diagnostics(
        "fn foo() { let x = true == true == true; }",
        expect![[r#"
            19..31 wgsl-analyzer Error 19: == expressions may only have unary operands. More complex operands must be this with parenthesized `()`
        "#]],
    );
}

#[test]
fn global_var_function_address_space_error() {
    check_diagnostics(
        "var<function> not_allowed_at_module_level: u32;",
        expect![[r#"
            0..3 wgsl-analyzer Error 12: address space is only valid in function-scope
            4..12 wgsl-analyzer Error 21: unexpected template argument
        "#]],
    );
}

#[test]
fn invalid_body() {
    check_diagnostics(
        "fn f() { let x: u32 = 1.0; }",
        expect![[r#"
            22..25 wgsl-analyzer Error 2: expected u32, found float
        "#]],
    );
}

#[test]
fn no_host_shareable_error_for_undefined_struct() {
    // https://github.com/wgsl-analyzer/wgsl-analyzer/issues/722
    // When referencing an undefined struct, we should NOT get a spurious
    // "not host-shareable" diagnostic — only the "unresolved" error.
    check_diagnostics(
        "
@group(0) @binding(0)
var<storage> lines: array<LineSegment>;
",
        expect![[r#"
            48..59 wgsl-analyzer Error 14: `LineSegment` not found in scope
        "#]],
    );
}

#[test]
fn reserved_identifier_double_underscore() {
    // https://github.com/wgsl-analyzer/wgsl-analyzer/issues/681
    // Identifiers starting with "__" are reserved by the WGSL spec.
    check_diagnostics(
        "
fn __my_func() {}
",
        expect![[r#"
            3..12 wgsl-analyzer Error 24: `__my_func` is not a valid name for an identifier
        "#]],
    );
}

#[test]
fn non_reserved_identifier_single_underscore() {
    // A single underscore prefix should NOT trigger the reserved identifier diagnostic.
    check_diagnostics(
        "
fn _my_func() {}
",
        expect![""],
    );
}

#[test]
fn incomplete_variable_error() {
    // https://github.com/wgsl-analyzer/wgsl-analyzer/issues/825
    check_diagnostics(
        "
@group(0) @binding(0)
var<storage, read> a: array<f32>;

@group(0) @binding(1) // line 4
var<storage
",
        expect![[r#"
            92..93 wgsl-analyzer Error 16: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            101..101 wgsl-analyzer Error 16: invalid syntax, expected one of: ':', '=', ';'
            22..25 wgsl-analyzer Error 12: address space is only valid for handle or texture types
            26..33 wgsl-analyzer Error 21: unexpected template argument
            26..33 wgsl-analyzer Error 21: unexpected template argument
            89..92 wgsl-analyzer Error 12: address space is only valid for handle or texture types
        "#]],
    );
}

#[test]
fn reserved_word_diagnostic() {
    // WGSL reserved words should produce a diagnostic.
    check_diagnostics(
        "
fn test() {
    let enum = 1u;
}
",
        expect![[r#"
            20..24 wgsl-analyzer Error 16: 'enum' is a reserved word in WGSL
            20..24 wgsl-analyzer Error 16: invalid syntax, expected: <identifier>
        "#]],
    );
}

#[test]
fn invalid_bitcast() {
    // TODO: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/908
    check_diagnostics(
        "
fn foo() { let bar: f32 = bitcast<f32>(vec4u(1, 2, 3, 4)); }
",
        expect![""],
    );
}

#[test]
fn invalid_identifier_underscore() {
    // An identifier must not be _ (a single underscore, U+005F).
    // https://www.w3.org/TR/WGSL/#identifiers
    check_diagnostics(
        "
fn _() {}
fn foo() { let _ = 1; }
",
        expect![[r#"
            3..4 wgsl-analyzer Error 16: invalid syntax, expected: <identifier>
            25..26 wgsl-analyzer Error 16: invalid syntax, expected: <identifier>
        "#]],
    );
}

#[test]
fn binding_array_validates() {
    check_diagnostics(
        "
@group(0) @binding(0) var textures: binding_array<texture_2d<f32>>;
",
        expect![""],
    );
}
