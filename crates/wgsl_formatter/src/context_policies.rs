use parser::SyntaxNode;

use crate::generators::{
    expressions::{index_expression, parenthesis_expression},
    statements::{
        assignment_statement, break_if_statement, const_assert_statement, for_statement,
        if_statement, return_statement, switch_statement, while_statement,
    },
};

pub fn statement_needs_semicolon_policy(node: &SyntaxNode) -> bool {
    !for_statement::skip_semicolons_rule(node)
}

pub fn expression_parens_are_irrelevant_policy(node: &SyntaxNode) -> bool {
    if_statement::remove_if_condition_parens(node)
        || switch_statement::remove_switch_subject_parens(node)
        || while_statement::remove_while_condition_parens(node)
        || return_statement::remove_return_value_parens(node)
        || break_if_statement::remove_break_if_condition_parens(node)
        || assignment_statement::remove_assignment_statement_parens(node)
        || const_assert_statement::remove_const_assert_condition_parens(node)
        || parenthesis_expression::remove_nested_parenthesis(node)
        || index_expression::remove_index_expression_nested_parenthesis(node)
}

pub fn collapse_one_liner_compound_statement_policy(node: &SyntaxNode) -> bool {
    switch_statement::collapse_one_liner_case_body_rule(node)
}
