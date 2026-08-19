use expect_test::expect;

use crate::test_util::check;

#[test]
pub fn ignore_on_if() {
    check(
        "
fn main() {
    // @wgslfmt(ignore)
    if ( vec3<   f32>(0.0)  >1.0 ) {

    }

    /* @wgslfmt(ignore) */
    if ( vec3<   f32>(0.0)  >1.0 ) {

    }

}
        ",
        expect![[r#"
            fn main() {
                // @wgslfmt(ignore)
                if ( vec3<   f32>(0.0)  >1.0 ) {

                }

                /* @wgslfmt(ignore) */
                if ( vec3<   f32>(0.0)  >1.0 ) {

                }
            }
        "#]],
    );
}
