use std::rc::Rc;

use dprint_core::formatting::{
    ConditionResolver, ConditionResolverContext, LineNumber, condition_helpers,
};
mod line_spacing;

use itertools::{Itertools as _, Position};
pub use line_spacing::*;

use crate::{print_item_buffer::PrintItemBuffer, reporting::FormatDocumentResult};

use super::print_item_buffer::request_folder::RequestItem;

/// In cases where the formatter is not yet complete we simply output source verbatim.
#[deprecated]
#[expect(
    clippy::unnecessary_wraps,
    reason = "Should follow the api of gen_* methods"
)]
pub fn todo_verbatim_wesl(source: &parser::SyntaxNode) -> FormatDocumentResult<PrintItemBuffer> {
    let mut items = PrintItemBuffer::default();

    for (pos, line) in source.to_string().lines().with_position() {
        items.push_string(line.to_owned());
        if pos != Position::Last && pos != Position::Only {
            items.force(RequestItem::LineBreak);
        }
    }
    Ok(items)
}

#[must_use]
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
