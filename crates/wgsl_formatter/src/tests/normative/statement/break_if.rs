use expect_test::expect;

use crate::{
    FormattingOptions,
    test_util::{assert_out_of_scope, check, check_with_options},
};

#[test]
pub fn format_loop_continuing_break_if_statement_with_needless_parens() {
    check(
        "fn main() {
        loop {
        continuing {
        break if (false);

        }
        }


        }",
        expect![[r#"
            fn main() {
                loop {
                    continuing {
                        break if false;
                    }
                }
            }
        "#]],
    );
}

#[test]
pub fn format_break_if_statement_without_loop() {
    assert_out_of_scope(
        "fn main() {
        break if false;
        }",
        "Wgsl disallows only allows break if statements as the last statement of a continuing block",
    );
}

#[test]
pub fn format_break_if_statement_without_continuing() {
    assert_out_of_scope(
        "fn main() {
        loop{
        break if false;
        }
        }",
        "Wgsl disallows only allows break if statements as the last statement of a continuing block",
    );
}

#[test]
fn format_long_break_if_statement_gets_indented_correctly() {
    check_with_options(
        "
        fn main() {
            loop{
            continuing {

            //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
            break if  aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa< bbbbbbbbbbbbbbbbbbbbb;
            }
            }
        }
        ",
        &expect![[r#"
            fn main() {
                loop {
                    continuing {
                        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
                        break if aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
                            < bbbbbbbbbbbbbbbbbbbbb;
                    }
                }
            }
        "#]],
        &FormattingOptions {
            max_line_width: 80,
            ..Default::default()
        },
        parser::Edition::LATEST,
    );
}
