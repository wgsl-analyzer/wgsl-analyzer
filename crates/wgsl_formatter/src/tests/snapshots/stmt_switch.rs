use expect_test::expect;

use crate::test_util::check;

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
    check(
        "fn main() {
            switch(a) {
                /* A */
                case
                /* B */
                default
                /* C */
                {
                /* D */
                    let a = 1;
                /* E */
                }
                /* F */
            }
        }",
        expect![[r#"
            fn main() {
                switch(a) { /* A */
                    default /* B */ /* C */ {
                        /* D */
                        let a = 1;
                        /* E */
                    } /* F */
                }
            }
        "#]],
    );
}

#[test]
pub fn format_switch_statement_block_comments_in_average_switch() {
    check(
        "fn main() {
        /* A */
            switch
            /* B */
            (
            /* C */
            a
            /* D */
            )
            /* E */
            {
            /* F */
            case
            /* G */
            1
            /* H */
            ,
            /* I */
            2
            /* J */
            ,
            /* K */
            :
            /* L */
            {
            /* M */
                let a = 1;
            /* N */
            }
            /* O */
                case
                /* P */
                default
                /* Q */
                {
                /* R */
                    let a = 1;
                /* S */
                }
                /* T */
            }
            /* U */
        }",
        expect![[r#"
            fn main() {
                /* A */
                switch /* B */ (/* C */ a /* D */) /* E */ { /* F */
                    case /* G */ 1 /* H */ , /* I */ 2 /* J */ /* K */ /* L */ {
                        /* M */
                        let a = 1;
                        /* N */
                    } /* O */
                    default /* P */ /* Q */ {
                        /* R */
                        let a = 1;
                        /* S */
                    } /* T */ 
                }
                /* U */
            }
        "#]],
    );
}
