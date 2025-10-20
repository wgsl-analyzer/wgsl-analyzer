use expect_test::expect;

use crate::test_util::check;

#[test]
pub fn format_if_statement_empty() {
    check(
        "fn main() {
        if
        false
        {
        }
        }",
        expect![["
            fn main() {
                if false {}
            }
        "]],
    );
}

#[test]
pub fn format_if_statement_simple_expr() {
    check(
        "fn main() {
        if
        false
        {
        let a = 1;
        }
        }",
        expect![["
            fn main() {
                if false {
                    let a = 1;
                }
            }
        "]],
    );
}

#[test]
pub fn format_if_else_statement_empty() {
    check(
        "fn main() {
        if
        false
        {
        }
        else
        {
        }
        }",
        expect![["
            fn main() {
                if false {} else {}
            }
        "]],
    );
}

#[test]
pub fn format_if_else_statement_simple_expr() {
    check(
        "fn main() {
        if
        false
        {
        let a = 1;
        }
        else
        {
        let b = 1;
        }
        }",
        expect![["
            fn main() {
                if false {
                    let a = 1;
                } else {
                    let b = 1;
                }
            }
        "]],
    );
}

#[test]
pub fn format_if_elseif_else_statement_empty() {
    check(
        "fn main() {
        if
        false
        {
        }
        else
        if
        true
        {
        }
        else
        {
        }
        }",
        expect![["
            fn main() {
                if false {} else if true {} else {}
            }
        "]],
    );
}

#[test]
pub fn format_if_elseif_else_statement_simple_expr() {
    check(
        "fn main() {
        if
        false
        {
        let a = 1;
        }
        else
        if
        true
        {
        let b = 1;
        }
        else
        {
        let c = 1;
        }
        }",
        expect![["
            fn main() {
                if false {
                    let a = 1;
                } else if true {
                    let b = 1;
                } else {
                    let c = 1;
                }
            }
        "]],
    );
}

#[test]
pub fn format_if_elseif_elseif_else_statement_empty() {
    check(
        "fn main() {
        if
        a
        {
        }
        else
        if
        b
        {
        }
        else
        if
        c
        {
        }
        else
        {
        }
        }",
        expect![["
            fn main() {
                if a {} else if b {} else if c {} else {}
            }
        "]],
    );
}

#[test]
pub fn format_if_elseif_elseif_else_statement_simple_expr() {
    check(
        "fn main() {
        if
        a
        {
        let a = 1;
        }
        else
        if
        b
        {
        let b = 1;
        }
        else
        if
        c
        {
        let c = 1;
        }
        else
        {
        let e = 1;
        }
        }",
        expect![["
            fn main() {
                if a {
                    let a = 1;
                } else if b {
                    let b = 1;
                } else if c {
                    let c = 1;
                } else {
                    let e = 1;
                }
            }
        "]],
    );
}

#[test]
pub fn format_if_elseif_else_statement_block_comments() {
    check(
        "fn main() {
        /* A */
        if
        /* B */
        a
        /* C */
        {
        /* D */
        }
        /* E */
        else
        /* F */
        if
        /* G */
        a
        /* H */
        {
        /* I */
        }
        /* J */
        else
        /* K */
        {
        /* L */
        }
        /* M */
        }",
        expect![["
            fn main() {
                /* A */
                if /* B */ a /* C */ {
                    /* D */
                } /* E */ else /* F */ if /* G */ a /* H */ {
                    /* I */
                } /* J */ else /* K */ {
                    /* L */
                }
                /* M */
            }
        "]],
    );
}

#[test]
pub fn format_if_elseif_else_statement_line_comments() {
    check(
        "fn main() {
        // A
        if
        // B
        a
        // C
        {
        // D
        }
        // E
        else
        // F
        if
        // G
        b
        // H
        {
        // I
        }
        // J
        else
        // K
        {
        // L
        }
        // M
        }",
        expect![[r#"
            fn main() {
                // A
                if // B
                a // C
                {
                    // D
                } // E
                else // F
                if // G
                b // H
                {
                    // I
                } // J
                else // K
                {
                    // L
                }
                // M
            }
        "#]],
    );
}

#[test]
pub fn format_if_else_statement_block_comments() {
    check(
        "fn main() {
        /* A */
        if
        /* B */
        a
        /* C */
        {
        /* D */
        }
        /* E */
        else
        /* F */
        {
        /* G */
        }
        /* H */
        }",
        expect![["
            fn main() {
                /* A */
                if /* B */ a /* C */ {
                    /* D */
                } /* E */ else /* F */ {
                    /* G */
                }
                /* H */
            }
        "]],
    );
}

#[test]
pub fn format_if_else_statement_line_comments() {
    check(
        "fn main() {
        // A
        if
        // B
        a
        // C
        {
        // D
        }
        // E
        else
        // F
        {
        // G
        }
        // H
        }",
        expect![["
            fn main() {
                // A
                if // B
                a // C
                {
                    // D
                } // E
                else // F
                {
                    // G
                }
                // H
            }
        "]],
    );
}
