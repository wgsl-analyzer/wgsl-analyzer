use expect_test::expect;

use crate::test_util::check;

#[test]
pub fn format_field_expr_prefer_breaking_other_stuff() {
    check(
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
    );
}

#[test]
pub fn format_field_expr_prefer_breaking_from_the_back() {
    check(
        "
        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
        fn main() {
            let a = aaaaaaaaaaaaaaaaaaaaaa.bbbbbbbbbbbbb.cccccccccccccc.ddddddddddddd.eeeeeeeeeeee.fffffffffff.ggggggggggg;
        }
        ",
        expect![[r#"
            //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
            fn main() {
                let a = aaaaaaaaaaaaaaaaaaaaaa.bbbbbbbbbbbbb.cccccccccccccc.ddddddddddddd
                        .eeeeeeeeeeee.fffffffffff.ggggggggggg;
            }
        "#]],
    );
}
