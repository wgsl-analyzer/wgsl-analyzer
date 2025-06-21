pub mod global_variable;
pub mod precedence;

use base_db::{FileRange, TextRange};
use hir_def::{
    HirFileId, InFile, body::BodySourceMap, expression::BinaryOperation, module_data::Name,
};
use hir_ty::{
    builtins::BuiltinId,
    database::HirDatabase,
    infer::{InferenceDiagnostic, TypeExpectation, TypeLoweringError},
    ty::Type,
    validate::AddressSpaceError,
};
use serde::Deserialize;
use syntax::{
    AstNode as _, ast,
    pointer::{AstPointer, SyntaxNodePointer},
};

use self::{global_variable::GlobalVariableDiagnostic, precedence::PrecedenceDiagnostic};
use crate::{Function, GlobalConstant, GlobalVariable, HasSource as _, Override, TypeAlias};

#[derive(Clone, Debug, Deserialize)]
pub enum NagaVersion {
    #[serde(rename = "0.14")]
    Naga14,
    #[serde(rename = "0.19")]
    Naga19,
    #[serde(rename = "0.22")]
    Naga22,
    #[serde(rename = "main")]
    NagaMain,
}

impl Default for NagaVersion {
    fn default() -> Self {
        Self::Naga14
    }
}

#[derive(Default, Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticsConfig {
    /// Whether native diagnostics are enabled.
    pub enabled: bool,
    pub type_errors: bool,
    pub naga_parsing_errors: bool,
    pub naga_validation_errors: bool,
    pub naga_version: NagaVersion,
}

pub enum AnyDiagnostic {
    ParseError {
        message: String,
        range: TextRange,
        file_id: HirFileId,
    },

    UnconfiguredCode {
        def: String,
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
        var: InFile<AstPointer<ast::GlobalVariableDeclaration>>,
    },
    InvalidAddressSpace {
        var: InFile<AstPointer<ast::GlobalVariableDeclaration>>,
        error: AddressSpaceError,
    },

    InvalidType {
        file_id: HirFileId,
        location: SyntaxNodePointer,
        error: TypeLoweringError,
    },

    UnresolvedImport {
        import: InFile<AstPointer<ast::Import>>,
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
        builtins: [BuiltinId; 2],
        r#type: Type,
        parameters: Vec<Type>,
    },
}

impl AnyDiagnostic {
    #[must_use]
    pub const fn file_id(&self) -> HirFileId {
        match self {
            Self::AssignmentNotAReference { left_side, .. } => left_side.file_id,
            Self::TypeMismatch { expression, .. } => expression.file_id,
            Self::NoSuchField { expression, .. } => expression.file_id,
            Self::ArrayAccessInvalidType { expression, .. } => expression.file_id,
            Self::UnresolvedName { expression, .. } => expression.file_id,
            Self::InvalidConstructionType { expression, .. } => expression.file_id,
            Self::FunctionCallArgCountMismatch { expression, .. } => expression.file_id,
            Self::NoBuiltinOverload { expression, .. } => expression.file_id,
            Self::AddressOfNotReference { expression, .. } => expression.file_id,
            Self::DerefNotPointer { expression, .. } => expression.file_id,
            Self::MissingAddressSpace { var } => var.file_id,
            Self::InvalidAddressSpace { var, .. } => var.file_id,
            Self::InvalidType { file_id, .. } => *file_id,
            Self::UnresolvedImport { import, .. } => import.file_id,
            Self::NagaValidationError { file_id, .. } => *file_id,
            Self::ParseError { file_id, .. } => *file_id,
            Self::UnconfiguredCode { file_id, .. } => *file_id,
            Self::NoConstructor { expression, .. } => expression.file_id,
            Self::PrecedenceParensRequired { expression, .. } => expression.file_id,
        }
    }
}

#[expect(clippy::too_many_lines, reason = "TODO")]
pub(crate) fn any_diag_from_infer_diagnostic(
    database: &dyn HirDatabase,
    infer_diagnostic: &InferenceDiagnostic,
    source_map: &BodySourceMap,
    file_id: HirFileId,
) -> Option<AnyDiagnostic> {
    Some(match *infer_diagnostic {
        InferenceDiagnostic::AssignmentNotAReference { left_side, actual } => {
            let pointer = source_map.expression_to_source(left_side).ok()?.clone();
            let source = InFile::new(file_id, pointer);
            AnyDiagnostic::AssignmentNotAReference {
                left_side: source,
                actual,
            }
        },
        InferenceDiagnostic::TypeMismatch {
            expression,
            ref expected,
            actual,
        } => {
            let pointer = source_map.expression_to_source(expression).ok()?.clone();
            let source = InFile::new(file_id, pointer);
            AnyDiagnostic::TypeMismatch {
                expression: source,
                expected: expected.clone(),
                actual,
            }
        },
        InferenceDiagnostic::NoSuchField {
            expression,
            ref name,
            r#type,
        } => {
            let pointer = source_map.expression_to_source(expression).ok()?.clone();
            let source = InFile::new(file_id, pointer);

            AnyDiagnostic::NoSuchField {
                expression: source,
                name: name.clone(),
                r#type,
            }
        },
        InferenceDiagnostic::ArrayAccessInvalidType { expression, r#type } => {
            let pointer = source_map.expression_to_source(expression).ok()?.clone();
            let source = InFile::new(file_id, pointer);

            AnyDiagnostic::ArrayAccessInvalidType {
                expression: source,
                r#type,
            }
        },
        InferenceDiagnostic::UnresolvedName {
            expression,
            ref name,
        } => {
            let pointer = source_map.expression_to_source(expression).ok()?.clone();
            let source = InFile::new(file_id, pointer);

            AnyDiagnostic::UnresolvedName {
                expression: source,
                name: name.clone(),
            }
        },
        InferenceDiagnostic::InvalidConstructionType { expression, r#type } => {
            let pointer = source_map.expression_to_source(expression).ok()?.clone();
            let source = InFile::new(file_id, pointer);

            AnyDiagnostic::InvalidConstructionType {
                expression: source,
                r#type,
            }
        },
        InferenceDiagnostic::NoConstructor {
            expression,
            r#type,
            ref builtins,
            ref parameters,
        } => {
            let pointer = source_map.expression_to_source(expression).ok()?.clone();
            let source = InFile::new(file_id, pointer);

            AnyDiagnostic::NoConstructor {
                expression: source,
                builtins: *builtins,
                r#type,
                parameters: parameters.clone(),
            }
        },
        InferenceDiagnostic::FunctionCallArgCountMismatch {
            expression,
            n_expected,
            n_actual,
        } => {
            let pointer = source_map.expression_to_source(expression).ok()?.clone();
            let source = InFile::new(file_id, pointer);

            AnyDiagnostic::FunctionCallArgCountMismatch {
                expression: source,
                n_expected,
                n_actual,
            }
        },
        InferenceDiagnostic::NoBuiltinOverload {
            expression,
            builtin,
            ref parameters,
            name,
        } => {
            let pointer = source_map.expression_to_source(expression).ok()?.clone();
            let source = InFile::new(file_id, pointer);

            AnyDiagnostic::NoBuiltinOverload {
                expression: source,
                builtin,
                name,
                parameters: parameters.clone(),
            }
        },
        InferenceDiagnostic::AddressOfNotReference { expression, actual } => {
            let pointer = source_map.expression_to_source(expression).ok()?.clone();
            let source = InFile::new(file_id, pointer);

            AnyDiagnostic::AddressOfNotReference {
                expression: source,
                actual,
            }
        },
        InferenceDiagnostic::DerefNotAPointer { expression, actual } => {
            let pointer = source_map.expression_to_source(expression).ok()?.clone();
            let source = InFile::new(file_id, pointer);

            AnyDiagnostic::DerefNotPointer {
                expression: source,
                actual,
            }
        },
        InferenceDiagnostic::InvalidType {
            ref container,
            ref error,
        } => {
            let location = match *container {
                hir_ty::infer::TypeContainer::Expr(expression) => {
                    let expression = source_map.expression_to_source(expression).ok()?;
                    expression.syntax_node_pointer()
                },
                hir_ty::infer::TypeContainer::GlobalVar(id) => {
                    let source = GlobalVariable { id }.source(database)?;
                    SyntaxNodePointer::new(source.value.ty()?.syntax())
                },
                hir_ty::infer::TypeContainer::GlobalConstant(id) => {
                    let source = GlobalConstant { id }.source(database)?;
                    SyntaxNodePointer::new(source.value.ty()?.syntax())
                },
                hir_ty::infer::TypeContainer::Override(id) => {
                    let source = Override { id }.source(database)?;
                    SyntaxNodePointer::new(source.value.ty()?.syntax())
                },
                hir_ty::infer::TypeContainer::FunctionParameter(_, binding) => {
                    let binding = source_map.binding_to_source(binding).ok()?;
                    binding.syntax_node_pointer()
                },
                hir_ty::infer::TypeContainer::FunctionReturn(id) => {
                    let source = Function { id }.source(database)?;
                    SyntaxNodePointer::new(source.value.return_type()?.syntax())
                },
                hir_ty::infer::TypeContainer::VariableStatement(statement) => {
                    let statement = source_map.statement_to_source(statement).ok()?;
                    statement.syntax_node_pointer()
                },
                hir_ty::infer::TypeContainer::TypeAlias(id) => {
                    let source = TypeAlias { id }.source(database)?;
                    SyntaxNodePointer::new(source.value.type_declaration()?.syntax())
                },
            };
            AnyDiagnostic::InvalidType {
                file_id,
                location,
                error: error.clone(),
            }
        },
    })
}

pub(crate) fn any_diag_from_global_var(
    var_diagnostic: GlobalVariableDiagnostic,
    var: InFile<AstPointer<ast::GlobalVariableDeclaration>>,
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
    source_map: &BodySourceMap,
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
