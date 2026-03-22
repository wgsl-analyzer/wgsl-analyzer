use expect_test::expect;

use crate::test_util::check;

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
