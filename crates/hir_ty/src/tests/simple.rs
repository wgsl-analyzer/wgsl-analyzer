use expect_test::expect;

use crate::tests::check_infer;

#[test]
fn type_alias_in_struct() {
    check_infer(
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
fn const_array() {
    check_infer(
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
    //
    check_infer(
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
fn array_index_is_integer() {
    check_infer(
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
        "
        const index = 12.0f;
        const arr = array<i32>(1, 2, 3);
        const a = arr[index];
        ",
        expect![[r#"
            6..11 'index': f32
            14..19 '12.0f': f32
            27..30 'arr': array<i32>
            33..52 'array<... 2, 3)': array<i32>
            44..45 '1': integer
            47..48 '2': integer
            50..51 '3': integer
            60..61 'a': i32
            64..67 'arr': array<i32>
            64..74 'arr[index]': i32
            68..73 'index': f32
            64..74 'arr[index]': expected i32 or u32 but got f32
        "#]],
    );
}

#[test]
fn array_index_is_not_abstract_float() {
    check_infer(
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
            62..72 'arr[index]': expected i32 or u32 but got float
        "#]],
    );
}

#[test]
fn array_index_is_not_bool() {
    check_infer(
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
            63..73 'arr[index]': expected i32 or u32 but got bool
        "#]],
    );
}
