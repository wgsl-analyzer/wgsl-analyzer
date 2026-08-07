use expect_test::expect;

use crate::test_util::{check, check_comments};

#[test]
fn format_mat2xn() {
    check(
        "
fn main() {
    let x = mat2x2(a,b,c,d);
    let y = mat2x3(a,b,c,d,e,f);
    let z = mat2x4(a,b,c,d,g,h,i,j);
}",
        expect![[r#"
            fn main() {
                let x = mat2x2(
                        a, b,
                        c, d,
                    );
                let y = mat2x3(
                        a, b, c,
                        d, e, f,
                    );
                let z = mat2x4(
                        a, b, c, d,
                        g, h, i, j,
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
    let x = mat3x2(a,b,c,d,e,f);
    let y = mat3x3(a,b,c,d,e,f,g,h,i);
    let z = mat3x4(a,b,c,d,g,h,i,j,i,j,k,l);
}",
        expect![[r#"
            fn main() {
                let x = mat3x2(
                        a, b,
                        c, d,
                        e, f,
                    );
                let y = mat3x3(
                        a, b, c,
                        d, e, f,
                        g, h, i,
                    );
                let z = mat3x4(
                        a, b, c, d,
                        g, h, i, j,
                        i, j, k, l,
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
    let x = mat4x2(a,b,c,d,e,f,g,h);
    let y = mat4x3(a,b,c,d,e,f,g,h,i,j,k,l);
    let z = mat4x4(a,b,c,d,g,h,i,j,i,j,k,l,m,n,o,p);
}",
        expect![[r#"
            fn main() {
                let x = mat4x2(
                        a, b,
                        c, d,
                        e, f,
                        g, h,
                    );
                let y = mat4x3(
                        a, b, c,
                        d, e, f,
                        g, h, i,
                        j, k, l,
                    );
                let z = mat4x4(
                        a, b, c, d,
                        g, h, i, j,
                        i, j, k, l,
                        m, n, o, p,
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
                        /* 1 */
                        a, /* 2 */ /* 3 */ b, /* 4 */ /* 5 */
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
    check_comments(
        "
fn main() {
    let x = mat4x4##(##a##,##b##,##c##,##d##,##e##,##f##,##g##,##h##,##i##,##j##,##k##,##l##,##m##,##n##,##o##,##p##)##;
}",
        expect![[r#"
            fn main() {
                let x = mat4x4 /* 0 */ (
                        /* 1 */
                        a, /* 2 */ /* 3 */ b, /* 4 */ /* 5 */ c, /* 6 */ /* 7 */ d, /* 8 */ /* 9 */
                        e, /* 10 */ /* 11 */ f, /* 12 */ /* 13 */ g, /* 14 */ /* 15 */ h, /* 16 */ /* 17 */
                        i, /* 18 */ /* 19 */ j, /* 20 */ /* 21 */ k, /* 22 */ /* 23 */ l, /* 24 */ /* 25 */
                        m, /* 26 */ /* 27 */ n, /* 28 */ /* 29 */ o, /* 30 */ /* 31 */ p, /* 32 */
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
