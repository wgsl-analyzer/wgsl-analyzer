use expect_test::expect;

use crate::test_util::{check, check_tabs};

//TODO Remove this comment == MODULES THAT FULLY PASS ==
mod comments;
mod expr_spacing;
mod exprs_field;
mod exprs_function_call;
pub mod exprs_index;
mod exprs_infix;
mod exprs_paren;
mod exprs_prefix;
mod exprs_trivial;
mod fn_body;
mod fn_signature;
mod statement_spacing;
mod stmt_continue;
mod stmt_discard;
mod stmt_let_declarations;
mod stmt_var_declarations;
mod stmt_while;
mod struct_def;

//TODO Remove this comment == MODULES THAT STILL CONTAIN FAILING TESTS ==
//mod discard_statement;
//mod bevy_reference;
//mod bindings;
//mod code_indentation;
//mod code_spacing;
//mod common_conventions;
//mod control_structures;
//mod fn_call;
//mod operators;

#[test]
fn format_empty() {
    check("", expect![[""]]);
}
