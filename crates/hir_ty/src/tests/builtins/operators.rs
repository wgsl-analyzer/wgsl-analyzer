use expect_test::expect;

use crate::tests::check_infer;

#[test]
fn mat_times_vec() {
    check_infer(
        "
fn foo() {
    let m = mat3x4f(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    let c = vec3f(1, 2, 3);
    let r = vec4f(1, 2, 3, 4);
    let rm = r * m;
    let mc = m * c;
}
        ",
        expect![[r#"
            19..20 'm': mat3x4<f32>
            23..69 'mat3x4...1, 12)': mat3x4<f32>
            31..32 '1': integer
            34..35 '2': integer
            37..38 '3': integer
            40..41 '4': integer
            43..44 '5': integer
            46..47 '6': integer
            49..50 '7': integer
            52..53 '8': integer
            55..56 '9': integer
            58..60 '10': integer
            62..64 '11': integer
            66..68 '12': integer
            79..80 'c': vec3<f32>
            83..97 'vec3f(1, 2, 3)': vec3<f32>
            89..90 '1': integer
            92..93 '2': integer
            95..96 '3': integer
            107..108 'r': vec4<f32>
            111..128 'vec4f(... 3, 4)': vec4<f32>
            117..118 '1': integer
            120..121 '2': integer
            123..124 '3': integer
            126..127 '4': integer
            138..140 'rm': vec3<f32>
            143..144 'r': vec4<f32>
            143..148 'r * m': vec3<f32>
            147..148 'm': mat3x4<f32>
            158..160 'mc': vec4<f32>
            163..164 'm': mat3x4<f32>
            163..168 'm * c': vec4<f32>
            167..168 'c': vec3<f32>
        "#]],
    );
}

#[test]
fn mat_times_vec_invalid() {
    check_infer(
        "
fn foo() {
    let m = mat3x4f(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    let c = vec3f(1, 2, 3);
    let r = vec4f(1, 2, 3, 4);
    let cm = c * m;
    let mr = m * r;
}
        ",
        expect![[r#"
            19..20 'm': mat3x4<f32>
            23..69 'mat3x4...1, 12)': mat3x4<f32>
            31..32 '1': integer
            34..35 '2': integer
            37..38 '3': integer
            40..41 '4': integer
            43..44 '5': integer
            46..47 '6': integer
            49..50 '7': integer
            52..53 '8': integer
            55..56 '9': integer
            58..60 '10': integer
            62..64 '11': integer
            66..68 '12': integer
            79..80 'c': vec3<f32>
            83..97 'vec3f(1, 2, 3)': vec3<f32>
            89..90 '1': integer
            92..93 '2': integer
            95..96 '3': integer
            107..108 'r': vec4<f32>
            111..128 'vec4f(... 3, 4)': vec4<f32>
            117..118 '1': integer
            120..121 '2': integer
            123..124 '3': integer
            126..127 '4': integer
            138..140 'cm': [error]
            143..144 'c': vec3<f32>
            143..148 'c * m': [error]
            147..148 'm': mat3x4<f32>
            158..160 'mr': [error]
            163..164 'm': mat3x4<f32>
            163..168 'm * r': [error]
            167..168 'r': vec4<f32>
            143..148 'c * m': cannot use binary operator `*` with operands `vec3<f32>` and `mat3x4<f32>`
            163..168 'm * r': cannot use binary operator `*` with operands `mat3x4<f32>` and `vec4<f32>`
        "#]],
    );
}
