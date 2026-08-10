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
    let abstract_float = determinant(mat3x2(1, 2, 3, 4, 5, 6));
    let float_32 = determinant(mat3x2(1f, 2f, 3f, 4f, 5f, 6f));
    let float_16 = determinant(mat3x2(1h, 2h, 3h, 4h, 5h, 6h));
}
",
        expect![[r#"
            31..45 'abstract_float': [error]
            48..85 'determ...5, 6))': [error]
            60..84 'mat3x2... 5, 6)': mat3x2<float>
            67..68 '1': integer
            70..71 '2': integer
            73..74 '3': integer
            76..77 '4': integer
            79..80 '5': integer
            82..83 '6': integer
            95..103 'float_32': [error]
            106..149 'determ..., 6f))': [error]
            118..148 'mat3x2...f, 6f)': mat3x2<f32>
            125..127 '1f': f32
            129..131 '2f': f32
            133..135 '3f': f32
            137..139 '4f': f32
            141..143 '5f': f32
            145..147 '6f': f32
            159..167 'float_16': [error]
            170..213 'determ..., 6h))': [error]
            182..212 'mat3x2...h, 6h)': mat3x2<f16>
            189..191 '1h': f16
            193..195 '2h': f16
            197..199 '3h': f16
            201..203 '4h': f16
            205..207 '5h': f16
            209..211 '6h': f16
            [EditionedFileId(Id(300))] WgslError { expression: Idx::<Expression>(7), message: "`determinant` expects a square matrix argument" } in Body
            [EditionedFileId(Id(300))] WgslError { expression: Idx::<Expression>(15), message: "`determinant` expects a square matrix argument" } in Body
            [EditionedFileId(Id(300))] WgslError { expression: Idx::<Expression>(23), message: "`determinant` expects a square matrix argument" } in Body
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
            [EditionedFileId(Id(300))] WgslError { expression: Idx::<Expression>(1), message: "`sign` argument must be a signed numeric scalar or vector" } in Body
            [EditionedFileId(Id(300))] WgslError { expression: Idx::<Expression>(4), message: "`sign` argument must be a signed numeric scalar or vector" } in Body
        "#]],
    );
}
