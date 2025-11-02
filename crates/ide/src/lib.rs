mod debug_command;
pub mod diagnostics;
mod formatting;
mod goto_definition;
mod helpers;
mod hover;
pub mod inlay_hints;
mod markup;
mod navigation_target;
mod syntax_tree;
mod typing;

use std::panic;

use rustc_hash::FxHashMap;
use triomphe::Arc;

use base_db::{
    FilePosition, FileRange, RangeInfo, SourceDatabase as _, TextRange, change::Change,
    input::SourceRootId,
};
use diagnostics::Diagnostic;
use hir::diagnostics::DiagnosticsConfig;
use hir_def::database::DefDatabase as _;
use ide_completion::{CompletionConfig, item::CompletionItem};

pub use crate::{
    // annotations::{Annotation, AnnotationConfig, AnnotationKind, AnnotationLocation},
    // call_hierarchy::{CallHierarchyConfig, CallItem},
    // expand_macro::ExpandedMacro,
    // file_structure::{StructureNode, StructureNodeKind},
    // folding_ranges::{Fold, FoldKind},
    // highlight_related::{HighlightRelatedConfig, HighlightedRange},
    hover::{
        HoverAction, HoverConfig, HoverDocFormat, HoverGotoTypeData, HoverResult,
        MemoryLayoutHoverConfig, MemoryLayoutHoverRenderKind, SubstTyLen,
    },
    inlay_hints::{
        // AdjustmentHints, AdjustmentHintsMode, ClosureReturnTypeHints, DiscriminantHints,
        // GenericParameterHints,
        InlayFieldsToResolve,
        InlayHint,
        InlayHintLabel,
        InlayHintLabelPart,
        InlayHintPosition,
        InlayHintsConfig,
        InlayKind,
        InlayTooltip,
        LazyProperty,
        // LifetimeElisionHints,
    },
    // join_lines::JoinLinesConfig,
    // markup::Markup,
    // moniker::{
    //     Moniker, MonikerDescriptorKind, MonikerIdentifier, MonikerKind, MonikerResult,
    //     PackageInformation, SymbolInformationKind,
    // },
    // move_item::Direction,
    navigation_target::{
        NavigationTarget,
        // TryToNavigationTarget, UpmappingResult
    },
    // references::ReferenceSearchResult,
    // rename::RenameError,
    // runnables::{Runnable, RunnableKind, TestId, UpdateTest},
    // signature_help::SignatureHelp,
    // static_index::{
    //     StaticIndex, StaticIndexedFile, TokenId, TokenStaticData, VendoredLibrariesConfig,
    // },
    // syntax_highlighting::{
    //     HighlightConfig, HlRange,
    //     tags::{Highlight, HlMod, HlMods, HlOperator, HlPunct, HlTag},
    // },
    // test_explorer::{TestItem, TestItemKind},
};
pub use line_index::{LineCol, LineIndex};
use salsa::{Cancelled, ParallelDatabase as _};
use syntax::{Parse, SyntaxNode};
use vfs::FileId;

pub type Cancellable<T> = Result<T, Cancelled>;

pub use ide_db::RootDatabase;

/// `base_db` is normally also needed in places where `ide_db` is used, so this re-export is for convenience.
pub use base_db;

#[derive(Debug)]
pub struct AnalysisHost {
    database: RootDatabase,
}

impl AnalysisHost {
    #[must_use]
    pub fn new(lru_capacity: Option<u16>) -> Self {
        Self {
            database: RootDatabase::new(lru_capacity),
        }
    }

    pub const fn with_database(database: RootDatabase) -> Self {
        Self { database }
    }

    pub const fn update_lru_capacity(
        &mut self,
        lru_capacity: Option<u16>,
    ) {
        self.database.update_base_query_lru_capacities(lru_capacity);
    }

    pub const fn update_lru_capacities(
        &mut self,
        lru_capacities: &FxHashMap<Box<str>, u16>,
    ) {
        self.database.update_lru_capacities(lru_capacities);
    }

    /// Returns a snapshot of the current state, which you can query for
    /// semantic information.
    pub fn analysis(&self) -> Analysis {
        Analysis {
            database: self.database.snapshot(),
        }
    }

    /// Applies changes to the current state of the world. If there are
    /// outstanding snapshots, they will be canceled.
    pub fn apply_change(
        &mut self,
        change: Change,
    ) {
        self.database.apply_change(change);
    }

    pub const fn raw_database(&self) -> &RootDatabase {
        &self.database
    }

    pub const fn raw_database_mut(&mut self) -> &mut RootDatabase {
        &mut self.database
    }
}

impl Default for AnalysisHost {
    fn default() -> Self {
        Self::new(None)
    }
}

pub struct Analysis {
    database: salsa::Snapshot<RootDatabase>,
}

impl Analysis {
    pub const SUPPORTED_TRIGGER_CHARS: &[char] = typing::TRIGGER_CHARS;

    pub fn with_db<Function, T>(
        &self,
        function: Function,
    ) -> Cancellable<T>
    where
        Function: FnOnce(&RootDatabase) -> T + panic::UnwindSafe,
    {
        Cancelled::catch(|| function(&self.database))
    }

    pub fn source_root_id(
        &self,
        file_id: FileId,
    ) -> Cancellable<SourceRootId> {
        self.with_db(|database| database.file_source_root(file_id))
    }

    /// Computes the set of parser level diagnostics for the given file.
    pub fn syntax_diagnostics(
        &self,
        _config: &DiagnosticsConfig,
        _file_id: FileId,
    ) -> Cancellable<Vec<Diagnostic>> {
        self.with_db(|_db| vec![])
    }

    /// Computes the set of semantic diagnostics for the given file.
    pub fn semantic_diagnostics(
        &self,
        _config: &DiagnosticsConfig,
        // resolve: AssistResolveStrategy,
        _file_id: FileId,
    ) -> Cancellable<Vec<Diagnostic>> {
        self.with_db(|_db| vec![])
    }

    /// Computes the set of both syntax and semantic diagnostics for the given file.
    pub fn full_diagnostics(
        &self,
        _config: &DiagnosticsConfig,
        // resolve: AssistResolveStrategy,
        _file_id: FileId,
    ) -> Cancellable<Vec<Diagnostic>> {
        self.with_db(|_db| vec![])
    }

    /// Gets the text of the source file.
    pub fn file_text(
        &self,
        file_id: FileId,
    ) -> Cancellable<Arc<String>> {
        self.with_db(|database| database.file_text(file_id))
    }

    /// Returns the full source code with imports resolved
    /// TODO: Hook up wesl-rs over here https://github.com/wgsl-analyzer/wgsl-analyzer/issues/324
    pub fn resolve_full_source(
        &self,
        file_id: FileId,
    ) -> Cancellable<Result<String, ()>> {
        self.with_db(|database| Ok(database.file_text(file_id.into()).to_string()))
    }

    /// Gets the syntax tree of the file.
    pub fn parse(
        &self,
        file_id: FileId,
    ) -> Cancellable<Parse> {
        self.with_db(|database| database.parse(file_id))
    }

    pub fn line_index(
        &self,
        file_id: FileId,
    ) -> Cancellable<Arc<LineIndex>> {
        self.with_db(|database| database.line_index(file_id))
    }

    pub fn syntax_tree(
        &self,
        file_id: FileId,
        range: Option<TextRange>,
    ) -> Cancellable<String> {
        self.with_db(|database| {
            syntax_tree::syntax_tree(database, file_id, range).unwrap_or_default()
        })
    }

    /// Returns a list of the places in the file where type hints can be displayed.
    pub fn inlay_hints(
        &self,
        config: &InlayHintsConfig,
        file_id: FileId,
        range: Option<TextRange>,
    ) -> Cancellable<Vec<InlayHint>> {
        self.with_db(|database| inlay_hints::inlay_hints(database, file_id, range, config))
    }

    pub fn diagnostics(
        &self,
        config: &DiagnosticsConfig,
        file_id: FileId,
    ) -> Cancellable<Vec<Diagnostic>> {
        self.with_db(|database| diagnostics::diagnostics(database, config, file_id))
    }

    pub fn goto_definition(
        &self,
        file_position: FilePosition,
    ) -> Cancellable<Option<NavigationTarget>> {
        self.with_db(|database| goto_definition::goto_definition(database, file_position))
    }

    /// Computes completions at the given position.
    pub fn completions(
        &self,
        config: &CompletionConfig,
        position: FilePosition,
        trigger_character: Option<char>,
    ) -> Cancellable<Option<Vec<CompletionItem>>> {
        self.with_db(|database| {
            ide_completion::completions2(database, config, position, trigger_character)
        })
    }

    pub fn format(
        &self,
        file_id: FileId,
        range: Option<TextRange>,
    ) -> Cancellable<Option<SyntaxNode>> {
        self.with_db(|database| formatting::format(database, file_id, range))
    }

    /// Returns a short text describing element at position.
    pub fn hover(
        &self,
        config: &HoverConfig,
        range: FileRange,
    ) -> Cancellable<Option<RangeInfo<HoverResult>>> {
        self.with_db(|database| hover::hover(database, range, config))
    }

    /// # Panics
    ///
    /// Panics if the command was cancelled.
    pub fn debug_command(
        &self,
        file_position: FilePosition,
    ) -> Cancellable<()> {
        self.with_db(|database| debug_command::debug_command(database, file_position))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests;
