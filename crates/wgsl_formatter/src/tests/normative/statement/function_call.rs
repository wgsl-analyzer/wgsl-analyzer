use expect_test::expect;

use crate::{
    FormattingOptions,
    test_util::{check, check_with_options},
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

#[ignore = "TODO"]
#[test]
fn format_template_elaborated_function_call_statement() {
    // TODO At the time of writing, this does not parse, however I think it should parse, and as soon as it does parse the formatter must handle it.
    check(
        "fn main() {
    my_function<f32>(x,y,z);
    my_function<array<f32, 28>>(x,y,z);
}",
        expect![[r#"
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
fn format_long_function_call_without_arguments_does_not_break_within_parens() {
    check_with_options(
        "fn main() {
            //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
            long_name_function_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa();
        }",
        &expect![[r#"
            fn main() {
                //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
                long_name_function_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa();
            }
        "#]],
        &FormattingOptions {
            max_line_width: 80,
            ..Default::default()
        },
        parser::Edition::LATEST,
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
        fn main() {
            //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
            let a = thing(aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,bla(2,aaaaaaaaa));
        }
        ",
        &expect![[r#"
            fn main() {
                //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
                let a = thing(
                        aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,
                        bla(2, aaaaaaaaa),
                    );
            }
        "#]],
        &FormattingOptions {
            max_line_width: 80,
            ..Default::default()
        },
        parser::Edition::LATEST,
    );
}

#[test]
pub fn format_long_function_call_linewidth_outside_inner_break_outer_arguments_leave_inner_alone() {
    // Please note that the amount of "aaaa" in this test is carefully chosen to play with the line lengths.
    // This the amount of aaa is such that, breaking the inner argument would still not satisfy the line width
    // requirement.
    check_with_options(
        "
        fn main() {
            //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
            let a = thing(aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,bla(2,aaaaaaaaa));
        }
        ",
        &expect![[r#"
            fn main() {
                //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
                let a = thing(
                        aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,
                        bla(2, aaaaaaaaa),
                    );
            }
        "#]],
        &FormattingOptions {
            max_line_width: 80,
            ..Default::default()
        },
        parser::Edition::LATEST
    );
}

#[test]
pub fn format_long_function_call_prefer_to_break_arguments_over_path() {
    // Please note that the amount of "aaaa" in this test is carefully chosen to play with the line lengths.
    // This the amount of aaa is such that, breaking the inner argument would still not satisfy the line width
    // requirement.
    check_with_options(
        "
        fn main() {
            //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
            let a = thing::blaaaaa::thing::blaaa::thing::blaaaaaaaaaaaaaaaaaaaa::thing(aaaa,bbbb,ccc,ddd);
        }
        ",
        &expect![[r#"
            fn main() {
                //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
                let a = thing(
                        aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,
                        bla(2, aaaaaaaaa),
                    );
            }
        "#]],
        &FormattingOptions {
            max_line_width: 80,
            ..Default::default()
        },
        parser::Edition::LATEST
    );
}

#[test]
pub fn format_long_function_call_break_path_if_necessary_but_keep_arguments_alone() {
    // Please note that the amount of "aaaa" in this test is carefully chosen to play with the line lengths.
    // This the amount of aaa is such that, breaking the inner argument would still not satisfy the line width
    // requirement.
    check_with_options(
        "
        fn main() {
            //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
            let a = thing::blaaaaa::thing::blaaa::thing::blaaaaaaaaaaaaaaaaaaaa::thing::loooong::paaath(aaaa,bbbb,ccc,ddd);
        }
        ",
        &expect![[r#"
            fn main() {
                //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
                let a = thing(
                        aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,
                        bla(2, aaaaaaaaa),
                    );
            }
        "#]],
        &FormattingOptions {
            max_line_width: 80,
            ..Default::default()
        },
        parser::Edition::LATEST
    );
}
