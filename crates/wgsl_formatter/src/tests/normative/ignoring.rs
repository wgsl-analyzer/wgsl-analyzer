use expect_test::expect;

use crate::test_util::check;

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
            1.0),,
                        3,
                    );
            }
        "#]],
    );
}
