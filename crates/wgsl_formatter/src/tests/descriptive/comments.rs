use expect_test::expect;

use crate::test_util::check;

#[test]
fn format_comment_indent_1() {
    check(
        "
        fn main() {

        let a = //a
        /* A */
        // b
        1;


        for(/* A */ var i = 0;
        // Force Multiline
        ; a++) {}
        }
        ",
        expect![[r#"
            fn main() {
                let a = //a
                    /* A */ // b
                    1;

                for(
                    /* A */
                    var i = 0; // Force Multiline
                    ;
                    a++
                ) {}
            }
        "#]],
    );
}
