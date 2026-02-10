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
        expect![["
            90..91 'a': S
            94..98 'S(5)': S
            96..97 '5': integer
            116..117 'b': u32
            120..121 'a': S
            120..123 'a.x': ref<u32>
            120..129 'a.x + 10u': u32
            126..129 '10u': u32
        "]],
    );
}

#[test]
fn const_array() {
    check_infer(
        "
        const a: array<f32, 1> = array(1);
        const b = array(1,2,3);
        ",
        expect![["
            15..16 'a': array<f32, 1>
            34..42 'array(1)': array<integer, 1>
            40..41 '1': integer
            58..59 'b': array<integer, 3>
            62..74 'array(1,2,3)': array<integer, 3>
            68..69 '1': integer
            70..71 '2': integer
            72..73 '3': integer
        "]],
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
        expect![["
            15..16 'a': vec3<u32>
            30..37 'vec3(1)': vec3<integer>
            35..36 '1': integer
            53..54 'b': vec2<f32>
            57..64 'vec2f()': vec2<f32>
            80..81 'c': vec2<integer>
            84..90 'vec2()': vec2<integer>
        "]],
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
            15..18 'pos': array<vec2<float>, 2>
            21..60 'array(...-1.0))': array<vec2<float>, 2>
            27..42 'vec2(1.0,  1.0)': vec2<float>
            32..35 '1.0': float
            38..41 '1.0': float
            44..59 'vec2(1.0, -1.0)': vec2<float>
            49..52 '1.0': float
            54..58 '-1.0': float
            55..58 '1.0': float
            76..88 'pos_explicit': array<vec2<f32>, 1>
            91..124 'array<...-1.0))': array<vec2<f32>, 1>
            107..123 'vec2(-... -1.0)': vec2<float>
            112..116 '-1.0': float
            113..116 '1.0': float
            118..122 '-1.0': float
            119..122 '1.0': float
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
            15..24 'maxLayers': u32
            27..30 '12u': u32
            44..50 'layers': ref<[error]>
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
            11..12 'x': i32
            20..21 '1': integer
            33..34 'y': i32
            37..38 'x': i32
            37..43 'x * -1': i32
            41..43 '-1': integer
            42..43 '1': integer
        "#]],
    );
}

#[test]
fn var_array() {
    check_infer(
        "
        @group(0) @binding(0) var<storage, read_write> data: array<f32>;
        ",
        expect![["
            56..60 'data': ref<array<f32>>
        "]],
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
        expect![["
            36..37 'a': i32
            40..41 '3': integer
            84..85 'a': i32
            84..89 'a > 2': bool
            88..89 '2': integer
        "]],
    );
}

#[test]
fn abstract_number_for_const() {
    check_infer(
        "
const some_integer = 1;
const some_i32: i32 = 1;
        ",
        expect![["
            7..19 'some_integer': integer
            22..23 '1': integer
            31..39 'some_i32': i32
            47..48 '1': integer
        "]],
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
        expect![["
            5..18 'i32_from_type': ref<i32>
            27..28 '3': integer
            47..55 'some_i32': i32
            58..59 '2': integer
            65..73 'some_u32': u32
            81..82 '2': integer
            88..101 'i32_from_type': ref<i32>
            110..111 '3': integer
            117..130 'f32_promotion': ref<f32>
            139..140 '5': integer
        "]],
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
            7..8 'a': integer
            11..13 '-4': integer
            12..13 '4': integer
            21..22 'b': f32
            30..34 '-3.5': float
            31..34 '3.5': float
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
        expect![["
            17..26 'u32_expr1': ref<u32>
            29..30 '6': integer
            29..35 '6 + 1u': u32
            33..35 '1u': u32
            41..50 'u32_expr2': ref<u32>
            53..55 '1u': u32
            53..65 '1u + (1 + 2)': u32
            59..60 '1': integer
            59..64 '1 + 2': integer
            63..64 '2': integer
        "]],
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
        expect![["
            17..31 'f32_promotion1': f32
            34..37 '1.0': float
            34..41 '1.0 + 2': float
            34..45 '1.0 + 2 + 3': float
            40..41 '2': integer
            44..45 '3': integer
            51..65 'f32_promotion2': f32
            68..69 '2': integer
            68..75 '2 + 1.0': float
            68..79 '2 + 1.0 + 3': float
            72..75 '1.0': float
            78..79 '3': integer
            85..99 'f32_promotion3': f32
            102..104 '1f': f32
            102..120 '1f + (...) + 4)': f32
            108..119 '(2 + 3) + 4': integer
            109..110 '2': integer
            109..114 '2 + 3': integer
            113..114 '3': integer
            118..119 '4': integer
            126..140 'f32_promotion4': f32
            144..162 '(2 + (...)) + 4': f32
            145..146 '2': integer
            145..157 '2 + (3 + 1f)': f32
            150..151 '3': integer
            150..156 '3 + 1f': f32
            154..156 '1f': f32
            161..162 '4': integer
        "]],
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
            17..26 'i32_clamp': i32
            29..44 'clamp(1, -5, 5)': integer
            35..36 '1': integer
            38..40 '-5': integer
            39..40 '5': integer
            42..43 '5': integer
            50..59 'u32_clamp': u32
            62..77 'clamp(5, 0, 1u)': u32
            68..69 '5': integer
            71..72 '0': integer
            74..76 '1u': u32
            83..92 'f32_clamp': f32
            95..110 'clamp(0, 1f, 1)': f32
            101..102 '0': integer
            104..106 '1f': f32
            108..109 '1': integer
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
            13..14 'x': f32
            39..41 '1u': u32
            66..67 'a': u32
            70..85 'make_one(0.333)': u32
            79..84 '0.333': float
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
        expect![["
            7..8 'a': vec3<f32>
            11..27 'vec3(1...f, 3f)': vec3<f32>
            16..18 '1f': f32
            20..22 '2f': f32
            24..26 '3f': f32
            45..46 'b': vec4<f32>
            49..68 'vec4(v...), 1f)': vec4<f32>
            54..63 'vec3f(1f)': vec3<f32>
            60..62 '1f': f32
            65..67 '1f': f32
        "]],
    );
}

#[test]
fn texture_storage_2d_template() {
    check_infer(
        "
var framebuffer : texture_storage_2d<rgba16float, write>;
    ",
        expect![[r#"
            5..16 'framebuffer': ref<texture_storage_2d<rgba16float,write>>
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
            15..16 'a': integer
            19..21 '29': integer
            44..46 '27': integer
            44..50 '27 < a': bool
            49..50 'a': integer
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
            15..16 'a': integer
            19..21 '29': integer
            44..46 '27': integer
            44..50 '27 + a': integer
            49..50 'a': integer
            44..50 '27 + a': expected bool but got integer
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
            18..20 'i2': u32
            23..25 '0u': u32
            35..37 'p0': u32
            40..56 '(i2 >>... & 0xf': u32
            41..43 'i2': u32
            41..49 'i2 >> 0u': u32
            47..49 '0u': u32
            53..56 '0xf': integer
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
