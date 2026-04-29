use expect_test::expect;
use syntax::ExtensionsConfig;

use crate::tests::check_infer;

#[test]
fn type_alias_in_struct() {
    check_infer(
        ExtensionsConfig::default(),
        "
        alias Foo = u32;
        struct S { x: Foo }

        fn foo() {
            let a = S(5);
            let b = a.x + 10u;
        }
        ",
        expect![[r#"
            57..58 'a': S
            61..65 'S(5)': S
            63..64 '5': integer
            75..76 'b': u32
            79..80 'a': S
            79..82 'a.x': ref<u32>
            79..88 'a.x + 10u': u32
            85..88 '10u': u32
        "#]],
    );
}

#[test]
fn struct_constructor_is_empty() {
    check_infer(
        ExtensionsConfig::default(),
        "
        struct S { u: u32, a: array<f32, 3> };

        fn foo() {
            let s = S();
        }
        ",
        expect![[r#"
            59..60 's': S
            63..66 'S()': S
        "#]],
    );
}

#[test]
fn struct_constructor_is_correct() {
    check_infer(
        ExtensionsConfig::default(),
        "
        struct S { u: u32, a: array<f32, 3> };

        fn foo() {
            let s = S(1u, array<f32, 3>(1.0, 2.0, 3.0));
        }
        ",
        expect![[r#"
            59..60 's': S
            63..98 'S(1u, ... 3.0))': S
            65..67 '1u': u32
            69..97 'array<..., 3.0)': array<f32, 3>
            83..86 '1.0': float
            88..91 '2.0': float
            93..96 '3.0': float
        "#]],
    );
}

#[test]
fn struct_constructor_unrefs() {
    check_infer(
        ExtensionsConfig::default(),
        "
        struct S { u: u32, a: array<f32, 3> };

        fn foo() {
            var u = 1u;
            var a = array<f32, 3>(1.0, 2.0, 3.0);
            let s = S(u, a);
        }
        ",
        expect![[r#"
            59..60 'u': ref<u32>
            63..65 '1u': u32
            75..76 'a': ref<array<f32, 3>>
            79..107 'array<..., 3.0)': array<f32, 3>
            93..96 '1.0': float
            98..101 '2.0': float
            103..106 '3.0': float
            117..118 's': S
            121..128 'S(u, a)': S
            123..124 'u': ref<u32>
            126..127 'a': ref<array<f32, 3>>
        "#]],
    );
}

#[test]
fn struct_constructor_not_enough_args() {
    check_infer(
        ExtensionsConfig::default(),
        "
        struct S { u: u32, a: array<f32, 3> };

        fn foo() {
            let s = S(1u);
        }
        ",
        expect![[r#"
            59..60 's': [error]
            63..68 'S(1u)': [error]
            65..67 '1u': u32
            InferenceDiagnostic { source: Body, kind: FunctionCallArgCountMismatch { expression: Idx::<Expression>(1), n_expected: 2, n_actual: 1 } }
        "#]],
    );
}

#[test]
fn struct_constructor_incorrect_types() {
    check_infer(
        ExtensionsConfig::default(),
        "
        struct S { u: u32, a: array<f32, 3> };

        fn foo() {
            let s = S(1.0f, vec3f(1.0, 2.0, 3.0));
        }
        ",
        expect![[r#"
            59..60 's': [error]
            63..92 'S(1.0f... 3.0))': [error]
            65..69 '1.0f': f32
            71..91 'vec3f(..., 3.0)': vec3<f32>
            77..80 '1.0': float
            82..85 '2.0': float
            87..90 '3.0': float
            65..69 '1.0f': expected u32 but got f32
            71..91 'vec3f(..., 3.0)': expected array<f32, 3> but got vec3<f32>
        "#]],
    );
}

#[test]
fn const_array() {
    check_infer(
        ExtensionsConfig::default(),
        "
        const a: array<f32, 1> = array(1);
        const b = array(1,2,3);
        ",
        expect![[r#"
            6..7 'a': array<f32, 1>
            25..33 'array(1)': array<integer, 1>
            31..32 '1': integer
            41..42 'b': array<integer, 3>
            45..57 'array(1,2,3)': array<integer, 3>
            51..52 '1': integer
            53..54 '2': integer
            55..56 '3': integer
        "#]],
    );
}

#[test]
fn const_vec() {
    check_infer(
        ExtensionsConfig::default(),
        "
        const a: vec3<u32> = vec3(1);
        const b = vec2f();
        const c = vec2();
        ",
        expect![[r#"
            6..7 'a': vec3<u32>
            21..28 'vec3(1)': vec3<integer>
            26..27 '1': integer
            36..37 'b': vec2<f32>
            40..47 'vec2f()': vec2<f32>
            55..56 'c': vec2<integer>
            59..65 'vec2()': vec2<integer>
        "#]],
    );
}

#[test]
fn const_array_of_vec() {
    check_infer(
        ExtensionsConfig::default(),
        "
        const pos = array(vec2(1.0,  1.0), vec2(1.0, -1.0));
        const pos_explicit = array<vec2f, 1>(vec2(-1.0, -1.0));
        ",
        expect![[r#"
            6..9 'pos': array<vec2<float>, 2>
            12..51 'array(...-1.0))': array<vec2<float>, 2>
            18..33 'vec2(1.0,  1.0)': vec2<float>
            23..26 '1.0': float
            29..32 '1.0': float
            35..50 'vec2(1.0, -1.0)': vec2<float>
            40..43 '1.0': float
            45..49 '-1.0': float
            46..49 '1.0': float
            59..71 'pos_explicit': array<vec2<f32>, 1>
            74..107 'array<...-1.0))': array<vec2<f32>, 1>
            90..106 'vec2(-... -1.0)': vec2<float>
            95..99 '-1.0': float
            96..99 '1.0': float
            101..105 '-1.0': float
            102..105 '1.0': float
        "#]],
    );
}

#[test]
fn const_u32_as_array_size() {
    check_infer(
        ExtensionsConfig::default(),
        "
        const maxLayers = 12u;
        var layers: array<f32, maxLayers>;
        ",
        expect![[r#"
            6..15 'maxLayers': u32
            18..21 '12u': u32
            27..33 'layers': ref<[error]>
            InferenceDiagnostic { source: Signature, kind: InvalidType { error: TypeLoweringError { container: Expression(Idx::<Expression>(1)), kind: UnexpectedTemplateArgument("a `u32` or a `i32` greater than `0`") } } }
            InferenceDiagnostic { source: Signature, kind: InvalidType { error: TypeLoweringError { container: Expression(Idx::<Expression>(1)), kind: UnexpectedTemplateArgument("a `u32` or a `i32` greater than `0`") } } }
        "#]],
    );
}

#[test]
fn multiply_with_minus_one() {
    check_infer(
        ExtensionsConfig::default(),
        r#"
    const x: i32 = 1;
    const y = x * -1;
        "#,
        expect![[r#"
            6..7 'x': i32
            15..16 '1': integer
            24..25 'y': i32
            28..29 'x': i32
            28..34 'x * -1': i32
            32..34 '-1': integer
            33..34 '1': integer
        "#]],
    );
}

#[test]
fn var_array() {
    check_infer(
        ExtensionsConfig::default(),
        "
        @group(0) @binding(0) var<storage, read_write> data: array<f32>;
        ",
        expect![[r#"
            47..51 'data': ref<array<f32>>
        "#]],
    );
}

#[test]
fn break_if_bool() {
    check_infer(
        ExtensionsConfig::default(),
        "
        fn foo() {
            let a = 3;
            loop { continuing { break if a > 2; } }
        }
        ",
        expect![[r#"
            19..20 'a': i32
            23..24 '3': integer
            59..60 'a': i32
            59..64 'a > 2': bool
            63..64 '2': integer
        "#]],
    );
}

#[test]
fn abstract_number_for_const() {
    check_infer(
        ExtensionsConfig::default(),
        "
const some_integer = 1;
const some_i32: i32 = 1;
        ",
        expect![[r#"
            6..18 'some_integer': integer
            21..22 '1': integer
            30..38 'some_i32': i32
            46..47 '1': integer
        "#]],
    );
}

#[test]
fn assign_abstract_number() {
    check_infer(
        ExtensionsConfig::default(),
        "
var i32_from_type : i32 = 3;

fn main() {
let some_i32 = 2;
let some_u32: u32 = 2;
var i32_from_type : i32 = 3;
var f32_promotion : f32 = 5;
}
        ",
        expect![[r#"
            4..17 'i32_from_type': ref<i32>
            26..27 '3': integer
            46..54 'some_i32': i32
            57..58 '2': integer
            64..72 'some_u32': u32
            80..81 '2': integer
            87..100 'i32_from_type': ref<i32>
            109..110 '3': integer
            116..129 'f32_promotion': ref<f32>
            138..139 '5': integer
        "#]],
    );
}

#[test]
fn negate_abstract_number() {
    check_infer(
        ExtensionsConfig::default(),
        "
const a = -4;
const b: f32 = -3.5;
        ",
        expect![[r#"
            6..7 'a': integer
            10..12 '-4': integer
            11..12 '4': integer
            20..21 'b': f32
            29..33 '-3.5': float
            30..33 '3.5': float
        "#]],
    );
}

#[test]
fn add_abstract_integers() {
    check_infer(
        ExtensionsConfig::default(),
        "
fn main() {
var u32_expr1 = 6 + 1u;
var u32_expr2 = 1u + (1 + 2);
}
    ",
        expect![[r#"
            16..25 'u32_expr1': ref<u32>
            28..29 '6': integer
            28..34 '6 + 1u': u32
            32..34 '1u': u32
            40..49 'u32_expr2': ref<u32>
            52..54 '1u': u32
            52..64 '1u + (1 + 2)': u32
            58..59 '1': integer
            58..63 '1 + 2': integer
            62..63 '2': integer
        "#]],
    );
}

#[test]
fn add_abstract_floats() {
    check_infer(
        ExtensionsConfig::default(),
        "
fn main() {
let f32_promotion1 = 1.0 + 2 + 3;
let f32_promotion2 = 2 + 1.0 + 3;
let f32_promotion3 = 1f + ((2 + 3) + 4);
let f32_promotion4 = ((2 + (3 + 1f)) + 4);
}
    ",
        expect![[r#"
            16..30 'f32_promotion1': f32
            33..36 '1.0': float
            33..40 '1.0 + 2': float
            33..44 '1.0 + 2 + 3': float
            39..40 '2': integer
            43..44 '3': integer
            50..64 'f32_promotion2': f32
            67..68 '2': integer
            67..74 '2 + 1.0': float
            67..78 '2 + 1.0 + 3': float
            71..74 '1.0': float
            77..78 '3': integer
            84..98 'f32_promotion3': f32
            101..103 '1f': f32
            101..119 '1f + (...) + 4)': f32
            107..118 '(2 + 3) + 4': integer
            108..109 '2': integer
            108..113 '2 + 3': integer
            112..113 '3': integer
            117..118 '4': integer
            125..139 'f32_promotion4': f32
            143..161 '(2 + (...)) + 4': f32
            144..145 '2': integer
            144..156 '2 + (3 + 1f)': f32
            149..150 '3': integer
            149..155 '3 + 1f': f32
            153..155 '1f': f32
            160..161 '4': integer
        "#]],
    );
}

#[test]
fn call_with_abstract_numbers() {
    check_infer(
        ExtensionsConfig::default(),
        "
fn main() {
let i32_clamp = clamp(1, -5, 5);
let u32_clamp = clamp(5, 0, 1u);
let f32_clamp = clamp(0, 1f, 1);
}
    ",
        expect![[r#"
            16..25 'i32_clamp': i32
            28..43 'clamp(1, -5, 5)': integer
            34..35 '1': integer
            37..39 '-5': integer
            38..39 '5': integer
            41..42 '5': integer
            49..58 'u32_clamp': u32
            61..76 'clamp(5, 0, 1u)': u32
            67..68 '5': integer
            70..71 '0': integer
            73..75 '1u': u32
            82..91 'f32_clamp': f32
            94..109 'clamp(0, 1f, 1)': f32
            100..101 '0': integer
            103..105 '1f': f32
            107..108 '1': integer
        "#]],
    );
}

#[test]
fn call_user_defined_with_abstract_numbers() {
    check_infer(
        ExtensionsConfig::default(),
        "
fn make_one(x: f32) -> u32 {
  return 1u;
}

fn main() {
    let a = make_one(0.333);
}


",
        expect![[r#"
            12..13 'x': f32
            38..40 '1u': u32
            65..66 'a': u32
            69..84 'make_one(0.333)': u32
            78..83 '0.333': float
        "#]],
    );
}

#[test]
fn vec_constructors() {
    check_infer(
        ExtensionsConfig::default(),
        "
const a = vec3(1f, 2f, 3f);
fn main() {
let b = vec4(vec3f(1f), 1f);
}
    ",
        expect![[r#"
            6..7 'a': vec3<f32>
            10..26 'vec3(1...f, 3f)': vec3<f32>
            15..17 '1f': f32
            19..21 '2f': f32
            23..25 '3f': f32
            44..45 'b': vec4<f32>
            48..67 'vec4(v...), 1f)': vec4<f32>
            53..62 'vec3f(1f)': vec3<f32>
            59..61 '1f': f32
            64..66 '1f': f32
        "#]],
    );
}

#[test]
fn texture_storage_2d_template() {
    check_infer(
        ExtensionsConfig::default(),
        "
var framebuffer : texture_storage_2d<rgba16float, write>;
    ",
        expect![[r#"
            4..15 'framebuffer': ref<texture_storage_2d<rgba16float,write>>
        "#]],
    );
}

#[test]
fn global_assert_statement_correct() {
    check_infer(
        ExtensionsConfig::default(),
        "
        const a = 29;
        const_assert 27 < a;
    ",
        expect![[r#"
            6..7 'a': integer
            10..12 '29': integer
            27..29 '27': integer
            27..33 '27 < a': bool
            32..33 'a': integer
        "#]],
    );
}

#[test]
fn global_assert_statement_wrong() {
    check_infer(
        ExtensionsConfig::default(),
        "
        const a = 29;
        const_assert 27 + a;
    ",
        expect![[r#"
            6..7 'a': integer
            10..12 '29': integer
            27..29 '27': integer
            27..33 '27 + a': integer
            32..33 'a': integer
            27..33 '27 + a': expected bool but got integer
        "#]],
    );
}

#[test]
fn global_var_function_address_space_error() {
    check_infer(
        ExtensionsConfig::default(),
        "var<function> not_allowed_at_module_level: u32;",
        expect![[r#"
            14..41 'not_al..._level': ref<u32>
            InferenceDiagnostic { source: Signature, kind: UnexpectedTemplateArgument { expression: Idx::<Expression>(0) } }
        "#]],
    );
}

#[test]
fn no_crash_on_hex_int() {
    // See: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/826
    check_infer(
        ExtensionsConfig::default(),
        "
fn f() {
    let i2 = 0u;
    let p0 = (i2 >> 0u) & 0xf
}
",
        expect![[r#"
            17..19 'i2': u32
            22..24 '0u': u32
            34..36 'p0': u32
            39..55 '(i2 >>... & 0xf': u32
            40..42 'i2': u32
            40..48 'i2 >> 0u': u32
            46..48 '0u': u32
            52..55 '0xf': integer
        "#]],
    );
}

#[test]

fn array_index_is_i32() {
    check_infer(
        ExtensionsConfig::default(),
        "
        const index = 1i;
        const arr = array<i32>(1, 2, 3);
        const a = arr[index];
        ",
        expect![[r#"
            6..11 'index': i32
            14..16 '1i': i32
            24..27 'arr': array<i32>
            30..49 'array<... 2, 3)': array<i32>
            41..42 '1': integer
            44..45 '2': integer
            47..48 '3': integer
            57..58 'a': i32
            61..64 'arr': array<i32>
            61..71 'arr[index]': i32
            65..70 'index': i32
        "#]],
    );
}

#[test]
fn array_index_is_u32() {
    check_infer(
        ExtensionsConfig::default(),
        "
        const index = 1u;
        const arr = array<i32>(1, 2, 3);
        const a = arr[index];
        ",
        expect![[r#"
            6..11 'index': u32
            14..16 '1u': u32
            24..27 'arr': array<i32>
            30..49 'array<... 2, 3)': array<i32>
            41..42 '1': integer
            44..45 '2': integer
            47..48 '3': integer
            57..58 'a': i32
            61..64 'arr': array<i32>
            61..71 'arr[index]': i32
            65..70 'index': u32
        "#]],
    );
}

#[test]
fn array_index_is_abstract_int() {
    check_infer(
        ExtensionsConfig::default(),
        "
        const index = 1;
        const arr = array<i32>(1, 2, 3);
        const a = arr[index];
        ",
        expect![[r#"
            6..11 'index': integer
            14..15 '1': integer
            23..26 'arr': array<i32>
            29..48 'array<... 2, 3)': array<i32>
            40..41 '1': integer
            43..44 '2': integer
            46..47 '3': integer
            56..57 'a': i32
            60..63 'arr': array<i32>
            60..70 'arr[index]': i32
            64..69 'index': integer
        "#]],
    );
}

#[test]
fn array_index_is_not_f32() {
    check_infer(
        ExtensionsConfig::default(),
        "
        const index = 1.0f;
        const arr = array<i32>(1, 2, 3);
        const a = arr[index];
        ",
        expect![[r#"
            6..11 'index': f32
            14..18 '1.0f': f32
            26..29 'arr': array<i32>
            32..51 'array<... 2, 3)': array<i32>
            43..44 '1': integer
            46..47 '2': integer
            49..50 '3': integer
            59..60 'a': i32
            63..66 'arr': array<i32>
            63..73 'arr[index]': i32
            67..72 'index': f32
            67..72 'index': expected i32 or u32 but got f32
        "#]],
    );
}

#[test]
fn array_index_is_ref_i32() {
    check_infer(
        ExtensionsConfig::default(),
        "
        fn test(arr: array<i32>) {
            var index = 1i;
            const a = arr[index];
        }
        ",
        expect![[r#"
            8..11 'arr': array<i32>
            35..40 'index': ref<i32>
            43..45 '1i': i32
            57..58 'a': i32
            61..64 'arr': array<i32>
            61..71 'arr[index]': i32
            65..70 'index': ref<i32>
        "#]],
    );
}

#[test]
fn array_index_is_not_ref_f32() {
    check_infer(
        ExtensionsConfig::default(),
        "
        fn test(arr: array<i32>) {
            var index = 1.0f;
            const a = arr[index];
        }
        ",
        expect![[r#"
            8..11 'arr': array<i32>
            35..40 'index': ref<f32>
            43..47 '1.0f': f32
            59..60 'a': i32
            63..66 'arr': array<i32>
            63..73 'arr[index]': i32
            67..72 'index': ref<f32>
            67..72 'index': expected i32 or u32 but got f32
        "#]],
    );
}

#[test]
fn array_index_is_not_abstract_float() {
    check_infer(
        ExtensionsConfig::default(),
        "
        const index = 1.0;
        const arr = array<i32>(1, 2, 3);
        const a = arr[index];
        ",
        expect![[r#"
            6..11 'index': float
            14..17 '1.0': float
            25..28 'arr': array<i32>
            31..50 'array<... 2, 3)': array<i32>
            42..43 '1': integer
            45..46 '2': integer
            48..49 '3': integer
            58..59 'a': i32
            62..65 'arr': array<i32>
            62..72 'arr[index]': i32
            66..71 'index': float
            66..71 'index': expected i32 or u32 but got float
        "#]],
    );
}

#[test]
fn array_index_is_not_bool() {
    check_infer(
        ExtensionsConfig::default(),
        "
        const index = true;
        const arr = array<i32>(1, 2, 3);
        const a = arr[index];
        ",
        expect![[r#"
            6..11 'index': bool
            14..18 'true': bool
            26..29 'arr': array<i32>
            32..51 'array<... 2, 3)': array<i32>
            43..44 '1': integer
            46..47 '2': integer
            49..50 '3': integer
            59..60 'a': i32
            63..66 'arr': array<i32>
            63..73 'arr[index]': i32
            67..72 'index': bool
            67..72 'index': expected i32 or u32 but got bool
        "#]],
    );
}

#[test]
fn vec_index_is_int() {
    check_infer(
        ExtensionsConfig::default(),
        "
        const index = 1i;
        const vec = vec3<f32>(1.0, 2.0, 3.0);
        const a = vec[index];
        ",
        expect![[r#"
            6..11 'index': i32
            14..16 '1i': i32
            24..27 'vec': vec3<f32>
            30..54 'vec3<f..., 3.0)': vec3<f32>
            40..43 '1.0': float
            45..48 '2.0': float
            50..53 '3.0': float
            62..63 'a': f32
            66..69 'vec': vec3<f32>
            66..76 'vec[index]': f32
            70..75 'index': i32
        "#]],
    );
}

#[test]
fn vec_index_is_not_f32() {
    check_infer(
        ExtensionsConfig::default(),
        "
        const index = 1.0f;
        const vec = vec3<f32>(1.0, 2.0, 3.0);
        const a = vec[index];
        ",
        expect![[r#"
            6..11 'index': f32
            14..18 '1.0f': f32
            26..29 'vec': vec3<f32>
            32..56 'vec3<f..., 3.0)': vec3<f32>
            42..45 '1.0': float
            47..50 '2.0': float
            52..55 '3.0': float
            64..65 'a': f32
            68..71 'vec': vec3<f32>
            68..78 'vec[index]': f32
            72..77 'index': f32
            72..77 'index': expected i32 or u32 but got f32
        "#]],
    );
}

// this is wrong
// https://github.com/wgsl-analyzer/wgsl-analyzer/issues/650
#[test]
fn vec_access_is_not_ref() {
    check_infer(
        ExtensionsConfig::default(),
        "
        const vec = vec2u(0, 1).x;
        ",
        expect![[r#"
            6..9 'vec': u32
            12..23 'vec2u(0, 1)': vec2<u32>
            12..25 'vec2u(0, 1).x': ref<u32>
            18..19 '0': integer
            21..22 '1': integer
        "#]],
    );
}

// this is wrong
// https://github.com/wgsl-analyzer/wgsl-analyzer/issues/650
#[test]
fn vec_swizzle_is_not_ref() {
    check_infer(
        ExtensionsConfig::default(),
        "
        const vec = vec3<f32>(1.0, 2.0, 3.0);
        vec.xy = v.yx;
        ",
        expect![[r#"
            6..9 'vec': vec3<f32>
            12..36 'vec3<f..., 3.0)': vec3<f32>
            22..25 '1.0': float
            27..30 '2.0': float
            32..35 '3.0': float
        "#]],
    );
}

// this is wrong
// https://github.com/wgsl-analyzer/wgsl-analyzer/issues/650
#[test]
fn struct_access_is_not_ref() {
    check_infer(
        ExtensionsConfig::default(),
        "
        struct S { x: f32 }
        const vec = S (1.0).x;
        ",
        expect![[r#"
            26..29 'vec': f32
            32..39 'S (1.0)': S
            32..41 'S (1.0).x': ref<f32>
            35..38 '1.0': float
        "#]],
    );
}

#[test]
fn mat_index_is_int() {
    check_infer(
        ExtensionsConfig::default(),
        "
        const index = 1i;
        const mat = mat2x2<f32>(1.0, 2.0, 3.0, 4.0);
        const a = mat[index][0];
        ",
        expect![[r#"
            6..11 'index': i32
            14..16 '1i': i32
            24..27 'mat': mat2x2<f32>
            30..61 'mat2x2..., 4.0)': mat2x2<f32>
            42..45 '1.0': float
            47..50 '2.0': float
            52..55 '3.0': float
            57..60 '4.0': float
            69..70 'a': f32
            73..76 'mat': mat2x2<f32>
            73..83 'mat[index]': vec2<f32>
            73..86 'mat[index][0]': f32
            77..82 'index': i32
            84..85 '0': integer
        "#]],
    );
}

#[test]
fn mat_index_i_is_not_f32() {
    check_infer(
        ExtensionsConfig::default(),
        "
        const index = 1.0f;
        const mat = mat2x2<f32>(1.0, 2.0, 3.0, 4.0);
        const a = mat[index][0];
        ",
        expect![[r#"
            6..11 'index': f32
            14..18 '1.0f': f32
            26..29 'mat': mat2x2<f32>
            32..63 'mat2x2..., 4.0)': mat2x2<f32>
            44..47 '1.0': float
            49..52 '2.0': float
            54..57 '3.0': float
            59..62 '4.0': float
            71..72 'a': f32
            75..78 'mat': mat2x2<f32>
            75..85 'mat[index]': vec2<f32>
            75..88 'mat[index][0]': f32
            79..84 'index': f32
            86..87 '0': integer
            79..84 'index': expected i32 or u32 but got f32
        "#]],
    );
}

#[test]
fn mat_index_j_is_not_f32() {
    check_infer(
        ExtensionsConfig::default(),
        "
        const index = 1.0f;
        const mat = mat2x2<f32>(1.0, 2.0, 3.0, 4.0);
        const a = mat[0][index];
        ",
        expect![[r#"
            6..11 'index': f32
            14..18 '1.0f': f32
            26..29 'mat': mat2x2<f32>
            32..63 'mat2x2..., 4.0)': mat2x2<f32>
            44..47 '1.0': float
            49..52 '2.0': float
            54..57 '3.0': float
            59..62 '4.0': float
            71..72 'a': f32
            75..78 'mat': mat2x2<f32>
            75..81 'mat[0]': vec2<f32>
            75..88 'mat[0][index]': f32
            79..80 '0': integer
            82..87 'index': f32
            82..87 'index': expected i32 or u32 but got f32
        "#]],
    );
}

#[test]
fn bitcast_builtin() {
    check_infer(
        ExtensionsConfig::default(),
        "
fn main() {
    let a = bitcast<f32>(1u);
    let b = bitcast<u32>(1.0f);
    let c = bitcast<i32>(1u);
    let d = bitcast<vec2<f32>>(vec2<u32>(1u, 2u));
    let e = bitcast<vec4<i32>>(vec4<f32>(1.0f, 2.0f, 3.0f, 4.0f));
    let f = bitcast<f32>(1u) + 1.0f;
    let g = bitcast<u32>(bitcast<f32>(42u));
}
    ",
        expect![[r#"
            20..21 'a': f32
            24..40 'bitcas...2>(1u)': f32
            37..39 '1u': u32
            50..51 'b': u32
            54..72 'bitcas...(1.0f)': u32
            67..71 '1.0f': f32
            82..83 'c': i32
            86..102 'bitcas...2>(1u)': i32
            99..101 '1u': u32
            112..113 'd': vec2<f32>
            116..153 'bitcas..., 2u))': vec2<f32>
            135..152 'vec2<u...u, 2u)': vec2<u32>
            145..147 '1u': u32
            149..151 '2u': u32
            163..164 'e': vec4<i32>
            167..220 'bitcas...4.0f))': vec4<i32>
            186..219 'vec4<f... 4.0f)': vec4<f32>
            196..200 '1.0f': f32
            202..206 '2.0f': f32
            208..212 '3.0f': f32
            214..218 '4.0f': f32
            230..231 'f': f32
            234..250 'bitcas...2>(1u)': f32
            234..257 'bitcas...+ 1.0f': f32
            247..249 '1u': u32
            253..257 '1.0f': f32
            267..268 'g': u32
            271..302 'bitcas...(42u))': u32
            284..301 'bitcas...>(42u)': f32
            297..300 '42u': u32
        "#]],
    );
}

#[test]
fn naga_shader_int64() {
    check_infer(
        ExtensionsConfig {
            shader_int64: true,
            ..Default::default()
        },
        "
fn foo(bar: i64, baz: u64) {}
",
        expect![[r#"
            7..10 'bar': i64
            17..20 'baz': u64
        "#]],
    );
}
