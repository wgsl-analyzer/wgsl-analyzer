use expect_test::expect;

use crate::test_util::{check, check_tabs};

//TODO Remove this comment == MODULES THAT FULLY PASS ==
mod bevy_reference;
mod comments;

//TODO Remove this comment == MODULES THAT STILL CONTAIN FAILING TESTS ==
mod bindings;
mod code_indentation;
mod code_spacing;
mod common_conventions;
mod control_structures;
mod fn_call;
mod fn_signature;
mod operators;
mod structs;

#[test]
fn format_empty() {
    check("", expect![[""]]);
}
