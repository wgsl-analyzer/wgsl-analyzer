pub mod global_variable;
pub mod precedence;

use base_db::{FileRange, TextRange};
use hir_def::{
    HirFileId, InFile,
    expression::BinaryOperation,
    expression_store::{ExpressionSourceMap, ExpressionStoreSource, path::Path},
    item_tree::Name,
};
use hir_ty::{
    builtins::BuiltinId,
    infer::{
        InferenceDiagnostic, InferenceDiagnosticKind, LoweredKind, TypeExpectation,
        TypeLoweringError, TypeLoweringErrorKind,
    },
    ty::Type,
    validate::AddressSpaceError,
};
use syntax::{ast, pointer::AstPointer};

use self::{global_variable::GlobalVariableDiagnostic, precedence::PrecedenceDiagnostic};

#[derive(Clone, Copy, Debug, Default)]
pub enum NagaVersion {
    Naga27,
    #[default]
    Naga28,
    NagaMain,
}

#[derive(Clone, Debug)]
pub struct DiagnosticsConfig {
    /// Whether native diagnostics are enabled.
    pub enabled: bool,
    pub type_errors: bool,
    pub naga_parsing_errors: bool,
    pub naga_validation_errors: bool,
    pub naga_version: NagaVersion,
}

impl Default for DiagnosticsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            type_errors: true,
            naga_parsing_errors: true,
            naga_validation_errors: true,
            naga_version: NagaVersion::default(),
        }
    }
}

pub enum AnyDiagnostic {
    ParseError {
        message: String,
        range: TextRange,
        file_id: HirFileId,
    },

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
    UnresolvedName {
        expression: InFile<AstPointer<ast::Expression>>,
        name: Name,
    },
    InvalidConstructionType {
        expression: InFile<AstPointer<ast::Expression>>,
        r#type: Type,
    },
    FunctionCallArgCountMismatch {
        expression: InFile<AstPointer<ast::Expression>>,
        n_expected: usize,
        n_actual: usize,
    },
    NoBuiltinOverload {
        expression: InFile<AstPointer<ast::Expression>>,
        builtin: BuiltinId,
        name: Option<&'static str>,
        parameters: Vec<Type>,
    },
    AddressOfNotReference {
        expression: InFile<AstPointer<ast::Expression>>,
        actual: Type,
    },
    DerefNotPointer {
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
        file_id: HirFileId,
        range: TextRange,
        message: String,
        related: Vec<(String, FileRange)>,
    },
    NoConstructor {
        expression: InFile<AstPointer<ast::Expression>>,
        builtins: BuiltinId,
        r#type: Type,
        parameters: Vec<Type>,
    },
    CyclicType {
        file_id: HirFileId,
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
    ExpectedLoweredKind {
        expression: InFile<AstPointer<ast::Expression>>,
        expected: LoweredKind,
        actual: LoweredKind,
        path: Path,
    },
}

impl AnyDiagnostic {
    #[must_use]
    pub const fn file_id(&self) -> HirFileId {
        match self {
            Self::AssignmentNotAReference { left_side, .. } => left_side.file_id,
            Self::TypeMismatch { expression, .. }
            | Self::NoSuchField { expression, .. }
            | Self::ArrayAccessInvalidType { expression, .. }
            | Self::UnresolvedName { expression, .. }
            | Self::InvalidConstructionType { expression, .. }
            | Self::FunctionCallArgCountMismatch { expression, .. }
            | Self::NoBuiltinOverload { expression, .. }
            | Self::AddressOfNotReference { expression, .. }
            | Self::DerefNotPointer { expression, .. }
            | Self::NoConstructor { expression, .. }
            | Self::PrecedenceParensRequired { expression, .. }
            | Self::UnexpectedTemplateArgument { expression, .. }
            | Self::WgslError { expression, .. }
            | Self::InvalidIdentExpression { expression, .. }
            | Self::ExpectedLoweredKind { expression, .. } => expression.file_id,
            Self::MissingAddressSpace { variable } | Self::InvalidAddressSpace { variable, .. } => {
                variable.file_id
            },
            Self::InvalidTypeSpecifier { type_specifier, .. } => type_specifier.file_id,
            Self::NagaValidationError { file_id, .. }
            | Self::ParseError { file_id, .. }
            | Self::CyclicType { file_id, .. } => *file_id,
        }
    }
}

#[expect(clippy::too_many_lines, reason = "long but simple match")]
pub(crate) fn any_diag_from_infer_diagnostic(
    infer_diagnostic: &InferenceDiagnosticKind,
    source_map: &ExpressionSourceMap,
    file_id: HirFileId,
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
                expected: expected.clone(),
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
        InferenceDiagnosticKind::UnresolvedName { expression, name } => {
            let pointer = source_map.expression_to_source(*expression).ok()?.clone();
            let source = InFile::new(file_id, pointer);

            AnyDiagnostic::UnresolvedName {
                expression: source,
                name: name.clone(),
            }
        },
        InferenceDiagnosticKind::InvalidConstructionType { expression, r#type } => {
            let pointer = source_map.expression_to_source(*expression).ok()?.clone();
            let source = InFile::new(file_id, pointer);

            AnyDiagnostic::InvalidConstructionType {
                expression: source,
                r#type: *r#type,
            }
        },
        InferenceDiagnosticKind::NoConstructor {
            expression,
            r#type,
            builtins,
            parameters,
        } => {
            let pointer = source_map.expression_to_source(*expression).ok()?.clone();
            let source = InFile::new(file_id, pointer);

            AnyDiagnostic::NoConstructor {
                expression: source,
                builtins: *builtins,
                r#type: *r#type,
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
        InferenceDiagnosticKind::NoBuiltinOverload {
            expression,
            builtin,
            parameters,
            name,
        } => {
            let pointer = source_map.expression_to_source(*expression).ok()?.clone();
            let source = InFile::new(file_id, pointer);

            AnyDiagnostic::NoBuiltinOverload {
                expression: source,
                builtin: *builtin,
                name: *name,
                parameters: parameters.clone(),
            }
        },
        InferenceDiagnosticKind::AddressOfNotReference { expression, actual } => {
            let pointer = source_map.expression_to_source(*expression).ok()?.clone();
            let source = InFile::new(file_id, pointer);

            AnyDiagnostic::AddressOfNotReference {
                expression: source,
                actual: *actual,
            }
        },
        InferenceDiagnosticKind::DerefNotAPointer { expression, actual } => {
            let pointer = source_map.expression_to_source(*expression).ok()?.clone();
            let source = InFile::new(file_id, pointer);

            AnyDiagnostic::DerefNotPointer {
                expression: source,
                actual: *actual,
            }
        },
        InferenceDiagnosticKind::InvalidType {
            error: TypeLoweringError { container, kind },
        } => match container {
            hir_ty::infer::TypeContainer::Expression(expression) => {
                let pointer = source_map.expression_to_source(*expression).ok()?.clone();
                let source = InFile::new(file_id, pointer);

                AnyDiagnostic::InvalidIdentExpression {
                    expression: source,
                    error: kind.clone(),
                }
            },
            hir_ty::infer::TypeContainer::TypeSpecifier(type_specifier) => {
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
        InferenceDiagnosticKind::ExpectedLoweredKind {
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
    })
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
    file_id: HirFileId,
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
