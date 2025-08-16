#![cfg(test)]

use expect_test::expect;

use crate::tests::util::check;
#[test]
fn format_fn_header() {
    check(
        "fn  main ( a :  b )  -> f32   {}",
        expect![["fn main(a: b) -> f32 {}"]],
    );
}

#[test]
fn format_fn_header_2() {
    check(
        "fn  main ( a :  b,  c : d )  -> f32   {}",
        expect![["fn main(a: b, c: d) -> f32 {}"]],
    );
}

#[test]
fn format_fn_header_comma_oneline() {
    check(
        "fn main(a: b , c: d ,)  -> f32   {}",
        expect![["fn main(a: b, c: d) -> f32 {}"]],
    );
}

#[test]
fn format_fn_header_comma_multiline() {
    check(
        "fn main(
                a: b , c: d ,)  -> f32   {}",
        expect![["
            fn main(
                a: b, c: d,
            ) -> f32 {}"]],
    );
}

#[test]
fn format_fn_header_missing_comma() {
    check(
        "fn main(a: b  c: d) {}",
        expect![["fn main(a: b, c: d) {}"]],
    );
}

#[test]
fn format_fn_header_no_ws() {
    check("fn main(a:b)->f32{}", expect![["fn main(a: b) -> f32 {}"]]);
}

#[test]
fn format_fn_newline() {
    check(
        "fn main(
    a:b
)->f32{}",
        expect![["
            fn main(
                a: b
            ) -> f32 {}"]],
    );
}

#[test]
fn format_fn_newline_2() {
    check(
        "fn main(
    a:b, c:d)->f32{}",
        expect![["
            fn main(
                a: b, c: d
            ) -> f32 {}"]],
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
            fn main(
                a: b,
                c: d
            ) -> f32 {}"]],
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
