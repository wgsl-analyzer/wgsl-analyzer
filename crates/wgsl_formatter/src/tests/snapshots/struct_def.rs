use crate::test_util::{assert_out_of_scope, check, check_comments, check_tabs};
use expect_test::expect;

#[test]
fn format_struct_def_empty() {
    assert_out_of_scope(
        "struct Foo {
        }",
        "Wgsl disallows empty structs.",
    );
}

#[test]
fn format_struct_def_members_1() {
    check(
        "struct Foo {a: i32}",
        expect![["
                struct Foo {
                    a: i32,
                }
                "]],
    );
}

#[test]
fn format_struct_def_members_2() {
    check(
        "struct Foo {a: i32,b:i32}",
        expect![["
                struct Foo {
                    a: i32,
                    b: i32,
                }
                "]],
    );
}

#[test]
fn format_struct_def_garbled_1() {
    check(
        "struct

        Foo

        {

        a

        :

        i32

        ,

        b

        :

        i32

        }
        ",
        expect![["
                struct Foo {
                    a: i32,
                    b: i32,
                }
                "]],
    );
}

#[test]
fn format_struct_def_members_with_attributes() {
    check(
        "
        struct  Test
        {  @location(0) @attribute(1) x: i32,                    a: i32,
        b: f32,

                }",
        expect![["
            struct Test {
                @location(0)
                @attribute(1)
                x: i32,
                a: i32,
                b: f32,
            }
        "]],
    );
}

#[test]
fn format_comments_in_struct_def_members_2() {
    check_comments(
        "## struct ## Foo ## { ## a ## : ## i32 ## , ## b ## : ## i32 ## }",
        expect![[r#"
            /* 0 */
            struct /* 1 */ Foo /* 2 */ {
                /* 3 */
                a: /* 4 */ /* 5 */ i32, /* 6 */ /* 7 */
                b: /* 8 */ /* 9 */ i32, /* 10 */
            }
        "#]],
        expect![[r#"
            // 0
            struct // 1
            Foo // 2
            {
                // 3
                a: // 4
                // 5
                i32, // 6
                // 7
                b: // 8
                // 9
                i32, // 10
            }
        "#]],
    );
}

#[test]
fn format_comments_in_struct_def_members_with_attributes() {
    check_comments(
        "
        ## struct ## Test ##
        { ##  @location ## ( ## 0 ## ) ## @attribute ## ( ## 1 ## ) ## x ## : ## i32 ## , ## a ## : ## i32 ## , ## b ## : ## f32 ## ,

                }",
        expect![["
        "]],
        expect![["
        "]],
    );
}
