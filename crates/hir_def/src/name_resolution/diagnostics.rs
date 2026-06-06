use base_db::EditionedFileId;
use syntax::{ast, pointer::AstPointer};
use vfs::FileId;

use crate::{InFile, database::Location, item_tree::Name};

#[derive(Debug, PartialEq, Eq)]
pub struct DefDiagnostic {
    pub in_module: EditionedFileId,
    pub kind: DefDiagnosticKind,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DefDiagnosticKind {
    UnresolvedImport {
        id: Location<ast::ImportStatement>,
        name: Name,
    },
    TooManySupers {
        id: Location<ast::ImportStatement>,
    },
    UnresolvedModule {
        id: Location<ast::ImportStatement>,
        name: Name,
    },
    DetachedFile {
        id: Location<ast::ImportStatement>,
    },
    EmptyImportStatement {
        id: Location<ast::ImportStatement>,
    },
}

impl DefDiagnostic {
    pub(super) const fn unresolved_import(
        container: EditionedFileId,
        id: Location<ast::ImportStatement>,
        name: Name,
    ) -> Self {
        Self {
            in_module: container,
            kind: DefDiagnosticKind::UnresolvedImport { id, name },
        }
    }

    pub(super) const fn super_escaping_root(
        container: EditionedFileId,
        id: Location<ast::ImportStatement>,
    ) -> Self {
        Self {
            in_module: container,
            kind: DefDiagnosticKind::TooManySupers { id },
        }
    }

    pub(super) const fn unresolved_module(
        container: EditionedFileId,
        id: Location<ast::ImportStatement>,
        name: Name,
    ) -> Self {
        Self {
            in_module: container,
            kind: DefDiagnosticKind::UnresolvedModule { id, name },
        }
    }

    pub(super) const fn detached_file(
        container: EditionedFileId,
        id: Location<ast::ImportStatement>,
    ) -> Self {
        Self {
            in_module: container,
            kind: DefDiagnosticKind::DetachedFile { id },
        }
    }
    pub(super) const fn empty_import_statement(
        container: EditionedFileId,
        id: Location<ast::ImportStatement>,
    ) -> Self {
        Self {
            in_module: container,
            kind: DefDiagnosticKind::EmptyImportStatement { id },
        }
    }
}
