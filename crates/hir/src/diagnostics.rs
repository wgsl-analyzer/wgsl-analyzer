pub mod global_variable;
pub mod precedence;

use base_db::{EditionedFileId, FileRange, TextRange};
use hir_def::{
    HasSource as _, InFile,
    expression::BinaryOperation,
    expression_store::{ExpressionSourceMap, path::Path},
    item_tree::Name,
    name_resolution::{DefDiagnostic, DefDiagnosticKind},
};
use hir_ty::{
    db::HirDatabase,
    diagnostics::InferenceDiagnosticKind,
    infer::TypeExpectation,
    lower::{LoweredKind, TypeContainer, TypeLoweringError, TypeLoweringErrorKind},
    ty::Type,
    validate::AddressSpaceError,
};
use syntax::{ast, pointer::AstPointer};

use self::{global_variable::GlobalVariableDiagnostic, precedence::PrecedenceDiagnostic};

pub enum AnyDiagnostic {
    ParseError {
        message: String,
        range: TextRange,
        file_id: EditionedFileId,
    },

    // Module system errors
    UnnamedImport {
        id: InFile<AstPointer<ast::ImportStatement>>,
    },
    UnresolvedPackage {
        id: InFile<AstPointer<ast::ImportStatement>>,
        name: Name,
    },
    UnresolvedImport {
        id: InFile<AstPointer<ast::ImportStatement>>,
    },
    TooManySupers {
        id: InFile<AstPointer<ast::ImportStatement>>,
    },
    DetachedFile {
        id: InFile<AstPointer<ast::ImportStatement>>,
    },
    NameConflict {
        item: InFile<AstPointer<ast::Item>>,
        name: Name,
    },

    // Type checking errors
    AssignmentNotAReference {
        left_side: InFile<AstPointer<ast::Expression>>,
        actual: Type,
    },
    TypeMismatch {
        expression: InFile<AstPointer<ast::Expression>>,
        expected: TypeExpectation,
        actual: Type,
    },
    NoSuchField {
        expression: InFile<AstPointer<ast::Expression>>,
        name: Name,
        r#type: Type,
    },
    ArrayAccessInvalidType {
        expression: InFile<AstPointer<ast::Expression>>,
        r#type: Type,
    },
    NotConstructible {
        expression: InFile<AstPointer<ast::Expression>>,
        r#type: Type,
    },
    FunctionCallArgCountMismatch {
        expression: InFile<AstPointer<ast::Expression>>,
        n_expected: usize,
        n_actual: usize,
    },
    StoreTypeMustBeStorable {
        expression: InFile<AstPointer<ast::Expression>>,
        actual: Type,
    },
    MissingAddressSpace {
        variable: InFile<AstPointer<ast::VariableDeclaration>>,
    },
    InvalidAddressSpace {
        variable: InFile<AstPointer<ast::VariableDeclaration>>,
        error: AddressSpaceError,
    },
    InvalidTypeSpecifier {
        type_specifier: InFile<AstPointer<ast::TypeSpecifier>>,
        error: TypeLoweringErrorKind,
    },
    InvalidIdentExpression {
        expression: InFile<AstPointer<ast::Expression>>,
        error: TypeLoweringErrorKind,
    },
    PrecedenceParensRequired {
        expression: InFile<AstPointer<ast::Expression>>,
        operation: BinaryOperation,
        sequence_permitted: bool,
    },
    NagaValidationError {
        file_id: EditionedFileId,
        range: TextRange,
        message: String,
        related: Vec<(String, FileRange)>,
    },
    TintValidationError {
        file_id: EditionedFileId,
        range: TextRange,
        message: String,
        severity: Severity,
    },
    NoConstructor {
        expression: InFile<AstPointer<ast::Expression>>,
        r#type: Type,
        parameters: Vec<Type>,
    },
    NoOverload {
        expression: InFile<AstPointer<ast::Expression>>,
        parameters: Vec<Type>,
        name: Name,
    },
    CyclicType {
        file_id: EditionedFileId,
        name: Name,
        range: TextRange,
    },
    UnexpectedTemplateArgument {
        expression: InFile<AstPointer<ast::Expression>>,
    },
    WgslError {
        expression: InFile<AstPointer<ast::Expression>>,
        message: String,
    },
    InvalidIdentifier {
        file_id: EditionedFileId,
        name: Name,
        range: TextRange,
    },
    ExpectedLoweredKind {
        expression: InFile<AstPointer<ast::Expression>>,
        expected: LoweredKind,
        actual: LoweredKind,
        path: Path,
    },
    UnexpectedReturnValue {
        expression: InFile<AstPointer<ast::Expression>>,
        actual: Type,
    },
}

#[derive(Clone, Copy)]
pub enum Severity {
    Error,
    Warning,
    Information,
    Hint,
}

impl AnyDiagnostic {
    #[must_use]
    #[rustfmt::skip]
    pub const fn file_id(&self) -> EditionedFileId {
        match self {
            Self::AssignmentNotAReference { left_side, actual: _  } => {
                left_side.file_id
            },

            Self::TypeMismatch { expression, expected: _, actual: _  }
            | Self::NoSuchField { expression, name: _, r#type: _  }
            | Self::ArrayAccessInvalidType { expression, r#type: _  }
            | Self::NotConstructible { expression, r#type: _ }
            | Self::FunctionCallArgCountMismatch { expression, n_expected: _, n_actual: _  }
            | Self::StoreTypeMustBeStorable { expression, actual: _  }
            | Self::NoConstructor { expression, r#type: _, parameters: _  }
            | Self::NoOverload { expression, name: _, parameters: _ }
            | Self::PrecedenceParensRequired { expression, operation: _, sequence_permitted: _ }
            | Self::UnexpectedTemplateArgument { expression }
            | Self::WgslError { expression, message: _ }
            | Self::InvalidIdentExpression { expression, error: _ }
            | Self::UnexpectedReturnValue { expression, actual: _ }
            | Self::ExpectedLoweredKind { expression, actual: _, expected: _, path: _  } => {
                expression.file_id
            },

            Self::MissingAddressSpace { variable }
            | Self::InvalidAddressSpace { variable, error: _ } => {
                variable.file_id
            },

            Self::InvalidTypeSpecifier { type_specifier, error: _ } => {
                type_specifier.file_id
            },

            Self::NagaValidationError { file_id, range: _, message: _, related: _  }
            | Self::TintValidationError { file_id, range: _, message: _, severity: _  }
            | Self::ParseError { file_id, message: _, range: _ }
            | Self::CyclicType { file_id, name: _, range: _ }
            | Self::InvalidIdentifier { file_id, name: _, range: _ } => {
                *file_id
            },

            Self::UnnamedImport { id }
            | Self::UnresolvedPackage { id, name: _ }
            | Self::UnresolvedImport { id }
            | Self::TooManySupers { id }
            | Self::DetachedFile { id } => {
                id.file_id
            },

            Self::NameConflict { item, name: _ } => {
                item.file_id
            },
        }
    }
}

#[expect(clippy::too_many_lines, reason = "long but simple match")]
pub(crate) fn to_any_diagnostic(
    infer_diagnostic: &InferenceDiagnosticKind,
    source_map: &ExpressionSourceMap,
    file_id: EditionedFileId,
) -> Option<AnyDiagnostic> {
    Some(match infer_diagnostic {
        InferenceDiagnosticKind::AssignmentNotAReference { left_side, actual } => {
            let pointer = source_map.expression_to_source(*left_side).ok()?.clone();
            let source = InFile::new(file_id, pointer);
            AnyDiagnostic::AssignmentNotAReference {
                left_side: source,
                actual: *actual,
            }
        },
        InferenceDiagnosticKind::TypeMismatch {
            expression,
            expected,
            actual,
        } => {
            let pointer = source_map.expression_to_source(*expression).ok()?.clone();
            let source = InFile::new(file_id, pointer);
            AnyDiagnostic::TypeMismatch {
                expression: source,
                expected: *expected,
                actual: *actual,
            }
        },
        InferenceDiagnosticKind::NoSuchField {
            expression,
            name,
            r#type,
        } => {
            let pointer = source_map.expression_to_source(*expression).ok()?.clone();
            let source = InFile::new(file_id, pointer);
            AnyDiagnostic::NoSuchField {
                expression: source,
                name: name.clone(),
                r#type: *r#type,
            }
        },
        InferenceDiagnosticKind::ArrayAccessInvalidType { expression, r#type } => {
            let pointer = source_map.expression_to_source(*expression).ok()?.clone();
            let source = InFile::new(file_id, pointer);
            AnyDiagnostic::ArrayAccessInvalidType {
                expression: source,
                r#type: *r#type,
            }
        },
        InferenceDiagnosticKind::NotConstructible { expression, r#type } => {
            let pointer = source_map.expression_to_source(*expression).ok()?.clone();
            let source = InFile::new(file_id, pointer);
            AnyDiagnostic::NotConstructible {
                expression: source,
                r#type: *r#type,
            }
        },
        InferenceDiagnosticKind::NoConstructor {
            expression,
            r#type,
            parameters,
        } => {
            let pointer = source_map.expression_to_source(*expression).ok()?.clone();
            let source = InFile::new(file_id, pointer);
            AnyDiagnostic::NoConstructor {
                expression: source,
                r#type: *r#type,
                parameters: parameters.clone(),
            }
        },
        InferenceDiagnosticKind::NoOverload {
            expression,
            parameters,
            name,
        } => {
            let pointer = source_map.expression_to_source(*expression).ok()?.clone();
            let source = InFile::new(file_id, pointer);
            AnyDiagnostic::NoOverload {
                expression: source,
                name: name.clone(),
                parameters: parameters.clone(),
            }
        },
        InferenceDiagnosticKind::FunctionCallArgCountMismatch {
            expression,
            n_expected,
            n_actual,
        } => {
            let pointer = source_map.expression_to_source(*expression).ok()?.clone();
            let source = InFile::new(file_id, pointer);
            AnyDiagnostic::FunctionCallArgCountMismatch {
                expression: source,
                n_expected: *n_expected,
                n_actual: *n_actual,
            }
        },
        InferenceDiagnosticKind::InvalidType {
            error: TypeLoweringError { container, kind },
        } => match container {
            TypeContainer::Expression(expression) => {
                let pointer = source_map.expression_to_source(*expression).ok()?.clone();
                let source = InFile::new(file_id, pointer);
                AnyDiagnostic::InvalidIdentExpression {
                    expression: source,
                    error: kind.clone(),
                }
            },
            TypeContainer::TypeSpecifier(type_specifier) => {
                let pointer = source_map
                    .type_specifier_to_source(*type_specifier)
                    .ok()?
                    .clone();
                let source = InFile::new(file_id, pointer);
                AnyDiagnostic::InvalidTypeSpecifier {
                    type_specifier: source,
                    error: kind.clone(),
                }
            },
        },
        InferenceDiagnosticKind::CyclicType { name, range } => AnyDiagnostic::CyclicType {
            file_id,
            name: name.clone(),
            range: *range,
        },
        InferenceDiagnosticKind::UnexpectedTemplateArgument { expression } => {
            let pointer = source_map.expression_to_source(*expression).ok()?.clone();
            let source = InFile::new(file_id, pointer);
            AnyDiagnostic::UnexpectedTemplateArgument { expression: source }
        },
        InferenceDiagnosticKind::WgslError {
            expression,
            message,
        } => {
            let pointer = source_map.expression_to_source(*expression).ok()?.clone();
            let source = InFile::new(file_id, pointer);
            AnyDiagnostic::WgslError {
                expression: source,
                message: message.clone(),
            }
        },
        InferenceDiagnosticKind::UnexpectedLoweredKind {
            expression,
            expected,
            actual,
            path,
        } => {
            let pointer = source_map.expression_to_source(*expression).ok()?.clone();
            let source = InFile::new(file_id, pointer);
            AnyDiagnostic::ExpectedLoweredKind {
                expression: source,
                path: path.clone(),
                expected: *expected,
                actual: *actual,
            }
        },
        InferenceDiagnosticKind::StoreTypeMustBeStorable { actual, expression } => {
            let pointer = source_map.expression_to_source(*expression).ok()?.clone();
            let source = InFile::new(file_id, pointer);
            AnyDiagnostic::StoreTypeMustBeStorable {
                expression: source,
                actual: *actual,
            }
        },
        InferenceDiagnosticKind::UnexpectedReturnValue { expression, actual } => {
            let pointer = source_map.expression_to_source(*expression).ok()?.clone();
            let source = InFile::new(file_id, pointer);
            AnyDiagnostic::UnexpectedReturnValue {
                expression: source,
                actual: *actual,
            }
        },
    })
}

pub(crate) fn any_diag_from_def_diagnostic(
    db: &dyn HirDatabase,
    def_diagnostic: &DefDiagnostic,
) -> AnyDiagnostic {
    match &def_diagnostic.kind {
        DefDiagnosticKind::UnnamedImport { id } => {
            AnyDiagnostic::UnnamedImport { id: id.ast_ptr(db) }
        },
        DefDiagnosticKind::UnresolvedPackage { id, name } => AnyDiagnostic::UnresolvedPackage {
            id: id.ast_ptr(db),
            name: name.clone(),
        },
        DefDiagnosticKind::UnresolvedImport { id } => {
            AnyDiagnostic::UnresolvedImport { id: id.ast_ptr(db) }
        },
        DefDiagnosticKind::TooManySupers { id } => {
            AnyDiagnostic::TooManySupers { id: id.ast_ptr(db) }
        },
        DefDiagnosticKind::DetachedFile { id } => {
            AnyDiagnostic::DetachedFile { id: id.ast_ptr(db) }
        },
        DefDiagnosticKind::NameConflict { item, previous } => AnyDiagnostic::NameConflict {
            item: item.ast_ptr(db),
            name: previous.clone(),
        },
    }
}

pub(crate) fn any_diag_from_global_var(
    variable_diagnostic: GlobalVariableDiagnostic,
    variable: InFile<AstPointer<ast::VariableDeclaration>>,
) -> AnyDiagnostic {
    match variable_diagnostic {
        GlobalVariableDiagnostic::MissingAddressSpace => {
            AnyDiagnostic::MissingAddressSpace { variable }
        },
        GlobalVariableDiagnostic::AddressSpaceError(error) => {
            AnyDiagnostic::InvalidAddressSpace { variable, error }
        },
    }
}

pub(crate) fn any_diag_from_shift(
    error: &PrecedenceDiagnostic,
    source_map: &ExpressionSourceMap,
    file_id: EditionedFileId,
) -> Option<AnyDiagnostic> {
    match *error {
        PrecedenceDiagnostic::NeverNested(expression, operation) => {
            let pointer = source_map.expression_to_source(expression).ok()?.clone();
            let source = InFile::new(file_id, pointer);
            Some(AnyDiagnostic::PrecedenceParensRequired {
                expression: source,
                operation,
                sequence_permitted: false,
            })
        },
        PrecedenceDiagnostic::SequencesAllowed(expression, operation) => {
            let pointer = source_map.expression_to_source(expression).ok()?.clone();
            let source = InFile::new(file_id, pointer);
            Some(AnyDiagnostic::PrecedenceParensRequired {
                expression: source,
                operation,
                sequence_permitted: true,
            })
        },
    }
}
