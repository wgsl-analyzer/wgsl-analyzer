use std::collections::BTreeSet;

use dprint_core_macros::sc;
use itertools::put_back;
use parser::SyntaxKind;
use syntax::{AstNode as _, ast};

use crate::{
    ast_parse::{parse_end, parse_token_optional},
    generators::gen_comments::{Comment, gen_comment, parse_comment_optional},
    print_item_buffer::{PrintItemBuffer, request_folder::Request},
    reporting::FormatDocumentResult,
};

pub fn gen_path(path: &ast::Path) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(path.syntax().children_with_tokens());

    enum PathItem {
        Identifier(ast::SyntaxToken),
        Comment(Comment),
        ColonColon,
    }

    let mut items = Vec::new();

    #[expect(clippy::redundant_pattern_matching, reason = "Looks neater")]
    loop {
        if let Some(identifier) = parse_token_optional(&mut syntax, SyntaxKind::Identifier) {
            items.push(PathItem::Identifier(identifier));
        } else if let Some(_) = parse_token_optional(&mut syntax, SyntaxKind::ColonColon) {
            items.push(PathItem::ColonColon);
        } else if let Some(_) = parse_token_optional(&mut syntax, SyntaxKind::Blankspace) {
            // We ignore blankspace
        } else if let Some(comment) = parse_comment_optional(&mut syntax) {
            items.push(PathItem::Comment(comment));
        } else {
            break;
        }
    }

    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();

    for item in items {
        match item {
            PathItem::Comment(comment) => {
                formatted.extend(gen_comment(&comment));
            },
            PathItem::Identifier(syntax_token) => {
                formatted.push_string(syntax_token.text().to_owned());
            },
            PathItem::ColonColon => {
                formatted.start_indent();
                formatted.start_new_line_group();
                formatted.request(Request::Unconditional {
                    expected: BTreeSet::new(),
                    discouraged: BTreeSet::new(),
                    forced: BTreeSet::new(),
                    suggest_linebreak: true,
                });
                formatted.push_sc(sc!("::"));
                formatted.finish_new_line_group();
                formatted.finish_indent();
            },
        }
    }
    Ok(formatted)
}
