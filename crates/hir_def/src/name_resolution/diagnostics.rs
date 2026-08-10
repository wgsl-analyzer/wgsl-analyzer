use base_db::EditionedFileId;
use syntax::ast;

use crate::{db::Location, item_tree::Name};

#[derive(Debug, PartialEq, Eq)]
pub struct DefDiagnostic {
    pub in_module: EditionedFileId,
    pub kind: DefDiagnosticKind,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DefDiagnosticKind {
    /// Currently cannot be triggered, as the grammar guarantees that import statements have a name.
    /// May be triggered in the future if a `self` keyword is added. See <https://github.com/webgpu-tools/wesl-spec/issues/120>.
    UnnamedImport {
        id: Location<ast::ImportStatement>,
    },
    UnresolvedPackage {
        id: Location<ast::ImportStatement>,
        name: Name,
    },
    UnresolvedImport {
        id: Location<ast::ImportStatement>,
    },
    TooManySupers {
        id: Location<ast::ImportStatement>,
    },
    /// Cannot resolve an import statement, because the current file is not a part of a package.
    DetachedFile {
        id: Location<ast::ImportStatement>,
    },
    NameConflict {
        item: Location<ast::Item>,
        previous: Name,
    },
}

impl DefDiagnostic {
    pub(crate) const fn unnamed_import(
        container: EditionedFileId,
        id: Location<ast::ImportStatement>,
    ) -> Self {
        Self {
            in_module: container,
            kind: DefDiagnosticKind::UnnamedImport { id },
        }
    }

    pub(crate) const fn unresolved_package(
        container: EditionedFileId,
        id: Location<ast::ImportStatement>,
        name: Name,
    ) -> Self {
        Self {
            in_module: container,
            kind: DefDiagnosticKind::UnresolvedPackage { id, name },
        }
    }

    pub(crate) const fn unresolved_import(
        container: EditionedFileId,
        id: Location<ast::ImportStatement>,
    ) -> Self {
        Self {
            in_module: container,
            kind: DefDiagnosticKind::UnresolvedImport { id },
        }
    }

    pub(crate) const fn super_escaping_root(
        container: EditionedFileId,
        id: Location<ast::ImportStatement>,
    ) -> Self {
        Self {
            in_module: container,
            kind: DefDiagnosticKind::TooManySupers { id },
        }
    }

    pub(crate) const fn detached_file(
        container: EditionedFileId,
        id: Location<ast::ImportStatement>,
    ) -> Self {
        Self {
            in_module: container,
            kind: DefDiagnosticKind::DetachedFile { id },
        }
    }
    pub(crate) const fn name_conflict(
        container: EditionedFileId,
        item: Location<ast::Item>,
        previous: Name,
    ) -> Self {
        Self {
            in_module: container,
            kind: DefDiagnosticKind::NameConflict { item, previous },
        }
    }
}
