use std::fmt::format;

use base_db::Lookup as _;
use hir::HirDatabase as _;
use hir_def::{
    db::{DefinitionWithBodyId, ModuleDefinitionId},
    item_tree::Name,
    resolver::ScopeDef,
};
use hir_ty::{
    infer::InferenceResult,
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
        if name == &Name::missing() {
            return;
        }
        let kind = match &item {
            ScopeDef::Module => CompletionItemKind::Module,
            ScopeDef::ModuleDefinition(ModuleDefinitionId::Function(_)) => {
                CompletionItemKind::Function
            },
            ScopeDef::ModuleDefinition(ModuleDefinitionId::GlobalVariable(_))
            | ScopeDef::Local(_) => CompletionItemKind::Variable,
            ScopeDef::ModuleDefinition(
                ModuleDefinitionId::GlobalConstant(_) | ModuleDefinitionId::Override(_),
            ) => CompletionItemKind::Constant,
            ScopeDef::ModuleDefinition(ModuleDefinitionId::Struct(_)) => CompletionItemKind::Struct,
            ScopeDef::ModuleDefinition(ModuleDefinitionId::TypeAlias(_)) => {
                CompletionItemKind::TypeAlias
            },
            ScopeDef::ModuleDefinition(ModuleDefinitionId::GlobalAssertStatement(_)) => {
                return;
            },
            ScopeDef::BuiltIn(kind) => CompletionItemKind::Builtin(match kind {
                hir_def::resolver::BuiltInKind::Alias(name) => crate::item::BuiltInKind::Alias,
                hir_def::resolver::BuiltInKind::Constructor(name) => {
                    crate::item::BuiltInKind::Constructor
                },
                hir_def::resolver::BuiltInKind::Declaration(name) => {
                    crate::item::BuiltInKind::Declaration
                },
                hir_def::resolver::BuiltInKind::Enumerant(name) => {
                    crate::item::BuiltInKind::Enumerant
                },
                hir_def::resolver::BuiltInKind::Function(name) => {
                    crate::item::BuiltInKind::Function
                },
                hir_def::resolver::BuiltInKind::Struct(name) => crate::item::BuiltInKind::Struct,
                hir_def::resolver::BuiltInKind::TypeGenerator(name) => {
                    crate::item::BuiltInKind::TypeGenerator
                },
                hir_def::resolver::BuiltInKind::Type(name) => crate::item::BuiltInKind::Type,
            }),
        };

        let detail = match item {
            ScopeDef::Local(local) => context
                .container
                .and_then(hir::ChildContainer::as_def_with_body_id)
                .map(|definition| {
                    let inference = InferenceResult::of(context.db, definition);
                    inference[local]
                })
                .map(|r#type| pretty_type(context.db, r#type)),
            ScopeDef::ModuleDefinition(item) => {
                let detail = render_detail(context, name, item);
                Some(detail)
            },
            ScopeDef::Module => Some(format!("path {}", name.as_str())),
            ScopeDef::BuiltIn(_) => None,
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
        completion.add_to(accumulator, context.db);
    });
    None
}

fn render_detail(
    context: &CompletionContext<'_>,
    name: &Name,
    item: ModuleDefinitionId,
) -> String {
    let db = context.db;

    match item {
        ModuleDefinitionId::Function(id) => {
            let function_type = db.function_type(id);
            pretty_fn_with_verbosity(db, function_type.lookup(db), TypeVerbosity::Compact)
        },
        ModuleDefinitionId::Struct(_) => {
            format!("struct {}", name.as_str())
        },
        ModuleDefinitionId::GlobalVariable(id) => {
            let variable_type = InferenceResult::of(db, DefinitionWithBodyId::GlobalVariable(id));
            format!(
                "var {}: {}",
                name.as_str(),
                pretty_type_with_verbosity(db, variable_type.return_type(), TypeVerbosity::Compact)
            )
        },
        ModuleDefinitionId::GlobalConstant(id) => {
            let constant_type = InferenceResult::of(db, DefinitionWithBodyId::GlobalConstant(id));
            format!(
                "const {}: {}",
                name.as_str(),
                pretty_type_with_verbosity(db, constant_type.return_type(), TypeVerbosity::Compact)
            )
        },
        ModuleDefinitionId::Override(id) => {
            let override_type = InferenceResult::of(db, DefinitionWithBodyId::Override(id));
            format!(
                "override {}: {}",
                name.as_str(),
                pretty_type_with_verbosity(db, override_type.return_type(), TypeVerbosity::Compact)
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
