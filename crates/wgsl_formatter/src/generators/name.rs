use itertools::put_back;
use syntax::{AstNode as _, ast};

use crate::{
    ast_parse::{parse_end, parse_token},
    print_item_buffer::PrintItemBuffer,
    reporting::FormatDocumentResult,
};

pub fn gen_name(name: &ast::Name) -> FormatDocumentResult<PrintItemBuffer> {
    let mut syntax = put_back(name.syntax().children_with_tokens());
    let identifier = parse_token(&mut syntax, ast::SyntaxKind::Identifier)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    formatted.push_string(identifier.text().to_owned());
    Ok(formatted)
}
