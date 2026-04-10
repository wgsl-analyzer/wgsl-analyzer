use crate::test_util::{assert_out_of_scope, check, check_comments, check_tabs};
use expect_test::expect;

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
        expect![[r#"
            struct Foo {
                a: i32,

                b: i32,
            }
        "#]],
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
        expect![[r#"
            struct Test {
                @attribute(1)
                @location(0)
                x: i32,
                a: i32,
                b: f32,
            }
        "#]],
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
        { ##  @attribute ## ( ## 0 ## ) ## @location ## ( ## 1 ## ) ## x ## : ## i32 ## , ## a ## : ## i32 ## , ## b ## : ## f32 ## ,

                }",
        expect![[r#"
            /* 0 */
            struct /* 1 */ Test /* 2 */ {
                /* 3 */
                @attribute /* 4 */ (/* 5 */ 0 /* 6 */) /* 7 */
                @location /* 8 */ (/* 9 */ 1 /* 10 */) /* 11 */
                x: /* 12 */ /* 13 */ i32, /* 14 */ /* 15 */
                a: /* 16 */ /* 17 */ i32, /* 18 */ /* 19 */
                b: /* 20 */ /* 21 */ f32, /* 22 */
            }
        "#]],
        expect![[r#"
            // 0
            struct // 1
            Test // 2
            {
                // 3
                @attribute // 4
                (
                    // 5
                    0, // 6
                ) // 7
                @location // 8
                (
                    // 9
                    1, // 10
                ) // 11
                x: // 12
                // 13
                i32, // 14
                // 15
                a: // 16
                // 17
                i32, // 18
                // 19
                b: // 20
                // 21
                f32, // 22
            }
        "#]],
    );
}

#[test]
fn format_line_comments_on_multiple_struct_members() {
    check(
        "struct Foo {
            // This comment describes A
            a: i32,
            // This comment describes B
            b:i32
        }",
        expect![[r#"
            struct Foo {
                // This comment describes A
                a: i32,
                // This comment describes B
                b: i32,
            }
        "#]],
    );
}

#[test]
fn format_struct_member_comment_simple() {
    check(
        "
        struct A {
        // This comment should stick to the member
        a: i32
        }
        ",
        expect![["
            struct A {
                // This comment should stick to the member
                a: i32,
            }
        "]],
    );
}

#[test]
fn format_struct_member_comment_after_opening_braces() {
    // Following rustfmt, even if the comment is on the same line as the struct
    // we assume it belongs to the member. If the user wanted to comment on the
    // struct, they should put the comment in front of the struct.
    check(
        "
        struct A { // This comment should stick to the member
        a: i32
        }
        ",
        expect![["
            struct A {
                // This comment should stick to the member
                a: i32,
            }
        "]],
    );
}

#[test]
fn format_struct_def_with_rogue_semicolon() {
    check(
        "
        struct VertexInput {
            @builtin(vertex_index) vertexIndex: u32,
        };
        ",
        expect![[r#"
            struct VertexInput {
                @builtin(vertex_index) vertexIndex: u32,
            }
        "#]],
    );
}
