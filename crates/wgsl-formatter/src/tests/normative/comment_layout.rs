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

#[test]
fn format_comments_with_tabs() {
    check("// Hello\tTab", "// Hello\tTab\n");
    check("/* Hello\tTab */", "/* Hello\tTab */\n");
}
