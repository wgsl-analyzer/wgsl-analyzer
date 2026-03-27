use expect_test::expect;

use crate::{
    FormattingOptions,
    test_util::{check, check_with_options},
};

#[test]
pub fn format_return_statement_removes_needless_parens() {
    check(
        "fn main() {
return (1);


        }",
        expect![["
            fn main() {
                return 1;
            }
        "]],
    );
}

#[test]
fn format_long_return_statement_gets_indented_correctly() {
    check_with_options(
        "
        fn main() {
            //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
            return aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa+ bbbbbbbbbbbbbbbbbbbbbb;
        }
        ",
        &expect![[r#"
            fn main() {
                //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
                return aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
                + bbbbbbbbbbbbbbbbbbbbbb;
            }
        "#]],
        &FormattingOptions {
            max_line_width: 80,
            ..Default::default()
        },
        parser::Edition::LATEST,
    );
}
