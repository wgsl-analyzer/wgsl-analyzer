mod line_spacing;

use std::rc::Rc;

use dprint_core::formatting::{
    ConditionResolver, ConditionResolverContext, LineNumber, condition_helpers,
};
use parser::{SyntaxNode, SyntaxToken};
use rowan::NodeOrToken;

use crate::format::{
    ast_parse::SyntaxIter,
    print_item_buffer::{PrintItemBuffer, SeparationPolicy, SeparationRequest},
    reporting::FormatDocumentResult,
};

pub use line_spacing::*;

/// In cases where the formatter is not yet complete we simply output source verbatim.
#[deprecated]
#[expect(
    clippy::unnecessary_wraps,
    reason = "Should follow the api of gen_* methods"
)]
pub fn todo_verbatim(source: &parser::SyntaxNode) -> FormatDocumentResult<PrintItemBuffer> {
    let mut items = PrintItemBuffer::default();

    for line in source.to_string().split_inclusive('\n') {
        if line.ends_with('\n') {
            items.push_string(line[0..(line.len() - 1)].to_owned());
            items.request(SeparationRequest {
                line_break: SeparationPolicy::Forced,
                ..Default::default()
            });
        }
        items.push_string(line.to_owned());
    }
    Ok(items)
}

pub fn create_is_multiple_lines_resolver(
    start_ln: LineNumber,
    end_ln: LineNumber,
) -> ConditionResolver {
    Rc::new(
        move |condition_context: &mut ConditionResolverContext<'_, '_>| {
            // // no items, so format on the same line
            // if child_positions.is_empty() {
            //   return Some(false);
            // }
            // // first child is on a different line than the start of the parent
            // // so format all the children as multi-line
            // if parent_position.line_number < child_positions[0].line_number {
            //   return Some(true);
            // }

            // check if it spans multiple lines, and if it does then make it multi-line
            condition_helpers::is_multiple_lines(condition_context, start_ln, end_ln)
        },
    )
}
