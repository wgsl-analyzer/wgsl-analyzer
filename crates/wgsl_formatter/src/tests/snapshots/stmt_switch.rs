use expect_test::expect;

use crate::test_util::{check, check_comments};

#[test]
pub fn format_switch_statement_case_colon() {
    check(
        "fn main() {
            switch(a) {
                case 1: {
                    let a = 1;
                }
            }
        }",
        expect![[r#"
            fn main() {
                switch(a) {
                    case 1 {
                        let a = 1;
                    }
                }
            }
        "#]],
    );
}

#[test]
pub fn format_switch_statement_case_without_colon() {
    check(
        "fn main() {
            switch(a) {
                case 1 {
                    let a = 1;
                }
            }
        }",
        expect![[r#"
            fn main() {
                switch(a) {
                    case 1 {
                        let a = 1;
                    }
                }
            }
        "#]],
    );
}

#[test]
pub fn format_switch_statement_default_amidst_other_cases() {
    check(
        "fn main() {
            switch(a) {
                case 1 {
                    let a = 1;
                }
                default {
                    let a = 1;
                }
                case 2 {
                    let a = 1;
                }
            }
        }",
        expect![[r#"
            fn main() {
                switch(a) {
                    case 1 {
                        let a = 1;
                    }
                    default {
                        let a = 1;
                    }
                    case 2 {
                        let a = 1;
                    }
                }
            }
        "#]],
    );
}

#[test]
pub fn format_switch_statement_default_amidst_other_options_in_one_case() {
    check(
        "fn main() {
            switch(a) {
                case 1, default, 2 {
                    let a = 1;
                }
            }
        }",
        expect![[r#"
            fn main() {
                switch(a) {
                    case 1, default, 2 {
                        let a = 1;
                    }
                }
            }
        "#]],
    );
}

#[test]
pub fn format_switch_statement_case_default_only() {
    check(
        "fn main() {
            switch(a) {
                case default {
                    let a = 1;
                }
            }
        }",
        expect![[r#"
            fn main() {
                switch(a) {
                    default {
                        let a = 1;
                    }
                }
            }
        "#]],
    );
}

#[test]
pub fn format_switch_statement_trailing_comma() {
    check(
        "fn main() {
            switch(a) {
                case 1, 2, {
                    let a = 1;
                }
            }
        }",
        expect![[r#"
            fn main() {
                switch(a) {
                    case 1, 2 {
                        let a = 1;
                    }
                }
            }
        "#]],
    );
}

#[test]
pub fn format_switch_statement_comma_and_colon() {
    check(
        "fn main() {
            switch(a) {
                case 1,: {
                    let a = 1;
                }
            }
        }",
        expect![[r#"
            fn main() {
                switch(a) {
                    case 1 {
                        let a = 1;
                    }
                }
            }
        "#]],
    );
}

#[test]
pub fn format_switch_statement_const_expression() {
    check(
        "fn main() {
            switch(a) {
                case 1, c: {
                    let a = 1;
                }
            }
        }",
        expect![[r#"
            fn main() {
                switch(a) {
                    case 1, c {
                        let a = 1;
                    }
                }
            }
        "#]],
    );
}

#[test]
pub fn format_switch_statement_block_comments_in_case_default_only() {
    check_comments(
        "fn main() {
            switch(a) {
                ##
                case
                ##
                default
                ##
                {
                ##
                    let a = 1;
                    ##
                }
                ##
            }
        }",
        expect![[r#"
            fn main() {
                switch(a) { /* 0 */
                    default /* 1 */ /* 2 */ {
                        /* 3 */
                        let a = 1;
                        /* 4 */
                    } /* 5 */ 
                }
            }
        "#]],
        expect![[r#"
            fn main() {
                switch(a) { // 0
                    default // 1
                    // 2
                    {
                        // 3

                        let a = 1;
                        // 4
                    } // 5

                }
            }
        "#]],
    );
}

#[test]
pub fn format_switch_statement_comments_in_average_switch() {
    check_comments(
        "fn main() {
            ## switch ## ( ## a ## ) ## {
                ## case ## 1 ## , ## 2 ## , ## default ## {
                    ## let a = 1;
                ## }
                ## case ## 3 ## {
                    ## let a = 1;
                ## }
                ## case ## default ## {
                    ## let a = 1;
                ## }
            }
        }",
        expect![[r#"
            fn main() {
                /* 0 */
                switch /* 1 */ (/* 2 */ a /* 3 */) /* 4 */ { /* 5 */
                    case /* 6 */ 1 /* 7 */ , /* 8 */ 2 /* 9 */ , /* 10 */ default /* 11 */ {
                        /* 12 */
                        let a = 1;
                        /* 13 */
                    } /* 14 */
                    case /* 15 */ 3 /* 16 */ {
                        /* 17 */
                        let a = 1;
                        /* 18 */
                    } /* 19 */
                    default /* 20 */ /* 21 */ {
                        /* 22 */
                        let a = 1;
                        /* 23 */
                    }
                }
            }
        "#]],
        expect![[r#"
            fn main() {
                // 0
                switch // 1
                (// 2
                    a // 3
                ) // 4
                { // 5
                    case // 6
                    1 // 7
                    , // 8
                    2 // 9
                    , // 10
                    default // 11
                    {
                        // 12
                        let a = 1;
                        // 13
                    } // 14
                    case // 15
                    3 // 16
                    {
                        // 17
                        let a = 1;
                        // 18
                    } // 19
                    default // 20
                    // 21
                    {
                        // 22
                        let a = 1;
                        // 23
                    }
                }
            }
        "#]],
    );
}
