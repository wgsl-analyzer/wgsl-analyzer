use expect_test::expect;

use crate::test_util::{check, check_comments};

#[test]
pub fn format_const_declaration_simple_literal_1() {
    check(
        "fn main() {
        const
        a
        =
        1
        ;
        }",
        expect![["
            fn main() {
                const a = 1;
            }
        "]],
    );
}

#[test]
pub fn format_const_declaration_simple_statement_1() {
    check(
        "fn main() {
        const
        a
        =
        1
        +
        1
        +
        1
        ;
        }",
        expect![["
            fn main() {
                const a = 1 + 1 + 1;
            }
        "]],
    );
}

#[test]
pub fn format_comment_in_const_declaration() {
    check_comments(
        "fn main() {
        ## const ## a ## = ## 1 ## ; ##
        }",
        expect![[r#"
            fn main() {
                /* 0 */
                const /* 1 */ a /* 2 */ = /* 3 */ 1 /* 4 */; /* 5 */
            }
        "#]],
        expect![[r#"
            fn main() {
                // 0
                const // 1
                    a // 2
                    = // 3
                    1 // 4
                    ; // 5
            }
        "#]],
    );
}

#[test]
pub fn format_const_declaration_keeps_line_comment_in_same_place_same_line() {
    check(
        "fn main() {

        const a = 1; // Assign 1


        }",
        expect![["
            fn main() {
                const a = 1; // Assign 1
            }
        "]],
    );
}

#[test]
pub fn format_const_declaration_with_type() {
    check(
        "fn main() {

        const a
        :
        u32
        =
        1;


        }",
        expect![[r#"
            fn main() {
                const a: u32 = 1;
            }
        "#]],
    );
}

#[test]
pub fn format_const_declaration_with_complex_type() {
    check(
        "fn main() {

        const a
        :
        array
        <
        u32
        ,
        28
        >
        = 1;


        }",
        expect![["
            fn main() {
                const a: array<u32, 28> = 1;
            }
        "]],
    );
}

#[test]
pub fn format_const_declaration_with_comments_in_complex_type() {
    check_comments(
        "fn main() {
            ## const ## a ## : ## array ## < ## u32 ## , ## 28 ## > ## = ## 1 ## ; ##
        }",
        expect![[r#"
            fn main() {
                /* 0 */
                const /* 1 */ a /* 2 */: /* 3 */ array /* 4 */ <
                        /* 5 */ u32, /* 6 */ /* 7 */
                        28, /* 8 */
                    > /* 9 */ = /* 10 */ 1 /* 11 */; /* 12 */
            }
        "#]],
        expect![[r#"
            fn main() {
                // 0
                const // 1
                    a // 2
                    : // 3
                    array // 4
                    <
                        // 5
                        u32, // 6
                        // 7
                        28, // 8
                    > // 9
                    = // 10
                    1 // 11
                    ; // 12
            }
        "#]],
    );
}

#[test]
pub fn format_global_const_declaration_simple_literal_1() {
    check(
        "
        const
        a
        =
        1
        ;
        ",
        expect![[r#"
            const a = 1;
        "#]],
    );
}

#[test]
pub fn format_global_const_declaration_with_type_1() {
    check(
        "
        const
        a: array<u32, 28>
        =
        make_array()
        ;
        ",
        expect![[r#"
            const a: array<u32, 28> = make_array();
        "#]],
    );
}
