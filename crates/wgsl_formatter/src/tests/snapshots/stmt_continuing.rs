use expect_test::expect;

use crate::test_util::{assert_out_of_scope, check};

#[test]
pub fn format_loop_continuing_statement_empty() {
    check(
        "fn main() {
        loop {
        continuing {

        }
        }


        }",
        expect![["
            fn main() {
                loop {
                    continuing {}
                }
            }
        "]],
    );
}

#[test]
pub fn format_continuing_statement_without_loop() {
    assert_out_of_scope(
        "fn main() {
        continuing {

        }


        }",
        "Wgsl disallows continuing statements outside of loop statements",
    );
}

#[test]
pub fn format_loop_continuing_statement_single_statement() {
    check(
        "fn main() {
        loop {
        continuing{

        let a = 3;
        }
        }


        }",
        expect![["
            fn main() {
                loop {
                    continuing {
                        let a = 3;
                    }
                }
            }
        "]],
    );
}

#[test]
pub fn format_loop_statement_continue_statement() {
    // This is just a very simple smoke test for completeness, more fine grained tests are in stmt_continue.rs
    check(
        "fn main() {
        loop {
        let a = 3;
        continue;
        let b = 3;
        continuing {
        let c = 3;

        }
        }


        }",
        expect![["
            fn main() {
                loop {
                    let a = 3;
                    continue;
                    let b = 3;
                    continuing {
                        let c = 3;
                    }
                }
            }
        "]],
    );
}

#[test]
pub fn format_loop_continuing_statement_block_comments() {
    check(
        "fn main() {
        /* A */
        loop
        /* B */
        {
        /* C */
        continuing
        /* D */
        {
        /* E */
        }
        /* F */
        }
        /* G */
        }",
        expect![["
            fn main() {
                /* A */
                loop /* B */ {
                    /* C */
                    continuing /* D */ {
                        /* E */
                    }
                    /* F */
                }
                /* G */
            }
        "]],
    );
}

#[test]
pub fn format_loop_continuing_statement_line_comments() {
    check(
        "fn main() {
        // A
        loop
        // B
        {
        // C
        continuing
        // D
        {
        // E
        }
        // F
        }
        // G
        }",
        expect![["
            fn main() {
                // A
                loop // B
                {
                    // C
                    continuing // D
                    {
                        // E
                    }
                    // F
                }
                // G
            }
        "]],
    );
}
