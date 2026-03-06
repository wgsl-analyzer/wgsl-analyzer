use parser::SyntaxNode;
use syntax::{AstNode, ast::SourceFile};

use crate::format::{
    gen_source_file::gen_source_file, print_item_buffer::PrintItemBuffer,
    reporting::FormatDocumentResult,
};

pub fn gen_node(node: &SyntaxNode) -> FormatDocumentResult<PrintItemBuffer> {
    // TODO Do this better. These clones are all unnecessary.
    if let Some(source_file) = SourceFile::cast(node.clone()) {
        gen_source_file(&source_file)
    } else {
        todo!();
    }
}
