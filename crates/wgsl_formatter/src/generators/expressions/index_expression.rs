use parser::{SyntaxKind, SyntaxNode};
use syntax::{
    AstNode as _,
    ast::{self},
};

use crate::{
    ast_parse::{IgnoreBlankspace, NoTrivia, parse_end, parse_node_with, syntax_iter},
    generators::node::gen_node_with_trivia,
    multiline_group::MultilineGroup,
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem},
    },
    reporting::FormatDocumentResult,
};

pub fn gen_index_expression(
    index_expression: &ast::IndexExpression
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = syntax_iter(index_expression.syntax());
    let item_array_expr =
        parse_node_with(&mut syntax, IgnoreBlankspace).expect_castable_kind::<ast::Expression>()?;
    let item_bracket_left =
        parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::BracketLeft)?;
    let item_actual_index =
        parse_node_with(&mut syntax, IgnoreBlankspace).expect_castable_kind::<ast::Expression>()?;
    let item_bracket_right =
        parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::BracketRight)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();

    formatted.extend(gen_node_with_trivia(&item_array_expr)?);

    let mut multiline_group = MultilineGroup::new_before_requests(&mut formatted);

    multiline_group.extend(gen_node_with_trivia(&item_bracket_left)?);
    multiline_group.start_indent_before_requests();
    multiline_group.grouped_possible_newline();
    multiline_group.request(Request::discourage(RequestItem::EmptyLine));
    multiline_group.request(Request::discourage(RequestItem::Space));

    multiline_group.extend(gen_node_with_trivia(&item_actual_index)?);
    multiline_group.grouped_newline_or_space();

    multiline_group.finish_indent();
    multiline_group.request(Request::discourage(RequestItem::Space));

    multiline_group.extend(gen_node_with_trivia(&item_bracket_right)?);

    multiline_group.end_before_requests();

    Ok(formatted)
}

pub fn remove_index_expression_nested_parens_rule(node: &SyntaxNode) -> bool {
    let Some(parent) = node.parent() else {
        return false;
    };

    matches!(parent.kind(), SyntaxKind::IndexExpression)
}
