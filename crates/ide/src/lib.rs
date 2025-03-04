#![allow(dead_code)]

mod db;
mod debug_command;
pub mod diagnostics;
mod formatting;
mod goto_definition;
mod helpers;
mod hover;
pub mod inlay_hints;
mod syntax_tree;

use std::sync::Arc;

use base_db::{
    FilePosition, FileRange, RangeInfo, SourceDatabase, TextRange, change::Change,
    input::SourceRootId,
};
use diagnostics::Diagnostic;
use goto_definition::NavigationTarget;
use hir::diagnostics::DiagnosticsConfig;
use hir_def::db::DefDatabase;
pub use hover::HoverResult;
use ide_completion::item::CompletionItem;
use inlay_hints::{InlayHint, InlayHintsConfig};
pub use line_index::{LineCol, LineIndex};
use salsa::{Cancelled, ParallelDatabase};
use syntax::{Parse, SyntaxNode};
use vfs::FileId;

pub type Cancellable<T> = Result<T, Cancelled>;

pub use db::RootDatabase;

/// `base_db` is normally also needed in places where `ide_db` is used, so this re-export is for convenience.
pub use base_db;

#[derive(Debug)]
pub struct AnalysisHost {
    db: RootDatabase,
}

impl AnalysisHost {
    #[allow(clippy::new_without_default)]
    pub fn new() -> AnalysisHost {
        let mut this = AnalysisHost {
            db: RootDatabase::new(),
        };
        this.db.set_custom_imports(Arc::new(Default::default()));
        this
    }

    /// Returns a snapshot of the current state, which you can query for
    /// semantic information.
    pub fn analysis(&self) -> Analysis {
        Analysis {
            db: self.db.snapshot(),
        }
    }

    pub fn apply_change(
        &mut self,
        change: Change,
    ) {
        self.db.apply_change(change)
    }

    pub fn snapshot(&self) -> Analysis {
        Analysis {
            db: self.db.snapshot(),
        }
    }

    pub fn raw_database(&mut self) -> &RootDatabase {
        &self.db
    }

    pub fn raw_database_mut(&mut self) -> &mut RootDatabase {
        &mut self.db
    }
}

pub struct Analysis {
    db: salsa::Snapshot<RootDatabase>,
}

impl Analysis {
    pub fn with_db<F, T>(
        &self,
        f: F,
    ) -> Cancellable<T>
    where
        F: FnOnce(&RootDatabase) -> T + std::panic::UnwindSafe,
    {
        Cancelled::catch(|| f(&self.db))
    }

    pub fn source_root_id(
        &self,
        file_id: FileId,
    ) -> Cancellable<SourceRootId> {
        self.with_db(|db| db.file_source_root(file_id))
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
        self.with_db(|db| db.file_text(file_id))
    }

    // Returns the full source code with imports resolved
    pub fn resolve_full_source(
        &self,
        file_id: FileId,
    ) -> Cancellable<Result<String, ()>> {
        self.with_db(|db| db.resolve_full_source(file_id.into()))
    }

    /// Gets the syntax tree of the file.
    pub fn parse(
        &self,
        file_id: FileId,
    ) -> Cancellable<Parse> {
        self.with_db(|db| db.parse(file_id))
    }

    pub fn line_index(
        &self,
        file_id: FileId,
    ) -> Cancellable<Arc<LineIndex>> {
        self.with_db(|db| db.line_index(file_id))
    }

    pub fn syntax_tree(
        &self,
        file_id: FileId,
        range: Option<TextRange>,
    ) -> Cancellable<String> {
        self.with_db(|db| syntax_tree::syntax_tree(db, file_id, range).unwrap_or_default())
    }

    pub fn inlay_hints(
        &self,
        config: &InlayHintsConfig,
        file_id: FileId,
        range: Option<FileRange>,
    ) -> Cancellable<Vec<InlayHint>> {
        self.with_db(|db| inlay_hints::inlay_hints(db, file_id, range, config))
    }

    pub fn diagnostics(
        &self,
        config: &DiagnosticsConfig,
        file_id: FileId,
    ) -> Cancellable<Vec<Diagnostic>> {
        self.with_db(|db| diagnostics::diagnostics(db, config, file_id))
    }

    pub fn goto_definition(
        &self,
        file_position: FilePosition,
    ) -> Cancellable<Option<NavigationTarget>> {
        self.with_db(|db| goto_definition::goto_definition(db, file_position))
    }

    /// Computes completions at the given position.
    pub fn completions(
        &self,
        // config: &CompletionConfig<'_>,
        position: FilePosition,
        trigger_character: Option<char>,
    ) -> Cancellable<Option<Vec<CompletionItem>>> {
        self.with_db(|db| {
            ide_completion::completions2(
                db,
                // config,
                position,
                trigger_character,
            )
        })
    }

    pub fn format(
        &self,
        file_id: FileId,
        range: Option<TextRange>,
    ) -> Cancellable<Option<SyntaxNode>> {
        self.with_db(|db| formatting::format(db, file_id, range))
    }

    pub fn hover(
        &self,
        range: FileRange,
    ) -> Cancellable<Option<RangeInfo<HoverResult>>> {
        self.with_db(|db| hover::hover(db, range))
    }

    pub fn debug_command(
        &self,
        file_position: FilePosition,
    ) -> Cancellable<()> {
        self.with_db(|db| debug_command::debug_command(db, file_position))
            .unwrap();
        Ok(())
    }
}

#[cfg(test)]
mod tests;
