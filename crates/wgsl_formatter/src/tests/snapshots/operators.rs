use crate::tests::util::{check, check_tabs};
use expect_test::expect;

#[test]
fn format_infix_expression() {
    check(
        "fn main() {
    let a=x+y*z;
}",
        expect![["
            fn main() {
                let a = x + y * z;
            }"]],
    );
}

#[test]
fn format_expression_shift_right() {
    check(
        "fn main() { let x = 1u >> 3u; }",
        expect![["fn main() { let x = 1u >> 3u; }"]],
    );
}

#[test]
fn format_expression_shift_left() {
    check(
        "fn main() { let x = 1u << 3u; }",
        expect![["fn main() { let x = 1u << 3u; }"]],
    );
}

#[test]
fn format_expression_bitcast() {
    check(
        "fn main() { bitcast   <  vec4<u32>  >  ( x+5 ) }",
        expect!["fn main() { bitcast<vec4<u32>>(x + 5) }"],
    );
}
