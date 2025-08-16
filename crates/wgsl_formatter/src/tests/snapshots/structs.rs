use crate::test_util::{check, check_tabs};
use expect_test::expect;

#[test]
fn format_struct() {
    check(
        "
 struct  Test  {}
",
        expect![["
                struct Test {}
            "]],
    );
}

#[test]
fn format_struct_body() {
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
