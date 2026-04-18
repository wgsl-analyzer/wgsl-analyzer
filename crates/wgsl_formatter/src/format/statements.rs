use syntax::ast;

use crate::format::{
    print_item_buffer::PrintItemBuffer,
    reporting::FormatDocumentError,
    statements::{
        gen_assignment::{
            gen_assignment_statement, gen_compound_assignment_statement,
            gen_phony_assignment_statement,
        },
        gen_break::gen_break_statement,
        gen_break_if::gen_break_if_statement,
        gen_compound::gen_compound_statement,
        gen_const_assert::gen_const_assert_statement,
        gen_continue::gen_continue_statement,
        gen_continuing::gen_continuing_statement,
        gen_discard::gen_discard_statement,
        gen_for::gen_for_statement,
        gen_function_call::gen_function_call_statement,
        gen_if::gen_if_statement,
        gen_increment_decrement::gen_increment_decrement_statement,
        gen_loop::gen_loop_statement,
        gen_return::gen_return_statement,
        gen_switch::gen_switch_statement,
        gen_var_let_const_override::{
            gen_const_declaration_statement, gen_let_declaration_statement,
            gen_var_declaration_statement,
        },
        gen_while::gen_while_statement,
    },
};

pub mod gen_assignment;
pub mod gen_break;
pub mod gen_break_if;
pub mod gen_compound;
pub mod gen_const_assert;
pub mod gen_continue;
pub mod gen_continuing;
pub mod gen_discard;
pub mod gen_for;
pub mod gen_function_call;
pub mod gen_if;
pub mod gen_import;
pub mod gen_increment_decrement;
pub mod gen_loop;
pub mod gen_return;
pub mod gen_statement;
pub mod gen_switch;
pub mod gen_var_let_const_override;
pub mod gen_while;

pub fn gen_statement_maybe_semicolon(
    item: &ast::Statement,
    // TODO Consider absorbing semicolon handling into PrintItemBuffer,
    // passing around random flags is bad, as it leads to spaghetti code and if
    // one gen_* function forgets that it would need the flag, that will lead to weird
    // corner cases bugs. Things like
    // PrintItemBuffer::request_semicolon() and PrintItemBuffer::forbid_semicolon()
    // would be much better. But there are many design questions if the PIB has to handle
    // more than just spaces, and i don't know about all the use cases until the formatter is
    // done.
    include_semicolon: bool,
) -> Result<PrintItemBuffer, FormatDocumentError> {
    match item {
        ast::Statement::IfStatement(if_statement) => gen_if_statement(if_statement),
        ast::Statement::SwitchStatement(switch_statement) => gen_switch_statement(switch_statement),
        ast::Statement::LoopStatement(loop_statement) => gen_loop_statement(loop_statement),
        ast::Statement::ForStatement(for_statement) => gen_for_statement(for_statement),
        ast::Statement::WhileStatement(while_statement) => gen_while_statement(while_statement),
        ast::Statement::CompoundStatement(compound_statement) => {
            gen_compound_statement(compound_statement)
        },
        ast::Statement::FunctionCallStatement(function_call_statement) => {
            gen_function_call_statement(function_call_statement, include_semicolon)
        },
        ast::Statement::VariableDeclaration(variable_declaration) => {
            gen_var_declaration_statement(variable_declaration, include_semicolon)
        },
        ast::Statement::LetDeclaration(let_declaration) => {
            gen_let_declaration_statement(let_declaration, include_semicolon)
        },
        ast::Statement::ConstantDeclaration(constant_declaration) => {
            gen_const_declaration_statement(constant_declaration, include_semicolon)
        },
        ast::Statement::AssignmentStatement(assignment_statement) => {
            gen_assignment_statement(assignment_statement, include_semicolon)
        },
        ast::Statement::PhonyAssignmentStatement(phony_assignment_statement) => {
            gen_phony_assignment_statement(phony_assignment_statement, include_semicolon)
        },
        ast::Statement::CompoundAssignmentStatement(compound_assignment_statement) => {
            gen_compound_assignment_statement(compound_assignment_statement, include_semicolon)
        },
        ast::Statement::IncrementDecrementStatement(increment_decrement_statement) => {
            gen_increment_decrement_statement(increment_decrement_statement, include_semicolon)
        },
        ast::Statement::ContinuingStatement(continuing_statement) => {
            gen_continuing_statement(continuing_statement)
        },
        ast::Statement::ReturnStatement(return_statement) => {
            gen_return_statement(return_statement, include_semicolon)
        },
        ast::Statement::BreakStatement(break_statement) => {
            gen_break_statement(break_statement, include_semicolon)
        },
        ast::Statement::ContinueStatement(continue_statement) => {
            gen_continue_statement(continue_statement, include_semicolon)
        },
        ast::Statement::DiscardStatement(discard) => {
            gen_discard_statement(discard, include_semicolon)
        },
        ast::Statement::AssertStatement(assert_statement) => {
            gen_const_assert_statement(assert_statement, include_semicolon)
        },
        ast::Statement::BreakIfStatement(break_if_statement) => {
            gen_break_if_statement(break_if_statement, include_semicolon)
        },
    }
}
