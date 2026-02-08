use syntax::{ast, pointer::AstPointer};
use vfs::FileId;

use crate::{InFile, database::Location, item_tree::ImportStatement};

#[derive(Debug, PartialEq, Eq)]
pub struct DefDiagnostic {
    pub in_module: FileId,
    pub kind: DefDiagnosticKind,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DefDiagnosticKind {
    UnresolvedImport {
        // TODO: This location stores too much info, it redundantly stores the file id
        id: Location<ImportStatement>,
    },
    TooManySupers {
        id: Location<ImportStatement>,
    },
    UnresolvedModule {
        id: Location<ImportStatement>,
        candidates: Vec<String>,
    },
}

impl DefDiagnostic {
    pub(super) fn unresolved_import(
        container: FileId,
        id: Location<ImportStatement>,
    ) -> Self {
        Self {
            in_module: container,
            kind: DefDiagnosticKind::UnresolvedImport { id },
        }
    }

    pub(super) fn super_escaping_root(
        container: FileId,
        id: Location<ImportStatement>,
    ) -> Self {
        Self {
            in_module: container,
            kind: DefDiagnosticKind::UnresolvedImport { id },
        }
    }

    pub(super) fn unresolved_module(
        container: FileId,
        id: Location<ImportStatement>,
        candidates: Vec<String>,
    ) -> Self {
        Self {
            in_module: container,
            kind: DefDiagnosticKind::UnresolvedModule { id, candidates },
        }
    }
}
