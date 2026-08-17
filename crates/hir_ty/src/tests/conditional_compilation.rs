use expect_test::expect;

use crate::tests::check_infer;

#[test]
fn module_compound_if_true() {
    check_infer(
        "
        fn f() {} @if(true) { const_assert true; fn foo() {} struct bar { x: u32 } }
        ",
        expect![[r#"
            35..39 'true': bool
        "#]],
    );
}

#[test]
fn module_compound_if_false() {
    check_infer(
        "
        fn f() {} @if(false) { const_assert true; fn foo() {} struct bar { x: u32 } }
        ",
        expect![[r#"
            36..40 'true': bool
        "#]],
    );
}

#[test]
fn module_compound_if_false_elif_true() {
    check_infer(
        "
        @if(false) { const foo: u32 = 0; } @elif(true) { const bar: u32 = 0; }
        ",
        expect![[r#"
            19..22 'foo': u32
            30..31 '0': integer
            55..58 'bar': u32
            66..67 '0': integer
        "#]],
    );
}

#[test]
fn module_compound_if_true_compound_elif_true() {
    check_infer(
        "
        @if(true) { const foo: u32 = 0; } @elif(true) { const bar: u32 = 0; }
        ",
        expect![[r#"
            18..21 'foo': u32
            29..30 '0': integer
            54..57 'bar': u32
            65..66 '0': integer
        "#]],
    );
}

#[test]
fn function_compound_nested() {
    check_infer(
        "
        fn foo() { @if(true) { var x = 0; { @if(true) x++; } @if(true) { x--; } } }
        ",
        expect![[r#"
            27..28 'x': ref<function, i32, read_write>
            31..32 '0': integer
            46..47 'x': ref<function, i32, read_write>
            65..66 'x': ref<function, i32, read_write>
        "#]],
    );
}

#[test]
fn module_if_false_compound_elif_true() {
    check_infer(
        "@if(false) const foo: u32 = 0; @elif(true) { const bar: u32 = 0; }",
        expect![[r#"
            17..20 'foo': u32
            28..29 '0': integer
            51..54 'bar': u32
            62..63 '0': integer
        "#]],
    );
}

#[test]
fn module_if_true_compound_elif_true() {
    check_infer(
        "@if(true) const foo: u32 = 0; @elif(true) { const bar: u32 = 0; }",
        expect![[r#"
            16..19 'foo': u32
            27..28 '0': integer
            50..53 'bar': u32
            61..62 '0': integer
        "#]],
    );
}

#[test]
fn module_compound_else_hit() {
    check_infer(
        "@if(false) { const foo: u32 = 0; } @elif(false) { const bar: u32 = 0; } @else { const baz: u32 = 0; }",
        expect![[r#"
            19..22 'foo': u32
            30..31 '0': integer
            56..59 'bar': u32
            67..68 '0': integer
            86..89 'baz': u32
            97..98 '0': integer
        "#]],
    );
}

#[test]
fn module_compound_else_skipped() {
    check_infer(
        "@if(false) { const foo: u32 = 0; } @elif(true) { const bar: u32 = 0; } @else { const baz: u32 = 0; }",
        expect![[r#"
            19..22 'foo': u32
            30..31 '0': integer
            55..58 'bar': u32
            66..67 '0': integer
            85..88 'baz': u32
            96..97 '0': integer
        "#]],
    );
}

#[test]
fn module_compound_noop() {
    check_infer(
        "{ fn foo() {} }",
        expect![[r#"

"#]],
    );
}

#[test]
fn module_compound_nested_noop() {
    check_infer(
        "@if (true) { fn foo() {} { @if(true) fn bar() {} } }",
        expect![[r#"

"#]],
    );
}

#[test]
fn module_compound_nested_if() {
    check_infer(
        "@if(true) { fn foo() {} @if(true) { fn bar() {} } }",
        expect![[r#"

"#]],
    );
}

#[test]
fn module_compound_nested_elif_hit() {
    check_infer(
        "@if(true) { @if(false) const foo: u32 = 0; @elif(true) { const bar: u32 = 0; } } @else { const baz: u32 = 0; }",
        expect![[r#"
            29..32 'foo': u32
            40..41 '0': integer
            63..66 'bar': u32
            74..75 '0': integer
            95..98 'baz': u32
            106..107 '0': integer
        "#]],
    );
}

#[test]
fn module_compound_nested_elif_skipped() {
    check_infer(
        "@if(true) { @if(true) const foo: u32 = 0; @elif(true) { const bar: u32 = 0; } } @else { const baz: u32 = 0; }",
        expect![[r#"
            28..31 'foo': u32
            39..40 '0': integer
            62..65 'bar': u32
            73..74 '0': integer
            94..97 'baz': u32
            105..106 '0': integer
        "#]],
    );
}

#[test]
fn module_compound_nested_else_hit() {
    check_infer(
        "@if(true) { @if(false) const foo: u32 = 0; @else { const bar: u32 = 0; } } @else { const baz: u32 = 0; }",
        expect![[r#"
            29..32 'foo': u32
            40..41 '0': integer
            57..60 'bar': u32
            68..69 '0': integer
            89..92 'baz': u32
            100..101 '0': integer
        "#]],
    );
}

#[test]
fn module_compound_nested_else_skipped() {
    check_infer(
        "@if(true) { @if(true) const foo: u32 = 0; @else { const bar: u32 = 0; } } @else { const baz: u32 = 0; }",
        expect![[r#"
            28..31 'foo': u32
            39..40 '0': integer
            56..59 'bar': u32
            67..68 '0': integer
            88..91 'baz': u32
            99..100 '0': integer
        "#]],
    );
}

#[test]
fn module_compound_shadow() {
    check_infer(
        "{ const foo: u32 = 0; } const foo: u32 = 1;",
        expect![[r#"
            8..11 'foo': u32
            19..20 '0': integer
            30..33 'foo': u32
            41..42 '1': integer
        "#]],
    );
}

#[test]
fn function_compound_if_true() {
    check_infer(
        "fn f() { @if(true) { const_assert true; const x: u32 = 0; } }",
        expect![[r#"
            34..38 'true': bool
            46..47 'x': u32
            55..56 '0': integer
        "#]],
    );
}

#[test]
fn function_compound_if_false() {
    check_infer(
        "fn f() { @if(false) { const_assert true; const x: u32 = 0; } }",
        expect![[r#"
            35..39 'true': bool
            47..48 'x': u32
            56..57 '0': integer
        "#]],
    );
}

#[test]
fn function_compound_if_false_compound_elif_true() {
    check_infer(
        "fn f() { @if(false) { const foo: u32 = 0; } @elif(true) { const bar: u32 = 0; } }",
        expect![[r#"
            28..31 'foo': u32
            39..40 '0': integer
            64..67 'bar': u32
            75..76 '0': integer
        "#]],
    );
}

#[test]
fn function_compound_if_true_compound_elif_true() {
    check_infer(
        "fn f() { @if(true) { const foo: u32 = 0; } @elif(true) { const bar: u32 = 0; } }",
        expect![[r#"
            27..30 'foo': u32
            38..39 '0': integer
            63..66 'bar': u32
            74..75 '0': integer
        "#]],
    );
}

#[test]
fn function_if_false_compound_elif_true() {
    check_infer(
        "fn f() { @if(false) const foo: u32 = 0; @elif(true) { const bar: u32 = 0; } }",
        expect![[r#"
            26..29 'foo': u32
            37..38 '0': integer
            60..63 'bar': u32
            71..72 '0': integer
        "#]],
    );
}

#[test]
fn function_if_true_compound_elif_true() {
    check_infer(
        "fn f() { @if(true) const foo: u32 = 0; @elif(true) { const bar: u32 = 0; } }",
        expect![[r#"
            25..28 'foo': u32
            36..37 '0': integer
            59..62 'bar': u32
            70..71 '0': integer
        "#]],
    );
}

#[test]
fn function_compound_else_hit() {
    check_infer(
        "fn f() { @if(false) { const foo: u32 = 0; } @elif(false) { const bar: u32 = 0; } @else { const baz: u32 = 0; } }",
        expect![[r#"
            28..31 'foo': u32
            39..40 '0': integer
            65..68 'bar': u32
            76..77 '0': integer
            95..98 'baz': u32
            106..107 '0': integer
        "#]],
    );
}

#[test]
fn function_compound_else_skipped() {
    check_infer(
        "fn f() { @if(false) { const foo: u32 = 0; } @elif(true) { const bar: u32 = 0; } @else { const baz: u32 = 0; } }",
        expect![[r#"
            28..31 'foo': u32
            39..40 '0': integer
            64..67 'bar': u32
            75..76 '0': integer
            94..97 'baz': u32
            105..106 '0': integer
        "#]],
    );
}

#[test]
fn function_compound_nested_if() {
    check_infer(
        "fn f() { @if(true) { const foo: u32 = 0; @if(true) { const bar: u32 = 0; } } }",
        expect![[r#"
            27..30 'foo': u32
            38..39 '0': integer
            59..62 'bar': u32
            70..71 '0': integer
        "#]],
    );
}

#[test]
fn function_compound_nested_elif_hit() {
    check_infer(
        "fn f() { @if(true) { @if(false) const foo: u32 = 0; @elif(true) { const bar: u32 = 0; } } @else { const baz: u32 = 0; } }",
        expect![[r#"
            38..41 'foo': u32
            49..50 '0': integer
            72..75 'bar': u32
            83..84 '0': integer
            104..107 'baz': u32
            115..116 '0': integer
        "#]],
    );
}

#[test]
fn function_compound_nested_elif_skipped() {
    check_infer(
        "fn f() { @if(true) { @if(true) const foo: u32 = 0; @elif(true) { const bar: u32 = 0; } } @else { const baz: u32 = 0; } }",
        expect![[r#"
            37..40 'foo': u32
            48..49 '0': integer
            71..74 'bar': u32
            82..83 '0': integer
            103..106 'baz': u32
            114..115 '0': integer
        "#]],
    );
}

#[test]
fn function_compound_nested_else_hit() {
    check_infer(
        "fn f() { @if(true) { @if(false) const foo: u32 = 0; @else { const bar: u32 = 0; } } @else { const baz: u32 = 0; } }",
        expect![[r#"
            38..41 'foo': u32
            49..50 '0': integer
            66..69 'bar': u32
            77..78 '0': integer
            98..101 'baz': u32
            109..110 '0': integer
        "#]],
    );
}

#[test]
fn function_compound_nested_else_skipped() {
    check_infer(
        "fn f() { @if(true) { @if(true) const foo: u32 = 0; @else { const bar: u32 = 0; } } @else { const baz: u32 = 0; } }",
        expect![[r#"
            37..40 'foo': u32
            48..49 '0': integer
            65..68 'bar': u32
            76..77 '0': integer
            97..100 'baz': u32
            108..109 '0': integer
        "#]],
    );
}
