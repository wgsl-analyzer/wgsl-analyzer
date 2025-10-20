use expect_test::expect;

use crate::test_util::{assert_out_of_scope, check};

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
pub fn format_break_if_statement_without_loop() {
    assert_out_of_scope(
        "fn main() {
        break if false;
        }",
        "Wgsl disallows only allows break if statements as the last statement of a continuing block",
    );
}

#[test]
pub fn format_break_if_statement_without_continuing() {
    assert_out_of_scope(
        "fn main() {
        loop{
        break if false;
        }
        }",
        "Wgsl disallows only allows break if statements as the last statement of a continuing block",
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
pub fn format_loop_continuing_break_if_statement_block_comments() {
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
        break
        /* F */
        if
        /* G */
        false
        /* H */
        ;
        }
        /* I */
        }
        /* J */
        }",
        expect![["
            fn main() {
                /* A */
                loop /* B */ {
                    /* C */
                    continuing /* D */ {
                        /* E */
                        break /* F */ if /* G */ false /* H */;
                    }
                    /* I */
                }
                /* J */
            }
        "]],
    );
}

#[test]
pub fn format_loop_continuing_break_if_statement_line_comments() {
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
        break
        // F
        if
        // G
        false
        // H
        ;
        }
        // I
        }
        // J
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
                        break // F
                        if // G
                        false // H
                        ;
                    }
                    // I
                }
                // J
            }
        "]],
    );
}
