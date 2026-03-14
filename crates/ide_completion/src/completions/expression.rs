use hir::{Definition, HirDatabase as _};
use hir_def::{
    database::{DefDatabase as _, InternDatabase as _, Location},
    item_tree::{ModuleItem, Name},
    resolver::ScopeDef,
};
use hir_ty::{builtins::Builtin, ty::pretty::pretty_fn};

use crate::config::CallableSnippets;

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
            if name == Name::missing() {
                return;
            }
            let kind = match item {
                ScopeDef::ModuleItem(_, ModuleItem::Function(_)) => CompletionItemKind::Function,
                ScopeDef::ModuleItem(_, ModuleItem::GlobalVariable(_)) | ScopeDef::Local(_) => {
                    CompletionItemKind::Variable
                },
                ScopeDef::ModuleItem(
                    _,
                    ModuleItem::GlobalConstant(_) | ModuleItem::Override(_),
                ) => CompletionItemKind::Constant,
                ScopeDef::ModuleItem(_, ModuleItem::Struct(_)) => CompletionItemKind::Struct,
                ScopeDef::ModuleItem(_, ModuleItem::TypeAlias(_)) => CompletionItemKind::TypeAlias,
                ScopeDef::ModuleItem(_, ModuleItem::ImportStatement(_)) => {
                    // TODO: Resolve the import statement, and then set the correct CompletionItemKind from there
                    // TODO: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/632
                    CompletionItemKind::Module
                },
                ScopeDef::ModuleItem(_, ModuleItem::GlobalAssertStatement(_)) => {
                    return;
                },
            };

            // Resolve to a Definition for shared detail/doc logic
            let definition = match item {
                ScopeDef::Local(local) => context
                    .container
                    .and_then(hir::ChildContainer::as_def_with_body_id)
                    .and_then(|def_with_body| {
                        if let hir_def::database::DefinitionWithBodyId::Function(func_id) =
                            def_with_body
                        {
                            Some(Definition::Local(hir::Local {
                                parent: func_id,
                                binding: local,
                            }))
                        } else {
                            None
                        }
                    }),
                ScopeDef::ModuleItem(file_id, item) => {
                    Definition::from_module_item(context.database, file_id, item)
                },
            };

            let detail = definition
                .as_ref()
                .and_then(|d| d.detail_text(context.database));
            let doc = definition
                .as_ref()
                .and_then(|d| d.doc_comments(context.database));

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
            completion.set_documentation(doc);

            // Add function call snippet with parameter placeholders
            if let ScopeDef::ModuleItem(file_id, ModuleItem::Function(id)) = item {
                if let Some(callable) = &context.config.callable {
                    let function_id = context.database.intern_function(Location::new(file_id, id));
                    let function_type = context.database.function_type(function_id);
                    let details = function_type.lookup(context.database);
                    let snippet = build_fn_snippet(name.as_str(), &details, callable);
                    completion.insert_text(snippet);
                    completion.mark_as_snippet();
                }
            }

            completion.add_to(accumulator, context.database);
        });
    for name in Builtin::ALL_BUILTINS {
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

        // Look up the builtin to get its signature for the detail string
        if let Some(builtin) = Builtin::for_name(context.database, &Name::from(*name)) {
            if let Some((_, overload)) = builtin.overloads().next() {
                let function_details = overload.r#type.lookup(context.database);
                let detail = pretty_fn(context.database, &function_details);
                builder.set_detail(Some(detail));
            }
        }

        builder.add_to(accumulator, context.database);
    }
    None
}

/// Build a snippet string for a function call.
///
/// - `FillArguments`: `func_name(${1:param1}, ${2:param2})`
/// - `AddParentheses`: `func_name($0)`
fn build_fn_snippet(
    name: &str,
    details: &hir_ty::function::FunctionDetails,
    callable: &CallableSnippets,
) -> String {
    match callable {
        CallableSnippets::AddParentheses => {
            format!("{name}($0)")
        },
        CallableSnippets::FillArguments => {
            let params: Vec<_> = details.parameters_with_names().collect();
            if params.is_empty() {
                format!("{name}()$0")
            } else {
                let param_snippets: Vec<String> = params
                    .iter()
                    .enumerate()
                    .map(|(i, (_, param_name))| {
                        let label = if param_name.is_empty()
                            || hir_def::item_tree::Name::is_missing(param_name)
                        {
                            format!("arg{}", i + 1)
                        } else {
                            param_name.to_string()
                        };
                        format!("${{{}:{}}}", i + 1, label)
                    })
                    .collect();
                format!("{name}({})$0", param_snippets.join(", "))
            }
        },
    }
}
