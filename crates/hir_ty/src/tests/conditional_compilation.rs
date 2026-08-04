use expect_test::expect;
use syntax::ExtensionsConfig;

use crate::tests::check_infer;

#[test]
fn module_compound() {
    check_infer(
        ExtensionsConfig::default(),
        "
        fn f() {} @if(true) { const_assert true; fn foo() {} struct bar { x: u32 } }
        ",
        expect![[r#"

        "#]],
    );
}

#[test]
fn module_compound_nested() {
    check_infer(
        ExtensionsConfig::default(),
        "
        @if(true) { fn foo() {} { @if(true) fn bar() {} } @if(true) { fn baz() {} } }
        ",
        expect![[r#"

        "#]],
    );
}

#[test]
fn module_compound_shadow() {
    check_infer(
        ExtensionsConfig::default(),
        "
        { const foo: u32 = 0; } const foo: u32 = 1;
        ",
        expect![[r#"
            30..33 'foo': u32
            41..42 '1': integer
        "#]],
    );
}

#[test]
fn function_compound() {
    check_infer(
        ExtensionsConfig::default(),
        "
        fn foo() { @if(true) { var x = 0; } x++; }
        ",
        expect![[r#"
            27..28 'x': ref<function, i32, read_write>
            31..32 '0': integer
            36..37 'x': [error]
            InvalidType { error: TypeLoweringError { container: Expression(Idx::<Expression>(1)), kind: UnresolvedPath { path: Path(ModPath("x")), failed_segment: 0 } } } in Body
            ExpectedLoweredKind { expression: Idx::<Expression>(1), expected: Variable, actual: Type, path: Path(ModPath("x")) } in Body
            AssignmentNotAReference { left_side: Idx::<Expression>(1), actual: Type(2400) } in Body
        "#]],
    );
}

#[test]
fn function_compound_nested() {
    check_infer(
        ExtensionsConfig::default(),
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
