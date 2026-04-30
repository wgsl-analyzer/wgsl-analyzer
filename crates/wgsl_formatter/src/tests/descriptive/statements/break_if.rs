use expect_test::expect;

use crate::test_util::{check, check_comments};

#[test]
pub fn format_loop_continuing_break_if_statement_empty() {
    check(
        "fn main() {
        loop {
        continuing {
        break if false;

        }
        }


        }",
        expect![["
            fn main() {
                loop {
                    continuing {
                        break if false;
                    }
                }
            }
        "]],
    );
}

#[test]
pub fn format_loop_continuing_break_if_statement_with_important_parens() {
    check(
        "fn main() {
        loop {
        continuing {
        break if (1 + (1 + 1));

        }
        }


        }",
        expect![[r#"
            fn main() {
                loop {
                    continuing {
                        break if 1 + (1 + 1);
                    }
                }
            }
        "#]],
    );
}

#[test]
pub fn format_loop_continuing_break_if_statement_simple() {
    check(
        "fn main() {
        loop {
        continuing{

        break if false;
        }
        }


        }",
        expect![["
            fn main() {
                loop {
                    continuing {
                        break if false;
                    }
                }
            }
        "]],
    );
}

#[test]
pub fn format_loop_continuing_break_if_statement_complex_expression() {
    check(
        "fn main() {
        loop {
        continuing{

        break if a == 3 && b < 4 && ((a < 4) == (b > 9)) && c < 5 && do_the_thing(d);
        }
        }


        }",
        expect![["
            fn main() {
                loop {
                    continuing {
                        break if a == 3 && b < 4 && ((a < 4) == (b > 9)) && c < 5
                            && do_the_thing(d);
                    }
                }
            }
        "]],
    );
}

#[test]
pub fn format_comments_in_loop_continuing_break_if_statement() {
    check_comments(
        "fn main() { ## loop ## { ## continuing ## { ## break ## if ## false ## ; ## } ## } ## }",
        expect![[r#"
            fn main() {
                /* 0 */
                loop /* 1 */ {
                    /* 2 */
                    continuing /* 3 */ {
                        /* 4 */
                        break /* 5 */ if /* 6 */ false /* 7 */; /* 8 */
                    }
                    /* 9 */
                }
                /* 10 */
            }
        "#]],
        expect![[r#"
            fn main() {
                // 0
                loop // 1
                {
                    // 2
                    continuing // 3
                    {
                        // 4
                        break // 5
                        if // 6
                            false // 7
                            ; // 8
                    }
                    // 9
                }
                // 10
            }
        "#]],
    );
}

#[test]
pub fn format_comments_in_loop_continuing_break_if_with_needless_parens_statement() {
    check_comments(
        "fn main() { ## loop ## { ## continuing ## { ## break ## if ## ( ## false ## ) ## ; ## ## } ## } ## }",
        expect![[r#"
            fn main() {
                /* 0 */
                loop /* 1 */ {
                    /* 2 */
                    continuing /* 3 */ {
                        /* 4 */
                        break /* 5 */ if /* 6 */ /* 7 */ false /* 8 */ /* 9 */; /* 10 */ /* 11 */
                    }
                    /* 12 */
                }
                /* 13 */
            }
        "#]],
        expect![[r#"
            fn main() {
                // 0
                loop // 1
                {
                    // 2
                    continuing // 3
                    {
                        // 4
                        break // 5
                        if // 6
                            // 7
                            false // 8
                            // 9
                            ; // 10
                        // 11
                    }
                    // 12
                }
                // 13
            }
        "#]],
    );
}
