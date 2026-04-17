use expect_test::expect;

use crate::test_util::{check, check_comments};

#[test]
pub fn format_comments_in_import_single_line() {
    check_comments(
        "## import ## a ## :: ## b ## :: ## c ## as ## d ## ; ##",
        expect![[r#"
            /* 0 */
            import /* 1 */ a /* 2 */ :: /* 3 */ b /* 4 */
                :: /* 5 */ c /* 6 */ as /* 7 */ d /* 8 */; /* 9 */
        "#]],
        expect![[r#"
            // 0
            import // 1
            a // 2
                :: // 3
            b // 4
                :: // 5
            c // 6
            as // 7
            d // 8
            ; // 9
        "#]],
    );
}

#[test]
pub fn format_comments_in_import_collection() {
    check_comments(
        "## import ## a ## :: ## b ## :: ## { ## a ## , ## b ## :: ## d ## as ## e ## } ## ;",
        expect![[r#"
            /* 0 */
            import /* 1 */ a /* 2 */ :: /* 3 */ b /* 4 */ :: /* 5 */ {
                /* 6 */ a /* 7 */,
                /* 8 */ b /* 9 */ :: /* 10 */ d /* 11 */ as /* 12 */ e /* 13 */
            } /* 14 */;
        "#]],
        expect![[r#"
            // 0
            import // 1
            a // 2
                :: // 3
            b // 4
                :: // 5
            {
                // 6
                a // 7
                , // 8
                b // 9
                    :: // 10
                d // 11
                as // 12
                e // 13

            } // 14
            ;
        "#]],
    );
}

#[test]
pub fn format_wildly_nested_import_items() {
    check(
        "import aaaaaaaaa::{bbbbbbbb::{cdddddd::{dddddd::{eeeeeee::{ffffffff::{gggggg::{hhhhhh::{iiiiii::jjjjjjjjjj, kkkkkkkk}}}}}}}};",
        expect![[r#"
            import aaaaaaaaa::{
                bbbbbbbb::{
                    cdddddd::{
                        dddddd::{
                            eeeeeee::{
                                ffffffff::{gggggg::{hhhhhh::{kkkkkkkk, iiiiii::jjjjjjjjjj}}}
                            }
                        }
                    }
                }
            };
        "#]],
    );
}

#[test]
pub fn format_wildly_nested_import_with_paths() {
    check(
        "import aaaaaaaaa::bbbbbbbb::{cdddddd::dddddd::{eeeeeee::ffffffff::{gggggg::hhhhhh::{iiiiii::jjjjjjjjjj, kkkkkkkk}}}};",
        expect![[r#"
            import aaaaaaaaa::bbbbbbbb::{
                cdddddd::dddddd::{
                    eeeeeee::ffffffff::{gggggg::hhhhhh::{kkkkkkkk, iiiiii::jjjjjjjjjj}}
                }
            };
        "#]],
    );
}
