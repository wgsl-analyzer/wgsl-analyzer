use dprint_core_macros::sc;
use itertools::put_back;
use parser::{SyntaxKind, SyntaxNode};
use syntax::{
    AstNode as _,
    ast::{self},
};

use crate::{
    ast_parse::{BareSyntaxKind, UntilSyntaxKind, parse_end, parse_node_with},
    generators::node::gen_node_with_trivia,
    multiline_group::MultilineGroup,
    print_item_buffer::PrintItemBuffer,
    reporting::FormatDocumentResult,
};

pub fn gen_index_expression(
    index_expression: &ast::IndexExpression
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(index_expression.syntax().children_with_tokens());
    let item_array_expr = parse_node_with(&mut syntax, UntilSyntaxKind(SyntaxKind::BracketLeft))
        .expect_castable_kind::<ast::Expression>()?;
    let item_bracket_left = parse_node_with(&mut syntax, BareSyntaxKind(SyntaxKind::BracketLeft))
        .expect_kind(SyntaxKind::BracketLeft)?;
    let item_actual_index = parse_node_with(&mut syntax, UntilSyntaxKind(SyntaxKind::BracketRight))
        .expect_castable_kind::<ast::Expression>()?;
    let item_bracket_right = parse_node_with(&mut syntax, BareSyntaxKind(SyntaxKind::BracketRight))
        .expect_kind(SyntaxKind::BracketRight)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();

    formatted.extend(gen_node_with_trivia(&item_array_expr)?);

    let mut multiline_group = MultilineGroup::new(&mut formatted);

    multiline_group.extend(gen_node_with_trivia(&item_bracket_left)?);

    multiline_group.start_indent();

    multiline_group.extend(gen_node_with_trivia(&item_actual_index)?);
    multiline_group.grouped_newline_or_space();

    multiline_group.finish_indent();

    multiline_group.extend(gen_node_with_trivia(&item_bracket_right)?);

    multiline_group.end();

    Ok(formatted)
}

pub fn remove_index_expression_nested_parens_rule(node: &SyntaxNode) -> bool {
    let Some(parent) = node.parent() else {
        return false;
    };

    matches!(parent.kind(), SyntaxKind::IndexExpression)
}
