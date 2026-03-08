use expect_test::expect;

use crate::test_util::check;

#[test]
pub fn format_switch_statement_empty_gets_collapsed() {
    check(
        "fn main() {
            switch(a) {
            }
        }",
        expect![[r#"
            fn main() {
                switch a {}
            }
        "#]],
    );
}

#[test]
pub fn format_switch_statement_case_colon_gets_removed() {
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
                switch a {
                    case 1 {
                        let a = 1;
                    }
                }
            }
        "#]],
    );
}

#[test]
pub fn format_switch_statement_default_amidst_other_cases_does_not_get_moved() {
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
                switch a {
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
pub fn format_switch_statement_case_default_gets_turned_to_default() {
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
                    default {
                        let a = 1;
                    }
                }
            }
        "#]],
    );
}

#[test]
pub fn format_switch_statement_case_trailing_comma_gets_removed() {
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
                    case 1, 2 {
                        let a = 1;
                    }
                }
            }
        "#]],
    );
}
