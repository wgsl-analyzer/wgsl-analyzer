use hir::HirDatabase;
use hir_def::{
    db::DefDatabase,
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
    ctx: &CompletionContext,
) -> Option<()> {
    match ctx.completion_location {
        Some(ImmediateLocation::InsideStatement) => {},
        _ => return None,
    }

    ctx.resolver.process_value_names(|name, item| {
        if name == Name::missing() {
            return;
        }
        let kind = match item {
            ScopeDef::Local(_) => CompletionItemKind::Variable,
            ScopeDef::ModuleItem(_, ModuleItem::Function(_)) => CompletionItemKind::Function,
            ScopeDef::ModuleItem(_, ModuleItem::GlobalVariable(_)) => CompletionItemKind::Variable,
            ScopeDef::ModuleItem(_, ModuleItem::GlobalConstant(_)) => CompletionItemKind::Constant,
            ScopeDef::ModuleItem(_, ModuleItem::Override(_)) => CompletionItemKind::Constant,
            ScopeDef::ModuleItem(_, ModuleItem::Struct(_))
            | ScopeDef::ModuleItem(_, ModuleItem::TypeAlias(_))
            | ScopeDef::ModuleItem(_, ModuleItem::Import(_)) => {
                unreachable!()
            },
        };

        let detail = match item {
            ScopeDef::Local(local) => ctx
                .container
                .and_then(|def| {
                    let inference = ctx.db.infer(def);
                    inference.type_of_binding.get(local).copied()
                })
                .map(|r#type| pretty_type(ctx.db, r#type)),
            ScopeDef::ModuleItem(file_id, item) => {
                let module_info = ctx.db.module_info(file_id);
                let detail = pretty_module_item(&item, &module_info, ctx.db);
                Some(detail)
            },
        };

        let mut completion = CompletionItem::new(kind, ctx.source_range(), name.as_str());
        completion.set_relevance(CompletionRelevance {
            exact_name_match: false,
            type_match: None,
            is_local: matches!(item, ScopeDef::Local(_)),
            postfix_match: None,
            is_builtin: false,
            swizzle_index: None,
            function: None,
            is_name_already_imported: false,
            requires_import: false,
            is_private_editable: false,
            is_skipping_completion: false,
        });
        completion.set_detail(detail);
        completion.add_to(accumulator, ctx.db);
    });
    accumulator.add_all(Builtin::ALL_BUILTINS.iter().map(|name| {
        let mut builder =
            CompletionItem::new(CompletionItemKind::Function, ctx.source_range(), *name);
        builder.with_relevance(|r| CompletionRelevance {
            exact_name_match: false,
            type_match: None,
            is_local: false,
            postfix_match: None,
            is_builtin: true,
            swizzle_index: None,
            ..r
        });
        builder.build(ctx.db)
    }));
    None
}
