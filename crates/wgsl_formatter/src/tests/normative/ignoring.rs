use expect_test::expect;

use crate::test_util::check;

#[test]
pub fn ignore_on_function_simple() {
    check(
        "
// @wgslfmt(ignore)
fn
a
(
u: u32
) {
let
a=1
;
}

/* @wgslfmt(ignore) */
fn
a
(
u: u32
) {
let
a=1
;
}
        ",
        expect![[r#"
            // @wgslfmt(ignore)
            fn
            a
            (
            u: u32
            ) {
            let
            a=1
            ;
            }

            /* @wgslfmt(ignore) */
            fn
            a
            (
            u: u32
            ) {
            let
            a=1
            ;
            }
        "#]],
    );
}

#[test]
pub fn ignore_on_assignment_mat3x3() {
    check(
        "
fn main() {
    // @wgslfmt(ignore)
    let a = mat3x3(
        1.,           2.,       3.,
        4., long_fn_bla(2.),    3.,
        5.,           2.,       3.,

    );
}
        ",
        expect![[r#"
            fn main() {
                // @wgslfmt(ignore)
                let a = mat3x3(
                    1.,           2.,       3.,
                    4., long_fn_bla(2.),    3.,
                    5.,           2.,       3.,

                );
            }
        "#]],
    );
}

#[test]
pub fn ignore_on_assignment_array() {
    check(
        "
fn main() {
    // @wgslfmt(ignore)
    let verts: array<u32, 16> = array(
        1., 0., 0., 1., // The thing
        4., 0., 0., 1., // The second vertex
        1., 7., 0., 1.,//Who added this comment?!
        1., 0., 0., 1., // The last vertex!
    );
}
        ",
        expect![[r#"
            fn main() {
                // @wgslfmt(ignore)
                let verts: array<u32, 16> = array(
                    1., 0., 0., 1., // The thing
                    4., 0., 0., 1., // The second vertex
                    1., 7., 0., 1.,//Who added this comment?!
                    1., 0., 0., 1., // The last vertex!
                );
            }
        "#]],
    );
}

#[test]
pub fn ignore_on_if_simple() {
    check(
        "
fn main() {
    // @wgslfmt(ignore)
    if ( vec3<   f32>(0.0)  >1.0 ) {}

    /* @wgslfmt(ignore) */
    if ( vec3<   f32>(0.0)  >1.0 ) {}
}
        ",
        expect![[r#"
            fn main() {
                // @wgslfmt(ignore)
                if ( vec3<   f32>(0.0)  >1.0 ) {}

                /* @wgslfmt(ignore) */
                if ( vec3<   f32>(0.0)  >1.0 ) {}
            }
        "#]],
    );
}

#[test]
pub fn ignore_on_if_with_emptyline() {
    check(
        "
fn main() {

    // @wgslfmt(ignore)
    if ( vec3<   f32>(0.0)  >1.0 ) {

    }

}
        ",
        expect![[r#"
            fn main() {

                // @wgslfmt(ignore)
                if ( vec3<   f32>(0.0)  >1.0 ) {

                }
            }
        "#]],
    );
}

#[test]
pub fn ignore_on_if_with_double_emptyline() {
    check(
        "
fn main() {


    // @wgslfmt(ignore)
    if ( vec3<   f32>(0.0)  >1.0 ) {


    }


}
        ",
        expect![[r#"
            fn main() {


                // @wgslfmt(ignore)
                if ( vec3<   f32>(0.0)  >1.0 ) {


                }
            }
        "#]],
    );
}

#[test]
pub fn ignore_on_statement() {
    check(
        "
fn main() {
    // @wgslfmt(ignore)
    let a = 1 + 2
        +   3 - 4;
}
        ",
        expect![[r#"
            fn main() {
                // @wgslfmt(ignore)
                let a = 1 + 2
                    +   3 - 4;
            }
        "#]],
    );
}

#[test]
pub fn ignore_on_fn_param() {
    check(
        "
fn main() {

let a = thing(
1,
2,
/* @wgslfmt(ignore) */ vec3<   f32>(1.0,
0.0,
1.0),
3
);

}
        ",
        expect![[r#"
            fn main() {
                let a = thing(
                        1,
                        2,
            /* @wgslfmt(ignore) */ vec3<   f32>(1.0,
            0.0,
            1.0),
                        3,
                    );
            }
        "#]],
    );
}

#[test]
pub fn ignore_on_fn_arg() {
    check(
        "
fn bla(/* @wgslfmt(ignore) */ a:             u32          ,          b: u32) {}
        ",
        expect![[r#"
            fn bla(/* @wgslfmt(ignore) */ a:             u32, b: u32) {}
        "#]],
    );
}

#[test]
pub fn ignore_within_source_file_simple() {
    check(
        "
        // @!wgslfmt(ignore)
fn bla(a:             u32          ,          b: u32) {}",
        expect![[r#"
            // @!wgslfmt(ignore)
            fn bla(a:             u32          ,          b: u32) {}"#]],
    );
}

#[test]
pub fn ignore_within_source_file_trailing_newlines() {
    check(
        "
        // @!wgslfmt(ignore)
fn bla(a:             u32          ,          b: u32) {}



",
        expect![[r#"
            // @!wgslfmt(ignore)
            fn bla(a:             u32          ,          b: u32) {}



        "#]],
    );
}

#[test]
pub fn ignore_within_compound_statement() {
    check(
        "
fn
a
(
u: u32
) {
// @!wgslfmt(ignore)
let
a=1
;
}

fn
a
(
u: u32
) {
/* @!wgslfmt(ignore) */
let
a=1
;
}
        ",
        expect![[r#"
            fn a(u: u32) {
            // @!wgslfmt(ignore)
            let
            a=1
            ;
            }

            fn a(u: u32) {
            /* @!wgslfmt(ignore) */
            let
            a=1
            ;
            }
        "#]],
    );
}
