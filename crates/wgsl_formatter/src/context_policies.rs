//! A collection of policies that enable generator function to make decisions about their context.
//!
//! A great example of a use case for context policies is the question whether a statement requires a semicolon to be generated.
//! The generator function for a `for` statement for example expects that the initializer statement does not generate its own semicolon,
//! because the `for` logic handles them itself.
//! However there is nothing fundamentally special about a statement inside the for-initializer, so it would be a bad idea to have the
//! "is this statement inside a for initializer" question colocated with the statement logic - utterly disconnected from the implementation
//! detail in the `for`-generator that was the reason for it.
//!
//! In such a case, we re-route the logic through [`context_policies`](self) that answer neutral statements about some code.
//! The logic for generating statements can very generally ask:
//! [Is it the case that a statement needs a semicolon](statement_needs_semicolon_policy)?
//! And if yes, generate the semicolon and otherwise omit it.
//!
//! The [`statement_needs_semicolon_policy`] can then delegate to the [`for_statement::skip_semicolons_rule`] to answer that question,
//! which is colocated with the implementation details about the for-initializer.
//!
//! If some other generator now also requires semicolon-less statements, a new rule is easily added to the policy, and as such everything is neat and tidy
//! and the spaghetti monster stays hungry.
//!
//! [`statement_needs_semicolon_policy`]: statement_needs_semicolon_policy
//! [`for_statement::skip_semicolons_rule`]: crate::generators::statements::for_statement::skip_semicolons_rule
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
