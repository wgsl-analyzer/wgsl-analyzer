use expect_test::expect;

use crate::test_util::{assert_out_of_scope, check};

#[test]
pub fn format_fn_body_collapses_empty_body() {
    check(
        "fn main() {



        }",
        expect![["
            fn main() {}
        "]],
    );
}

/// This is debatable. For now it seems like a sane way to do it this way, and it causes less edge cases.
#[test]
pub fn format_fn_body_puts_block_comment_on_seperate_line() {
    check(
        "fn main() {/* Hello */}",
        expect![["
            fn main() {
                /* Hello */
            }
        "]],
    );
}

#[test]
fn format_fn_header_unfinished_return_1() {
    assert_out_of_scope("fn  main ( a :  b )       ->     {}", "Not valid wgsl.");
}

#[test]
fn format_fn_header_malformed_return_2() {
    assert_out_of_scope("fn  main ( a :  b )  u32  {}", "Not valild wgsl.");
}

#[test]
fn format_fn_header_missing_comma() {
    assert_out_of_scope(
        "fn main(a: b  c: d) {}",
        "We don't try to guess missing commas.",
    );
}

#[test]
fn format_fn_header_with_parameters_spacing() {
    check(
        "fn main(
            // This comment describes A
            a: u32,
            // This comment describes B
            b: u32,

            c: u32,



            d: u32
        ) {}",
        expect![[r#"
            fn main(
                // This comment describes A
                a: u32,
                // This comment describes B
                b: u32,
                c: u32,
                d: u32,
            ) {}
        "#]],
    );
}

#[test]
fn format_fn_header_with_parameters_inline_line_comments() {
    check(
        "fn main(
            a: u32, // This comment describes B
            b: u32, // This comment describes C

            // This comment describes C
            c: u32,
            // This comment describes D
            d: u32
        ) {}",
        expect![[r#"
            fn main(
                a: u32, // This comment describes B
                b: u32, // This comment describes C

                // This comment describes C
                c: u32,
                // This comment describes D
                d: u32,
            ) {}
        "#]],
    );
}

#[test]
fn format_fn_header_with_blockcomment_after_last_parameter() {
    // This unit check exists, because at some point the formatter mistakenly put
    // commas after the last parameter
    // fn main(a: b, /*fff*/) -> f32 {}
    check(
        "fn main (a: b /*fff*/) -> f32 {}",
        expect![[r#"
            fn main(a: b /*fff*/) -> f32 {}
        "#]],
    );
}

#[test]
fn format_fn_header_with_linecomment_after_last_parameter() {
    check(
        "fn main (a: b // Hi
        ) -> f32 {}",
        expect![[r#"
            fn main(
                a: b, // Hi
            ) -> f32 {}
        "#]],
    );
}

// TODO Tests for how function parameters are broken up onto multiple lines
