use expect_test::expect;

use crate::{
    FormattingOptions,
    test_util::{check, check_with_options},
};

// TODO (MonaMayrhofer, post-1.0)
#[ignore = "TODO"]
#[test]
pub fn format_vec3f_as_vec3x3f() {
    // When this gets implemented, other vecNf, vecNh, matNxNf, matNxNh
    check(
        "
        alias Test = vec3f;

        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
        alias Teeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeest = vec3f;
        ;
        ",
        expect![[r#"
            alias Test = vec3<f32>;

            //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
            alias Teeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeest = vec3<
                f32,
            >;
        "#]],
    );
}
