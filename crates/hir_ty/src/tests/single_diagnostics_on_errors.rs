use expect_test::expect;

use crate::tests::check_infer;

#[test]
fn array_generator_error_argument() {
    check_infer(
        "
fn foo() {
    let y = &0;
    let x = array(y);
}
        ",
        expect![[r#"
            19..20 'y': [error]
            23..25 '&0': [error]
            24..25 '0': integer
            35..36 'x': array<[error], 1>
            39..47 'array(y)': array<[error], 1>
            45..46 'y': [error]
            23..25 '&0': cannot use unary operator `&` on type `AbstractInt`
        "#]],
    );
}

#[test]
fn vector_generator_error_argument() {
    check_infer(
        "
fn foo() {
    let y = &0;
    let x = vec2(y);
}
        ",
        expect![[r#"
            19..20 'y': [error]
            23..25 '&0': [error]
            24..25 '0': integer
            35..36 'x': vec2<[error]>
            39..46 'vec2(y)': vec2<[error]>
            44..45 'y': [error]
            23..25 '&0': cannot use unary operator `&` on type `AbstractInt`
        "#]],
    );
}

#[test]
fn matrix_generator_error_argument() {
    check_infer(
        "
fn foo() {
    let y = &0.0;
    let x = mat2x2(y, y, y, y);
}
        ",
        expect![[r#"
            19..20 'y': [error]
            23..27 '&0.0': [error]
            24..27 '0.0': float
            37..38 'x': mat2x2<[error]>
            41..59 'mat2x2... y, y)': mat2x2<[error]>
            48..49 'y': [error]
            51..52 'y': [error]
            54..55 'y': [error]
            57..58 'y': [error]
            23..27 '&0.0': cannot use unary operator `&` on type `AbstractFloat`
        "#]],
    );
}

#[test]
fn scalar_constructor_error_argument() {
    check_infer(
        "
fn foo() {
    let y = &0;
    let x = u32(y);
}
        ",
        expect![[r#"
            19..20 'y': [error]
            23..25 '&0': [error]
            24..25 '0': integer
            35..36 'x': u32
            39..45 'u32(y)': u32
            43..44 'y': [error]
            23..25 '&0': cannot use unary operator `&` on type `AbstractInt`
        "#]],
    );
}

#[test]
fn array_constructor_error_argument() {
    check_infer(
        "
fn foo() {
    let y = &0;
    let x = array<i32, 1>(y);
}
        ",
        expect![[r#"
            19..20 'y': [error]
            23..25 '&0': [error]
            24..25 '0': integer
            35..36 'x': array<i32, 1>
            39..55 'array<... 1>(y)': array<i32, 1>
            53..54 'y': [error]
            23..25 '&0': cannot use unary operator `&` on type `AbstractInt`
        "#]],
    );
}

#[test]
fn vector_constructor_error_argument() {
    check_infer(
        "
fn foo() {
    let y = &0;
    let x = vec2f(y);
}
        ",
        expect![[r#"
            19..20 'y': [error]
            23..25 '&0': [error]
            24..25 '0': integer
            35..36 'x': vec2<f32>
            39..47 'vec2f(y)': vec2<f32>
            45..46 'y': [error]
            23..25 '&0': cannot use unary operator `&` on type `AbstractInt`
            39..47 'vec2f(y)': no constructor found for type `vec2<f32>` with parameters `[error]`
        "#]],
    );
}

#[test]
fn matrix_constructor_error_argument() {
    check_infer(
        "
fn foo() {
    let y = &0.0;
    let x = u32(y);
}
        ",
        expect![[r#"
            19..20 'y': [error]
            23..27 '&0.0': [error]
            24..27 '0.0': float
            37..38 'x': u32
            41..47 'u32(y)': u32
            45..46 'y': [error]
            23..27 '&0.0': cannot use unary operator `&` on type `AbstractFloat`
        "#]],
    );
}

#[test]
fn struct_constructor_error_argument() {
    check_infer(
        "
struct Foo { foo: u32 }
fn foo() {
    let y = &0;
    let x = Foo(y);
}
        ",
        expect![[r#"
            43..44 'y': [error]
            47..49 '&0': [error]
            48..49 '0': integer
            59..60 'x': Foo
            63..69 'Foo(y)': Foo
            67..68 'y': [error]
            47..49 '&0': cannot use unary operator `&` on type `AbstractInt`
        "#]],
    );
}
