use expect_test::expect;

use crate::tests::check_infer;


#[test]
fn vector_generator_error_argument() {
    check_infer(
        "
fn foo() {
    let y = &0;
    let x = vec2<f32>(y);
}
        ",
        expect![[r#"
            19..20 'y': [error]
            23..25 '&0': [error]
            24..25 '0': integer
            35..36 'x': vec2<f32>
            39..51 'vec2<f32>(y)': vec2<f32>
            49..50 'y': [error]
            23..25 '&0': cannot use unary operator `&` on type `AbstractInt`
            39..51 'vec2<f32>(y)': no constructor found for type `vec2<f32>` with parameters `[error]`
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
