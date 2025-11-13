use expect_test::expect;

use crate::test_util::{check, check_comments};

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
fn format_if_statement_remove_parens() {
    check(
        "fn main() {
    if(x < 1){}
    if    x < 1     {}
}",
        expect![[r#"
            fn main() {
                if x < 1 {}
                if x < 1 {}
            }
        "#]],
    );
}

#[test]
fn format_if_statement_average_2() {
    check(
        "fn main() {
    if(x < 1){}
    else if (x > 2){
        let a = 3;
    }else     if(  x > 2 ){}
}",
        expect![[r#"
            fn main() {
                if x < 1 {} else if x > 2 {
                    let a = 3;
                } else if x > 2 {}
            }
        "#]],
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
pub fn format_comments_in_if_elseif_else_statement() {
    check_comments(
        "fn main() {
        ## if ## a ## { ## } ##
        ## else ## if ## a ## { ## } ##
        ## else ## { ## } ##
        }",
        expect![[r#"
            fn main() {
                /* 0 */
                if /* 1 */ a /* 2 */ {
                    /* 3 */
                } /* 4 */ /* 5 */ else /* 6 */ if /* 7 */ a /* 8 */ {
                    /* 9 */
                } /* 10 */ /* 11 */ else /* 12 */ {
                    /* 13 */
                } /* 14 */
            }
        "#]],
        expect![[r#"
            fn main() {
                // 0
                if // 1
                a // 2
                {
                    // 3
                } // 4
                // 5
                else // 6
                if // 7
                a // 8
                {
                    // 9
                } // 10
                // 11
                else // 12
                {
                    // 13
                } // 14
            }
        "#]],
    );
}

#[test]
pub fn format_comments_in_if_else_statement() {
    check_comments(
        "fn main() {
        ## if ## a ## { ## }
        ## else ## { ## }
        ##
        }",
        expect![[r#"
            fn main() {
                /* 0 */
                if /* 1 */ a /* 2 */ {
                    /* 3 */
                } /* 4 */ else /* 5 */ {
                    /* 6 */
                }
                /* 7 */
            }
        "#]],
        expect![[r#"
            fn main() {
                // 0
                if // 1
                a // 2
                {
                    // 3
                } // 4
                else // 5
                {
                    // 6
                }
                // 7
            }
        "#]],
    );
}
