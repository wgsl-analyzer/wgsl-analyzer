use parser::SyntaxNode;

use crate::generators::{
    expressions::{index_expression, parenthesis_expression},
    statements::{
        assignment_statement, break_if_statement, const_assert_statement, for_statement,
        if_statement, return_statement, switch_statement, var_let_const_override_statement,
        while_statement,
    },
};

#[must_use]
pub fn statement_needs_semicolon_policy(node: &SyntaxNode) -> bool {
    !for_statement::skip_semicolons_rule(node)
}

#[must_use]
pub fn expression_parens_are_irrelevant_policy(node: &SyntaxNode) -> bool {
    if_statement::remove_if_condition_parens_rule(node)
        || switch_statement::remove_switch_subject_parens_rule(node)
        || while_statement::remove_while_condition_parens_rule(node)
        || return_statement::remove_return_value_parens_rule(node)
        || break_if_statement::remove_break_if_condition_parens_rule(node)
        || assignment_statement::remove_assignment_statement_parens_rule(node)
        || const_assert_statement::remove_const_assert_condition_parens_rule(node)
        || parenthesis_expression::remove_nested_parens_rule(node)
        || index_expression::remove_index_expression_nested_parens_rule(node)
}

#[must_use]
pub fn collapse_one_liner_compound_statement_policy(node: &SyntaxNode) -> bool {
    switch_statement::collapse_one_liner_case_body_rule(node)
}

#[must_use]
pub fn template_must_be_one_line_policy(node: &SyntaxNode) -> bool {
    var_let_const_override_statement::template_must_be_on_one_line_rule(node)
}
