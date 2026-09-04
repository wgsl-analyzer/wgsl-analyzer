use expect_test::expect;

use crate::{
    FormattingOptions,
    test_util::{CheckOptions, check, check_with_options},
};

#[test]
pub fn format_function_call_statement_trailing_comma_with_multiline_arguments() {
    check(
        "fn main() {
        bla(12, // Force break
        bar(), 1 + vubble);
        }",
        expect![[r#"
            fn main() {
                bla(
                    12, // Force break
                    bar(),
                    1 + vubble,
                );
            }
        "#]],
    );
}

#[test]
pub fn format_function_call_statement_no_trailing_comma_with_singleline_arguments() {
    check(
        "fn main() {
        bla(12, bar(), 1 + vubble, );
        }",
        expect![[r#"
            fn main() {
                bla(12, bar(), 1 + vubble);
            }
        "#]],
    );
}

#[test]
fn format_function_call_multiline_argument_breaks_into_multiple_lines() {
    check(
        "fn main() {
    min(
        min(
            1, // Force break
            2,
        ), min(1,2)
    );
}",
        expect![[r#"
            fn main() {
                min(
                    min(
                        1, // Force break
                        2,
                    ),
                    min(1, 2),
                );
            }
        "#]],
    );
}

#[test]
fn format_template_elaborated_function_call_statement() {
    check(
        "fn main() {
    my_function<f32>(x,y,z);
    my_function<array<f32, 28>>(x,y,z);
}",
        expect![[r#"
            fn main() {
                my_function<f32>(x, y, z);
                my_function<array<f32, 28>>(x, y, z);
            }
        "#]],
    );
}

#[test]
pub fn format_function_call_statement_with_comment_has_no_trailing_whitespace() {
    check(
        "fn main() {
        bla(12, bar() /* a */    );
        }",
        expect![[r#"
            fn main() {
                bla(12, bar() /* a */);
            }
        "#]],
    );
}

#[test]
pub fn format_function_call_multiline_arguments_keeps_comments_in_position() {
    // Following "the formatter should not unnecessarily move comments around" - if programmer wants them there, we will let them have it.
    check(
        "fn main() {
            bla(
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
            );
        }",
        expect![[r#"
            fn main() {
                bla(
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
                );
            }
        "#]],
    );
}

#[test]
fn format_long_function_call_without_arguments_does_not_break_within_parens() {
    check_with_options(
        "
        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
        fn main() {
            long_name_function_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa();
        }",
        expect![[r#"
            //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
            fn main() {
                long_name_function_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa();
            }
        "#]],
        &CheckOptions {
            assert_line_width: None,
            formatting: FormattingOptions {
                max_line_width: 80,
                ..Default::default()
            },
            ..Default::default()
        },
    );
}

#[test]
pub fn format_long_function_call_linewidth_within_inner_break_outer_arguments_leave_inner_alone() {
    // Please note that the amount of "aaaa" in this test is carefully chosen to play with the line lengths.
    // This the amount of aaa is such that, breaking the inner argument would satisfy the line width requirement.
    // The formatter should prefer breaking the outer argument list, even tho breaking the inner one
    // would be enough
    check_with_options(
        "
        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
        fn main() {
            let a = thing(aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,bla(2,aaaaaaaaa));
        }
        ",
        expect![[r#"
            //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
            fn main() {
                let a = thing(
                        aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,
                        bla(2, aaaaaaaaa),
                    );
            }
        "#]],
        &FormattingOptions {
            max_line_width: 80,
            ..Default::default()
        }.into(),
    );
}

#[test]
pub fn format_long_function_call_linewidth_outside_inner_break_outer_arguments_leave_inner_alone() {
    // Please note that the amount of characters in this test is carefully chosen to play with the line lengths.
    // This the amount of aaa is such that, breaking the inner argument would still not satisfy the line width
    // requirement.
    check_with_options(
        "
        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
        fn main() {
            let a = thing(carefully_chosen_amount_of_characters_xx_do_not_change_do_not_change,bla(2,aaaaaaaaa));
        }
        ",
        expect![[r#"
            //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
            fn main() {
                let a = thing(
                        carefully_chosen_amount_of_characters_xx_do_not_change_do_not_change,
                        bla(2, aaaaaaaaa),
                    );
            }
        "#]],
        &CheckOptions {
            assert_line_width: None,
            formatting: FormattingOptions {
                max_line_width: 80,
                ..Default::default()
            },
            ..Default::default()
        },
    );
}

// TODO
#[test]
fn format_long_function_call_leave_arguments_alone_if_breaking_at_commas_suffices() {
    check_with_options(
        "
        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
        fn fragment() -> vec4<f32> {
            return vec2<f32>(
                    fooooooobarrr(
                        aa.bb,
                        vec2<f32>(fooooo_barr, -offset_strength)
                    ).r,
                    fooooooobarrr(aa.bb, vec2<f32>(-offset_strength, 0.0)).g,
                );
        }
        ",
        expect![[r#"
            //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
            fn fragment() -> vec4<f32> {
                return vec2<f32>(
                        fooooooobarrr(aa.bb, vec2<f32>(fooooo_barr, -offset_strength)).r,
                        fooooooobarrr(aa.bb, vec2<f32>(-offset_strength, 0.0)).g,
                    );
            }
        "#]],
        &FormattingOptions {
            max_line_width: 80,
            ..Default::default()
        }
        .into(),
    );
}

#[test]
pub fn format_long_function_call_prefer_to_break_arguments_over_path() {
    // Please note that the amount of "aaaa" in this test is carefully chosen to play with the line lengths.
    // This the amount of aaa is such that, breaking the inner argument would still not satisfy the line width
    // requirement.
    check_with_options(
        "
        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
        fn main() {
            let a = thing::blaaaaa::thing::blaaa::thing::blaaaaaaaaaaaaaaaaaaaa::thing(aaaa,bbbb,ccc,ddd);
        }
        ",
        expect![[r#"
            //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
            fn main() {
                let a = thing::blaaaaa::thing::blaaa::thing::blaaaaaaaaaaaaaaaaaaaa::thing(
                        aaaa,
                        bbbb,
                        ccc,
                        ddd,
                    );
            }
        "#]],
        &FormattingOptions {
            max_line_width: 80,
            ..Default::default()
        }.into(),
    );
}

#[test]
pub fn format_long_function_call_dont_break_path() {
    check_with_options(
        "
        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
        fn main() {
            let a = thing::blaaaaa::thing::blaaa::thing::blaaaaaaaaaaaaaaaaaaaa::thing::loooong::paaath(aaaa,bbbb,ccc,ddd);
        }
        ",
        expect![[r#"
            //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
            fn main() {
                let a =
                    thing::blaaaaa::thing::blaaa::thing::blaaaaaaaaaaaaaaaaaaaa::thing::loooong::paaath(
                        aaaa,
                        bbbb,
                        ccc,
                        ddd,
                    );
            }
        "#]],
        &CheckOptions {
            assert_line_width: None,
            formatting: FormattingOptions {
                max_line_width: 80,
                ..Default::default()
            },
            ..Default::default()
        },
    );
}

#[test]
pub fn format_function_call_with_field_expr_prefer_breaking_field_expr() {
    // This tests exists to document this behavior
    // This is the easier way to do it - i think its fine this way, it follows
    // the way how function chains would be expected to be formatted
    //
    // However there was no discussion about this behavior, so this can be changed.
    check_with_options(
        "
        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
        fn main() {
            let a = thing(aaaaaaaaaaaaa, bbbbbbbbbbbbbbbbbbbb, ccccccccccccccccc, ddddd).xxxxxxxxxx;
        }
        ",
        expect![[r#"
            //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
            fn main() {
                let a = thing(aaaaaaaaaaaaa, bbbbbbbbbbbbbbbbbbbb, ccccccccccccccccc, ddddd)
                        .xxxxxxxxxx;
            }
        "#]],
        &FormattingOptions {
            max_line_width: 80,
            ..Default::default()
        }
        .into(),
    );
}

#[test]
fn format_function_call_dont_break_single_arg_functions_with_singleline_content() {
    check_with_options(
        "
        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
        fn main() {
            let aaaaaaaaaaaaaaaaa =
                i32(bbb_bbbb_bbbbb(cccccc.cccccccccccccc).fffffffffffffffffffffffffff);
            let aaaaaaaaaaaaaaaaa =
                i32(bbb_bbbb_bbbbbffffffffffffffffffffffffffff(cccccc.cccccccccccccc));
        }",
        expect![[r#"
            //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
            fn main() {
                let aaaaaaaaaaaaaaaaa =
                    i32(bbb_bbbb_bbbbb(cccccc.cccccccccccccc).fffffffffffffffffffffffffff);
                let aaaaaaaaaaaaaaaaa =
                    i32(bbb_bbbb_bbbbbffffffffffffffffffffffffffff(cccccc.cccccccccccccc));
            }
        "#]],
        &FormattingOptions {
            max_line_width: 80,
            ..Default::default()
        }
        .into(),
    );
}

#[test]
fn format_function_call_do_break_single_arg_function_if_contents_are_multiline() {
    check_with_options(
        "
        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
        fn main() {
            let aaaaaaaaaaaaaaaaa =
                i32(bbb_bbbb_bbbbb(cccccc.cccccccccccccc, ffffffffff).fffffffffffffffffffffffffff.xxxxxxxxxxxxx.zzzzzzzzzzzz);
        }",
        expect![[r#"
            //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
            fn main() {
                let aaaaaaaaaaaaaaaaa = i32(
                        bbb_bbbb_bbbbb(cccccc.cccccccccccccc, ffffffffff)
                            .fffffffffffffffffffffffffff.xxxxxxxxxxxxx.zzzzzzzzzzzz,
                    );
            }
        "#]],
        &FormattingOptions {
            max_line_width: 80,
            ..Default::default()
        }
        .into(),
    );
}
