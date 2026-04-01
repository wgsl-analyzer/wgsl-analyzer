#[cfg(test)]
mod fixture;

mod debug_command;
pub mod diagnostics;
mod folding_ranges;
mod formatting;
mod goto_definition;
mod helpers;
mod hover;
pub mod inlay_hints;
mod markup;
mod navigation_target;
pub mod signature_help;
mod typing;
mod view_syntax_tree;

use std::panic;

use base_db::{
    EditionedFileId, FilePosition, FileRange, FileSet, RangeInfo, RootQueryDb as _,
    SourceDatabase as _, SourceRoot, TextRange, change::Change, input::SourceRootId,
};
use diagnostics::Diagnostic;
use hir::diagnostics::DiagnosticsConfig;
use hir_def::database::DefDatabase as _;
use ide_completion::{CompletionConfig, item::CompletionItem};
use ide_db::LineIndexDatabase as _;
pub use line_index::{LineCol, LineIndex};
use rustc_hash::FxHashMap;
use salsa::{Cancelled, Database as _};
use syntax::{Edition, Parse, SyntaxNode};
use triomphe::Arc;
use vfs::{AbsPathBuf, FileId, VfsPath};
use wgsl_formatter::FormattingOptions;

use crate::signature_help::SignatureHelp;
pub use crate::{
    // annotations::{Annotation, AnnotationConfig, AnnotationKind, AnnotationLocation},
    // call_hierarchy::{CallHierarchyConfig, CallItem},
    // expand_macro::ExpandedMacro,
    // file_structure::{StructureNode, StructureNodeKind},
    folding_ranges::{Fold, FoldKind},
    // highlight_related::{HighlightRelatedConfig, HighlightedRange},
    hover::{
        HoverAction, HoverConfig, HoverDocFormat, HoverGotoTypeData, HoverResult,
        MemoryLayoutHoverConfig, MemoryLayoutHoverRenderKind, SubstitutionTypeLength,
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
    navigation_target::NavigationTarget,
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

pub type Cancellable<T> = Result<T, Cancelled>;

/// `base_db` is normally also needed in places where `ide_db` is used, so this re-export is for convenience.
pub use base_db;
pub use ide_db::{
    // Severity,
    // SymbolKind,
    // assists::ExprFillDefaultMode,
    // base_db::{
    //     Crate,
    //     CrateGraphBuilder,
    //     FileChange,
    //     SourceRoot,
    //     SourceRootId
    // },
    // documentation::Documentation,
    // label::Label,
    // line_index::{
    //     LineCol,
    //     LineIndex
    // },
    // prime_caches::ParallelPrimeCachesProgress,
    // search::{
    //     ReferenceCategory,
    //     SearchScope
    // },,
    // FileId,
    // FilePosition,
    // FileRange,
    RootDatabase,
    // symbol_index::Query,
    text_edit::{
        // Indel,
        TextEdit,
    },
};

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
            database: self.database.clone(),
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

    pub fn trigger_cancellation(&mut self) {
        self.database.trigger_cancellation();
    }

    pub fn trigger_garbage_collection(&mut self) {
        self.database.trigger_lru_eviction();
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
    database: RootDatabase,
}

// As a general design guideline, `Analysis` API are intended to be independent
// from the language server protocol. That is, when exposing some functionality
// we should think in terms of "what API makes most sense" and not in terms of
// "what types LSP uses". Although currently LSP is the only consumer of the
// API, the API should in theory be usable as a library, or via a different
// protocol.
impl Analysis {
    /// Creates an analysis instance for a single file, without any external
    /// dependencies or ability to apply changes.
    /// See [`AnalysisHost`] for creating a fully-featured analysis.
    #[must_use]
    pub fn from_single_file(text: String) -> (Self, FileId) {
        let mut host = AnalysisHost::default();
        let file_id = FileId::from_raw(0);
        let mut file_set = FileSet::default();
        file_set.insert(
            file_id,
            VfsPath::new_virtual_path("/shader.wesl".to_owned()),
        );
        let source_root = SourceRoot::new_local(file_set);
        let mut change = Change::default();
        change.set_roots(vec![source_root]);
        change.change_file(file_id, Some(text));
        host.apply_change(change);
        (host.analysis(), file_id)
    }

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
        self.with_db(|database| database.file_source_root(file_id).source_root_id(database))
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
    ) -> Cancellable<Arc<str>> {
        self.with_db(|database| database.file_text(file_id).text(database).clone())
    }

    /// Returns the full source code with imports resolved.
    pub fn resolve_full_source(
        &self,
        file_id: FileId,
    ) -> Cancellable<Result<String, ()>> {
        self.with_db(|database| Ok(database.file_text(file_id).text(database).to_string()))
    }

    /// Gets the syntax tree of the file.
    pub fn parse(
        &self,
        file_id: FileId,
    ) -> Cancellable<Parse> {
        self.with_db(|database| database.parse(EditionedFileId::from_file(database, file_id)))
    }

    pub fn line_index(
        &self,
        file_id: FileId,
    ) -> Cancellable<Arc<LineIndex>> {
        self.with_db(|database| database.line_index(file_id))
    }

    pub fn view_syntax_tree(
        &self,
        file_id: FileId,
    ) -> Cancellable<String> {
        self.with_db(|database| view_syntax_tree::view_syntax_tree(database, file_id))
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

    /// Returns the set of folding ranges.
    pub fn folding_ranges(
        &self,
        file_id: FileId,
    ) -> Cancellable<Vec<Fold>> {
        self.with_db(|database| {
            folding_ranges::folding_ranges(
                &database
                    .parse(EditionedFileId::from_file(database, file_id))
                    .tree(),
            )
        })
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
        config: &FormattingOptions,
        file_id: FileId,
        range: Option<TextRange>,
    ) -> Cancellable<Option<wgsl_formatter::FormattedRange>> {
        self.with_db(|database| formatting::format(database, config, file_id, range))
    }

    /// Returns a short text describing element at position.
    pub fn hover(
        &self,
        config: &HoverConfig,
        range: FileRange,
    ) -> Cancellable<Option<RangeInfo<HoverResult>>> {
        self.with_db(|database| hover::hover(database, range, config))
    }

    pub fn signature_help(
        &self,
        position: FilePosition,
    ) -> Cancellable<Option<SignatureHelp>> {
        self.with_db(|database| signature_help::signature_help(database, position))
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
