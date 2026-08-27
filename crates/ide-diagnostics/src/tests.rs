use std::fmt::Write as _;

use expect_test::{Expect, expect};
use ide_db::RootDatabase;
use itertools::Itertools;
use syntax::ExtensionsConfig;
use test_fixture::WithFixture as _;

use crate::{Diagnostic, DiagnosticsConfig, Severity};

mod tint;

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
    check_diagnostics_with_config(&config, source, expect);
}

#[expect(clippy::needless_pass_by_value, reason = "Matches expect! macro")]
#[expect(clippy::use_debug, reason = "useful in tests")]
fn check_diagnostics_with_config(
    config: &DiagnosticsConfig,
    source: &str,
    expect: Expect,
) {
    let (db, file_id) = RootDatabase::with_single_file(source);
    let diagnostics = crate::diagnostics(&db, config, file_id.file_id(&db));
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
fn infer_incr_decr_must_be_integer_scalar() {
    check_diagnostics(
        "fn foo() { var x = true; x++; }",
        expect![[r#"
            25..26 wgsl-analyzer Error 2: expected i32 or u32, found bool
        "#]],
    );
}

#[test]
fn infer_assert_expect() {
    check_diagnostics("fn foo() { const_assert 1 != 0; }", expect![""]);
}

#[test]
fn infer_field_scalar_no_such_field() {
    check_diagnostics(
        "fn foo() { let x = 1; let y = x.nonsense; }",
        expect![[r#"
            30..40 wgsl-analyzer Error 3: no field `nonsense` on type i32
        "#]],
    );
}

#[test]
fn infer_field_builtin_struct_no_such_field() {
    check_diagnostics(
        "fn foo() { let x = modf(1.0); let y = x.nonsense; }",
        expect![[r#"
            38..48 wgsl-analyzer Error 3: no field `nonsense` on type __modf_result_abstract
        "#]],
    );
}

#[test]
fn store_type_must_be_storable() {
    check_diagnostics(
        "fn foo() { var x = 1; var y = &x; }",
        expect![[r#"
            30..32 wgsl-analyzer Error 32: store type must be storable, found ptr<i32>
        "#]],
    );
}

#[test]
fn unexpected_return_value() {
    check_diagnostics(
        "fn foo() { return 0; }",
        expect![[r#"
            18..19 wgsl-analyzer Error 33: unexpected return value of type `integer` in function with no return type
        "#]],
    );
}

#[test]
fn no_builtin_overload() {
    check_diagnostics(
        "fn foo() { var x = 1f + mat2x2f(); }",
        expect![[r#"
            19..33 wesl-rs Error 22: cannot use binary operator `+` with operands `f32` and `mat2x2<f32>`
        "#]],
    );
}

#[test]
fn no_generator_overload_no_arguments() {
    check_diagnostics(
        "fn foo() { var x = mat2x2(); }",
        expect![[r#"
            19..27 wgsl-analyzer Error 18: no overload of function `mat2x2` found that takes no arguments
        "#]],
    );
}

#[test]
fn no_generator_overload_some_arguments() {
    check_diagnostics(
        "fn foo() { var x = mat2x2(1, 2, 3, 4, 5); }",
        expect![[r#"
            19..40 wgsl-analyzer Error 18: no overload of constructor `mat2x2` found for arguments of type (integer, integer, integer, integer, integer)
        "#]],
    );
}

#[test]
fn deref_not_a_pointer() {
    check_diagnostics(
        "fn foo() { var x = *1f; }",
        expect![[r#"
            19..22 wesl-rs Error 22: cannot use unary operator `*` on type `f32`
        "#]],
    );
}

#[test]
fn no_constructor() {
    check_diagnostics(
        "fn foo() { var x = vec2f(1, 2, 3); }",
        expect![[r#"
            19..33 wgsl-analyzer Error 17: no overload of constructor `vec2<f32>` found for arguments of type (integer, integer, integer)
        "#]],
    );
}

#[test]
fn precedence_sequence_allowed() {
    check_diagnostics(
        "fn foo() { let x = true == true & true; }",
        expect![[r#"
            19..38 wesl-rs Error 22: cannot use binary operator `&` with operands `bool` and `bool`
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
        expect![[r#"
            26..57 wesl-rs Error 22: `bitcast` argument must have the same byte length as the template type
        "#]],
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

#[test]
fn binding_array_invalid() {
    check_diagnostics(
        "
@group(0) @binding(0) var textures: binding_array;
",
        expect![[r#"
            36..49 wgsl-analyzer Error 13: missing template arguments
        "#]],
    );
}

#[test]
fn invalid_translate_attribute_function_return_type() {
    check_diagnostics(
        "
fn foo() ->
@if(true) bool
{ _ = 1; }
",
        expect![[r#"
            12..21 wgsl-analyzer Error 16: translate-time attribute `@if` is not allowed on a function return type
        "#]],
    );
}

#[test]
fn invalid_translate_attribute_body_function_declaration() {
    check_diagnostics(
        "
fn foo()
@if(true) { _ = 1; }
",
        expect![[r#"
            9..18 wgsl-analyzer Error 16: translate-time attribute `@if` is not allowed on a function body
        "#]],
    );
}

#[test]
fn invalid_translate_attribute_body_switch_statement() {
    check_diagnostics(
        "
fn foo()
{
switch true
@if(true)
{
    case true: { return; }
    default: { return; }
}
}
",
        expect![[r#"
            23..32 wgsl-analyzer Error 16: translate-time attribute `@if` is not allowed on a switch body
        "#]],
    );
}

#[test]
fn invalid_translate_attribute_body_switch_clause() {
    check_diagnostics(
        "
fn foo() {
switch true
{
    case true: @if(true) { return; }
    default: { return; }
}
}
",
        expect![[r#"
            40..49 wgsl-analyzer Error 16: translate-time attribute `@if` is not allowed on a switch default clause body
        "#]],
    );
}

#[test]
fn invalid_translate_attribute_body_loop_statement() {
    check_diagnostics(
        "
fn foo() {
loop
@if(true)
{ continuing {} }
}
",
        expect![[r#"
            16..25 wgsl-analyzer Error 16: translate-time attribute `@if` is not allowed on a loop body
            40..41 wgsl-analyzer Error 16: attributes must precede a statement here
        "#]],
    );
}

#[test]
fn invalid_translate_attribute_body_for_statement() {
    check_diagnostics(
        "
fn foo() {
for(; ;)
@if(true)
{ return; }
}
",
        expect![[r#"
            20..29 wgsl-analyzer Error 16: translate-time attribute `@if` is not allowed on a for body
        "#]],
    );
}

#[test]
fn invalid_translate_attribute_body_while_statement() {
    check_diagnostics(
        "
fn foo() {
while true
@if(true)
{ return; }
}
",
        expect![[r#"
            22..31 wgsl-analyzer Error 16: translate-time attribute `@if` is not allowed on a while body
        "#]],
    );
}

#[test]
fn invalid_translate_attribute_body_if_else_statement() {
    check_diagnostics(
        "
fn foo() {
if true
@if(true)
{ return; }
else
@if(true)
{ return; }
}
",
        expect![[r#"
            19..28 wgsl-analyzer Error 16: translate-time attribute `@if` is not allowed on an if/else body
            46..55 wgsl-analyzer Error 16: translate-time attribute `@if` is not allowed on an if/else body
        "#]],
    );
}

#[test]
fn invalid_translate_attribute_body_continuing_statement() {
    check_diagnostics(
        "
fn foo() {
loop {
    continuing
    @if(true)
    {}
}
}
",
        expect![[r#"
            37..46 wgsl-analyzer Error 16: translate-time attribute `@if` is not allowed on a continuing body
            52..53 wgsl-analyzer Error 16: attributes must precede a statement here
        "#]],
    );
}

#[test]
fn task_payload_incompatible() {
    check_diagnostics(
        "
var<task_payload> foo: f16;
",
        expect![[r#"
            0..3 wgsl-analyzer Error 12: type is not compatible with `task_payload` address space
        "#]],
    );
}

#[test]
fn task_payload_compatible() {
    check_diagnostics(
        "
struct TaskPayload { foo: f32 }
var<task_payload> foo: TaskPayload;
",
        expect![""],
    );
}

#[test]
fn not_constructible() {
    check_diagnostics(
        "
enable f16;
struct Foo { bar: atomic<u32> }
fn foo() {
    let boolean_array = array<bool>();
    let signed_integer_32_array = array<i32>();
    let unsigned_integer_32_array = array<u32>();
    let float_32_array = array<f32>();
    let float_16_array = array<f16>();

    let signed_integer_32_atomic = atomic<i32>();
    let unsigned_integer_32_atomic = atomic<u32>();

    let structure = Foo();

    let pointer = ptr<function, u32, read>();
    // let reference = ref<function, u32, read>();
    let tex = texture_storage_2d<rgba16float, write>();
}
",
        expect![[r#"
            79..92 wgsl-analyzer Error 6: type `array<bool>` is not constructible
            128..140 wgsl-analyzer Error 6: type `array<i32>` is not constructible
            178..190 wgsl-analyzer Error 6: type `array<u32>` is not constructible
            217..229 wgsl-analyzer Error 6: type `array<f32>` is not constructible
            256..268 wgsl-analyzer Error 6: type `array<f16>` is not constructible
            306..319 wgsl-analyzer Error 6: type `atomic<i32>` is not constructible
            358..371 wgsl-analyzer Error 6: type `atomic<u32>` is not constructible
            394..399 wgsl-analyzer Error 6: type `Foo` is not constructible
            420..446 wgsl-analyzer Error 6: type `ptr<u32>` is not constructible
            513..553 wgsl-analyzer Error 6: type `texture_storage_2d<rgba16float,write>` is not constructible
        "#]],
    );
}

#[test]
fn arg_count_mismatch() {
    check_diagnostics(
        "
fn foo() {
    let x = foo(1);
}
",
        expect![[r#"
            15..30 wgsl-analyzer Error 7: expected 0 parameters, found 1
        "#]],
    );
}

#[test]
fn arg_count_mismatch_no_type() {
    check_diagnostics(
        "
fn foo() {
    let x = foo(foo());
}
",
        expect![[r#"
            15..34 wgsl-analyzer Error 7: expected 0 parameters, found 1
        "#]],
    );
}

#[test]
fn expected_template_builtin() {
    check_diagnostics(
        "
fn foo() {
    let x = bitcast(1f);
}
",
        expect![[r#"
            23..34 wesl-rs Error 22: invalid function call signature: `bitcast(f32)`
        "#]],
    );
}

#[test]
// https://github.com/webgpu-tools/wesl-rs/pull/255
fn unexpected_template_builtin() {
    check_diagnostics(
        "
fn foo() {
    let x = sqrt<f32>(1f);
}
",
        expect![[r#"
            23..36 wesl-rs Error 22: invalid function call signature: `sqrt<f32>(f32)`
        "#]],
    );
}

#[test]
fn invalid_array_access() {
    check_diagnostics(
        "
fn foo() {
    let x = true;
    let z = x[1];
}
",
        expect![[r#"
            41..45 wgsl-analyzer Error 4: cannot index into type bool
        "#]],
    );
}

#[test]
fn not_host_shareable() {
    check_diagnostics(
        "
@group(0) @binding(0)
var<storage> x: ray_query;

@group(0) @binding(0)
var<storage> y: vec2<bool>;
        ",
        expect![[r#"
            22..25 wgsl-analyzer Error 12: type is not host-shareable
            72..75 wgsl-analyzer Error 12: type is not host-shareable
        "#]],
    );
}

#[test]
fn not_index_scalar() {
    check_diagnostics(
        "
@group(0) @binding(0)
var<storage> x: u32 = vec2(1, 2)[1.0];
        ",
        expect![[r#"
            55..58 wgsl-analyzer Error 2: expected i32 or u32, found float
        "#]],
    );
}

#[test]
fn not_index_other() {
    check_diagnostics(
        "
@group(0) @binding(0)
var<storage> y: u32 = 1;

@group(0) @binding(0)
var<storage> x: u32 = vec2(1, 2)[&y];
        ",
        expect![[r#"
            103..105 wgsl-analyzer Error 2: expected i32 or u32, found ptr<u32>
        "#]],
    );
}

#[test]
fn indeterminate_index() {
    check_diagnostics(
        "
@group(0) @binding(0)
var<storage> x: u32 = vec2(1, 2)[y];
        ",
        expect![[r#"
            55..56 wgsl-analyzer Error 14: `y` not found in scope
        "#]],
    );
}

#[test]
fn workgroup_runtime_sized_array() {
    check_diagnostics(
        "
struct Foo { foo: array<u32> }

var<workgroup> x: Foo;
        ",
        expect![[r#"
            32..35 wgsl-analyzer Error 12: type is not workgroup compatible
        "#]],
    );
}
