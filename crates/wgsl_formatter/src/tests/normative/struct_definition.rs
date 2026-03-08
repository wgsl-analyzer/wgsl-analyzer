use expect_test::expect;

use crate::test_util::{assert_out_of_scope, check};

#[test]
fn format_struct_definition_empty() {
    assert_out_of_scope(
        "struct Foo {
        }",
        "Wgsl disallows empty structs.",
    );
}

#[test]
fn format_struct_definition_members_get_split_on_separate_lines() {
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
fn format_struct_definition_comments_keep_their_prefix_position() {
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
fn format_struct_definition_members_keep_spacing_with_at_most_one_empty_line() {
    check(
        "struct Foo {
            a: i32,
            b: i32,

            c:i32,


            d:i32,
        }",
        expect![[r#"
            struct Foo {
                a: i32,
                b: i32,

                c: i32,

                d: i32,
            }
        "#]],
    );
}

#[test]
fn format_struct_definition_has_no_leading_empty_lines() {
    check(
        "struct Foo {


            a: i32,
        }",
        expect![[r#"
            struct Foo {
                a: i32,
            }
        "#]],
    );
}

#[test]
fn format_struct_definition_has_no_trailing_empty_lines() {
    check(
        "struct Foo {
            a: i32,


        }",
        expect![[r#"
            struct Foo {
                a: i32,
            }
        "#]],
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
fn format_struct_member_spacing_with_line_comments() {
    check(
        "struct Foo {
            // This comment describes A
            a: i32,
            b: i32,

            // This comment describes C
            // This comment also describes C
            c:i32,

            d:i32, // This comment describes d
            e:i32, // This comment describes e

            // A lonesome comment?

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

                d: i32, // This comment describes d
                e: i32, // This comment describes e

                // A lonesome comment?

                d: i32,
            }
        "#]],
    );
}

#[test]
fn format_struct_member_spacing_with_block_comments() {
    check(
        "struct Foo {
            /* This comment describes A */
            a: i32,
            b: i32,

            /* This comment describes C */
            /* This comment also describes C */
            c:i32,

            d:i32, /* This comment describes d */
            e:i32, /* This comment describes e */

            /* A lonesome comment? */

            d: i32
        }",
        expect![[r#"
            struct Foo {
                /* This comment describes A */
                a: i32,
                b: i32,

                /* This comment describes C */
                /* This comment also describes C */
                c: i32,

                d: i32, /* This comment describes d */
                e: i32, /* This comment describes e */

                /* A lonesome comment? */

                d: i32,
            }
        "#]],
    );
}

#[test]
fn format_struct_definition_removes_trailing_semicolon() {
    check(
        "
        struct VertexInput {
            @builtin(vertex_index) vertexIndex: u32,
        };
        ",
        expect![[r#"
            struct VertexInput {
                @builtin(vertex_index)
                vertexIndex: u32,
            }
        "#]],
    );
}

#[test]
fn format_struct_definition_adds_comma_after_last_member() {
    check(
        "
        struct VertexInput {vertexIndex: u32};
        ",
        expect![[r#"
            struct VertexInput {
                vertexIndex: u32,
            }
        "#]],
    );
}
