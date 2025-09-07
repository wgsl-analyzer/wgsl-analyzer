use crate::test_util::{assert_out_of_scope, check, check_tabs};
use expect_test::expect;

struct A {
    a: i32,
    b: i32,
}

#[test]
fn format_struct_def_empty() {
    assert_out_of_scope(
        "struct Foo {
        }",
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
        {  @location(0) x: i32,                    a: i32,
        b: f32,

                }",
        expect![["
            struct Test {
                @location(0) x: i32,
                a: i32,
                b: f32,
            }"]],
    );
}
