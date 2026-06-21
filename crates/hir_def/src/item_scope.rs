use crate::{
    database::{DefDatabase, ImportId, ModuleDefinitionId},
    item_tree::Name,
    name_resolution::{DefDiagnostic, DefDiagnosticKind, collect_module},
    visibility::Visibility,
};
use base_db::EditionedFileId;
use rustc_hash::FxHashMap;
use std::fmt::Write as _;
use triomphe::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ModuleItem {
    pub definition: ModuleDefinitionId,
    pub visibility: Visibility,
    pub import: Option<ImportId>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct ItemScope {
    /// Items visible in this scope. Includes both declarations and imported items.
    pub items: FxHashMap<Name, ModuleItem>,

    /// The diagnostics that need to be emitted for this module.
    pub diagnostics: Vec<DefDiagnostic>,
}

#[salsa::tracked]
impl ItemScope {
    #[salsa::tracked]
    pub fn of(
        db: &dyn DefDatabase,
        file_id: EditionedFileId,
    ) -> Arc<ItemScope> {
        Arc::new(collect_module(db, file_id))
    }
}

impl ItemScope {
    /// Pushes an item and returns the old value if there is one.
    #[must_use]
    pub(crate) fn push_item(
        &mut self,
        name: Name,
        definition: ModuleItem,
    ) -> Option<ModuleItem> {
        self.items.insert(name, definition)
    }

    pub(crate) fn push_diagnostic(
        &mut self,
        diagnostic: DefDiagnostic,
    ) {
        self.diagnostics.push(diagnostic);
    }

    /// Get a name from current module scope.
    #[must_use]
    pub fn get(
        &self,
        name: &Name,
    ) -> Option<ModuleItem> {
        self.items.get(name).copied()
    }

    pub(crate) fn dump(
        &self,
        buffer: &mut String,
    ) {
        let mut entries: Vec<_> = self.items.iter().collect();
        entries.sort_by_key(|(name, _)| *name);

        for (name, value) in entries {
            let r#type = match value.definition {
                ModuleDefinitionId::Module(_) => "module",
                ModuleDefinitionId::Function(_) => "fn",
                ModuleDefinitionId::GlobalVariable(_) => "var",
                ModuleDefinitionId::GlobalConstant(_) => "const",
                ModuleDefinitionId::GlobalAssertStatement(_) => "const_assert",
                ModuleDefinitionId::Override(_) => "override",
                ModuleDefinitionId::Struct(_) => "struct",
                ModuleDefinitionId::TypeAlias(_) => "alias",
            };
            let description = if value.import.is_some() {
                " (import)"
            } else {
                ""
            };
            writeln!(buffer, "- {type} {}{description}", name.as_str());
        }

        for diagnostic in &self.diagnostics {
            buffer.push_str("error: ");
            match &diagnostic.kind {
                DefDiagnosticKind::UnresolvedImport { name, .. } => {
                    writeln!(buffer, "unresolved import for {}", name.as_str())
                },
                DefDiagnosticKind::TooManySupers { .. } => writeln!(buffer, "too many supers"),
                DefDiagnosticKind::DetachedFile { .. } => writeln!(buffer, "detached filed"),
                DefDiagnosticKind::NameConflict { previous, .. } => {
                    writeln!(buffer, "name conflict for {}", previous.as_str())
                },
            };
        }
    }

    pub(crate) fn shrink_to_fit(&mut self) {
        // Exhaustive match to require handling new fields.
        let Self { items, diagnostics } = self;
        items.shrink_to_fit();
        diagnostics.shrink_to_fit();
    }
}
