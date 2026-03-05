use expect_test::expect;

use crate::test_util::{check, check_comments};

#[test]
pub fn format_type_simple() {
    check(
        "
        alias Test =
        i32
        ;
        ",
        expect![[r#"
            alias Test = i32;
        "#]],
    );
}

#[test]
pub fn format_type_with_template_simple() {
    check(
        "
        alias Test =
        array<i32, 17>
        ;
        ",
        expect![[r#"
            alias Test = array<i32, 17>;
        "#]],
    );
}

#[test]
pub fn format_type_with_template_multiline() {
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
pub fn format_type_with_template_nested_array() {
    check(
        "
        alias Test =
        array<
        array
        <i32,
        19>,
        17>
        ;
        ",
        expect![[r#"
            alias Test = array<array<i32, 19>, 17>;
        "#]],
    );
}

#[test]
pub fn format_type_with_template_nested_multiline_array() {
    check(
        "
        alias Test =
        array<
        array

        <i32,
        // Miep
        19>,
        17>
        ;
        ",
        expect![[r#"
            alias Test = array<
                array<
                    i32, // Miep
                    19,
                >,
                17,
            >;
        "#]],
    );
}
