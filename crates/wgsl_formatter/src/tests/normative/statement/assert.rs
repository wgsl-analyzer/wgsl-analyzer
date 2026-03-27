use expect_test::expect;

use crate::{
    FormattingOptions,
    test_util::{check, check_with_options},
};

#[test]
pub fn format_assert_statement_remove_parens() {
    check(
        "
        const_assert(x<y);
        const_assert
        (x
        < y
        );
        ",
        expect![["
            const_assert x < y;
            const_assert x < y;
        "]],
    );
}

#[test]
fn format_long_assert_statement_gets_indented_correctly() {
    check_with_options(
        "
        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
        const_assert(xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx<yyyyyyyyyyyyyyyyyyyyyyyyyy);
        ",
        &expect![[r#"
            //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
            const_assert xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
                < yyyyyyyyyyyyyyyyyyyyyyyyyy;
        "#]],
        &FormattingOptions {
            max_line_width: 80,
            ..Default::default()
        },
        parser::Edition::LATEST,
    );
}
