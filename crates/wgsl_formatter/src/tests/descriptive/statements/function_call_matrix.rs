use expect_test::expect;

use crate::test_util::{check, check_comments};

#[test]
fn format_mat2xn() {
    check(
        "
fn main() {
    let x = mat2x2<f32>(a,b,c,d);
    let y = mat2x3<f32>(a,b,c,d,e,f);
    let z = mat2x4<f32>(a,b,c,d,g,h,i,j);

    let h = mat2x2f(a,b,c,d);
    let f = mat2x2h(a,b,c,d);
}",
        expect![[r#"
            fn main() {
                let x = mat2x2<f32>(
                        a, b,
                        c, d,
                    );
                let y = mat2x3<f32>(
                        a, b, c,
                        d, e, f,
                    );
                let z = mat2x4<f32>(
                        a, b, c, d,
                        g, h, i, j,
                    );

                let h = mat2x2f(
                        a, b,
                        c, d,
                    );
                let f = mat2x2h(
                        a, b,
                        c, d,
                    );
            }
        "#]],
    );
}

#[test]
fn format_mat3xn() {
    check(
        "
fn main() {
    let x = mat3x2<f32>(a,b,c,d,e,f);
    let y = mat3x3<f32>(a,b,c,d,e,f,g,h,i);
    let z = mat3x4<f32>(a,b,c,d,g,h,i,j,i,j,k,l);

    let h = mat3x2h(a,b,c,d,e,f);
    let f = mat3x2f(a,b,c,d,e,f);
}",
        expect![[r#"
            fn main() {
                let x = mat3x2<f32>(
                        a, b,
                        c, d,
                        e, f,
                    );
                let y = mat3x3<f32>(
                        a, b, c,
                        d, e, f,
                        g, h, i,
                    );
                let z = mat3x4<f32>(
                        a, b, c, d,
                        g, h, i, j,
                        i, j, k, l,
                    );

                let h = mat3x2h(
                        a, b,
                        c, d,
                        e, f,
                    );
                let f = mat3x2f(
                        a, b,
                        c, d,
                        e, f,
                    );
            }
        "#]],
    );
}

#[test]
fn format_mat4xn() {
    check(
        "
fn main() {
    let x = mat4x2<f32>(a,b,c,d,e,f,g,h);
    let y = mat4x3<f32>(a,b,c,d,e,f,g,h,i,j,k,l);
    let z = mat4x4<f32>(a,b,c,d,g,h,i,j,i,j,k,l,m,n,o,p);

    let h = mat4x2h(a,b,c,d,e,f,g,h);
    let f = mat4x2f(a,b,c,d,e,f,g,h);
}",
        expect![[r#"
            fn main() {
                let x = mat4x2<f32>(
                        a, b,
                        c, d,
                        e, f,
                        g, h,
                    );
                let y = mat4x3<f32>(
                        a, b, c,
                        d, e, f,
                        g, h, i,
                        j, k, l,
                    );
                let z = mat4x4<f32>(
                        a, b, c, d,
                        g, h, i, j,
                        i, j, k, l,
                        m, n, o, p,
                    );

                let h = mat4x2h(
                        a, b,
                        c, d,
                        e, f,
                        g, h,
                    );
                let f = mat4x2f(
                        a, b,
                        c, d,
                        e, f,
                        g, h,
                    );
            }
        "#]],
    );
}

#[test]
fn format_comments_in_mat2x2() {
    check_comments(
        "
fn main() {
    let x = mat2x2##(##a##,##b##,##c##,##d##)##;
}",
        expect![[r#"
            fn main() {
                let x = mat2x2 /* 0 */ (
                        /* 1 */ a, /* 2 */ /* 3 */ b, /* 4 */ /* 5 */
                        c, /* 6 */ /* 7 */ d, /* 8 */
                    ) /* 9 */;
            }
        "#]],
        expect![[r#"
            fn main() {
                let x = mat2x2 // 0
                    (
                        // 1
                        a, // 2
                        // 3
                        b, // 4
                        // 5
                        c, // 6
                        // 7
                        d, // 8
                    ) // 9
                    ;
            }
        "#]],
    );
}

#[test]
fn format_comments_in_mat4x4() {
    // The block comments should linebreak because they are too long to fit on the line
    check_comments(
        "
fn main() {
    let x = mat4x4##(##a##,##b##,##c##,##d##,##e##,##f##,##g##,##h##,##i##,##j##,##k##,##l##,##m##,##n##,##o##,##p##)##;
}",
        expect![[r#"
            fn main() {
                let x = mat4x4 /* 0 */ (
                        /* 1 */ a, /* 2 */ /* 3 */
                        b, /* 4 */ /* 5 */
                        c, /* 6 */ /* 7 */
                        d, /* 8 */ /* 9 */
                        e, /* 10 */ /* 11 */
                        f, /* 12 */ /* 13 */
                        g, /* 14 */ /* 15 */
                        h, /* 16 */ /* 17 */
                        i, /* 18 */ /* 19 */
                        j, /* 20 */ /* 21 */
                        k, /* 22 */ /* 23 */
                        l, /* 24 */ /* 25 */
                        m, /* 26 */ /* 27 */
                        n, /* 28 */ /* 29 */
                        o, /* 30 */ /* 31 */
                        p, /* 32 */
                    ) /* 33 */;
            }
        "#]],
        expect![[r#"
            fn main() {
                let x = mat4x4 // 0
                    (
                        // 1
                        a, // 2
                        // 3
                        b, // 4
                        // 5
                        c, // 6
                        // 7
                        d, // 8
                        // 9
                        e, // 10
                        // 11
                        f, // 12
                        // 13
                        g, // 14
                        // 15
                        h, // 16
                        // 17
                        i, // 18
                        // 19
                        j, // 20
                        // 21
                        k, // 22
                        // 23
                        l, // 24
                        // 25
                        m, // 26
                        // 27
                        n, // 28
                        // 29
                        o, // 30
                        // 31
                        p, // 32
                    ) // 33
                    ;
            }
        "#]],
    );
}
