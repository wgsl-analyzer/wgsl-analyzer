//! Settings for tweaking completion.
//!
//! The fun thing here is `SnippetCapability` - this type can only be created in this
//! module, and we use to statically check that we only produce snippet
//! completions if we are allowed to.

// use hir::ImportPathConfig;
use ide_db::{
    SnippetCapability,
    // imports::insert_import::InsertImportConfig
};

use crate::{
    // snippet::Snippet,
    CompletionFieldsToResolve,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompletionConfig {
    // TODO: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/913
    // pub enable_postfix_completions: bool,
    // TODO: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/914
    // pub enable_imports_on_the_fly: bool,
    // pub enable_private_editable: bool,
    // TODO: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/915
    // pub enable_term_search: bool,
    // TODO: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/915
    // pub term_search_fuel: u64,
    // TODO: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/916
    // pub full_function_signatures: bool,
    // TODO: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/917
    // pub callable: Option<CallableSnippets>,
    // TODO: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/920
    // pub snippet_capability: Option<SnippetCapability>,
    // TODO: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/914
    // pub insert_import: InsertImportConfig,
    // TODO: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/922
    // pub prefer_prelude: bool,
    // TODO: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/921
    // pub snippets: Vec<Snippet>,
    // TODO: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/919
    pub limit: Option<usize>,
    pub fields_to_resolve: CompletionFieldsToResolve,
    // TODO: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/914
    // pub exclude_flyimport: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CallableSnippets {
    FillArguments,
    AddParentheses,
}

impl CompletionConfig {
    // TODO: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/921
    // pub fn postfix_snippets(&self) -> impl Iterator<Item = (&str, &Snippet)> {
    //     self.snippets
    //         .iter()
    //         .flat_map(|snip| snip.postfix_triggers.iter().map(move |trigger| (&**trigger, snip)))
    // }

    // pub fn prefix_snippets(&self) -> impl Iterator<Item = (&str, &Snippet)> {
    //     self.snippets
    //         .iter()
    //         .flat_map(|snip| snip.prefix_triggers.iter().map(move |trigger| (&**trigger, snip)))
    // }

    // pub fn import_path_config(&self) -> ImportPathConfig {
    //     ImportPathConfig {
    //         prefer_prelude: self.prefer_prelude,
    //     }
    // }
}
