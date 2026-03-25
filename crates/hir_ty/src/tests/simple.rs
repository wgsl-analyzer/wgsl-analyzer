use expect_test::expect;
use hir_def::database::ExtensionsConfig;

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
        ExtensionsConfig { shader_int64: true },
        "
fn foo(bar: i64, baz: u64) {}
",
        expect![[r#"
            7..10 'bar': i64
            17..20 'baz': u64
        "#]],
    );
}

#[test]
fn no_ref_swizzle() {
    // See: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/650
    check_infer(
        "
fn f() {
    let v = vec2(0, 0);
    v.xy = v.yx;
}
",
        expect![[r#"
            18..19 'v': vec2<i32>
            22..32 'vec2(0, 0)': vec2<integer>
            27..28 '0': integer
            30..31 '0': integer
            38..39 'v': vec2<i32>
            38..42 'v.xy': vec2<i32>
            45..46 'v': vec2<i32>
            45..49 'v.yx': vec2<i32>
            InferenceDiagnostic { source: Body, kind: AssignmentNotAReference { left_side: Idx::<Expression>(4), actual: Type { id: 7 } } }
        "#]],
    );
}

#[test]
fn vec_ref_access_is_ref() {
    // See: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/650
    check_infer(
        "
fn f() {
    let x = vec2(0, 0).x;
}
",
        expect![[r#"
            18..19 'x': i32
            22..32 'vec2(0, 0)': vec2<integer>
            22..34 'vec2(0, 0).x': ref<integer>
            27..28 '0': integer
            30..31 '0': integer
        "#]],
    );
}

#[test]
fn vec_field_access_is_not_ref() {
    // See: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/650
    check_infer(
        "
fn f() {
    let v = vec2(0, 0);
    v.x = v.y;
}
",
        expect![[r#"
            18..19 'v': vec2<i32>
            22..32 'vec2(0, 0)': vec2<integer>
            27..28 '0': integer
            30..31 '0': integer
            38..39 'v': vec2<i32>
            38..41 'v.x': ref<i32>
            44..45 'v': vec2<i32>
            44..47 'v.y': ref<i32>
        "#]],
    );
}

#[test]
fn struct_field_access_is_not_ref() {
    // See: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/650
    check_infer(
        "
struct Foo {
    bar: i32,
}
fn f() {
    let my_foo = Foo { bar: 1 };
    foo.bar = foo.bar;
}
",
        expect![[r#"
            47..53 'my_foo': [error]
            56..59 'Foo': [error]
            62..65 'bar': [error]
            76..79 'foo': [error]
            86..89 'foo': [error]
            InferenceDiagnostic { source: Body, kind: ExpectedLoweredKind { expression: Idx::<Expression>(0), expected: Variable, actual: Type, path: Path(ModPath { kind: Plain, segments: [Name("Foo")] }) } }
            InferenceDiagnostic { source: Body, kind: InvalidType { error: TypeLoweringError { container: Expression(Idx::<Expression>(1)), kind: UnresolvedName(Name("bar")) } } }
            InferenceDiagnostic { source: Body, kind: ExpectedLoweredKind { expression: Idx::<Expression>(1), expected: Variable, actual: Type, path: Path(ModPath { kind: Plain, segments: [Name("bar")] }) } }
            InferenceDiagnostic { source: Body, kind: AssignmentNotAReference { left_side: Idx::<Expression>(1), actual: Type { id: 0 } } }
            InferenceDiagnostic { source: Body, kind: InvalidType { error: TypeLoweringError { container: Expression(Idx::<Expression>(3)), kind: UnresolvedName(Name("foo")) } } }
            InferenceDiagnostic { source: Body, kind: ExpectedLoweredKind { expression: Idx::<Expression>(3), expected: Variable, actual: Type, path: Path(ModPath { kind: Plain, segments: [Name("foo")] }) } }
            InferenceDiagnostic { source: Body, kind: AssignmentNotAReference { left_side: Idx::<Expression>(4), actual: Type { id: 0 } } }
            InferenceDiagnostic { source: Body, kind: InvalidType { error: TypeLoweringError { container: Expression(Idx::<Expression>(5)), kind: UnresolvedName(Name("foo")) } } }
            InferenceDiagnostic { source: Body, kind: ExpectedLoweredKind { expression: Idx::<Expression>(5), expected: Variable, actual: Type, path: Path(ModPath { kind: Plain, segments: [Name("foo")] }) } }
        "#]],
    );
}

#[test]
fn component_reference_from_a_composite_reference() {
    check_infer(
        "
struct S {
    age: i32,
    weight: f32
}
var<private> person: S;
// Elsewhere, 'person' denotes the reference to the memory underlying the variable,
// and will have type ref<private,S,read_write>.

fn f() {
    var uv: vec2<f32>;
    // For the remainder of this function body, 'uv' denotes the reference
    // to the memory underlying the variable, and will have type
    // ref<function,vec2<f32>,read_write>.

    // Evaluate the left-hand side of the assignment:
    //   Evaluate 'uv.x' to yield a reference:
    //   1. First evaluate 'uv', yielding a reference to the memory for
    //      the 'uv' variable. The result has type ref<function,vec2<f32>,read_write>.
    //   2. Then apply the '.x' vector access phrase, yielding a reference to
    //      the memory for the first component of the vector pointed at by the
    //      reference value from the previous step.
    //      The result has type ref<function,f32,read_write>.
    // Evaluating the right-hand side of the assignment yields the f32 value 1.0.
    // Store the f32 value 1.0 into the storage memory locations referenced by uv.x.
    uv.x = 1.0;

    // Evaluate the left-hand side of the assignment:
    //   Evaluate 'uv[1]' to yield a reference:
    //   1. First evaluate 'uv', yielding a reference to the memory for
    //      the 'uv' variable. The result has type ref<function,vec2<f32>,read_write>.
    //   2. Then apply the '[1]' array index phrase, yielding a reference to
    //      the memory for second component of the vector referenced from
    //      the previous step.  The result has type ref<function,f32,read_write>.
    // Evaluating the right-hand side of the assignment yields the f32 value 2.0.
    // Store the f32 value 2.0 into the storage memory locations referenced by uv[1].
    uv[1] = 2.0;

    var m: mat3x2<f32>;
    // When evaluating 'm[2]':
    // 1. First evaluate 'm', yielding a reference to the memory for
    //    the 'm' variable. The result has type ref<function,mat3x2<f32>,read_write>.
    // 2. Then apply the '[2]' array index phrase, yielding a reference to
    //    the memory for the third column vector pointed at by the reference
    //    value from the previous step.
    //    Therefore the 'm[2]' expression has type ref<function,vec2<f32>,read_write>.
    // The 'let' declaration is for type vec2<f32>, so the declaration
    // statement requires the initializer to be of type vec2<f32>.
    // The Load Rule applies (because no other type rule can apply), and
    // the evaluation of the initializer yields the vec2<f32> value loaded
    // from the memory locations referenced by 'm[2]' at the time the declaration
    // is executed.
    let p_m_col2: vec2<f32> = m[2];

    var A: array<i32,5>;
    // When evaluating 'A[4]'
    // 1. First evaluate 'A', yielding a reference to the memory for
    //    the 'A' variable. The result has type ref<function,array<i32,5>,read_write>.
    // 2. Then apply the '[4]' array index phrase, yielding a reference to
    //    the memory for the fifth element of the array referenced by
    //    the reference value from the previous step.
    //    The result value has type ref<function,i32,read_write>.
    // The let-declaration requires the right-hand-side to be of type i32.
    // The Load Rule applies (because no other type rule can apply), and
    // the evaluation of the initializer yields the i32 value loaded from
    // the memory locations referenced by 'A[4]' at the time the declaration
    // is executed.
    let A_4_value: i32 = A[4];

    // When evaluating 'person.weight'
    // 1. First evaluate 'person', yielding a reference to the memory for
    //    the 'person' variable declared at module scope.
    //    The result has type ref<private,S,read_write>.
    // 2. Then apply the '.weight' member access phrase, yielding a reference to
    //    the memory for the second member of the memory referenced by
    //    the reference value from the previous step.
    //    The result has type ref<private,f32,read_write>.
    // The let-declaration requires the right-hand-side to be of type f32.
    // The Load Rule applies (because no other type rule can apply), and
    // the evaluation of the initializer yields the f32 value loaded from
    // the memory locations referenced by 'person.weight' at the time the
    // declaration is executed.
    let person_weight: f32 = person.weight;

    // Alternatively, references can also be formed from pointers using
    // the same syntax.

    let uv_ptr = &uv;
    // For the remainder of this function body, 'uv_ptr' denotes a pointer
    // to the memory underlying 'uv', and will have the type
    // ptr<function,vec2<f32>,read_write>.

    // Evaluate the left-hand side of the assignment:
    //   Evaluate '*uv_ptr' to yield a reference:
    //   1. First evaluate 'uv_ptr', yielding a pointer to the memory for
    //      the 'uv' variable. The result has type ptr<function,vec2<f32>,read_write>.
    //   2. Then apply the indirection expression operator, yielding a
    //      reference to memory for 'uv'.
    // Evaluating the right-hand side of the assignment yields the vec2<f32> value (1.0, 2.0).
    // Store the value (1.0, 2.0) into the storage memory locations referenced by uv.
    *uv_ptr = vec2f(1.0, 2.0);

    // Evaluate the left-hand side of the assignment:
    //   Evaluate 'uv_ptr.x' to yield a reference:
    //   1. First evaluate 'uv_ptr', yielding a pointer to the memory for
    //      the 'uv' variable. The result has type ptr<function,vec2<f32>,read_write>.
    //   2. Then apply the '.x' vector access phrase, yielding a reference to
    //      the memory for the first component of the vector pointed at by the
    //      reference value from the previous step.
    //      The result has type ref<function,f32,read_write>.
    // Evaluating the right-hand side of the assignment yields the f32 value 1.0.
    // Store the f32 value 1.0 into the storage memory locations referenced by uv.x.
    uv_ptr.x = 1.0;

    // Evaluate the left-hand side of the assignment:
    //   Evaluate 'uv_ptr[1]' to yield a reference:
    //   1. First evaluate 'uv_ptr', yielding a pointer to the memory for
    //      the 'uv' variable. The result has type ptr<function,vec2<f32>,read_write>.
    //   2. Then apply the '[1]' array index phrase, yielding a reference to
    //      the memory for second component of the vector referenced from
    //      the previous step.  The result has type ref<function,f32,read_write>.
    // Evaluating the right-hand side of the assignment yields the f32 value 2.0.
    // Store the f32 value 2.0 into the storage memory locations referenced by uv[1].
    uv_ptr[1] = 2.0;

    let m_ptr = &m;
    // When evaluating 'm_ptr[2]':
    // 1. First evaluate 'm_ptr', yielding a pointer to the memory for
    //    the 'm' variable. The result has type ptr<function,mat3x2<f32>,read_write>.
    // 2. Then apply the '[2]' array index phrase, yielding a reference to
    //    the memory for the third column vector pointed at by the reference
    //    value from the previous step.
    //    Therefore the 'm[2]' expression has type ref<function,vec2<f32>,read_write>.
    // The 'let' declaration is for type vec2<f32>, so the declaration
    // statement requires the initializer to be of type vec2<f32>.
    // The Load Rule applies (because no other type rule can apply), and
    // the evaluation of the initializer yields the vec2<f32> value loaded
    // from the memory locations referenced by 'm[2]' at the time the declaration
    // is executed.
    let p_m_col2: vec2<f32> = m_ptr[2];

    let A_ptr = &A;
    // When evaluating 'A[4]'
    // 1. First evaluate 'A', yielding a pointer to the memory for
    //    the 'A' variable. The result has type ptr<function,array<i32,5>,read_write>.
    // 2. Then apply the '[4]' array index phrase, yielding a reference to
    //    the memory for the fifth element of the array referenced by
    //    the reference value from the previous step.
    //    The result value has type ref<function,i32,read_write>.
    // The let-declaration requires the right-hand-side to be of type i32.
    // The Load Rule applies (because no other type rule can apply), and
    // the evaluation of the initializer yields the i32 value loaded from
    // the memory locations referenced by 'A[4]' at the time the declaration
    // is executed.
    let A_4_value: i32 = A_ptr[4];

    let person_ptr = &person;
    // When evaluating 'person.weight'
    // 1. First evaluate 'person_ptr', yielding a pointer to the memory for
    //    the 'person' variable declared at module scope.
    //    The result has type ptr<private,S,read_write>.
    // 2. Then apply the '.weight' member access phrase, yielding a reference to
    //    the memory for the second member of the memory referenced by
    //    the reference value from the previous step.
    //    The result has type ref<private,f32,read_write>.
    // The let-declaration requires the right-hand-side to be of type f32.
    // The Load Rule applies (because no other type rule can apply), and
    // the evaluation of the initializer yields the f32 value loaded from
    // the memory locations referenced by 'person.weight' at the time the
    // declaration is executed.
    let person_weight: f32 = person_ptr.weight;
}
",
        expect![[r#"
            57..63 'person': ref<S>
            219..221 'uv': ref<vec2<f32>>
            1120..1122 'uv': ref<vec2<f32>>
            1120..1124 'uv.x': ref<f32>
            1127..1130 '1.0': float
            1799..1801 'uv': ref<vec2<f32>>
            1799..1804 'uv[1]': ref<f32>
            1802..1803 '1': integer
            1807..1810 '2.0': float
            1821..1822 'm': ref<mat3x2<f32>>
            2698..2706 'p_m_col2': vec2<f32>
            2720..2721 'm': ref<mat3x2<f32>>
            2720..2724 'm[2]': ref<vec2<f32>>
            2722..2723 '2': integer
            2735..2736 'A': ref<array<i32, 5>>
            3530..3539 'A_4_value': i32
            3547..3548 'A': ref<array<i32, 5>>
            3547..3551 'A[4]': ref<i32>
            3549..3550 '4': integer
            4383..4396 'person_weight': f32
            4404..4410 'person': ref<S>
            4404..4417 'person.weight': ref<f32>
            4525..4531 'uv_ptr': ptr<vec2<f32>>
            4534..4537 '&uv': ptr<vec2<f32>>
            4535..4537 'uv': ref<vec2<f32>>
            5282..5289 '*uv_ptr': ref<vec2<f32>>
            5283..5289 'uv_ptr': ptr<vec2<f32>>
            5292..5307 'vec2f(1.0, 2.0)': vec2<f32>
            5298..5301 '1.0': float
            5303..5306 '2.0': float
            6018..6024 'uv_ptr': ptr<vec2<f32>>
            6018..6026 'uv_ptr.x': ref<f32>
            6029..6032 '1.0': float
            6707..6713 'uv_ptr': ptr<vec2<f32>>
            6707..6716 'uv_ptr[1]': ref<f32>
            6714..6715 '1': integer
            6719..6722 '2.0': float
            6733..6738 'm_ptr': ptr<mat3x2<f32>>
            6741..6743 '&m': ptr<mat3x2<f32>>
            6742..6743 'm': ref<mat3x2<f32>>
            7612..7620 'p_m_col2': vec2<f32>
            7634..7639 'm_ptr': ptr<mat3x2<f32>>
            7634..7642 'm_ptr[2]': ref<vec2<f32>>
            7640..7641 '2': integer
            7653..7658 'A_ptr': ptr<array<i32, 5>>
            7661..7663 '&A': ptr<array<i32, 5>>
            7662..7663 'A': ref<array<i32, 5>>
            8441..8450 'A_4_value': i32
            8458..8463 'A_ptr': ptr<array<i32, 5>>
            8458..8466 'A_ptr[4]': ref<i32>
            8464..8465 '4': integer
            8477..8487 'person_ptr': ptr<S>
            8490..8497 '&person': ptr<S>
            8491..8497 'person': ref<S>
            9330..9343 'person_weight': f32
            9351..9361 'person_ptr': ptr<S>
            9351..9368 'person...weight': ref<f32>
        "#]],
    );
}
