use expect_test::expect;

use crate::test_util::{check, check_comments};

#[test]
fn format_assignment_statement_simple() {
    check(
        "fn main() {
        a
        =
        b
        ;
        }",
        expect![[r#"
            fn main() {
                a = b;
            }
        "#]],
    );
}

#[test]
fn format_assignment_statement_parenthesis() {
    check(
        "fn main() {
        (a) = (b + c)
        ;
        }",
        expect![[r#"
            fn main() {
                a = b + c;
            }
        "#]],
    );
}

#[test]
fn format_assignment_statement_to_field() {
    check(
        "fn main() {
        foo.a
        =
        b
        ;
        }",
        expect![[r#"
            fn main() {
                foo.a = b;
            }
        "#]],
    );
}

#[test]
fn format_assignment_statement_to_index() {
    check(
        "fn main() {
        foo[0]
        =
        b
        ;
        }",
        expect![[r#"
            fn main() {
                foo[0] = b;
            }
        "#]],
    );
}

#[test]
fn format_assignment_statement_within_for() {
    check(
        "fn main() {
        for(a
        = 27;
        a < 100
        ; a
        = process(a)){}
        }",
        expect![[r#"
            fn main() {
                for(a = 27; a < 100; a = process(a)) {}
            }
        "#]],
    );
}

#[test]
fn format_comments_within_assignment_statement() {
    check_comments(
        "fn main() {
        ## a ## = ## b ## ; ##
        }",
        expect![[r#"
            fn main() {
                /* 0 */
                a /* 1 */ = /* 2 */ b /* 3 */; /* 4 */
            }
        "#]],
        expect![[r#"
            fn main() {
                // 0
                a // 1
                = // 2
                b // 3
                ; // 4
            }
        "#]],
    );
}

#[test]
fn format_comments_within_complex_assignment_statement() {
    check_comments(
        "fn main() {
        ## a ## = ## b ## + ## get ## ( ## 17 ## ) ## ; ##
        }",
        expect![[r#"
            fn main() {
                /* 0 */
                a /* 1 */ = /* 2 */ b /* 3 */ + /* 4 */ get /* 5 */ (
                    /* 6 */ 17, /* 7 */
                ) /* 8 */; /* 9 */
            }
        "#]],
        expect![[r#"
            fn main() {
                // 0
                a // 1
                = // 2
                b // 3
                + // 4
                get // 5
                (
                    // 6
                    17, // 7
                ) // 8
                ; // 9
            }
        "#]],
    );
}
