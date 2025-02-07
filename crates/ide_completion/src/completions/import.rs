use super::Completions;
use crate::{
	context::{CompletionContext, ImmediateLocation},
	item::{CompletionItem, CompletionItemKind},
};

pub(crate) fn complete_import(
	acc: &mut Completions,
	ctx: &CompletionContext,
) -> Option<()> {
	match &ctx.completion_location {
		Some(ImmediateLocation::Import) => {},
		_ => return None,
	};

	let custom_imports = ctx.db.custom_imports();
	let imports = custom_imports.keys().map(|import| {
		CompletionItem::new(
			CompletionItemKind::Module,
			ctx.source_range(),
			import.to_string(),
		)
		.build()
	});
	acc.add_all(imports);

	Some(())
}
