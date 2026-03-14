use ide_db::wgsl_attributes::WGSL_ATTRIBUTES;

use super::Completions;
use crate::{
    context::{CompletionContext, ImmediateLocation},
    item::{CompletionItem, CompletionItemKind},
};

pub(crate) fn complete_attributes(
    accumulator: &mut Completions,
    context: &CompletionContext<'_>,
) -> Option<()> {
    match context.completion_location {
        Some(ImmediateLocation::Attribute) => {},
        _ => return None,
    }

    for attr in WGSL_ATTRIBUTES {
        let mut builder =
            CompletionItem::new(CompletionItemKind::Keyword, context.source_range(), attr.name);
        builder.detail(attr.syntax);
        builder.set_documentation(Some(attr.description.to_owned()));
        builder.add_to(accumulator, context.database);
    }

    Some(())
}
