//! `completions` crate provides utilities for generating completions of user input.

mod completions;
mod config;
mod context;
pub mod item;
mod patterns;

use base_db::FilePosition;
use ide_db::RootDatabase;
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
    #[must_use]
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

    #[must_use]
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

pub fn completions2(
    database: &RootDatabase,
    config: &CompletionConfig,
    position: FilePosition,
    _trigger_character: Option<char>,
) -> Option<Vec<CompletionItem>> {
    let mut accumulator = Completions::default();

    let context = CompletionContext::new(database, position, config)?;
    completions::dot::complete_dot(&mut accumulator, &context);
    completions::expression::complete_names_in_scope(&mut accumulator, &context);
    completions::keyword::complete_keywords(&mut accumulator, &context);
    completions::types::complete_types(&mut accumulator, &context);
    completions::attribute::complete_attributes(&mut accumulator, &context);

    Some(accumulator.into())
}

#[cfg(test)]
mod tests {
    use expect_test::{Expect, expect};
    use test_fixture::ChangeFixture;

    use crate::{CompletionConfig, CompletionFieldsToResolve, CompletionItemKind, completions2};

    fn get_completion_items(source: &str) -> Vec<crate::CompletionItem> {
        let fixture = ChangeFixture::parse(source);
        let mut database = ide_db::RootDatabase::new(None);
        database.apply_change(fixture.change);
        let (file_id, range_or_offset) = fixture
            .file_position
            .expect("Add $0 to mark cursor position");
        let position = base_db::FilePosition {
            file_id,
            offset: range_or_offset.expect_offset(),
        };
        let config = CompletionConfig {
            enable_postfix_completions: false,
            enable_imports_on_the_fly: false,
            enable_self_on_the_fly: false,
            enable_auto_iter: false,
            enable_auto_await: false,
            enable_private_editable: false,
            enable_term_search: false,
            term_search_fuel: 400,
            full_function_signatures: false,
            callable: None,
            add_semicolon_to_unit: false,
            prefer_no_std: false,
            prefer_prelude: false,
            prefer_absolute: false,
            limit: None,
            fields_to_resolve: CompletionFieldsToResolve::empty(),
            exclude_flyimport: vec![],
        };
        completions2(&database, &config, position, None).unwrap_or_default()
    }

    fn get_completions(source: &str) -> Vec<(CompletionItemKind, String)> {
        get_completion_items(source)
            .into_iter()
            .map(|item| (item.kind, item.label.primary.to_string()))
            .collect()
    }

    fn check_completions_contain(
        source: &str,
        expected_kind: CompletionItemKind,
        expected_labels: &[&str],
    ) {
        let completions = get_completions(source);
        for label in expected_labels {
            assert!(
                completions
                    .iter()
                    .any(|(kind, l)| *kind == expected_kind && l == label),
                "Expected completion '{label}' with kind '{expected_kind:?}' not found.\nGot: {completions:?}"
            );
        }
    }

    fn check_completions_absent(
        source: &str,
        labels: &[&str],
    ) {
        let completions = get_completions(source);
        for label in labels {
            assert!(
                !completions.iter().any(|(_, l)| l == label),
                "Completion '{label}' should NOT be present.\nGot: {completions:?}"
            );
        }
    }

    // --- Keyword completions ---

    #[test]
    fn keyword_completions_at_top_level() {
        // Use a partial identifier so the parser produces a token at the cursor
        check_completions_contain(
            "f$0",
            CompletionItemKind::Keyword,
            &[
                "fn", "struct", "var", "const", "alias", "enable", "requires",
            ],
        );
    }

    #[test]
    fn keyword_completions_inside_function() {
        check_completions_contain(
            "
fn test() {
    $0
}
",
            CompletionItemKind::Keyword,
            &[
                "let", "var", "const", "if", "for", "while", "loop", "switch", "return", "break",
                "continue", "discard",
            ],
        );
    }

    #[test]
    fn no_statement_keywords_at_top_level() {
        check_completions_absent(
            "f$0",
            &["if", "for", "while", "loop", "return", "break", "discard"],
        );
    }

    // --- Type completions ---

    #[test]
    fn type_completions_at_top_level() {
        check_completions_contain(
            "f$0",
            CompletionItemKind::TypeAlias,
            &[
                "f32", "i32", "u32", "bool", "vec3", "vec4f", "mat4x4", "array",
            ],
        );
    }

    #[test]
    fn type_completions_inside_function() {
        check_completions_contain(
            "
fn test() {
    $0
}
",
            CompletionItemKind::TypeAlias,
            &["f32", "vec3f", "mat4x4f", "sampler", "texture_2d"],
        );
    }

    // --- Attribute completions ---

    #[test]
    fn attribute_completions_before_fn() {
        check_completions_contain(
            "
@$0
fn test() {}
",
            CompletionItemKind::Keyword,
            &["vertex", "fragment", "compute"],
        );
    }

    // --- Builtin detail completions (#291) ---

    #[test]
    fn builtin_completions_have_signature_detail() {
        let items = get_completion_items(
            "
fn test() {
    $0
}
",
        );
        // Find the `abs` builtin completion
        let abs_item = items
            .iter()
            .find(|item| item.kind == CompletionItemKind::Function && item.label.primary == "abs")
            .expect("Expected 'abs' builtin completion");
        let detail = abs_item
            .detail
            .as_deref()
            .expect("Expected 'abs' to have a detail string");
        // The detail should be a function signature like "fn abs(T) -> T"
        assert!(
            detail.starts_with("fn abs("),
            "Expected detail to start with 'fn abs(', got: {detail}"
        );
    }

    #[test]
    fn no_attribute_completions_inside_function() {
        // Inside a function body, we should NOT get attribute completions
        let completions = get_completions(
            "
fn test() {
    $0
}
",
        );
        // Attribute completions should not appear in statement context
        assert!(
            !completions
                .iter()
                .any(|(_, l)| l == "vertex" || l == "fragment" || l == "compute"),
            "Attribute completions should not appear inside function body.\nGot: {completions:?}"
        );
    }
}
