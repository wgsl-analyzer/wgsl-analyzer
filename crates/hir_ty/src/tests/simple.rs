use expect_test::expect;

use crate::tests::check_infer;

#[test]
fn type_alias_in_struct() {
    check_infer(
        r#"
        alias Foo = u32;
        struct S { x: Foo }

        fn foo() {
            let a = S(5);
            let b = a.x + 10u;
        }
        "#,
        expect![[r#"
            90..91 'a': S
            94..98 'S(5)': S
            96..97 '5': integer
            116..117 'b': u32
            120..121 'a': S
            120..123 'a.x': ref<u32>
            120..129 'a.x + 10u': u32
            126..129 '10u': u32
        "#]],
    );
}

#[test]
fn const_array() {
    check_infer(
        r#"
        const a: array<f32, 1> = array(1);
        "#,
        expect![[r#"
            15..16 'a': array<f32, 1>
            34..42 'array(1)': array<integer, 1>
            40..41 '1': integer
        "#]],
    );
}

#[test]
fn var_array() {
    check_infer(
        r#"
        @group(0) @binding(0) var<storage, read_write> data: array<f32>;
        "#,
        expect![[r#"
            56..60 'data': ref<array<f32>>
        "#]],
    );
}

#[test]
fn break_if_bool() {
    check_infer(
        r#"
        fn foo() {
            let a = 3;
            loop { continuing { break if a > 2; } }
        }
        "#,
        expect![[r#"
            36..37 'a': i32
            40..41 '3': integer
            84..85 'a': i32
            84..89 'a > 2': [error]
            88..89 '2': integer
            NoBuiltinOverload { expression: Idx::<Expression>(1), builtin: BuiltinId(0), name: Some(">"), parameters: [Type { id: 2 }, Type { id: 1 }] }
        "#]],
    );
}

#[test]
fn abstract_number_for_const() {
    check_infer(
        r#"
const some_integer = 1;
const some_i32: i32 = 1;
        "#,
        expect![[r#"
            7..19 'some_integer': integer
            22..23 '1': integer
            31..39 'some_i32': i32
            47..48 '1': integer
        "#]],
    );
}

#[test]
fn assign_abstract_number() {
    check_infer(
        r#"
var i32_from_type : i32 = 3;

fn main() {
let some_i32 = 2;
let some_u32: u32 = 2;
var i32_from_type : i32 = 3;
var f32_promotion : f32 = 5;
}
        "#,
        expect![[r#"
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
        "#]],
    );
}

#[test]
fn negate_abstract_number() {
    check_infer(
        r#"
const a = -4;
const b: f32 = -3.5;
        "#,
        expect![[r#"
            7..8 'a': [error]
            11..13 '-4': [error]
            12..13 '4': integer
            NoBuiltinOverload { expression: Idx::<Expression>(0), builtin: BuiltinId(0), name: Some("-"), parameters: [Type { id: 1 }] }
            21..22 'b': f32
            30..34 '-3.5': [error]
            31..34 '3.5': float
            NoBuiltinOverload { expression: Idx::<Expression>(0), builtin: BuiltinId(0), name: Some("-"), parameters: [Type { id: 6 }] }
        "#]],
    );
}

#[test]
fn add_abstract_integers() {
    check_infer(
        r#"
fn main() {
var u32_expr1 = 6 + 1u;
var u32_expr2 = 1u + (1 + 2);
}   
    "#,
        expect![[r#"
            17..26 'u32_expr1': ref<[error]>
            29..30 '6': integer
            29..35 '6 + 1u': [error]
            33..35 '1u': u32
            41..50 'u32_expr2': ref<[error]>
            53..55 '1u': u32
            53..65 '1u + (1 + 2)': [error]
            59..60 '1': integer
            59..64 '1 + 2': integer
            63..64 '2': integer
            NoBuiltinOverload { expression: Idx::<Expression>(0), builtin: BuiltinId(0), name: Some("+"), parameters: [Type { id: 1 }, Type { id: 2 }] }
            NoBuiltinOverload { expression: Idx::<Expression>(3), builtin: BuiltinId(0), name: Some("+"), parameters: [Type { id: 2 }, Type { id: 1 }] }
        "#]],
    );
}

#[test]
fn add_abstract_floats() {
    check_infer(
        r#"
fn main() {
let f32_promotion1 = 1.0 + 2 + 3;
let f32_promotion2 = 2 + 1.0 + 3;
let f32_promotion3 = 1f + ((2 + 3) + 4);
let f32_promotion4 = ((2 + (3 + 1f)) + 4);
}   
    "#,
        expect![[r#"
            17..31 'f32_promotion1': [error]
            34..37 '1.0': float
            34..41 '1.0 + 2': [error]
            34..45 '1.0 + 2 + 3': [error]
            40..41 '2': integer
            44..45 '3': integer
            51..65 'f32_promotion2': [error]
            68..69 '2': integer
            68..75 '2 + 1.0': [error]
            68..79 '2 + 1.0 + 3': [error]
            72..75 '1.0': float
            78..79 '3': integer
            85..99 'f32_promotion3': [error]
            102..104 '1f': f32
            102..120 '1f + (...) + 4)': [error]
            108..119 '(2 + 3) + 4': integer
            109..110 '2': integer
            109..114 '2 + 3': integer
            113..114 '3': integer
            118..119 '4': integer
            126..140 'f32_promotion4': [error]
            144..162 '(2 + (...)) + 4': [error]
            145..146 '2': integer
            145..157 '2 + (3 + 1f)': [error]
            150..151 '3': integer
            150..156 '3 + 1f': [error]
            154..156 '1f': f32
            161..162 '4': integer
            NoBuiltinOverload { expression: Idx::<Expression>(0), builtin: BuiltinId(0), name: Some("+"), parameters: [Type { id: 1 }, Type { id: 2 }] }
            NoBuiltinOverload { expression: Idx::<Expression>(5), builtin: BuiltinId(0), name: Some("+"), parameters: [Type { id: 2 }, Type { id: 1 }] }
            NoBuiltinOverload { expression: Idx::<Expression>(10), builtin: BuiltinId(0), name: Some("+"), parameters: [Type { id: 7 }, Type { id: 2 }] }
            NoBuiltinOverload { expression: Idx::<Expression>(18), builtin: BuiltinId(0), name: Some("+"), parameters: [Type { id: 2 }, Type { id: 7 }] }
        "#]],
    );
}

#[test]
fn call_with_abstract_numbers() {
    check_infer(
        r#"
fn main() {
let i32_clamp = clamp(1, -5, 5);
let u32_clamp = clamp(5, 0, 1u);
let f32_clamp = clamp(0, 1f, 1);
}   
    "#,
        expect![[r#"
            17..26 'i32_clamp': [error]
            29..44 'clamp(1, -5, 5)': [error]
            35..36 '1': integer
            38..40 '-5': [error]
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
            NoBuiltinOverload { expression: Idx::<Expression>(1), builtin: BuiltinId(0), name: Some("-"), parameters: [Type { id: 1 }] }
        "#]],
    );
}
