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
            94..98 'S(5)': S
            96..97 '5': integer
            120..121 'a': S
            120..123 'a.x': ref<u32>
            120..129 'a.x + 10u': u32
            126..129 '10u': u32
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
            40..41 '3': integer
            84..85 'a': integer
            84..89 'a > 2': bool
            88..89 '2': integer
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
            22..23 '1': integer
            47..48 '1': integer
        "#]],
    );
}

#[test]
fn assign_abstract_number() {
    check_infer(
        r#"
fn main() {
let some_i32 = 2;

var i32_from_type : i32 = 3;
var u32_from_type : u32 = 4;
var f32_promotion : f32 = 5;
}
        "#,
        expect![[r#"
            28..29 '2': integer
            58..59 '3': integer
            87..88 '4': integer
            116..117 '5': integer
        "#]],
    );
}

#[test]
fn add_nested_abstract_numbers() {
    check_infer(
        r#"
fn main() {
var u32_from_expr = 6 + u32_1;
var i32_from_expr = 7 + i32_1;

var u32_expr1 = (1 + (1 + (1 + (1 + 1)))) + 1u;
var u32_expr2 = 1u + (1 + (1 + (1 + (1 + 1))));
var u32_expr3 = (1 + (1 + (1 + (1u + 1)))) + 1;
var u32_expr4 = 1 + (1 + (1 + (1 + (1u + 1))));

// Inference based on built-in function parameters.

let i32_clamp = clamp(1, -5, 5);
let u32_clamp = clamp(5, 0, u32_from_expr);
let f32_clamp = clamp(0, f32_1, 1);

let f32_promotion1 = 1.0 + 2 + 3 + 4;
let f32_promotion2 = 2 + 1.0 + 3 + 4;
let f32_promotion3 = 1f + ((2 + 3) + 4);
let f32_promotion4 = ((2 + (3 + 1f)) + 4);
}   
    "#,
        expect![[r#"
            33..34 '6': integer
            33..42 '6 + u32_1': [error]
            37..42 'u32_1': [error]
            64..65 '7': integer
            64..73 '7 + i32_1': [error]
            68..73 'i32_1': [error]
            92..122 '(1 + (...) + 1u': [error]
            93..94 '1': integer
            93..116 '1 + (1...+ 1)))': integer
            98..99 '1': integer
            98..115 '1 + (1... + 1))': integer
            103..104 '1': integer
            103..114 '1 + (1 + 1)': integer
            108..109 '1': integer
            108..113 '1 + 1': integer
            112..113 '1': integer
            120..122 '1u': u32
            140..142 '1u': u32
            140..170 '1u + (... 1))))': [error]
            146..147 '1': integer
            146..169 '1 + (1...+ 1)))': integer
            151..152 '1': integer
            151..168 '1 + (1... + 1))': integer
            156..157 '1': integer
            156..167 '1 + (1 + 1)': integer
            161..162 '1': integer
            161..166 '1 + 1': integer
            165..166 '1': integer
            188..218 '(1 + (...)) + 1': [error]
            189..190 '1': integer
            189..213 '1 + (1...+ 1)))': [error]
            194..195 '1': integer
            194..212 '1 + (1... + 1))': [error]
            199..200 '1': integer
            199..211 '1 + (1u + 1)': [error]
            204..206 '1u': u32
            204..210 '1u + 1': [error]
            209..210 '1': integer
            217..218 '1': integer
            236..237 '1': integer
            236..266 '1 + (1... 1))))': [error]
            241..242 '1': integer
            241..265 '1 + (1...+ 1)))': [error]
            246..247 '1': integer
            246..264 '1 + (1... + 1))': [error]
            251..252 '1': integer
            251..263 '1 + (1u + 1)': [error]
            256..258 '1u': u32
            256..262 '1u + 1': [error]
            261..262 '1': integer
            338..353 'clamp(1, -5, 5)': [error]
            344..345 '1': integer
            347..349 '-5': [error]
            348..349 '5': integer
            351..352 '5': integer
            371..397 'clamp(..._expr)': [error]
            377..378 '5': integer
            380..381 '0': integer
            383..396 'u32_from_expr': ref<[error]>
            415..433 'clamp(..._1, 1)': [error]
            421..422 '0': integer
            424..429 'f32_1': [error]
            431..432 '1': integer
            457..460 '1.0': float
            457..464 '1.0 + 2': [error]
            457..468 '1.0 + 2 + 3': [error]
            457..472 '1.0 + 2 + 3 + 4': [error]
            463..464 '2': integer
            467..468 '3': integer
            471..472 '4': integer
            495..496 '2': integer
            495..502 '2 + 1.0': [error]
            495..506 '2 + 1.0 + 3': [error]
            495..510 '2 + 1.0 + 3 + 4': [error]
            499..502 '1.0': float
            505..506 '3': integer
            509..510 '4': integer
            533..535 '1f': f32
            533..551 '1f + (...) + 4)': [error]
            539..550 '(2 + 3) + 4': integer
            540..541 '2': integer
            540..545 '2 + 3': integer
            544..545 '3': integer
            549..550 '4': integer
            575..593 '(2 + (...)) + 4': [error]
            576..577 '2': integer
            576..588 '2 + (3 + 1f)': [error]
            581..582 '3': integer
            581..587 '3 + 1f': [error]
            585..587 '1f': f32
            592..593 '4': integer
            NoBuiltinOverload { expression: Idx::<Expression>(14), builtin: BuiltinId(0), name: Some("+"), parameters: [Type { id: 1 }, Type { id: 9 }] }NoBuiltinOverload { expression: Idx::<Expression>(17), builtin: BuiltinId(0), name: Some("+"), parameters: [Type { id: 9 }, Type { id: 1 }] }NoBuiltinOverload { expression: Idx::<Expression>(31), builtin: BuiltinId(0), name: Some("+"), parameters: [Type { id: 9 }, Type { id: 1 }] }NoBuiltinOverload { expression: Idx::<Expression>(43), builtin: BuiltinId(0), name: Some("+"), parameters: [Type { id: 9 }, Type { id: 1 }] }NoBuiltinOverload { expression: Idx::<Expression>(51), builtin: BuiltinId(1), name: Some("-"), parameters: [Type { id: 1 }] }NoBuiltinOverload { expression: Idx::<Expression>(63), builtin: BuiltinId(0), name: Some("+"), parameters: [Type { id: 13 }, Type { id: 1 }] }NoBuiltinOverload { expression: Idx::<Expression>(70), builtin: BuiltinId(0), name: Some("+"), parameters: [Type { id: 1 }, Type { id: 13 }] }NoBuiltinOverload { expression: Idx::<Expression>(77), builtin: BuiltinId(0), name: Some("+"), parameters: [Type { id: 7 }, Type { id: 1 }] }NoBuiltinOverload { expression: Idx::<Expression>(85), builtin: BuiltinId(0), name: Some("+"), parameters: [Type { id: 1 }, Type { id: 7 }] }
        "#]],
    );
}
