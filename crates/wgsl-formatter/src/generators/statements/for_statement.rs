use dprint_core::formatting::Arena;
use dprint_core_macros::sc;
use syntax::{
    AstNode as _, SyntaxKind, SyntaxNode,
    ast::{self, CompoundStatement, Expression, Statement},
};

use crate::{
    ast_parse::{
        DiscardBlankspace, NoTrivia, StopAtNewline, Succeeding, parse_end, parse_node_with,
        syntax_iter,
    },
    generators::node::gen_node_with_trivia,
    multiline_group::MultilineGroup,
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem},
    },
    reporting::FormatDocumentResult,
};

pub(crate) fn gen_for_statement<'buffer>(
    arena: &'buffer Arena,
    statement: &ast::ForStatement,
) -> FormatDocumentResult<PrintItemBuffer<'buffer>> {
    // ==== Parse ====
    let mut syntax = syntax_iter(statement.syntax());
    let item_for = parse_node_with(&mut syntax, DiscardBlankspace).expect_kind(SyntaxKind::For)?;
    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::ParenthesisLeft)?;
    let item_initializer = parse_node_with(&mut syntax, DiscardBlankspace)
        .only_if_kind(SyntaxKind::ForInitializer, &mut syntax);
    let item_semicolon_1 = parse_node_with(&mut syntax, Succeeding(StopAtNewline))
        .expect_kind(SyntaxKind::Semicolon)?;
    let item_condition = parse_node_with(&mut syntax, DiscardBlankspace)
        .only_if_kind(SyntaxKind::ForCondition, &mut syntax);
    let item_semicolon_2 = parse_node_with(&mut syntax, Succeeding(StopAtNewline))
        .expect_kind(SyntaxKind::Semicolon)?;
    let item_continuing = parse_node_with(&mut syntax, DiscardBlankspace)
        .only_if_kind(SyntaxKind::ForContinuingPart, &mut syntax);

    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::ParenthesisRight)?;
    let item_body =
        parse_node_with(&mut syntax, DiscardBlankspace).expect_ast_node::<CompoundStatement>()?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new(arena);
    formatted.extend(arena, gen_node_with_trivia(arena, &item_for)?);
    formatted.push_sc(arena, sc!("("));
    formatted.request(Request::discourage(RequestItem::EmptyLine));
    formatted.request(Request::discourage(RequestItem::Space));

    let mut multiline_group = MultilineGroup::new_before_requests(arena, &mut formatted);
    multiline_group.start_indent_before_requests(arena);

    multiline_group.grouped_newline_or_space();

    if let Some(item_initializer) = item_initializer {
        multiline_group.extend(arena, gen_node_with_trivia(arena, &item_initializer)?);
    } else {
        multiline_group.request(Request::discourage(RequestItem::Space));
    }
    multiline_group.request(Request::discourage(RequestItem::Space));
    multiline_group.extend(arena, gen_node_with_trivia(arena, &item_semicolon_1)?);

    multiline_group.grouped_newline_or_space();
    if let Some(item_condition) = item_condition {
        multiline_group.extend(arena, gen_node_with_trivia(arena, &item_condition)?);
    } else {
        multiline_group.request(Request::discourage(RequestItem::Space));
    }
    multiline_group.request(Request::discourage(RequestItem::Space));
    multiline_group.extend(arena, gen_node_with_trivia(arena, &item_semicolon_2)?);

    multiline_group.grouped_newline_or_space();
    if let Some(item_continuing) = item_continuing {
        multiline_group.extend(arena, gen_node_with_trivia(arena, &item_continuing)?);
    } else {
        multiline_group.request(Request::discourage(RequestItem::Space));
    }
    multiline_group.request(Request::discourage(RequestItem::Space));

    multiline_group.grouped_newline_or_space();

    multiline_group.finish_indent_before_requests(arena);
    multiline_group.request(Request::discourage(RequestItem::Space));

    multiline_group.push_sc(arena, sc!(")"));

    multiline_group.end_before_requests(arena);

    formatted.request(Request::expect(RequestItem::Space));
    formatted.extend(arena, gen_node_with_trivia(arena, &item_body)?);
    Ok(formatted)
}

pub(crate) fn gen_for_statement_initializer<'buffer>(
    arena: &'buffer Arena,
    node: &ast::SyntaxNode,
) -> FormatDocumentResult<PrintItemBuffer<'buffer>> {
    // === Parse ===
    let mut syntax = syntax_iter(node.syntax());
    let item_statement =
        parse_node_with(&mut syntax, DiscardBlankspace).expect_ast_node::<Statement>()?;
    parse_end(&mut syntax)?;

    // === Format ===
    gen_node_with_trivia(arena, &item_statement)
}

pub(crate) fn gen_for_statement_condition<'buffer>(
    arena: &'buffer Arena,
    node: &ast::SyntaxNode,
) -> FormatDocumentResult<PrintItemBuffer<'buffer>> {
    // === Parse ===
    let mut sub_syntax = syntax_iter(node.syntax());
    let item_condition =
        parse_node_with(&mut sub_syntax, DiscardBlankspace).expect_ast_node::<Expression>()?;
    parse_end(&mut sub_syntax)?;

    // === Format ===
    gen_node_with_trivia(arena, &item_condition)
}

pub(crate) fn gen_for_statement_continuing_part<'buffer>(
    arena: &'buffer Arena,
    node: &ast::SyntaxNode,
) -> FormatDocumentResult<PrintItemBuffer<'buffer>> {
    let mut syntax = syntax_iter(node.syntax());

    // item_continuing is either a ast::Statement or a ast::FunctionCall
    let item_continuing = parse_node_with(&mut syntax, DiscardBlankspace);
    parse_end(&mut syntax)?;

    gen_node_with_trivia(arena, &item_continuing)
}

#[must_use]
pub(crate) fn skip_semicolons_rule(node: &SyntaxNode) -> bool {
    let Some(parent) = node.parent() else {
        return false;
    };
    matches!(
        parent.kind(),
        SyntaxKind::ForInitializer | SyntaxKind::ForCondition | SyntaxKind::ForContinuingPart
    )
}
