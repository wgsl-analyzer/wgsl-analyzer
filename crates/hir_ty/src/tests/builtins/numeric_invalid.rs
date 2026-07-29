use expect_test::expect;
use syntax::ExtensionsConfig;

use crate::tests::check_infer;

#[test]
fn determinant() {
    check_infer(
        ExtensionsConfig {
            f16: true,
            ..Default::default()
        },
        "
enable f16;
fn foo() {
    let abstract_float = determinant(mat3x2(1.0));
    let float_32 = determinant(mat3x2(1.0f));
    let float_16 = determinant(mat3x2(1.0h));
}
",
        expect![[r#"
            31..45 'abstract_float': [error]
            48..72 'determ...(1.0))': [error]
            60..71 'mat3x2(1.0)': mat3x2<float>
            67..70 '1.0': float
            82..90 'float_32': [error]
            93..118 'determ...1.0f))': [error]
            105..117 'mat3x2(1.0f)': mat3x2<f32>
            112..116 '1.0f': f32
            128..136 'float_16': [error]
            139..164 'determ...1.0h))': [error]
            151..163 'mat3x2(1.0h)': mat3x2<f16>
            158..162 '1.0h': f16
            [EditionedFileId(Id(1c00))] WgslError { expression: Idx::<Expression>(2), message: "`determinant` expects a square matrix argument" } in Body
            [EditionedFileId(Id(1c00))] WgslError { expression: Idx::<Expression>(5), message: "`determinant` expects a square matrix argument" } in Body
            [EditionedFileId(Id(1c00))] WgslError { expression: Idx::<Expression>(8), message: "`determinant` expects a square matrix argument" } in Body
        "#]],
    );
}

#[test]
fn sign() {
    check_infer(
        ExtensionsConfig::default(),
        "
fn foo() {
    let unsigned_integer_32 = sign(1u);
    let unsigned_integer_32_vec = sign(vec2(1u));
}
",
        expect![[r#"
            19..38 'unsign...ger_32': [error]
            41..49 'sign(1u)': [error]
            46..48 '1u': u32
            59..82 'unsign...32_vec': [error]
            85..99 'sign(vec2(1u))': [error]
            90..98 'vec2(1u)': vec2<u32>
            95..97 '1u': u32
            [EditionedFileId(Id(1c00))] WgslError { expression: Idx::<Expression>(1), message: "`sign` argument must be a signed numeric scalar or vector" } in Body
            [EditionedFileId(Id(1c00))] WgslError { expression: Idx::<Expression>(4), message: "`sign` argument must be a signed numeric scalar or vector" } in Body
        "#]],
    );
}
