use itertools::put_back;
use parser::SyntaxKind;
use syntax::{
    AstNode as _,
    ast::{self, Item},
};

use crate::format::{
    ast_parse::{parse_end, parse_node_optional, parse_token_optional},
    gen_comments::{Comment, gen_comment, parse_comment_optional},
    gen_function::gen_function_declaration,
    gen_statement::gen_const_assert_statement,
    gen_struct::gen_struct_declaration,
    gen_type_alias_declaration::gen_type_alias_declaration,
    gen_var_let_const_override_statement::{
        gen_const_declaration_statement, gen_override_declaration_statement,
        gen_var_declaration_statement,
    },
    helpers::{LineSpacing, gen_line_spacing, line_spacing, todo_verbatim_wesl},
    print_item_buffer::{PrintItemBuffer, SeparationPolicy, SeparationRequest},
    reporting::FormatDocumentResult,
};

fn gen_item(node: &Item) -> FormatDocumentResult<PrintItemBuffer> {
    match node {
        Item::FunctionDeclaration(function_declaration) => {
            gen_function_declaration(function_declaration)
        },
        Item::StructDeclaration(struct_declaration) => gen_struct_declaration(struct_declaration),
        Item::VariableDeclaration(variable_declaration) => {
            gen_var_declaration_statement(variable_declaration, true)
        },
        Item::ConstantDeclaration(constant_declaration) => {
            gen_const_declaration_statement(constant_declaration, true)
        },
        Item::OverrideDeclaration(override_declaration) => {
            gen_override_declaration_statement(override_declaration, true)
        },
        Item::TypeAliasDeclaration(type_alias_declaration) => {
            gen_type_alias_declaration(type_alias_declaration, true)
        },
        Item::AssertStatement(assert_statement) => {
            gen_const_assert_statement(assert_statement, true)
        },
        Item::ImportStatement(import_statement) => todo_verbatim_wesl(import_statement.syntax()),
    }
}

pub fn gen_source_file(node: &ast::SourceFile) -> FormatDocumentResult<PrintItemBuffer> {
    enum SourceFileItem {
        Item(Item),
        Comment(Comment),
        LineSpacing(LineSpacing),
    }

    // ==== Parse ====

    let mut syntax = put_back(node.syntax().children_with_tokens());

    let mut items = Vec::new();
    // TODO(MonaMayrhofer) This is basically duplicated code from compound statement, and the user would
    // expect them to behave similarly so they should be combined.
    loop {
        if let Some(spacing) = line_spacing(&mut syntax) {
            items.push(SourceFileItem::LineSpacing(spacing));
        } else if let Some(_statement) = parse_token_optional(&mut syntax, SyntaxKind::Blankspace) {
            // If its not a line_spacing blankspace, then we simply discard it
        } else if let Some(_statement) = parse_token_optional(&mut syntax, SyntaxKind::Semicolon) {
            // Top level semicolons, like after struct defs
        } else if let Some(item) = parse_node_optional::<Item>(&mut syntax) {
            items.push(SourceFileItem::Item(item));
        } else if let Some(comment) = parse_comment_optional(&mut syntax) {
            items.push(SourceFileItem::Comment(comment));
        } else {
            break;
        }
    }

    parse_end(&mut syntax)?;

    // ==== Format ====

    let mut formatted = PrintItemBuffer::new();
    formatted.request(SeparationRequest::discouraged());

    for item in items {
        match item {
            SourceFileItem::Item(item) => {
                // Every item should start on a new line.
                formatted.expect_line_break();
                formatted.extend(gen_item(&item)?);
            },
            SourceFileItem::Comment(comment) => {
                formatted.extend(gen_comment(&comment));
            },
            SourceFileItem::LineSpacing(line_spacing) => {
                formatted.extend(gen_line_spacing(&line_spacing)?);
            },
        }
    }

    //There should be a newline, but no empty line at the end of the file
    formatted.request(SeparationRequest {
        empty_line: SeparationPolicy::Discouraged,
        line_break: SeparationPolicy::Expected,
        ..Default::default()
    });

    Ok(formatted)
}
