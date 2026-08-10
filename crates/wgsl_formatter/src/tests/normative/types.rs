use expect_test::expect;

use crate::{
    FormattingOptions,
    test_util::{CheckOptions, check, check_with_options},
};

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

#[test]
pub fn format_type_multiline_arguments_keep_comments_in_position() {
    // Following "the formatter should not unnecessarily move comments around" - if programmer wants them there, we will let them have it.
    check(
        "
        alias Test =
        array<
11 /* after 11 */,
12, /* after 12 */
/*before 13*/ 13,
/*line before 14*/
14,
// line before 15
15,

16, // after 16

17
// line after 17
>
        ;
        ",
        expect![[r#"
            alias Test = array<
                11, /* after 11 */
                12, /* after 12 */
                /*before 13*/ 13,
                /*line before 14*/
                14,
                // line before 15
                15,
                16, // after 16
                17,
                // line after 17
            >;
        "#]],
    );
}

#[test]
pub fn format_long_type_alias_linewidth_within_inner_break_outer_arguments_leave_inner_alone() {
    // Please note that the amount of "aaaa" in this test is carefully chosen to play with the line lengths.
    // This the amount of aaa is such that, breaking the inner argument would satisfy the line width requirement.
    // The formatter should prefer breaking the outer argument list, even tho breaking the inner one
    // would be enough
    check_with_options(
        "
        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
        alias Test = array<aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,array<2,aaaaaaaaa>>;
        ",
        &expect![[r#"
            //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
            alias Test = array<
                aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,
                array<2, aaaaaaaaa>,
            >;
        "#]],
        &FormattingOptions {
            max_line_width: 80,
            ..Default::default()
        }.into(),
        parser::Edition::LATEST,
    );
}

#[test]
pub fn format_long_type_alias_linewidth_outside_inner_break_outer_arguments_leave_inner_alone() {
    // Please note that the amount of "aaaa" in this test is carefully chosen to play with the line lengths.
    // This the amount of aaa is such that, breaking the inner argument would still not satisfy the line width
    // requirement.
    check_with_options(
        "
        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
        alias Test = array<aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,array<2,aaaaaaaaa>>;
        ",
        &expect![[r#"
            //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
            alias Test = array<
                aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,
                array<2, aaaaaaaaa>,
            >;
        "#]],
        &FormattingOptions {
            max_line_width: 80,
            ..Default::default()
        }.into(),
        parser::Edition::LATEST
    );
}
