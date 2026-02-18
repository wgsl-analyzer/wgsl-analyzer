use hir::HirDatabase as _;
use hir_def::{
    database::{
        DefDatabase as _, DefinitionWithBodyId, InternDatabase as _, Location, ModuleDefinitionId,
    },
    item_tree::{ModuleItem, Name},
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

    context
        .resolver
        .process_all_names(context.database, |name, item| {
            if name == &Name::missing() {
                return;
            }
            let kind = match item {
                ScopeDef::ModuleDefinition(ModuleDefinitionId::Module(_)) => {
                    CompletionItemKind::Module
                },
                ScopeDef::ModuleDefinition(ModuleDefinitionId::Function(_)) => {
                    CompletionItemKind::Function
                },
                ScopeDef::ModuleDefinition(ModuleDefinitionId::GlobalVariable(_))
                | ScopeDef::Local(_) => CompletionItemKind::Variable,
                ScopeDef::ModuleDefinition(
                    ModuleDefinitionId::GlobalConstant(_) | ModuleDefinitionId::Override(_),
                ) => CompletionItemKind::Constant,
                ScopeDef::ModuleDefinition(ModuleDefinitionId::Struct(_)) => {
                    CompletionItemKind::Struct
                },
                ScopeDef::ModuleDefinition(ModuleDefinitionId::TypeAlias(_)) => {
                    CompletionItemKind::TypeAlias
                },
                ScopeDef::ModuleDefinition(ModuleDefinitionId::GlobalAssertStatement(_)) => {
                    return;
                },
            };

            let detail = match item {
                ScopeDef::Local(local) => context
                    .container
                    .and_then(hir::ChildContainer::as_def_with_body_id)
                    .map(|definition| {
                        let inference = context.database.infer(definition);
                        inference[local]
                    })
                    .map(|r#type| pretty_type(context.database, r#type)),
                ScopeDef::ModuleDefinition(item) => {
                    let detail = render_detail(context, name, item);
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
    name: &Name,
    item: ModuleDefinitionId,
) -> String {
    let database = context.database;
    match item {
        ModuleDefinitionId::Module(_id) => {
            format!("module {}", name.as_str())
        },
        ModuleDefinitionId::Function(id) => {
            let function_type = database.function_type(id);

            pretty_fn_with_verbosity(
                database,
                &function_type.lookup(database),
                TypeVerbosity::Compact,
            )
        },
        ModuleDefinitionId::Struct(_) => {
            format!("struct {}", name.as_str())
        },
        ModuleDefinitionId::GlobalVariable(id) => {
            let variable_type = database.infer(DefinitionWithBodyId::GlobalVariable(id));

            format!(
                "var {}: {}",
                name.as_str(),
                pretty_type_with_verbosity(
                    database,
                    variable_type.return_type(),
                    TypeVerbosity::Compact
                )
            )
        },
        ModuleDefinitionId::GlobalConstant(id) => {
            let constant_type = database.infer(DefinitionWithBodyId::GlobalConstant(id));

            format!(
                "const {}: {}",
                name.as_str(),
                pretty_type_with_verbosity(
                    database,
                    constant_type.return_type(),
                    TypeVerbosity::Compact
                )
            )
        },
        ModuleDefinitionId::Override(id) => {
            let override_type = database.infer(DefinitionWithBodyId::Override(id));

            format!(
                "override {}: {}",
                name.as_str(),
                pretty_type_with_verbosity(
                    database,
                    override_type.return_type(),
                    TypeVerbosity::Compact
                )
            )
        },
        ModuleDefinitionId::TypeAlias(_) => {
            format!("alias {}", name.as_str())
        },
        ModuleDefinitionId::GlobalAssertStatement(_) => {
            // const_asserts don't have a name or binding, and will probably never be autocompleted - or will their
            // details have to be rendered. We implement this anyways to achieve consistency.
            String::from("const_assert ...")
        },
    }
}
