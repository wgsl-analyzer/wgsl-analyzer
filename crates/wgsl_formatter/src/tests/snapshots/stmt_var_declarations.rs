use expect_test::expect;

use crate::test_util::{check, check_comments};

#[test]
pub fn format_var_decl_simple_literal_1() {
    check(
        "fn main() {
        var
        a
        =
        1
        ;
        }",
        expect![["
            fn main() {
                var a = 1;
            }
        "]],
    );
}

#[test]
pub fn format_var_decl_simple_statement_1() {
    check(
        "fn main() {
        var
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
                var a = 1 + 1 + 1;
            }
        "]],
    );
}

#[test]
fn format_var_decl_with_simple_type() {
    check(
        "fn main() {var x   : u32=0;}",
        expect![[r#"
            fn main() {
                var x: u32 = 0;
            }
        "#]],
    );
}

#[test]
pub fn format_var_decl_simple_statement_with_trailing_comment() {
    check(
        "fn main() {
        var a_multiline_binding = 1 // The thing
                + 1 // The other thing
                + 7 // The other thing
                // Seperate
                ;

        }",
        expect![["
            fn main() {
                var a_multiline_binding = 1 // The thing
                    + 1 // The other thing
                    + 7 // The other thing
                    // Seperate
                    ;
            }
        "]],
    );
}

#[test]
pub fn format_comments_in_var_decl() {
    check_comments(
        "fn main() {
        ## var ## a ## = ## 1 ## ; ##
        }",
        expect![[r#"
            fn main() {
                /* 0 */
                var /* 1 */ a /* 2 */ = /* 3 */ 1 /* 4 */; /* 5 */
            }
        "#]],
        expect![[r#"
            fn main() {
                // 0
                var // 1
                    a // 2
                    = // 3
                    1 // 4
                    ; // 5
            }
        "#]],
    );
}

#[test]
pub fn format_var_decl_keeps_line_comment_in_same_place_same_line() {
    check(
        "fn main() {

        var a = 1; // Assign 1


        }",
        expect![["
            fn main() {
                var a = 1; // Assign 1
            }
        "]],
    );
}

#[test]
pub fn format_var_decl_has_no_space_before_semicolon() {
    check(
        "fn main() {

        var a = 1 /* A */ ;


        }",
        expect![["
            fn main() {
                var a = 1 /* A */;
            }
        "]],
    );
}

#[test]
pub fn format_global_var_decl_simple_literal_1() {
    check(
        "
        var
        a
        =
        1
        ;
        ",
        expect![[r#"
            var a = 1;
        "#]],
    );
}

#[test]
pub fn format_global_var_decl_with_type_1() {
    check(
        "
        var
        a: array<u32, 28>
        =
        make_array()
        ;
        ",
        expect![[r#"
            var a: array<u32, 28> = make_array();
        "#]],
    );
}

#[test]
pub fn format_global_var_decl_with_address_space_and_type_1() {
    check(
        "
        var<workgroup>
        a: array<u32, 28>
        ;
        ",
        expect![[r#"
            var<workgroup> a: array<u32, 28>;
        "#]],
    );
}

#[test]
pub fn format_global_var_decl_with_address_space_and_type_2() {
    check(
        "
        var<storage,read_write>
        a: array<u32, 28>
        ;
        ",
        expect![[r#"
            var<storage, read_write> a: array<u32, 28>;
        "#]],
    );
}
