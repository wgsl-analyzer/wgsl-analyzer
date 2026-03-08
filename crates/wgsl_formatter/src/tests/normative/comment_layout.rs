use expect_test::expect;

use crate::test_util::check;

#[test]
fn format_multiline_block_comment_gets_indented() {
    check(
        "
        fn a() {
            /* AAA
            AAA
            AAA
            */
        }
        ",
        expect![],
    );
}
