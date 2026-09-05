use expect_test::expect;

use crate::test_util::{check, check_comments};

// TODO (MonaMayrhofer, post-1.0)
#[test]
#[ignore = "TODO https://github.com/wgsl-analyzer/wgsl-analyzer/issues/1380"]
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

// TODO (MonaMayrhofer, post-1.0)
#[test]
#[ignore = "TODO https://github.com/wgsl-analyzer/wgsl-analyzer/issues/1380"]
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

// TODO (MonaMayrhofer, post-1.0)
#[test]
#[ignore = "TODO https://github.com/wgsl-analyzer/wgsl-analyzer/issues/1380"]
pub fn format_loop_statement_continue_statement() {
    // This is just a very simple smoke test for completeness, more fine grained tests are in continue.rs
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

// TODO (MonaMayrhofer, post-1.0)
#[test]
#[ignore = "TODO https://github.com/wgsl-analyzer/wgsl-analyzer/issues/1380"]
pub fn format_loop_continuing_statement_block_comments() {
    check_comments(
        "fn main() {
        ## loop ## {
        ## continuing ## {
        ## }
        ## }
        }",
        expect![[r#"
            fn main() {
                /* 0 */
                loop /* 1 */ {
                    /* 2 */
                    continuing /* 3 */ {
                        /* 4 */
                    }
                    /* 5 */
                }
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
                    }
                    // 5
                }
            }
        "#]],
    );
}
