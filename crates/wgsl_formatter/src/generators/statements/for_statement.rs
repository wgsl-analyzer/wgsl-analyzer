use dprint_core_macros::sc;
use itertools::put_back;
use parser::SyntaxKind;
use syntax::{
    AstNode as _,
    ast::{self, CompoundStatement, Expression, Statement},
};

use crate::{
    ast_parse::{parse_end, parse_node, parse_node_by_kind_optional, parse_token},
    generators::{
        expressions::gen_expression,
        gen_attributes::{AttributeLayout, gen_attributes, parse_many_attributes},
        gen_comments::{gen_comments, parse_many_comments_and_blankspace},
        statements::{compound_statement::gen_compound_statement, gen_statement_maybe_semicolon},
    },
    multiline_group::MultilineGroup,
    print_item_buffer::{PrintItemBuffer, request_folder::RequestItem},
    reporting::FormatDocumentResult,
};

pub fn gen_for_statement(statement: &ast::ForStatement) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(statement.syntax().children_with_tokens());
    let item_attributes = parse_many_attributes(&mut syntax)?;
    parse_token(&mut syntax, SyntaxKind::For)?;
    let comments_after_for = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, SyntaxKind::ParenthesisLeft)?;
    let comments_after_open_paren = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_initializer = parse_node_by_kind_optional(&mut syntax, SyntaxKind::ForInitializer);
    let comments_after_initializer = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, SyntaxKind::Semicolon)?;
    let comments_after_initializer_semicolon = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_condition = parse_node_by_kind_optional(&mut syntax, SyntaxKind::ForCondition);
    let comments_after_condition = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, SyntaxKind::Semicolon)?;
    let comments_after_condition_semicolon = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_continuing = parse_node_by_kind_optional(&mut syntax, SyntaxKind::ForContinuingPart);
    let comments_after_continuing = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, SyntaxKind::ParenthesisRight)?;
    let comments_after_close_paren = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_body = parse_node::<CompoundStatement>(&mut syntax)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    formatted.extend(gen_attributes(
        &item_attributes,
        AttributeLayout::Multiline,
    )?);
    formatted.push_sc(sc!("for"));
    formatted.extend(gen_comments(&comments_after_for));
    formatted.push_sc(sc!("("));

    let mut multiline_group = MultilineGroup::new(&mut formatted);
    multiline_group.start_indent();

    multiline_group.extend(gen_comments(&comments_after_open_paren));

    multiline_group.grouped_newline_or_space();
    if let Some(item_initializer) = item_initializer {
        multiline_group.extend(gen_for_statement_initializer(&item_initializer)?);
    } else {
        multiline_group.discourage(RequestItem::Space);
    }
    multiline_group.extend(gen_comments(&comments_after_initializer));
    multiline_group.discourage(RequestItem::Space);
    multiline_group.push_sc(sc!(";"));
    multiline_group.extend(gen_comments(&comments_after_initializer_semicolon));

    multiline_group.grouped_newline_or_space();
    if let Some(item_condition) = item_condition {
        multiline_group.extend(gen_for_statement_condition(&item_condition)?);
    } else {
        multiline_group.discourage(RequestItem::Space);
    }
    multiline_group.extend(gen_comments(&comments_after_condition));
    multiline_group.discourage(RequestItem::Space);
    multiline_group.push_sc(sc!(";"));
    multiline_group.extend(gen_comments(&comments_after_condition_semicolon));

    multiline_group.grouped_newline_or_space();
    if let Some(item_continuing) = item_continuing {
        multiline_group.extend(gen_for_statement_continuing_part(&item_continuing)?);
    } else {
        multiline_group.discourage(RequestItem::Space);
    }
    multiline_group.extend(gen_comments(&comments_after_continuing));
    multiline_group.discourage(RequestItem::Space);

    multiline_group.grouped_newline_or_space();

    multiline_group.finish_indent();

    multiline_group.push_sc(sc!(")"));

    multiline_group.end();

    formatted.expect(RequestItem::Space);
    formatted.extend(gen_comments(&comments_after_close_paren));
    formatted.extend(gen_compound_statement(&item_body)?);
    Ok(formatted)
}

pub fn gen_for_statement_initializer(
    node: &ast::SyntaxNode
) -> FormatDocumentResult<PrintItemBuffer> {
    // === Parse ===
    let mut syntax = put_back(node.syntax().children_with_tokens());
    let item_statement = parse_node::<Statement>(&mut syntax)?;
    parse_end(&mut syntax)?;

    // === Format ===
    gen_statement_maybe_semicolon(&item_statement, false)
}

pub fn gen_for_statement_condition(
    node: &ast::SyntaxNode
) -> FormatDocumentResult<PrintItemBuffer> {
    // === Parse ===
    let mut sub_syntax = put_back(node.syntax().children_with_tokens());
    let item_condition = parse_node::<Expression>(&mut sub_syntax)?;
    parse_end(&mut sub_syntax)?;

    // === Format ===
    gen_expression(&item_condition, false)
}

pub fn gen_for_statement_continuing_part(
    node: &ast::SyntaxNode
) -> FormatDocumentResult<PrintItemBuffer> {
    let mut sub_syntax = put_back(node.syntax().children_with_tokens());
    let item_continuing = parse_node::<Statement>(&mut sub_syntax)?;
    parse_end(&mut sub_syntax)?;

    gen_statement_maybe_semicolon(&item_continuing, false)
}
