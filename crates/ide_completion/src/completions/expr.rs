use hir_def::{
    module_data::{pretty::pretty_module_item, ModuleItem, Name},
    resolver::ScopeDef,
};
use hir_ty::{builtins::Builtin, ty::pretty::pretty_type};

use crate::{
    context::{CompletionContext, ImmediateLocation},
    item::{CompletionItem, CompletionItemKind, CompletionRelevance},
};

use super::Completions;

pub(crate) fn complete_names_in_scope(
    acc: &mut Completions,
    ctx: &CompletionContext,
) -> Option<()> {
    match ctx.completion_location {
        Some(ImmediateLocation::InsideStatement) => {}
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
            ScopeDef::ModuleItem(_, ModuleItem::Struct(_))
            | ScopeDef::ModuleItem(_, ModuleItem::Import(_)) => {
                unreachable!()
            }
        };

        let detail = match item {
            ScopeDef::Local(local) => ctx
                .container
                .and_then(|def| {
                    let inference = ctx.db.infer(def);
                    inference.type_of_binding.get(local).copied()
                })
                .map(|ty| pretty_type(ctx.db, ty)),
            ScopeDef::ModuleItem(file_id, item) => {
                let module_info = ctx.db.module_info(file_id);
                let detail = pretty_module_item(&item, &module_info, ctx.db.upcast());
                Some(detail)
            }
        };

        let mut completion = CompletionItem::new(kind, ctx.source_range(), name.as_str());
        completion.set_relevance(CompletionRelevance {
            exact_name_match: false,
            type_match: None,
            is_local: matches!(item, ScopeDef::Local(_)),
            exact_postfix_snippet_match: false,
            is_builtin: false,
            swizzle_index: None,
        });
        completion.set_detail(detail);
        completion.add_to(acc);
    });

    acc.add_all(Builtin::ALL_BUILTINS.iter().map(|name| {
        CompletionItem::new(CompletionItemKind::Function, ctx.source_range(), *name)
            .with_relevance(CompletionRelevance {
                exact_name_match: false,
                type_match: None,
                is_local: false,
                exact_postfix_snippet_match: false,
                is_builtin: true,
                swizzle_index: None,
            })
            .build()
    }));

    None
}
