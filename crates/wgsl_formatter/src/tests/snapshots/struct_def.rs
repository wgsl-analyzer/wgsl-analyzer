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
        expect![[r#"
            /* 0 */
            struct /* 1 */ Test /* 2 */ {
                /* 3 */
                @location /* 4 */ (/* 5 */ 0 /* 6 */) /* 7 */
                @attribute /* 8 */ (/* 9 */ 1 /* 10 */) /* 11 */
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
                @location // 4
                (
                    // 5
                    0, // 6
                ) // 7
                @attribute // 8
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
        expect![["
                struct Foo {
                    a: i32,
                    b: i32,
                }
                "]],
    );
}

#[test]
fn format_struct_member_spacing() {
    check(
        "struct Foo {
            // This comment describes A
            a: i32,
            b: i32,

            // This comment describes C
            // This comment also describes C
            c:i32,




            d: i32
        }",
        expect![[r#"
            struct Foo {
                // This comment describes A
                a: i32,
                b: i32,

                // This comment describes C
                // This comment also describes C
                c: i32,

                d: i32,
            }
        "#]],
    );
}
