use super::Completions;
use crate::{
    context::{CompletionContext, ImmediateLocation},
    item::{CompletionItem, CompletionItemKind},
};

/// WGSL attributes (used after `@`).
/// See <https://www.w3.org/TR/WGSL/#attribute-names>.
const ATTRIBUTES: &[&str] = &[
    "align",
    "binding",
    "builtin",
    "compute",
    "const",
    "diagnostic",
    "fragment",
    "group",
    "id",
    "interpolate",
    "invariant",
    "location",
    "must_use",
    "size",
    "vertex",
    "workgroup_size",
];

pub(crate) fn complete_attributes(
    accumulator: &mut Completions,
    context: &CompletionContext<'_>,
) -> Option<()> {
    match context.completion_location {
        Some(ImmediateLocation::Attribute) => {},
        _ => return None,
    }

    for attr in ATTRIBUTES {
        CompletionItem::new(CompletionItemKind::Keyword, context.source_range(), *attr)
            .add_to(accumulator, context.database);
    }

    Some(())
}
