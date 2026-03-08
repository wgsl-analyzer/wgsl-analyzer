use expect_test::expect;

use crate::test_util::check;

#[test]
pub fn format_type_multiline_template_gets_broken_into_multiple_lines() {
    check(
        "
        alias Test =
        array<i32, // Length
        17>
        ;
        ",
        expect![[r#"
            alias Test = array<
                i32, // Length
                17,
            >;
        "#]],
    );
}

#[test]
pub fn format_type_nested_multiline_template_gets_broken_into_multiple_lines() {
    check(
        "
        alias Test =
        array<
        array<i32, // Length
        17>,18>
        ;
        ",
        expect![[r#"
            alias Test = array<
                array<
                    i32, // Length
                    17,
                >,
                18,
            >;
        "#]],
    );
}
