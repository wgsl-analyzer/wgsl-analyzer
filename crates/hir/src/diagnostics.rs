pub mod global_variable;
pub mod precedence;

use base_db::{FileRange, TextRange};
use hir_def::{
    HirFileId, InFile,
    body::BodySourceMap,
    expression::BinaryOperation,
    expression_store::{ExpressionSourceMap, ExpressionStoreSource},
    module_data::Name,
};
use hir_ty::{
    builtins::BuiltinId,
    database::{FieldInferenceDiagnostic, HirDatabase},
    infer::{
        InferenceDiagnostic, LoweredKind, TypeExpectation, TypeLoweringError, TypeLoweringErrorKind,
    },
    ty::Type,
    validate::AddressSpaceError,
};
use serde::Deserialize;
use syntax::{
    AstNode as _, ast,
    pointer::{AstPointer, SyntaxNodePointer},
};

use self::{global_variable::GlobalVariableDiagnostic, precedence::PrecedenceDiagnostic};
use crate::{
    Field, Function, GlobalConstant, GlobalVariable, HasSource as _, Override, Parameter, TypeAlias,
};

#[derive(Clone, Debug, Deserialize, Default)]
pub enum NagaVersion {
    #[serde(rename = "0.14")]
    #[default]
    Naga14,
    #[serde(rename = "0.19")]
    Naga19,
    #[serde(rename = "0.22")]
    Naga22,
    #[serde(rename = "main")]
    NagaMain,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
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
            naga_version: NagaVersion::Naga22,
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
        var: InFile<AstPointer<ast::VariableDeclaration>>,
    },
    InvalidAddressSpace {
        var: InFile<AstPointer<ast::VariableDeclaration>>,
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
        name: Name,
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
            Self::MissingAddressSpace { var } | Self::InvalidAddressSpace { var, .. } => {
                var.file_id
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
    infer_diagnostic: &InferenceDiagnostic,
    signature_map: &ExpressionSourceMap,
    source_map: &ExpressionSourceMap,
    file_id: HirFileId,
) -> Option<AnyDiagnostic> {
    Some(match infer_diagnostic {
        InferenceDiagnostic::AssignmentNotAReference { left_side, actual } => {
            let pointer = source_map.expression_to_source(*left_side).ok()?.clone();
            let source = InFile::new(file_id, pointer);
            AnyDiagnostic::AssignmentNotAReference {
                left_side: source,
                actual: *actual,
            }
        },
        InferenceDiagnostic::TypeMismatch {
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
        InferenceDiagnostic::NoSuchField {
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
        InferenceDiagnostic::ArrayAccessInvalidType { expression, r#type } => {
            let pointer = source_map.expression_to_source(*expression).ok()?.clone();
            let source = InFile::new(file_id, pointer);

            AnyDiagnostic::ArrayAccessInvalidType {
                expression: source,
                r#type: *r#type,
            }
        },
        InferenceDiagnostic::UnresolvedName { expression, name } => {
            let pointer = source_map.expression_to_source(*expression).ok()?.clone();
            let source = InFile::new(file_id, pointer);

            AnyDiagnostic::UnresolvedName {
                expression: source,
                name: name.clone(),
            }
        },
        InferenceDiagnostic::InvalidConstructionType { expression, r#type } => {
            let pointer = source_map.expression_to_source(*expression).ok()?.clone();
            let source = InFile::new(file_id, pointer);

            AnyDiagnostic::InvalidConstructionType {
                expression: source,
                r#type: *r#type,
            }
        },
        InferenceDiagnostic::NoConstructor {
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
        InferenceDiagnostic::FunctionCallArgCountMismatch {
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
        InferenceDiagnostic::NoBuiltinOverload {
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
        InferenceDiagnostic::AddressOfNotReference { expression, actual } => {
            let pointer = source_map.expression_to_source(*expression).ok()?.clone();
            let source = InFile::new(file_id, pointer);

            AnyDiagnostic::AddressOfNotReference {
                expression: source,
                actual: *actual,
            }
        },
        InferenceDiagnostic::DerefNotAPointer { expression, actual } => {
            let pointer = source_map.expression_to_source(*expression).ok()?.clone();
            let source = InFile::new(file_id, pointer);

            AnyDiagnostic::DerefNotPointer {
                expression: source,
                actual: *actual,
            }
        },
        InferenceDiagnostic::InvalidType {
            source,
            error: TypeLoweringError { container, kind },
        } => {
            let source_map = match source {
                ExpressionStoreSource::Body => source_map,
                ExpressionStoreSource::Signature => signature_map,
            };
            match container {
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
            }
        },
        InferenceDiagnostic::CyclicType { name, range } => AnyDiagnostic::CyclicType {
            file_id,
            name: name.clone(),
            range: *range,
        },
        InferenceDiagnostic::UnexpectedTemplateArgument { expression } => {
            let pointer = source_map.expression_to_source(*expression).ok()?.clone();
            let source = InFile::new(file_id, pointer);
            AnyDiagnostic::UnexpectedTemplateArgument { expression: source }
        },
        InferenceDiagnostic::WgslError {
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
        InferenceDiagnostic::ExpectedLoweredKind {
            expression,
            expected,
            actual,
            name,
        } => {
            let pointer = source_map.expression_to_source(*expression).ok()?.clone();
            let source = InFile::new(file_id, pointer);
            AnyDiagnostic::ExpectedLoweredKind {
                expression: source,
                name: name.clone(),
                expected: *expected,
                actual: *actual,
            }
        },
    })
}

pub(crate) fn any_diag_from_global_var(
    var_diagnostic: GlobalVariableDiagnostic,
    var: InFile<AstPointer<ast::VariableDeclaration>>,
) -> AnyDiagnostic {
    match var_diagnostic {
        GlobalVariableDiagnostic::MissingAddressSpace => AnyDiagnostic::MissingAddressSpace { var },
        GlobalVariableDiagnostic::AddressSpaceError(error) => {
            AnyDiagnostic::InvalidAddressSpace { var, error }
        },
    }
}

pub(crate) fn any_diag_from_shift(
    error: &PrecedenceDiagnostic,
    source_map: &ExpressionSourceMap,
    file_id: HirFileId,
) -> Option<AnyDiagnostic> {
    match error {
        PrecedenceDiagnostic::NeverNested(expression, op) => {
            let pointer = source_map.expression_to_source(*expression).ok()?.clone();
            let source = InFile::new(file_id, pointer);
            Some(AnyDiagnostic::PrecedenceParensRequired {
                expression: source,
                operation: *op,
                sequence_permitted: false,
            })
        },
        PrecedenceDiagnostic::SequencesAllowed(expression, op) => {
            let pointer = source_map.expression_to_source(*expression).ok()?.clone();
            let source = InFile::new(file_id, pointer);
            Some(AnyDiagnostic::PrecedenceParensRequired {
                expression: source,
                operation: *op,
                sequence_permitted: true,
            })
        },
    }
}
