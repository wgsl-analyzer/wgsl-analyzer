use hir::HirDatabase as _;
use hir_def::{
    database::{DefDatabase as _, DefinitionWithBodyId, InternDatabase as _, Location},
    item_tree::{ItemTreeNode, ModuleItem, Name},
    resolver::ScopeDef,
};
use hir_ty::{
    builtins::Builtin,
    ty::pretty::{
        TypeVerbosity, pretty_fn, pretty_fn_with_verbosity, pretty_type, pretty_type_with_verbosity,
    },
};
use syntax::{AstNode as _, AstToken as _, Direction, SyntaxNode, ast};

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

            let detail = match item {
                ScopeDef::Local(local) => context
                    .container
                    .and_then(hir::ChildContainer::as_def_with_body_id)
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

            let doc = match item {
                ScopeDef::ModuleItem(file_id, ref module_item) => {
                    render_doc_comments(context, file_id, module_item)
                },
                ScopeDef::Local(_) => None,
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
            completion.set_documentation(doc);

            // Add function call snippet with parameter placeholders
            if let ScopeDef::ModuleItem(file_id, ModuleItem::Function(id)) = item {
                if let Some(callable) = &context.config.callable {
                    let function_id =
                        context.database.intern_function(Location::new(file_id, id));
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
            let module_info = context.database.item_tree(file_id);
            format!("struct {}", module_info.get(id).name.as_str())
        },
        ModuleItem::GlobalVariable(id) => {
            let variable_id = context
                .database
                .intern_global_variable(Location::new(file_id, id));
            let variable_type = context
                .database
                .infer(DefinitionWithBodyId::GlobalVariable(variable_id));

            let module_info = context.database.item_tree(file_id);
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

            let module_info = context.database.item_tree(file_id);
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

            let module_info = context.database.item_tree(file_id);
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
            let module_info = context.database.item_tree(file_id);
            format!("alias {}", module_info.get(id).name.as_str())
        },
        ModuleItem::GlobalAssertStatement(_) => {
            // const_asserts don't have a name or binding, and will probably never be autocompleted - or will their
            // details have to be rendered. We implement this anyways to achieve consistency.
            String::from("const_assert ...")
        },
        ModuleItem::ImportStatement(_) => {
            // TODO: Support import statements somehow https://github.com/wgsl-analyzer/wgsl-analyzer/issues/632
            String::new()
        },
    }
}


/// Extract doc comments from the AST node of a module item.
fn render_doc_comments(
    context: &CompletionContext<'_>,
    file_id: hir_def::HirFileId,
    item: &ModuleItem,
) -> Option<String> {
    let item_tree = context.database.item_tree(file_id);
    let ast_id_map = context.database.ast_id_map(file_id);
    let root = context.database.parse_or_resolve(file_id);

    // Get the syntax node for this item via its ast_id
    let syntax_node: SyntaxNode = match item {
        ModuleItem::Function(id) => {
            let node = item_tree.get(*id);
            let ptr = ast_id_map.get(node.ast_id());
            ptr.to_node(&root.syntax()).syntax().clone()
        },
        ModuleItem::Struct(id) => {
            let node = item_tree.get(*id);
            let ptr = ast_id_map.get(node.ast_id());
            ptr.to_node(&root.syntax()).syntax().clone()
        },
        ModuleItem::GlobalVariable(id) => {
            let node = item_tree.get(*id);
            let ptr = ast_id_map.get(node.ast_id());
            ptr.to_node(&root.syntax()).syntax().clone()
        },
        ModuleItem::GlobalConstant(id) => {
            let node = item_tree.get(*id);
            let ptr = ast_id_map.get(node.ast_id());
            ptr.to_node(&root.syntax()).syntax().clone()
        },
        ModuleItem::Override(id) => {
            let node = item_tree.get(*id);
            let ptr = ast_id_map.get(node.ast_id());
            ptr.to_node(&root.syntax()).syntax().clone()
        },
        ModuleItem::TypeAlias(id) => {
            let node = item_tree.get(*id);
            let ptr = ast_id_map.get(node.ast_id());
            ptr.to_node(&root.syntax()).syntax().clone()
        },
        ModuleItem::GlobalAssertStatement(_) | ModuleItem::ImportStatement(_) => return None,
    };

    doc_comments_from_syntax(&syntax_node)
}

/// Extracts doc comments (`///`) from the preceding siblings of a syntax node.
fn doc_comments_from_syntax(node: &SyntaxNode) -> Option<String> {
    let mut doc_lines: Vec<String> = Vec::new();

    for sibling in node.siblings_with_tokens(Direction::Prev).skip(1) {
        if let Some(token) = sibling.as_token() {
            if let Some(comment) = ast::Comment::cast(token.clone()) {
                if let Some(doc_text) = comment.doc_comment() {
                    let text = doc_text.strip_prefix(' ').unwrap_or(doc_text);
                    doc_lines.push(text.to_string());
                    continue;
                }
            }
            if token.kind().is_whitespace() {
                continue;
            }
            break;
        } else {
            break;
        }
    }

    if doc_lines.is_empty() {
        return None;
    }

    doc_lines.reverse();
    Some(doc_lines.join("\n"))
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