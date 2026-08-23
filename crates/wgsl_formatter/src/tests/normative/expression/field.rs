use expect_test::expect;
use parser::Edition;

use crate::{FormattingOptions, test_util::check_with_options};

#[test]
pub fn format_field_expr_prefer_breaking_other_stuff() {
    check_with_options(
        "
        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
        fn main() {
            let a = aaaaaaaaaaaaaaaaaaaaaa + bbbbbbbbbbbbbbbbbbbbbb + cccccccccccccc + d.x;
        }
        ",
        expect![[r#"
            //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
            fn main() {
                let a = aaaaaaaaaaaaaaaaaaaaaa + bbbbbbbbbbbbbbbbbbbbbb + cccccccccccccc
                    + d.x;
            }
        "#]],
        &FormattingOptions {
            max_line_width: 80,
            ..Default::default()
        }
        .into(),
        Edition::LATEST,
    );
}

#[test]
pub fn format_field_expr_prefer_breaking_from_the_back() {
    check_with_options(
        "
        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
        fn main() {
            let a = aaaaaaaaaaaaaaaaaaaaaa.bbbbbbbbbbbbb.cccccccccccccc.ddddddddddddd.eeeeeeeeeeee.fffffffffff.ggggggggggg;
        }
        ",
        expect![[r#"
            //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
            fn main() {
                let a =
                    aaaaaaaaaaaaaaaaaaaaaa.bbbbbbbbbbbbb.cccccccccccccc.ddddddddddddd
                        .eeeeeeeeeeee.fffffffffff.ggggggggggg;
            }
        "#]],
        &FormattingOptions {
            max_line_width: 80,
            ..Default::default()
        }
        .into(),
        Edition::LATEST,
    );
}

#[test]
fn prefer_not_breaking_field_expression() {
    check_with_options(
        "
        fn main() {

        aaaaaaaaaa::bbbbbbbbbbbbbbbbbb[cccccc::dddddddddddddddddddddddddd] = ffffff.gggggggggggg;
                }
        ",
        expect![[r#"
            fn main() {
                aaaaaaaaaa::bbbbbbbbbbbbbbbbbb[cccccc::dddddddddddddddddddddddddd] =
                    ffffff.gggggggggggg;
            }
        "#]],
        &FormattingOptions {
            max_line_width: 80,
            ..Default::default()
        }
        .into(),
        parser::Edition::LATEST,
    );
}
