//! `completions` crate provides utilities for generating completions of user input.

mod completions;
mod config;
mod context;
pub mod item;
mod patterns;
// mod render;

// mod snippet;
// #[cfg(test)]
// mod tests;

use base_db::FilePosition;
use hir::HirDatabase;
use rustc_hash::FxHashSet;

use crate::{completions::Completions, context::CompletionContext};

pub use crate::{
    config::{AutoImportExclusionType, CallableSnippets, CompletionConfig},
    item::{CompletionItem, CompletionItemKind, CompletionRelevance, CompletionRelevanceTypeMatch},
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct CompletionFieldsToResolve {
    pub resolve_label_details: bool,
    pub resolve_tags: bool,
    pub resolve_detail: bool,
    pub resolve_documentation: bool,
    pub resolve_filter_text: bool,
    pub resolve_text_edit: bool,
    pub resolve_command: bool,
}

impl CompletionFieldsToResolve {
    pub fn from_client_capabilities(client_capability_fields: &FxHashSet<&str>) -> Self {
        Self {
            resolve_label_details: client_capability_fields.contains("labelDetails"),
            resolve_tags: client_capability_fields.contains("tags"),
            resolve_detail: client_capability_fields.contains("detail"),
            resolve_documentation: client_capability_fields.contains("documentation"),
            resolve_filter_text: client_capability_fields.contains("filterText"),
            resolve_text_edit: client_capability_fields.contains("textEdit"),
            resolve_command: client_capability_fields.contains("command"),
        }
    }

    pub const fn empty() -> Self {
        Self {
            resolve_label_details: false,
            resolve_tags: false,
            resolve_detail: false,
            resolve_documentation: false,
            resolve_filter_text: false,
            resolve_text_edit: false,
            resolve_command: false,
        }
    }
}

//FIXME: split the following feature into fine-grained features.

// Feature: Magic Completions
//
// In addition to usual reference completion, rust-analyzer provides some ✨magic✨
// completions as well:
//
// Keywords like `if`, `else` `while`, `loop` are completed with braces, and cursor
// is placed at the appropriate position. Even though `if` is easy to type, you
// still want to complete it, to get ` { }` for free! `return` is inserted with a
// space or `;` depending on the return type of the function.
//
// When completing a function call, `()` are automatically inserted. If a function
// takes arguments, the cursor is positioned inside the parenthesis.
//
// There are postfix completions, which can be triggered by typing something like
// `foo().if`. The word after `.` determines postfix completion. Possible variants are:
//
// - `expression.if` -> `if expression {}` or `if let ... {}` for `Option` or `Result`
// - `expression.match` -> `match expression {}`
// - `expression.while` -> `while expression {}` or `while let ... {}` for `Option` or `Result`
// - `expression.ref` -> `&expression`
// - `expression.refm` -> `&mut expression`
// - `expression.let` -> `let $0 = expression;`
// - `expression.lete` -> `let $1 = expression else { $0 };`
// - `expression.letm` -> `let mut $0 = expression;`
// - `expression.not` -> `!expression`
// - `expression.dbg` -> `dbg!(expression)`
// - `expression.dbgr` -> `dbg!(&expression)`
// - `expression.call` -> `(expression)`
//
// There also snippet completions:
//
// #### Expressions
//
// - `pd` -> `eprintln!(" = {:?}", );`
// - `ppd` -> `eprintln!(" = {:#?}", );`
//
// #### Items
//
// - `tfn` -> `#[test] fn feature(){}`
// - `tmod` ->
// ```rust
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_name() {}
// }
// ```
//
// And the auto import completions, enabled with the `rust-analyzer.completion.autoimport.enable` setting and the corresponding LSP client capabilities.
// Those are the additional completion options with automatic `use` import and options from all project importable items,
// fuzzy matched against the completion input.
//
// ![Magic Completions](https://user-images.githubusercontent.com/48062697/113020667-b72ab880-917a-11eb-8778-716cf26a0eb3.gif)

// /// Main entry point for completion. We run completion as a two-phase process.
// ///
// /// First, we look at the position and collect a so-called `CompletionContext`.
// /// This is a somewhat messy process, because, during completion, syntax tree is
// /// incomplete and can look really weird.
// ///
// /// Once the context is collected, we run a series of completion routines which
// /// look at the context and produce completion items. One subtlety about this
// /// phase is that completion engine should not filter by the substring which is
// /// already present, it should give all possible variants for the identifier at
// /// the caret. In other words, for
// ///
// /// ```ignore
// /// fn f() {
// ///     let foo = 92;
// ///     let _ = bar$0
// /// }
// /// ```
// ///
// /// `foo` *should* be present among the completion variants. Filtering by
// /// identifier prefix/fuzzy match should be done higher in the stack, together
// /// with ordering of completions (currently this is done by the client).
// ///
// /// # Speculative Completion Problem
// ///
// /// There's a curious unsolved problem in the current implementation. Often, you
// /// want to compute completions on a *slightly different* text document.
// ///
// /// In the simplest case, when the code looks like `let x = `, you want to
// /// insert a fake identifier to get a better syntax tree: `let x = complete_me`.
// ///
// /// We do this in `CompletionContext`, and it works OK-enough for *syntax*
// /// analysis. However, we might want to, eg, ask for the type of `complete_me`
// /// variable, and that's where our current infrastructure breaks down. salsa
// /// doesn't allow such "phantom" inputs.
// ///
// /// Another case where this would be instrumental is macro expansion. We want to
// /// insert a fake ident and re-expand code. There's `expand_speculative` as a
// /// workaround for this.
// ///
// /// A different use-case is completion of injection (examples and links in doc
// /// comments). When computing completion for a path in a doc-comment, you want
// /// to inject a fake path expression into the item being documented and complete
// /// that.
// ///
// /// IntelliJ has CodeFragment/Context infrastructure for that. You can create a
// /// temporary PSI node, and say that the context ("parent") of this node is some
// /// existing node. Asking for, eg, type of this `CodeFragment` node works
// /// correctly, as the underlying infrastructure makes use of contexts to do
// /// analysis.
// pub fn completions(
//     db: &RootDatabase,
//     config: &CompletionConfig<'_>,
//     position: FilePosition,
//     trigger_character: Option<char>,
// ) -> Option<Vec<CompletionItem>> {
//     let (ctx, analysis) = &CompletionContext::new(db, position, config)?;
//     let mut completions = Completions::default();

//     // prevent `(` from triggering unwanted completion noise
//     if trigger_character == Some('(') {
//         if let CompletionAnalysis::NameReference(NameReferenceContext {
//             kind:
//                 NameReferenceKind::Path(
//                     path_ctx @ PathCompletionCtx { kind: PathKind::Vis { has_in_token }, .. },
//                 ),
//             ..
//         }) = analysis
//         {
//             completions::vis::complete_vis_path(&mut completions, ctx, path_ctx, has_in_token);
//         }
//         return Some(completions.into());
//     }

//     // when the user types a bare `_` (that is it does not belong to an identifier)
//     // the user might just wanted to type a `_` for type inference or pattern discarding
//     // so try to suppress completions in those cases
//     if trigger_character == Some('_') && ctx.original_token.kind() == syntax::SyntaxKind::Underscore
//     {
//         if let CompletionAnalysis::NameReference(NameReferenceContext {
//             kind:
//                 NameReferenceKind::Path(
//                     path_ctx @ PathCompletionCtx {
//                         kind: PathKind::Type { .. } | PathKind::Pat { .. },
//                         ..
//                     },
//                 ),
//             ..
//         }) = analysis
//         {
//             if path_ctx.is_trivial_path() {
//                 return None;
//             }
//         }
//     }

//     {
//         let accumulator = &mut completions;

//         match analysis {
//             CompletionAnalysis::Name(name_ctx) => completions::complete_name(accumulator, ctx, name_ctx),
//             CompletionAnalysis::NameReference(name_ref_ctx) => {
//                 completions::complete_name_ref(accumulator, ctx, name_ref_ctx)
//             }
//             CompletionAnalysis::Lifetime(lifetime_ctx) => {
//                 completions::lifetime::complete_label(accumulator, ctx, lifetime_ctx);
//                 completions::lifetime::complete_lifetime(accumulator, ctx, lifetime_ctx);
//             }
//             CompletionAnalysis::String { original, expanded: Some(expanded) } => {
//                 completions::extern_abi::complete_extern_abi(accumulator, ctx, expanded);
//                 completions::format_string::format_string(accumulator, ctx, original, expanded);
//                 completions::env_vars::complete_cargo_env_vars(accumulator, ctx, original, expanded);
//             }
//             CompletionAnalysis::UnexpandedAttributeTT {
//                 colon_prefix,
//                 fake_attribute_under_caret: Some(attribute),
//                 extern_crate,
//             } => {
//                 completions::attribute::complete_known_attribute_input(
//                     accumulator,
//                     ctx,
//                     colon_prefix,
//                     attribute,
//                     extern_crate.as_ref(),
//                 );
//             }
//             CompletionAnalysis::UnexpandedAttributeTT { .. } | CompletionAnalysis::String { .. } => (),
//         }
//     }

//     Some(completions.into())
// }

pub fn completions2(
    db: &dyn HirDatabase,
    config: &CompletionConfig<'_>,
    position: FilePosition,
    _trigger_character: Option<char>,
) -> Option<Vec<CompletionItem>> {
    let mut accumulator = Completions::default();

    let ctx = CompletionContext::new(db, position, config)?;
    completions::import::complete_import(&mut accumulator, &ctx);
    completions::dot::complete_dot(&mut accumulator, &ctx);
    completions::expression::complete_names_in_scope(&mut accumulator, &ctx);

    Some(accumulator.into())
}

// /// Resolves additional completion data at the position given.
// /// This is used for import insertion done via completions like flyimport and custom user snippets.
// pub fn resolve_completion_edits(
//     db: &RootDatabase,
//     config: &CompletionConfig<'_>,
//     FilePosition { file_id, offset }: FilePosition,
//     imports: impl IntoIterator<Item = String>,
// ) -> Option<Vec<TextEdit>> {
//     let _p = tracing::info_span!("resolve_completion_edits").entered();
//     let sema = hir::Semantics::new(db);

//     let original_file = sema.parse(file_id);
//     let original_token =
//         syntax::AstNode::syntax(&original_file).token_at_offset(offset).left_biased()?;
//     let position_for_import = &original_token.parent()?;
//     let scope = ImportScope::find_insert_use_container(position_for_import, &sema)?;

//     // let current_module = sema.scope(position_for_import)?.module();
//     // let current_crate = current_module.krate();
//     // let current_edition = current_crate.edition(db);
//     let new_ast = scope.clone_for_update();
//     let mut import_insert = TextEdit::builder();

//     // imports.into_iter().for_each(|full_import_path| {
//     //     insert_use::insert_use(
//     //         &new_ast,
//     //         make::path_from_text_with_edition(&full_import_path, current_edition),
//     //         &config.insert_use,
//     //     );
//     // });

//     diff(scope.as_syntax_node(), new_ast.as_syntax_node()).into_text_edit(&mut import_insert);
//     Some(vec![import_insert.finish()])
// }
