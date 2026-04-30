use expect_test::expect;

use crate::test_util::check;

#[test]
pub fn format_infix_expr_very_long_let_statement() {
    check(
        "fn main() {
        let a = 111111111111111111111 + 2222222222222222222222222 + 3333333333333333333333333333 + 4444444444444444444444 + 555555555555555555555 + 666666666666666666666666 + 777777777777777777777 + 88888888888888888888 + 999999999999999999999;
        }",
        expect![["
            fn main() {
                let a = 111111111111111111111 + 2222222222222222222222222
                    + 3333333333333333333333333333 + 4444444444444444444444
                    + 555555555555555555555 + 666666666666666666666666
                    + 777777777777777777777 + 88888888888888888888 + 999999999999999999999;
            }
        "]],
    );
}

#[test]
pub fn format_infix_grouping_in_very_long_let_statement() {
    // a: The lines get split up between the short number groupings, as our control.
    // b: Now the short number groupings are within parens and should have a lower precedence of being split up
    // c: But if the short number groupings would be too long, they again get split up.
    check(
        "fn main() {
        let a = 111111111111111111111 + 2222222222222222222222222 + 333333333333333333333333333333 +  44444444444444 +                    55555555555 +                       66666666666666  + 777777777777777777777 + 88888888888888888888 + 999999999999999999999;
        let b = 111111111111111111111 + 2222222222222222222222222 + 333333333333333333333333333333 + (44444444444444 +                    55555555555 +                       66666666666666) + 777777777777777777777 + 88888888888888888888 + 999999999999999999999;
        let c = 111111111111111111111 + 2222222222222222222222222 + 333333333333333333333333333333 + (444444444444444444444444444444444 + 555555555555555555555555555555555 + 666666666666666666666666666666666666) + 777777777777777777777 + 88888888888888888888 + 999999999999999999999;
        }",
        expect![["
            fn main() {
                let a = 111111111111111111111 + 2222222222222222222222222
                    + 333333333333333333333333333333 + 44444444444444 + 55555555555
                    + 66666666666666 + 777777777777777777777 + 88888888888888888888
                    + 999999999999999999999;
                let b = 111111111111111111111 + 2222222222222222222222222
                    + 333333333333333333333333333333
                    + (44444444444444 + 55555555555 + 66666666666666)
                    + 777777777777777777777 + 88888888888888888888 + 999999999999999999999;
                let c = 111111111111111111111 + 2222222222222222222222222
                    + 333333333333333333333333333333
                    + (444444444444444444444444444444444 + 555555555555555555555555555555555
                        + 666666666666666666666666666666666666) + 777777777777777777777
                    + 88888888888888888888 + 999999999999999999999;
            }
        "]],
    );
}

#[test]
pub fn format_comment_position_in_multiline_expression() {
    check(
        "fn main() {
        const a_multiline_binding = 1 // The thing
                + 1 // The other thing
                + 7 // The other thing
                // Separate
                ;

        }",
        expect![["
            fn main() {
                const a_multiline_binding = 1 // The thing
                    + 1 // The other thing
                    + 7 // The other thing
                    // Separate
                    ;
            }
        "]],
    );
}

#[test]
pub fn format_infix_expr_very_long_break_outer_first() {
    check(
        "fn main() {
        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
        let aaaaaaaaaaaaa = 1 + 1 + long_function(aaaaaaaaaaa, bbbbbbbbbbb, ccccccccccccc, dddddddddddddd);
        }",
        expect![[r#"
            fn main() {
                //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
                let aaaaaaaaaaaaa = 1 + 1
                    + long_function(
                        aaaaaaaaaaa,
                        bbbbbbbbbbb,
                        ccccccccccccc,
                        dddddddddddddd,
                    );
            }
        "#]],
    );
}

#[test]
pub fn format_field_expr_deeply_nested() {
    check(
        "fn main() {
        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
        let a = foo.baaaaaar.booooooooor.buuuuuuuuuuur.biiiiiiiiir.beeeeeeer.buuuuuuuur.boooooooor.baaaaaaaaaaar.biiiiiiiiiiir.beeeeeeeeer;
        }",
        expect![[r#"
            fn main() {
                //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
                let a = foo.baaaaaar.booooooooor.buuuuuuuuuuur.biiiiiiiiir.beeeeeeer
                        .buuuuuuuur.boooooooor.baaaaaaaaaaar.biiiiiiiiiiir.beeeeeeeeer;
            }
        "#]],
    );
}

#[test]
pub fn format_index_expr_chained_breaks_in_the_middle() {
    check(
        "fn main() {
        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
        let a = aaaaaaaaa[bbbbbbbbbbbbbbb][cccccccccccc][ddddddddddddd][eeeeeeeeeeeee][ffffffffffffff];
        }",
        expect![[r#"
            fn main() {
                //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
                let a = aaaaaaaaa[bbbbbbbbbbbbbbb][cccccccccccc][ddddddddddddd][
                        eeeeeeeeeeeee
                    ][ffffffffffffff];
            }
        "#]],
    );
}

#[test]
pub fn format_index_expr_nested_breaks_outside_in() {
    check(
        "fn main() {
        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
        let a = aaaaaaaaa[bbbbbbbbbbbbbbb[cccccccccccc[dddddddddddd[eeeeeeeeeeeeeee[ffffffffffffff]]]]];
        }",
        expect![[r#"
            fn main() {
                //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
                let a = aaaaaaaaa[
                        bbbbbbbbbbbbbbb[
                            cccccccccccc[dddddddddddd[eeeeeeeeeeeeeee[ffffffffffffff]]]
                        ]
                    ];
            }
        "#]],
    );
}

#[test]
pub fn format_prefix_expr_no_space_after_prefix() {
    check(
        "fn main() {
        let a = - aaaa;
        }",
        expect![[r#"
            fn main() {
                let a = -aaaa;
            }
        "#]],
    );
}

#[test]
pub fn format_paren_expr_with_break_inside() {
    check(
        "fn main() {
        let a = 1+(1 + // Hi
        2);
        }",
        expect![[r#"
            fn main() {
                let a = 1 + (1 + // Hi
                        2);
            }
        "#]],
    );
}

#[test]
pub fn format_index_expr_with_break_inside() {
    check(
        "fn main() {
        let a = a[1 + // Hi
        2];
        }",
        expect![[r#"
            fn main() {
                let a = a[
                        1 + // Hi
                        2
                    ];
            }
        "#]],
    );
}
