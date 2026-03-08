use expect_test::expect;

use crate::test_util::check;

#[test]
fn format_multiline_block_comment_keeps_exact_indentation() {
    check(
        "
fn a() {
    loop{
        /* AAA
    AAA
        AAA
        */
    }
}
        ",
        expect![[r#"
            fn a() {
                loop {
                    /* AAA
                AAA
                    AAA
                    */
                }
            }
        "#]],
    );
}
