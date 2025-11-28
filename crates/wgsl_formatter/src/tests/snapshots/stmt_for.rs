use expect_test::expect;

use crate::test_util::{check, check_comments};

#[test]
pub fn format_for_statement_no_initializer() {
    check(
        "fn main() {
        for(;i<4;i++) {
        }


        }",
        expect![[r#"
            fn main() {
                for(; i < 4; i++) {}
            }
        "#]],
    );
}

#[test]
pub fn format_for_statement_no_condition() {
    check(
        "fn main() {
        for(var a = 0;;i++) {
        }


        }",
        expect![[r#"
            fn main() {
                for(var a = 0;; i++) {}
            }
        "#]],
    );
}

#[test]
pub fn format_for_statement_no_continuing() {
    check(
        "fn main() {
        for(var a = 0;a<4;) {
        }


        }",
        expect![[r#"
            fn main() {
                for(var a = 0; a < 4;) {}
            }
        "#]],
    );
}

#[test]
pub fn format_for_statement_no_anything() {
    check(
        "fn main() {
        for(;;) {
        }


        }",
        expect![[r#"
            fn main() {
                for(;;) {}
            }
        "#]],
    );
}

#[test]
pub fn format_for_statement_long_first_component() {
    check(
        "fn main() {
        for(let a = 1+1+1+1+alculate_something_really_long(172832782);a<3;a +=1) {
        }


        }",
        expect![[r#"
            fn main() {
                for(
                    let a = 1 + 1 + 1 + 1 + alculate_something_really_long(172832782);
                    a < 3;
                    a +=1
                ) {}
            }
        "#]],
    );
}

#[test]
pub fn format_for_statement_long_components() {
    check(
        "fn main() {
        for(let a = 1+1+1+1+alculate_something_really_long(172832782);compute_some_value(a % 12847248 * 1827348 + 182748) < A_LONG_CONSTANT;a = increment_but_fancy(a)) {
        }


        }",
        expect![[r#"
            fn main() {
                for(
                    let a = 1 + 1 + 1 + 1 + alculate_something_really_long(
                            172832782,
                        );
                    compute_some_value(a % 12847248 * 1827348 + 182748) < A_LONG_CONSTANT;
                    a = increment_but_fancy(a)
                ) {}
            }
        "#]],
    );
}

#[test]
pub fn format_for_statement_super_long_components() {
    check(
        "fn main() {
        for(let a = 1+1+1+1+1+1+calculate_something_really_long(172832782, 1827387428, 3487348342);compute_some_value_that_has_a_long_name_from(a % 12847248 * 1827348 + 182748) < AN_INCONVENIENTLY_LONG_CONSTANT_DECLARED_SOMEWHERE_ELSE;a = increment_but_in_a_very_fancy_manner(a)) {
        }


        }",
        expect![[r#"
            fn main() {
                for(
                    let a = 1 + 1 + 1 + 1 + 1 + 1 + calculate_something_really_long(
                            172832782,
                            1827387428,
                            3487348342,
                        );
                    compute_some_value_that_has_a_long_name_from(
                        a % 12847248 * 1827348 + 182748,
                    ) < AN_INCONVENIENTLY_LONG_CONSTANT_DECLARED_SOMEWHERE_ELSE;
                    a = increment_but_in_a_very_fancy_manner(
                        a,
                    )
                ) {}
            }
        "#]],
    );
}

#[test]
pub fn format_for_statement_simple_empty() {
    check(
        "fn main() {
        for(var i = 0; i < 4; i++) {
        }


        }",
        expect![[r#"
            fn main() {
                for(var i = 0; i < 4; i++) {}
            }
        "#]],
    );
}

#[test]
fn format_for_statement_average() {
    check(
        "fn main() {
    for( var i = 0;i < 100;   i = i + 1  ){}
}",
        expect![[r#"
            fn main() {
                for(var i = 0; i < 100; i = i + 1) {}
            }
        "#]],
    );
}

#[test]
pub fn format_comments_in_for_statement() {
    check_comments(
        "fn main() {
        ## for ## ( ## var ## i ## = ## 0 ## ; ## i ## < ## 4 ## ; ## i ## ++ ##) ## {
        ## } ##
        }",
        expect![[r#"
            fn main() {
                /* 0 */
                for /* 1 */ (
                    /* 2 */
                    var /* 3 */ i /* 4 */ = /* 5 */ 0 /* 6 */; /* 7 */
                    i /* 8 */ < /* 9 */ 4 /* 10 */; /* 11 */
                    i /* 12 */ ++ /* 13 */
                ) /* 14 */ {
                    /* 15 */
                } /* 16 */
            }
        "#]],
        expect![[r#"
            fn main() {
                // 0
                for // 1
                (
                    // 2
                    var // 3
                        i // 4
                        = // 5
                        0 // 6
                    ; // 7
                    i // 8
                    < // 9
                    4 // 10
                    ; // 11
                    i // 12
                    ++ // 13
                ) // 14
                {
                    // 15
                } // 16
            }
        "#]],
    );
}
