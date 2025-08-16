use crate::test_util::{check, check_tabs};
use expect_test::expect;

#[test]
fn format_function_call() {
    check(
        "fn main() {
    min  (  x,y );
}",
        expect![["
                fn main() {
                    min(x, y);
                }"]],
    );
}

#[test]
fn format_function_call_newline() {
    check(
        "fn main() {
    min  (
        x,y );
}",
        expect![["
            fn main() {
                min(
                    x, y
                );
            }"]],
    );
}

#[test]
fn format_function_call_newline_indent() {
    check(
        "fn main() {
    if (false) {
        min  (
            x,y );
    }
}",
        expect![["
            fn main() {
                if false {
                    min(
                        x, y
                    );
                }
            }"]],
    );
}

#[test]
fn format_function_call_newline_nested() {
    check(
        "fn main() {
    min(
        min(
            1,
            2,
        )
    )
}",
        expect![["
            fn main() {
                min(
                    min(
                        1,
                        2,
                    )
                )
            }"]],
    );
}

#[test]
fn format_function_call_2() {
    check(
        "fn main() {
    vec3  <f32>  (  x,y,z );
}",
        expect![["
                fn main() {
                    vec3<f32>(x, y, z);
                }"]],
    );
}
