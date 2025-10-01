use expect_test::expect;

use crate::test_util::{check, check_tabs};

//TODO Remove this comment == MODULES THAT FULLY PASS ==
mod comments;
mod expr_spacing;
mod exprs_trivial;
mod fn_body;
mod fn_signature;
mod infix_exprs;
mod let_and_var_declarations;
mod paren_exprs;
mod statement_spacing;
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
