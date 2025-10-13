use hir::HirDatabase as _;
use hir_def::{
    database::DefDatabase as _,
    module_data::{ModuleItem, Name, pretty::pretty_module_item},
    resolver::ScopeDef,
};
use hir_ty::{builtins::Builtin, ty::pretty::pretty_type};

use super::Completions;
use crate::{
    context::{CompletionContext, ImmediateLocation},
    item::{CompletionItem, CompletionItemKind, CompletionRelevance},
};

pub(crate) fn complete_names_in_scope(
    accumulator: &mut Completions,
    context: &CompletionContext<'_>,
) -> Option<()> {
    match context.completion_location {
        Some(ImmediateLocation::InsideStatement) => {},
        _ => return None,
    }

    context.resolver.process_value_names(|name, item| {
        if name == Name::missing() {
            return;
        }
        #[expect(clippy::unreachable, reason = "TODO")]
        let kind = match item {
            ScopeDef::ModuleItem(_, ModuleItem::Function(_)) => CompletionItemKind::Function,
            ScopeDef::ModuleItem(_, ModuleItem::GlobalVariable(_)) | ScopeDef::Local(_) => {
                CompletionItemKind::Variable
            },
            ScopeDef::ModuleItem(_, ModuleItem::GlobalConstant(_) | ModuleItem::Override(_)) => {
                CompletionItemKind::Constant
            },
            ScopeDef::ModuleItem(_, ModuleItem::Struct(_) | ModuleItem::TypeAlias(_)) => {
                unreachable!()
            },
        };

        let detail = match item {
            ScopeDef::Local(local) => context
                .container
                .map(|definition| {
                    let inference = context.database.infer(definition);
                    inference[local]
                })
                .map(|r#type| pretty_type(context.database, r#type)),
            ScopeDef::ModuleItem(file_id, item) => {
                let module_info = context.database.module_info(file_id);
                let detail = pretty_module_item(item, &module_info, context.database);
                Some(detail)
            },
        };

        let mut completion = CompletionItem::new(kind, context.source_range(), name.as_str());
        completion.set_relevance(CompletionRelevance {
            exact_name_match: false,
            type_match: None,
            is_local: matches!(item, ScopeDef::Local(_)),
            is_name_already_imported: false,
            requires_import: false,
            is_private_editable: false,
            postfix_match: None,
            function: None,
            is_skipping_completion: false,
            is_builtin: false,
        });
        completion.set_detail(detail);
        completion.add_to(accumulator, context.database);
    });
    accumulator.add_all(Builtin::ALL_BUILTINS.iter().map(|name| {
        let mut builder =
            CompletionItem::new(CompletionItemKind::Function, context.source_range(), *name);
        builder.with_relevance(|relevance| CompletionRelevance {
            exact_name_match: false,
            type_match: None,
            is_local: false,
            postfix_match: None,
            is_builtin: true,
            ..relevance
        });
        builder.build(context.database)
    }));
    None
}
