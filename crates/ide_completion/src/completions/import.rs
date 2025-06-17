use base_db::SourceDatabase;

use super::Completions;
use crate::{
    context::{CompletionContext, ImmediateLocation},
    item::{CompletionItem, CompletionItemKind},
};

pub(crate) fn complete_import(
    accumulator: &mut Completions,
    context: &CompletionContext,
) -> Option<()> {
    match &context.completion_location {
        Some(ImmediateLocation::Import) => {},
        _ => return None,
    };

    let custom_imports = context.database.custom_imports();
    let imports = custom_imports.keys().map(|import| {
        CompletionItem::new(
            CompletionItemKind::Module,
            context.source_range(),
            import.to_string(),
        )
        .build(context.database)
    });
    accumulator.add_all(imports);

    Some(())
}
