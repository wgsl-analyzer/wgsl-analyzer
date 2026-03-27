use expect_test::expect;

use crate::{
    FormattingOptions,
    test_util::{check, check_with_options},
};

#[test]
fn format_assignment_statement_parenthesis() {
    check(
        "fn main() {
        (a) = (b + c)
        ;
        }",
        expect![[r#"
            fn main() {
                a = b + c;
            }
        "#]],
    );
}

#[test]
fn format_long_assignment_statement_gets_indented_correctly() {
    check_with_options(
        "fn main() {
            //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
            a = long_name_function_aaaaaaaaaa() + long_name_function_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa();
        }",
        &expect![[r#"
            fn main() {
                //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
                a = long_name_function_aaaaaaaaaa()
                    + long_name_function_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa();
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
fn format_long_phony_statement_gets_indented_correctly() {
    check_with_options(
        "fn main() {
            //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
            _ = long_name_function_aaaaaaaaaa() + long_name_function_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa();
        }",
        &expect![[r#"
            fn main() {
                //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
                a = long_name_function_aaaaaaaaaa()
                    + long_name_function_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa();
            }
        "#]],
        &FormattingOptions {
            max_line_width: 80,
            ..Default::default()
        },
        parser::Edition::LATEST
    );
}
