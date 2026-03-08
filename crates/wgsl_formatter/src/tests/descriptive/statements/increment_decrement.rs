use expect_test::expect;

use crate::test_util::{check, check_comments};

#[test]
pub fn format_increment_decrement_statement_simple() {
    check(
        "fn main() {
        a
        ++
        ;
        b
        --
        ;
        }",
        expect![[r#"
            fn main() {
                a++;
                b--;
            }
        "#]],
    );
}

#[test]
pub fn format_increment_decrement_statement_indexed() {
    check(
        "fn main() {
        a[0]
        ++
        ;
        b[247]
        --
        ;
        }",
        expect![[r#"
            fn main() {
                a[0]++;
                b[247]--;
            }
        "#]],
    );
}

#[test]
pub fn format_increment_decrement_statement_within_for() {
    check(
        "fn main() {
        for(var a = 0; a < 17; a++) {}
        for(var a = 0; a < 17; a--) {}
        }",
        expect![[r#"
            fn main() {
                for(var a = 0; a < 17; a++) {}
                for(var a = 0; a < 17; a--) {}
            }
        "#]],
    );
}

#[test]
pub fn format_comments_in_increment_decrement_statement_simple() {
    check_comments(
        "fn main() {
        ## a ## ++ ## ; ##
        ## b ## -- ## ; ##
        }",
        expect![[r#"
            fn main() {
                /* 0 */
                a /* 1 */ ++ /* 2 */; /* 3 */
                /* 4 */
                b /* 5 */ -- /* 6 */; /* 7 */
            }
        "#]],
        expect![[r#"
            fn main() {
                // 0
                a // 1
                ++ // 2
                ; // 3

                // 4
                b // 5
                -- // 6
                ; // 7
            }
        "#]],
    );
}

#[test]
pub fn format_comments_in_increment_decrement_statement_index() {
    check_comments(
        "fn main() {
        ## a ## [ ## 0 ## ] ## ++ ## ; ##
        ## b ## [ ## 0 ## ] ## -- ## ; ##
        }",
        expect![[r#"
            fn main() {
                /* 0 */
                a /* 1 */ [/* 2 */ 0 /* 3 */] /* 4 */ ++ /* 5 */; /* 6 */
                /* 7 */
                b /* 8 */ [/* 9 */ 0 /* 10 */] /* 11 */ -- /* 12 */; /* 13 */
            }
        "#]],
        expect![[r#"
            fn main() {
                // 0
                a // 1
                [
                    // 2
                    0 // 3
                ] // 4
                ++ // 5
                ; // 6

                // 7
                b // 8
                [
                    // 9
                    0 // 10
                ] // 11
                -- // 12
                ; // 13
            }
        "#]],
    );
}
