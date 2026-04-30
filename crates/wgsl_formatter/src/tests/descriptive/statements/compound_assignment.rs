use expect_test::expect;

use crate::test_util::{assert_out_of_scope, check, check_comments};

#[test]
pub fn format_compound_assignment_simple_1() {
    // See https://gpuweb.github.io/gpuweb/wgsl/#compound-assignment-sec for a list of possible operators.
    check(
        "fn main() {
        a
        +=
        1
        ;
        }",
        expect![["
            fn main() {
                a += 1;
            }
        "]],
    );
}

#[test]
fn format_compound_assignment_simple_2() {
    check(
        "fn main() {
    y  +=  x + y;
}",
        expect![[r#"
            fn main() {
                y += x + y;
            }
        "#]],
    );
}

#[test]
pub fn format_compound_assignment_all_operators() {
    // See https://gpuweb.github.io/gpuweb/wgsl/#compound-assignment-sec for a list of possible operators.
    check(
        "fn main() {
        a
        += 1;
        a
        -= 1;
        a
        *= 1;
        a
        /= 1;
        a
        %= 1;
        a
        &= 1;
        a
        |= 1;
        a
        ^= 1;
        a
        <<= 1;
        a
        >>= 1;
        }",
        expect![["
            fn main() {
                a += 1;
                a -= 1;
                a *= 1;
                a /= 1;
                a %= 1;
                a &= 1;
                a |= 1;
                a ^= 1;
                a <<= 1;
                a >>= 1;
            }
        "]],
    );
}

#[test]
pub fn format_compound_assignment_to_index() {
    check(
        "fn main() {
        a[1] <<= 728;
        a[1] += 728;
        a[1] /= 728;
        }",
        expect![["
            fn main() {
                a[1] <<= 728;
                a[1] += 728;
                a[1] /= 728;
            }
        "]],
    );
}

#[test]
pub fn format_compound_assignment_to_field() {
    check(
        "fn main() {
        a.b <<= 728;
        a.b += 728;
        a.b /= 728;
        a.b /= a.b;
        }",
        expect![[r#"
            fn main() {
                a.b <<= 728;
                a.b += 728;
                a.b /= 728;
                a.b /= a.b;
            }
        "#]],
    );
}

#[test]
pub fn format_compound_assignment_long_rhs_long_lhs() {
    check(
        "fn main() {
        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
        aaaaaaaaaaaaaaaaaaaaa
        .bbbbbbbbbbbbbbbbbbb
        .cccccccccccccccccc
        .ddddddddddddddddddd <<= foooooooooooooooo(
        baaaaaaaaaaaaaaar(
        111111111111111111111
        +
        8228282828282882828282828828282
        )
        );
        }",
        expect![[r#"
            fn main() {
                //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
                aaaaaaaaaaaaaaaaaaaaa.bbbbbbbbbbbbbbbbbbb.cccccccccccccccccc
                    .ddddddddddddddddddd <<= foooooooooooooooo(
                        baaaaaaaaaaaaaaar(
                            111111111111111111111 + 8228282828282882828282828828282,
                        ),
                    );
            }
        "#]],
    );
}

#[test]
pub fn format_comments_in_compound_assignment_statement_simple() {
    check_comments(
        "fn main() {
        ## a ## <<= ## b ## ; ##
        }",
        expect![[r#"
            fn main() {
                /* 0 */
                a /* 1 */ <<= /* 2 */ b /* 3 */; /* 4 */
            }
        "#]],
        expect![[r#"
            fn main() {
                // 0
                a // 1
                <<= // 2
                    b // 3
                    ; // 4
            }
        "#]],
    );
}
