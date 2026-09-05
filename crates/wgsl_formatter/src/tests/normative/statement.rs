use expect_test::expect;

use crate::test_util::check;

mod assert;
mod assignment;
mod break_if;
mod compound_assignment;
mod compound_layout;
mod r#continue;
mod continuing;
mod r#for;
mod function_call;
mod function_call_matrix;
mod r#if;
mod r#loop;
mod r#return;
mod r#switch;
mod unscoped_compound;
mod var_declaration;
mod r#while;

#[test]
fn format_lonely_semicolon_gets_removed() {
    // Poor semicolon
    check(
        "fn main() {
        ;
        }",
        expect![[r#"
            fn main() {}
        "#]],
    );
}
