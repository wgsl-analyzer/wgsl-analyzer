//! Book keeping for keeping diagnostics easily in sync with the client.

// pub(crate) mod to_proto;

use std::mem;

type FileId = vfs::FileId;
use base_db::PackageId;
use ide::diagnostics::{Diagnostic, Severity};
use itertools::Itertools as _;
use lsp_types::DiagnosticSeverity;
use nohash_hasher::{IntMap, IntSet};
use rustc_hash::{FxHashMap, FxHashSet};
use salsa::Cancelled;
use stdx::iter_eq_by;
use triomphe::Arc;

use crate::{global_state::GlobalStateSnapshot, lsp, main_loop::DiagnosticsTaskKind};

pub(crate) type CheckFixes =
    Arc<IntMap<usize, FxHashMap<Option<Arc<PackageId>>, IntMap<FileId, Vec<Fix>>>>>;

#[derive(Debug, Default, Clone)]
pub(crate) struct DiagnosticsMapConfig {
    pub remap_prefix: FxHashMap<String, String>,
    pub warnings_as_info: Vec<String>,
    pub warnings_as_hint: Vec<String>,
    pub check_ignore: FxHashSet<String>,
}

pub(crate) type DiagnosticsGeneration = usize;

type Checks =
    IntMap<usize, FxHashMap<Option<Arc<PackageId>>, IntMap<FileId, Vec<lsp_types::Diagnostic>>>>;

#[derive(Debug, Default, Clone)]
pub(crate) struct DiagnosticCollection {
    // FIXME: should be IntMap<FileId, Vec<ra_id::Diagnostic>>
    pub(crate) native_syntax: IntMap<FileId, (DiagnosticsGeneration, Vec<lsp_types::Diagnostic>)>,
    pub(crate) native_semantic: IntMap<FileId, (DiagnosticsGeneration, Vec<lsp_types::Diagnostic>)>,
    // FIXME: should be Vec<flycheck::Diagnostic>
    pub(crate) check: Checks,
    pub(crate) check_fixes: CheckFixes,
    changes: IntSet<FileId>,
    /// Counter for supplying a new generation number for diagnostics.
    /// This is used to keep track of when to clear the diagnostics for a given file as we compute
    /// diagnostics on multiple worker threads simultaneously which may result in multiple diagnostics
    /// updates for the same file in a single generation update (due to macros affecting multiple files).
    generation: DiagnosticsGeneration,
}

#[derive(Debug, Clone)]
pub(crate) struct Fix {
    // Fixes may be triggerable from multiple ranges.
    pub(crate) ranges: Vec<lsp_types::Range>,
    pub(crate) action: lsp::extensions::CodeAction,
}

impl DiagnosticCollection {
    pub(crate) fn clear_check(
        &mut self,
        flycheck_id: usize,
    ) {
        let Some(check) = self.check.get_mut(&flycheck_id) else {
            return;
        };
        self.changes
            .extend(check.drain().flat_map(|(_, value)| value.into_keys()));
        if let Some(fixes) = Arc::make_mut(&mut self.check_fixes).get_mut(&flycheck_id) {
            fixes.clear();
        }
    }

    pub(crate) fn clear_check_all(&mut self) {
        Arc::make_mut(&mut self.check_fixes).clear();
        self.changes.extend(
            self.check.values_mut().flat_map(|check_value| {
                check_value.drain().flat_map(|(_, value)| value.into_keys())
            }),
        );
    }

    pub(crate) fn clear_check_for_package(
        &mut self,
        flycheck_id: usize,
        package_id: Arc<PackageId>,
    ) {
        let Some(check) = self.check.get_mut(&flycheck_id) else {
            return;
        };
        let package_id = Some(package_id);
        if let Some(checks) = check.remove(&package_id) {
            self.changes.extend(checks.into_keys());
        }
        if let Some(fixes) = Arc::make_mut(&mut self.check_fixes).get_mut(&flycheck_id) {
            fixes.remove(&package_id);
        }
    }

    pub(crate) fn clear_native_for(
        &mut self,
        file_id: FileId,
    ) {
        self.native_syntax.remove(&file_id);
        self.native_semantic.remove(&file_id);
        self.changes.insert(file_id);
    }

    #[expect(clippy::ref_option, reason = "necessary for the borrow checker")]
    pub(crate) fn add_check_diagnostic(
        &mut self,
        flycheck_id: usize,
        package_id: &Option<Arc<PackageId>>,
        file_id: FileId,
        diagnostic: lsp_types::Diagnostic,
        fix: Option<Box<Fix>>,
    ) {
        let diagnostics = self
            .check
            .entry(flycheck_id)
            .or_default()
            .entry(package_id.clone())
            .or_default()
            .entry(file_id)
            .or_default();
        for existing_diagnostic in diagnostics.iter() {
            if are_diagnostics_equal(existing_diagnostic, &diagnostic) {
                return;
            }
        }

        if let Some(fix) = fix {
            let check_fixes = Arc::make_mut(&mut self.check_fixes);
            check_fixes
                .entry(flycheck_id)
                .or_default()
                .entry(package_id.clone())
                .or_default()
                .entry(file_id)
                .or_default()
                .push(*fix);
        }
        diagnostics.push(diagnostic);
        self.changes.insert(file_id);
    }

    pub(crate) fn set_native_diagnostics(
        &mut self,
        kind: DiagnosticsTaskKind,
    ) {
        let (generation, diagnostics, target) = match kind {
            DiagnosticsTaskKind::Syntax(generation, diagnostics) => {
                (generation, diagnostics, &mut self.native_syntax)
            },
            DiagnosticsTaskKind::Semantic(generation, diagnostics) => {
                (generation, diagnostics, &mut self.native_semantic)
            },
        };

        for (file_id, mut diagnostics) in diagnostics {
            diagnostics.sort_by_key(|diagnostic| (diagnostic.range.start, diagnostic.range.end));

            if let Some((old_gen, existing_diagnostics)) = target.get_mut(&file_id) {
                if existing_diagnostics.len() == diagnostics.len()
                    && iter_eq_by(&diagnostics, &*existing_diagnostics, |new, existing| {
                        are_diagnostics_equal(new, existing)
                    })
                {
                    // don't signal an update if the diagnostics are the same
                    continue;
                }
                if *old_gen < generation || generation == 0 {
                    target.insert(file_id, (generation, diagnostics));
                } else {
                    existing_diagnostics.extend(diagnostics);
                    // FIXME: Doing the merge step of a merge sort here would be a bit more performant
                    // but eh
                    existing_diagnostics
                        .sort_by_key(|diagnostic| (diagnostic.range.start, diagnostic.range.end));
                }
            } else {
                target.insert(file_id, (generation, diagnostics));
            }
            self.changes.insert(file_id);
        }
    }

    pub(crate) fn diagnostics_for(
        &self,
        file_id: FileId,
    ) -> impl Iterator<Item = &lsp_types::Diagnostic> {
        let native_syntax = self
            .native_syntax
            .get(&file_id)
            .into_iter()
            .flat_map(|(_, diagnostics)| diagnostics);
        let native_semantic = self
            .native_semantic
            .get(&file_id)
            .into_iter()
            .flat_map(|(_, diagnostics)| diagnostics);
        let check = self
            .check
            .values()
            .flat_map(|check_value| check_value.values())
            .filter_map(move |check_value| check_value.get(&file_id))
            .flatten();
        native_syntax.chain(native_semantic).chain(check)
    }

    pub(crate) fn take_changes(&mut self) -> Option<IntSet<FileId>> {
        if self.changes.is_empty() {
            return None;
        }
        Some(mem::take(&mut self.changes))
    }

    pub(crate) fn make_updated(
        &mut self,
        file_id: FileId,
    ) {
        self.changes.insert(file_id);
    }

    pub(crate) const fn next_generation(&mut self) -> usize {
        self.generation += 1;
        self.generation
    }
}

fn are_diagnostics_equal(
    left: &lsp_types::Diagnostic,
    right: &lsp_types::Diagnostic,
) -> bool {
    left.source == right.source
        && left.severity == right.severity
        && left.range == right.range
        && left.message == right.message
}

pub(crate) enum NativeDiagnosticsFetchKind {
    Syntax,
    Semantic,
}

#[expect(clippy::needless_pass_by_value, reason = "wip")]
pub(crate) fn fetch_native_diagnostics(
    snapshot: &GlobalStateSnapshot,
    subscriptions: Arc<[FileId]>,
    slice: std::ops::Range<usize>,
    kind: NativeDiagnosticsFetchKind,
) -> Vec<(FileId, Vec<lsp_types::Diagnostic>)> {
    let _p = tracing::info_span!("fetch_native_diagnostics").entered();
    let _context = stdx::panic_context::enter("fetch_native_diagnostics".to_owned());

    subscriptions[slice]
        .iter()
        .copied()
        .filter_map(|file_id| {
            let line_index = snapshot.file_line_index(file_id).ok()?;
            let source_root = snapshot.analysis.source_root_id(file_id).ok()?;

            let config = &snapshot.config.diagnostics(Some(source_root));
            let diagnostics = match kind {
                NativeDiagnosticsFetchKind::Syntax => {
                    snapshot.analysis.syntax_diagnostics(config, file_id).ok()?
                },

                NativeDiagnosticsFetchKind::Semantic if config.enabled => snapshot
                    .analysis
                    .semantic_diagnostics(
                        config, /* ide::AssistResolveStrategy::None ,*/ file_id,
                    )
                    .ok()?,
                NativeDiagnosticsFetchKind::Semantic => return None,
            };
            let diagnostics = diagnostics
                .into_iter()
                .map(|diagnostic| convert_diagnostic(&line_index, diagnostic))
                .collect::<Vec<_>>();
            Some((file_id, diagnostics))
        })
        .collect()
}

pub(crate) fn convert_diagnostic(
    line_index: &crate::line_index::LineIndex,
    diagnostic: Diagnostic,
) -> lsp_types::Diagnostic {
    lsp_types::Diagnostic {
        range: lsp::to_proto::range(line_index, diagnostic.range),
        severity: Some(diagnostic_severity(diagnostic.severity)),
        code: Some(lsp_types::NumberOrString::String(
            diagnostic.code.as_str().to_owned(),
        )),
        code_description: None,
        source: Some(diagnostic.source.to_string()),
        message: diagnostic.message,
        related_information: None,
        tags: diagnostic
            .unused
            .then(|| vec![lsp_types::DiagnosticTag::UNNECESSARY]),
        data: None,
    }
}

pub(crate) fn convert_related_information(
    snapshot: &GlobalStateSnapshot,
    diagnostic: &mut Diagnostic,
) -> Result<Vec<lsp_types::DiagnosticRelatedInformation>, Cancelled> {
    diagnostic
        .related
        .drain(..)
        .map(|(message, range)| {
            Ok(lsp_types::DiagnosticRelatedInformation {
                location: lsp::to_proto::location(snapshot, range)?,
                message,
            })
        })
        .collect::<Result<Vec<_>, Cancelled>>()
}

const fn diagnostic_severity(severity: Severity) -> lsp_types::DiagnosticSeverity {
    match severity {
        Severity::Error => lsp_types::DiagnosticSeverity::ERROR,
        Severity::WeakWarning => lsp_types::DiagnosticSeverity::HINT,
    }
}
