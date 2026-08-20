use expect_test::expect;

use crate::test_util::{CheckOptions, assert_out_of_scope, check, check_with_options};

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
pub fn format_fn_body_puts_block_comment_on_separate_line() {
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
pub fn format_fn_header_keep_comments_in_position() {
    // Following "the formatter should not unnecessarily move comments around" - if programmer wants them there, we will let them have it.
    check(
        "
            fn main(
a: u32 /* after a */,
b: u32, /* after b */
/*before c*/ c: u32,
/*line before d*/
d: u32,

                // line before e
                e: u32,

                f: u32, // after f

                g: u32
                // line after g
) {}
        ",
        expect![[r#"
            fn main(
                a: u32, /* after a */
                b: u32, /* after b */
                /*before c*/ c: u32,
                /*line before d*/
                d: u32,
                // line before e
                e: u32,
                f: u32, // after f
                g: u32,
                // line after g
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

#[test]
fn format_fn_header_parameter_with_long_type_and_name() {
    check(
        "
        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
        fn main(
            a: u32,
            aaaaaaaaaaaaaaaaaaaa: looooooooooooooooooooooooooooooooooooooooooooooooong<parameter>,
            b: u32,
        ) {}
        ",
        expect![[r#"
            //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
            fn main(
                a: u32,
                aaaaaaaaaaaaaaaaaaaa: looooooooooooooooooooooooooooooooooooooooooooooooong<
                    parameter,
                >,
                b: u32,
            ) {}
        "#]],
    );
}

#[test]
pub fn format_type_next_to_long_parameter_does_not_get_broken_into_multiple_lines() {
    check_with_options(
        "
        fn a(
        a: texture_2d<f32>,
        bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb: f32
        ) {}
        ",
        &expect![[r#"
            fn a(
                a: texture_2d<f32>,
                bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb: f32,
            ) {}
        "#]],
        &CheckOptions {
            assert_line_width: None,
            ..Default::default()
        },
        parser::Edition::LATEST
    );
}
