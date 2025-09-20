use expect_test::expect;

use crate::test_util::{assert_out_of_scope, check};

#[test]
pub fn format_naked_paren_exprs_out_of_scope() {
    assert_out_of_scope(
        "fn main() {
        (
        1
        +
        1
        )
        ;
        }",
    );
}

#[test]
pub fn format_paren_expr_simple() {
    check(
        "fn main() {
        let a = (1+1);
        }",
        expect![["
            fn main() {
                let a = (1 + 1);
            }
        "]],
    );
}

#[test]
pub fn format_paren_expr_long_right_associated() {
    //TODO This is awful. Have another look at how this should be formatted, once more test cases for more common parenthesised expressions are there
    check(
        "fn main() {
        let a = (1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+1)))))))))))))))))))));
        }",
        expect![["
            fn main() {
                let a = (1
                         + (1
                             + (1
                                 + (1
                                     + (1
                                         + (1
                                             + (1
                                                 + (1
                                                     + (1
                                                         + (1
                                                             + (1
                                                                 + (1
                                                                     + (1
                                                                         + (1
                                                                             + (1
                                                                                 + (1
                                                                                     + (1
                                                                                         + (1
                                                                                             + (1
                                                                                                 + (1
                                                                                                     + (1
                                                                                                         + 1)))))))))))))))))))));
            }
        "]],
    );
}

#[test]
pub fn format_paren_expr_long_left_associated() {
    //TODO This is beyond awful. Have another look at how this should be formatted, once more test cases for more common parenthesised expressions are there
    check(
        "fn main() {
        let a = ((((((((((((((((((((((((((((1)+1)+1)+1)+1)+1)+1)+1)+1)+1)+1)+1)+1)+1)+1)+1)+1)+1)+1)+1)+1)+1)+1)+1)+1)+1)+1)+1);
        }",
        expect![["
            fn main() {
                let a = ((((((((((((((((((((((((((((1) + 1) + 1) + 1) + 1) + 1) + 1) + 1)
                                                                                                     + 1)
                                                                                                 + 1)
                                                                                             + 1)
                                                                                         + 1)
                                                                                     + 1)
                                                                                 + 1) + 1)
                                                                         + 1) + 1) + 1) + 1)
                                                         + 1) + 1) + 1) + 1) + 1) + 1) + 1)
                             + 1) + 1);
            }
        "]],
    );
}

#[test]
#[ignore = "This currently causes a stack overflow. There is no obvious 'quick fix' for that, we need to investigate further."]
pub fn format_paren_expr_very_long() {
    check(
        "fn main() {
        let a = (1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+1))))))))))))))))))))))))))))))))))))))))))))))))));
        }",
        expect![["
            fn main() {
                let a = 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1
                     + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1
                     + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1
                     + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1
                     + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1;
            }
        "]],
    );
}
