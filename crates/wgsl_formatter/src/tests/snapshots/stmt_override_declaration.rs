use expect_test::expect;

use crate::test_util::{check, check_comments};

#[test]
pub fn format_override_declaration_simple_literal_1() {
    check(
        "
        override
        a
        =
        1
        ;
        ",
        expect![[r#"
            override a = 1;
        "#]],
    );
}

#[test]
pub fn format_override_declaration_simple_statement_1() {
    check(
        "
        override
        a
        =
        1
        +
        1
        +
        1
        ;
        ",
        expect![[r#"
            override a = 1 + 1 + 1;
        "#]],
    );
}

#[test]
pub fn format_override_declaration_simple_statement_with_trailing_comment() {
    check(
        "
        override a_multiline_binding = 1 // The thing
                + 1 // The other thing
                + 7 // The other thing
                // Seperate
                ;

        ",
        expect![["
                override a_multiline_binding = 1 // The thing
                    + 1 // The other thing
                    + 7 // The other thing
                    // Seperate
                    ;
        "]],
    );
}

#[test]
pub fn format_comment_in_override_declaration() {
    check_comments(
        "
        ## override ## a ## = ## 1 ## ; ##
        ",
        expect![[r#"
            /* 0 */
            override /* 1 */ a /* 2 */ = /* 3 */ 1 /* 4 */; /* 5 */
        "#]],
        expect![[r#"
            // 0
            override // 1
                a // 2
                = // 3
                1 // 4
                ; // 5
        "#]],
    );
}

#[test]
pub fn format_override_declaration_keeps_line_comment_in_same_place_same_line() {
    check(
        "

        override a = 1; // Assign 1


        ",
        expect![[r#"
            override a = 1; // Assign 1
        "#]],
    );
}

#[test]
pub fn format_override_declaration_has_no_space_before_semicolon() {
    check(
        "

        override a = 1 /* A */ ;


        ",
        expect![[r#"
            override a = 1 /* A */;
        "#]],
    );
}

#[test]
pub fn format_override_declaration_with_type() {
    check(
        "

        override a
        :
        u32
        =
        1;


        ",
        expect![[r#"
            override a: u32 = 1;
        "#]],
    );
}

#[test]
pub fn format_override_declaration_with_complex_type() {
    check(
        "

        override a
        :
        array
        <
        u32
        ,
        28
        >
        = 1;


        ",
        expect![[r#"
            override a: array<u32, 28> = 1;
        "#]],
    );
}

#[test]
pub fn format_override_declaration_with_comments_in_complex_type() {
    check_comments(
        "
            ## override ## a ## : ## array ## < ## u32 ## , ## 28 ## > ## = ## 1 ## ; ##
        ",
        expect![[r#"
            /* 0 */
            override /* 1 */ a /* 2 */: /* 3 */ array /* 4 */ <
                    /* 5 */ u32, /* 6 */ /* 7 */
                    28, /* 8 */
                > /* 9 */ = /* 10 */ 1 /* 11 */; /* 12 */
        "#]],
        expect![[r#"
            // 0
            override // 1
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
        "#]],
    );
}
