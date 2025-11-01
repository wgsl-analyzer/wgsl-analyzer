use hir::HirDatabase as _;
use hir_def::{
    database::{DefDatabase as _, DefinitionWithBodyId, InternDatabase as _, Location},
    module_data::{ModuleItem, Name},
    resolver::ScopeDef,
};
use hir_ty::{
    builtins::Builtin,
    ty::pretty::{
        TypeVerbosity, pretty_fn_with_verbosity, pretty_type, pretty_type_with_verbosity,
    },
};

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

    context.resolver.process_all_names(|name, item| {
        if name == Name::missing() {
            return;
        }
        let kind = match item {
            ScopeDef::ModuleItem(_, ModuleItem::Function(_)) => CompletionItemKind::Function,
            ScopeDef::ModuleItem(_, ModuleItem::GlobalVariable(_)) | ScopeDef::Local(_) => {
                CompletionItemKind::Variable
            },
            ScopeDef::ModuleItem(_, ModuleItem::GlobalConstant(_) | ModuleItem::Override(_)) => {
                CompletionItemKind::Constant
            },
            ScopeDef::ModuleItem(_, ModuleItem::Struct(_)) => CompletionItemKind::Struct,
            ScopeDef::ModuleItem(_, ModuleItem::TypeAlias(_)) => CompletionItemKind::TypeAlias,
        };

        let detail = match item {
            ScopeDef::Local(local) => context
                .container
                .and_then(|container| container.as_def_with_body_id())
                .map(|definition| {
                    let inference = context.database.infer(definition);
                    inference[local]
                })
                .map(|r#type| pretty_type(context.database, r#type)),
            ScopeDef::ModuleItem(file_id, item) => {
                let detail = render_detail(context, file_id, item);
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

fn render_detail(
    context: &CompletionContext<'_>,
    file_id: hir_def::HirFileId,
    item: ModuleItem,
) -> String {
    match item {
        ModuleItem::Function(id) => {
            let function_id = context.database.intern_function(Location::new(file_id, id));
            let function_type = context.database.function_type(function_id);

            pretty_fn_with_verbosity(
                context.database,
                &function_type.lookup(context.database),
                TypeVerbosity::Compact,
            )
        },
        ModuleItem::Struct(id) => {
            let module_info = context.database.module_info(file_id);
            format!("struct {}", module_info.get(id).name.as_str())
        },
        ModuleItem::GlobalVariable(id) => {
            let variable_id = context
                .database
                .intern_global_variable(Location::new(file_id, id));
            let variable_type = context
                .database
                .infer(DefinitionWithBodyId::GlobalVariable(variable_id));

            let module_info = context.database.module_info(file_id);
            format!(
                "var {}: {}",
                module_info.get(id).name.as_str(),
                pretty_type_with_verbosity(
                    context.database,
                    variable_type.return_type(),
                    TypeVerbosity::Compact
                )
            )
        },
        ModuleItem::GlobalConstant(id) => {
            let constant_id = context
                .database
                .intern_global_constant(Location::new(file_id, id));
            let constant_type = context
                .database
                .infer(DefinitionWithBodyId::GlobalConstant(constant_id));

            let module_info = context.database.module_info(file_id);
            format!(
                "const {}: {}",
                module_info.get(id).name.as_str(),
                pretty_type_with_verbosity(
                    context.database,
                    constant_type.return_type(),
                    TypeVerbosity::Compact
                )
            )
        },
        ModuleItem::Override(id) => {
            let override_id = context.database.intern_override(Location::new(file_id, id));
            let override_type = context
                .database
                .infer(DefinitionWithBodyId::Override(override_id));

            let module_info = context.database.module_info(file_id);
            format!(
                "override {}: {}",
                module_info.get(id).name.as_str(),
                pretty_type_with_verbosity(
                    context.database,
                    override_type.return_type(),
                    TypeVerbosity::Compact
                )
            )
        },
        ModuleItem::TypeAlias(id) => {
            let module_info = context.database.module_info(file_id);
            format!("alias {}", module_info.get(id).name.as_str())
        },
    }
}
