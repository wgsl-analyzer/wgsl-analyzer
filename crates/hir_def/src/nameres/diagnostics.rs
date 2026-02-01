use syntax::ast;
use vfs::FileId;

use crate::FileAstId;

#[derive(Debug, PartialEq, Eq)]
pub struct DefDiagnostic {
    pub in_module: FileId,
    pub kind: DefDiagnosticKind,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DefDiagnosticKind {
    UnresolvedImport { id: FileAstId<ast::ImportTree> },
    // TODO: error for super::super::super imports that go out of the root
}

impl DefDiagnostic {
    pub(super) fn unresolved_import(
        container: FileId,
        id: FileAstId<ast::ImportTree>,
    ) -> Self {
        Self {
            in_module: container,
            kind: DefDiagnosticKind::UnresolvedImport { id },
        }
    }
}
