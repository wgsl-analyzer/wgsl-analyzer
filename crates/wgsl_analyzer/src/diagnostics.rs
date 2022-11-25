//! Book keeping for keeping diagnostics easily in sync with the client.

use std::{
    collections::{HashMap, HashSet},
    mem,
};

use vfs::FileId;

// PERF: FxHashMap/Set
// pub(crate) type CheckFixes = Arc<HashMap<FileId, Vec<Fix>>>;

#[derive(Debug, Default, Clone)]
pub struct DiagnosticsMapConfig {
    pub remap_prefix: HashMap<String, String>,
    pub warnings_as_info: Vec<String>,
    pub warnings_as_hint: Vec<String>,
}

#[derive(Debug, Default, Clone)]
pub struct DiagnosticCollection {
    // FIXME: should be FxHashMap<FileId, Vec<ra_id::Diagnostic>>
    pub(crate) native: HashMap<FileId, Vec<lsp_types::Diagnostic>>,
    changes: HashSet<FileId>,
}

/*#[derive(Debug, Clone)]
pub(crate) struct Fix {
    pub(crate) range: lsp_types::Range,
    pub(crate) action: lsp_ext::CodeAction,
}*/

impl DiagnosticCollection {
    /*pub(crate) fn clear_check(&mut self) {
        Arc::make_mut(&mut self.check_fixes).clear();
        self.changes
            .extend(self.check.drain().map(|(key, _value)| key))
    }

    pub(crate) fn add_check_diagnostic(
        &mut self,
        file_id: FileId,
        diagnostic: lsp_types::Diagnostic,
        fixes: Vec<lsp_ext::CodeAction>,
    ) {
        let diagnostics = self.check.entry(file_id).or_default();
        for existing_diagnostic in diagnostics.iter() {
            if are_diagnostics_equal(existing_diagnostic, &diagnostic) {
                return;
            }
        }

        let check_fixes = Arc::make_mut(&mut self.check_fixes);
        check_fixes
            .entry(file_id)
            .or_default()
            .extend(fixes.into_iter().map(|action| Fix {
                range: diagnostic.range,
                action,
            }));
        diagnostics.push(diagnostic);
        self.changes.insert(file_id);
    }*/

    pub(crate) fn set_native_diagnostics(
        &mut self,
        file_id: FileId,
        diagnostics: Vec<lsp_types::Diagnostic>,
    ) {
        if let Some(existing_diagnostics) = self.native.get(&file_id) {
            if existing_diagnostics.len() == diagnostics.len()
                && diagnostics
                    .iter()
                    .zip(existing_diagnostics)
                    .all(|(new, existing)| are_diagnostics_equal(new, existing))
            {
                return;
            }
        }

        self.native.insert(file_id, diagnostics);
        self.changes.insert(file_id);
    }

    pub(crate) fn diagnostics_for(
        &self,
        file_id: FileId,
    ) -> impl Iterator<Item = &lsp_types::Diagnostic> {
        let native = self.native.get(&file_id).into_iter().flatten();
        // let check = self.check.get(&file_id).into_iter().flatten();
        // native.chain(check)
        native
    }

    pub(crate) fn take_changes(&mut self) -> Option<HashSet<FileId>> {
        if self.changes.is_empty() {
            return None;
        }
        Some(mem::take(&mut self.changes))
    }

    pub(crate) fn make_updated(&mut self, file_id: FileId) {
        self.changes.insert(file_id);
    }
}

fn are_diagnostics_equal(left: &lsp_types::Diagnostic, right: &lsp_types::Diagnostic) -> bool {
    left.source == right.source
        && left.severity == right.severity
        && left.range == right.range
        && left.message == right.message
}
