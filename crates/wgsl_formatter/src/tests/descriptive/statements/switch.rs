use expect_test::expect;

use crate::test_util::{check, check_comments};

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
                switch a {
                    case 1 { let a = 1; }
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
                switch a {
                    case 1, default, 2 { let a = 1; }
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
                switch a {
                    default { let a = 1; }
                }
            }
        "#]],
    );
}

#[test]
pub fn format_comments_in_switch_statement_case_default_only() {
    check(
        "fn main() {
            switch(a) {
                /* ABC */ case /* DEF */ default {}
            }
            switch(b) {
                /* ABC */
                case /* DEF */ default {}
            }
            switch(c) {
                /* ABC */ case
                /* DEF */ default {}
            }
            switch(d) {
                /* ABC */ case /* DEF */
                default {}
            }
        }",
        expect![[r#"
            fn main() {
                switch a {
                    /* ABC */ /* DEF */ default {}
                }
                switch b {
                    /* ABC */
                    /* DEF */ default {}
                }
                switch c {
                    /* ABC */ /* DEF */ default {}
                }
                switch d {
                    /* ABC */ /* DEF */ default {}
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
                switch a {
                    case 1, 2 { let a = 1; }
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
                switch a {
                    case 1 { let a = 1; }
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
                switch a {
                    case 1, c { let a = 1; }
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
                ## case ## default ## { ## let a = 1; ## } ##
            }
        }",
        expect![[r#"
            fn main() {
                switch a {
                    /* 0 */ /* 1 */ default /* 2 */ { /* 3 */ let a = 1; /* 4 */ } /* 5 */
                }
            }
        "#]],
        expect![[r#"
            fn main() {
                switch a {
                    // 0
                    // 1
                    default // 2
                    {
                        // 3
                        let a = 1; // 4
                    } // 5
                }
            }
        "#]],
    );
}

#[test]
pub fn format_switch_statement_comments_in_average_switch() {
    check_comments(
        "fn main() { ## switch ## ( ## a ## ) ## { ## case ## 1 ## , ## default ## { ## let a = 1; ## } ## case ## 3 ## { ## let a = 1; ## } ## case ## default ## { ## let a = 1; ## } } }",
        expect![[r#"
            fn main() {
                /* 0 */ switch /* 1 */ /* 2 */ a /* 3 */ /* 4 */ {
                    /* 5 */ case /* 6 */ 1 /* 7 */ , /* 8 */ default /* 9 */ {
                        /* 10 */ let a = 1; /* 11 */
                    } /* 12 */
                    case /* 13 */ 3 /* 14 */ { /* 15 */ let a = 1; /* 16 */ } /* 17 */
                    /* 18 */ default /* 19 */ { /* 20 */ let a = 1; /* 21 */ }
                }
            }
        "#]],
        expect![[r#"
            fn main() {
                // 0
                switch // 1
                // 2
                a // 3
                // 4
                {
                    // 5
                    case // 6
                    1 // 7
                    , // 8
                    default // 9
                    {
                        // 10
                        let a = 1; // 11
                    } // 12
                    case // 13
                    3 // 14
                    {
                        // 15
                        let a = 1; // 16
                    } // 17
                    // 18
                    default // 19
                    {
                        // 20
                        let a = 1; // 21
                    }
                }
            }
        "#]],
    );
}

#[test]
pub fn format_block_comments_around_empty_switch_statement() {
    check(
        "
        fn main() {
            /* Before */
            switch a {} /* Can stay on the same line */
            /* After */
        }
        ",
        expect![[r#"
            fn main() {
                /* Before */
                switch a {} /* Can stay on the same line */
                /* After */
            }
        "#]],
    );
}
#[test]
pub fn format_block_comments_around_nonempty_switch_statement() {
    check(
        "
        fn main() {
            /* Before */
            switch a {default {}} /* Should be broken onto separate the same line */
            /* After */
        }
        ",
        expect![[r#"
            fn main() {
                /* Before */
                switch a {
                    default {}
                }
                /* Should be broken onto separate the same line */
                /* After */
            }
        "#]],
    );
}
