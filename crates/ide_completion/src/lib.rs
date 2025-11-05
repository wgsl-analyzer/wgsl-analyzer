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

    Some(accumulator.into())
}
