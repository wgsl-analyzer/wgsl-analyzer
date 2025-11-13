#![cfg(test)]

use expect_test::expect;

use crate::test_util::{assert_out_of_scope, check, check_comments, check_with_options};

#[test]
fn format_fn_header_with_parameters_1() {
    check(
        "fn        main         (           a    :    b )  -> f32   {}",
        expect![["
            fn main(a: b) -> f32 {}
        "]],
    );
}

#[test]
fn format_fn_header_with_parameters_2() {
    check(
        "fn  main ( a :  b,  c : d )  -> f32   {}",
        expect![["
            fn main(a: b, c: d) -> f32 {}
            "]],
    );
}

#[test]
fn format_fn_header_no_return_1() {
    check(
        "fn  main ( a :  b )    {}",
        expect![["
            fn main(a: b) {}
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
fn format_fn_header_long_name() {
    check(
        "fn  this_is_a_very_long_name_who_knows_when_it_will_end_because_it_just_goes_on_and_on_and_on( a :  b,c:d )  -> f32   {}",
        expect![["
            fn this_is_a_very_long_name_who_knows_when_it_will_end_because_it_just_goes_on_and_on_and_on(
                a: b,
                c: d,
            ) -> f32 {}
            "]],
    );
}

#[test]
fn format_fn_header_comma_oneline() {
    check(
        "fn main(a: b , c: d ,)  -> f32   {}",
        expect![["
            fn main(a: b, c: d) -> f32 {}
            "]],
    );
}

#[test]
fn format_fn_header_comma_multiline_wide() {
    check_with_options(
        "fn main(a: b , c: d ,)  -> f32   {}",
        &expect![["
            fn main(
                a: b,
                c: d,
            ) -> f32 {}
            "]],
        &crate::FormattingOptions {
            width: 26, //Just shy of what the fn would be laid out as on a single line
            ..Default::default()
        },
    );
}

#[test]
fn format_fn_header_comma_multiline_narrow() {
    check_with_options(
        "fn main(a: b , c: d ,)  -> f32   {}",
        &expect![["
            fn main(
                a: b,
                c: d,
            ) -> f32 {}
            "]],
        &crate::FormattingOptions {
            width: 4, //Just shy of what the fn would be laid out as on a single line
            ..Default::default()
        },
    );
}

#[test]
fn format_fn_header_missing_comma() {
    assert_out_of_scope(
        "fn main(a: b  c: d) {}",
        "We don't try to guess missing commas.",
    );
}

#[test]
fn format_fn_header_no_ws() {
    check(
        "fn main(a:b)->f32{}",
        expect![["
            fn main(a: b) -> f32 {}
        "]],
    );
}

#[test]
fn format_fn_newline() {
    check(
        "fn main(
    a:b
)->f32{}",
        expect![["
            fn main(a: b) -> f32 {}
            "]],
    );
}

#[test]
fn format_fn_newline_2() {
    check(
        "fn main(
    a:b, c:d)->f32{}",
        expect![["
            fn main(a: b, c: d) -> f32 {}
            "]],
    );
}

#[test]
fn format_fn_newline_3() {
    check(
        "fn main(
    a:b,
    c:d
)->f32{}",
        expect![["
            fn main(a: b, c: d) -> f32 {}
            "]],
    );
}

#[test]
fn format_multiple_fns() {
    check(
        "
 fn  main( a:  b )  -> f32   {}
  fn  main( a:  b )  -> f32   {}
",
        expect![["
                fn main(a: b) -> f32 {}
                fn main(a: b) -> f32 {}
            "]],
    );
}

#[test]
fn format_fn_header_incomplete() {
    assert_out_of_scope("fn  main ( a ", "We don't try to guess missing code.");
}

#[test]
fn format_comments_in_fn_signature() {
    check_comments(
        "
        ## fn ## main ## ( ## a ## : ## b ## , ## c ## : ## d ## ) ## { ##
        ## }
        ",
        expect![[r#"
            /* 0 */
            fn /* 1 */ main /* 2 */ (
                /* 3 */ a: /* 4 */ /* 5 */ b, /* 6 */ /* 7 */
                c: /* 8 */ /* 9 */ d, /* 10 */
            ) /* 11 */ {
                /* 12 */
                /* 13 */
            }
        "#]],
        expect![[r#"
            // 0
            fn // 1
            main // 2
            (
                // 3
                a: // 4
                // 5
                b, // 6
                // 7
                c: // 8
                // 9
                d, // 10
            ) // 11
            {
                // 12

                // 13
            }
        "#]],
    );
}
