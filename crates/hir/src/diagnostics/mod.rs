pub mod global_variable;
pub mod precedence;

use base_db::{FileRange, TextRange};
use hir_def::{HirFileId, InFile, body::BodySourceMap, expr::BinaryOp, module_data::Name};
use hir_ty::{
    HirDatabase,
    builtins::BuiltinId,
    infer::{InferenceDiagnostic, TypeExpectation, TypeLoweringError},
    ty::Ty,
    validate::StorageClassError,
};
use syntax::{
    AstNode, ast,
    ptr::{AstPtr, SyntaxNodePtr},
};

use self::{global_variable::GlobalVariableDiagnostic, precedence::PrecedenceDiagnostic};
use crate::{Function, GlobalConstant, GlobalVariable, HasSource, Override, TypeAlias};

pub struct DiagnosticsConfig {
    pub type_errors: bool,
    pub naga_parsing_errors: bool,
    pub naga_validation_errors: bool,
    pub naga_version: NagaVersion,
}

#[derive(Debug)]
pub enum NagaVersion {
    Naga14,
    Naga19,
    Naga22,
    NagaMain,
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
        lhs: InFile<AstPtr<ast::Expr>>,
        actual: Ty,
    },
    TypeMismatch {
        expr: InFile<AstPtr<ast::Expr>>,
        expected: TypeExpectation,
        actual: Ty,
    },
    NoSuchField {
        expr: InFile<AstPtr<ast::Expr>>,
        name: Name,
        ty: Ty,
    },
    ArrayAccessInvalidType {
        expr: InFile<AstPtr<ast::Expr>>,
        ty: Ty,
    },
    UnresolvedName {
        expr: InFile<AstPtr<ast::Expr>>,
        name: Name,
    },
    InvalidConstructionType {
        expr: InFile<AstPtr<ast::Expr>>,
        ty: Ty,
    },
    FunctionCallArgCountMismatch {
        expr: InFile<AstPtr<ast::Expr>>,
        n_expected: usize,
        n_actual: usize,
    },
    NoBuiltinOverload {
        expr: InFile<AstPtr<ast::Expr>>,
        builtin: BuiltinId,
        name: Option<&'static str>,
        parameters: Vec<Ty>,
    },
    AddrOfNotRef {
        expr: InFile<AstPtr<ast::Expr>>,
        actual: Ty,
    },
    DerefNotPointer {
        expression: InFile<AstPointer<ast::Expr>>,
        actual: Ty,
    },
    MissingStorageClass {
        var: InFile<AstPtr<ast::GlobalVariableDecl>>,
    },
    InvalidStorageClass {
        var: InFile<AstPtr<ast::GlobalVariableDecl>>,
        error: StorageClassError,
    },

    InvalidType {
        file_id: HirFileId,
        location: SyntaxNodePtr,
        error: TypeLoweringError,
    },

    UnresolvedImport {
        import: InFile<AstPtr<ast::Import>>,
    },

    PrecedenceParensRequired {
        expr: InFile<AstPtr<ast::Expr>>,
        op: BinaryOp,
        sequence_permitted: bool,
    },
    NagaValidationError {
        file_id: HirFileId,
        range: TextRange,
        message: String,
        related: Vec<(String, FileRange)>,
    },
    NoConstructor {
        expr: InFile<AstPtr<ast::Expr>>,
        builtins: [BuiltinId; 2],
        ty: Ty,
        parameters: Vec<Ty>,
    },
}

impl AnyDiagnostic {
    pub fn file_id(&self) -> HirFileId {
        match self {
            AnyDiagnostic::AssignmentNotAReference { left_side, .. } => left_side.file_id,
            AnyDiagnostic::TypeMismatch { expression, .. } => expression.file_id,
            AnyDiagnostic::NoSuchField { expression, .. } => expression.file_id,
            AnyDiagnostic::ArrayAccessInvalidType { expression, .. } => expression.file_id,
            AnyDiagnostic::UnresolvedName { expression, .. } => expression.file_id,
            AnyDiagnostic::InvalidConstructionType { expression, .. } => expression.file_id,
            AnyDiagnostic::FunctionCallArgCountMismatch { expression, .. } => expression.file_id,
            AnyDiagnostic::NoBuiltinOverload { expression, .. } => expression.file_id,
            AnyDiagnostic::AddrOfNotRef { expression, .. } => expression.file_id,
            AnyDiagnostic::DerefNotPointer { expression, .. } => expression.file_id,
            AnyDiagnostic::MissingStorageClass { var } => var.file_id,
            AnyDiagnostic::InvalidStorageClass { var, .. } => var.file_id,
            AnyDiagnostic::InvalidType { file_id, .. } => *file_id,
            AnyDiagnostic::UnresolvedImport { import, .. } => import.file_id,
            AnyDiagnostic::NagaValidationError { file_id, .. } => *file_id,
            AnyDiagnostic::ParseError { file_id, .. } => *file_id,
            AnyDiagnostic::UnconfiguredCode { file_id, .. } => *file_id,
            AnyDiagnostic::NoConstructor { expr, .. } => expr.file_id,
            AnyDiagnostic::PrecedenceParensRequired { expr, .. } => expr.file_id,
        }
    }
}

pub(crate) fn any_diag_from_infer_diagnostic(
    db: &dyn HirDatabase,
    infer_diagnostic: &InferenceDiagnostic,
    source_map: &BodySourceMap,
    file_id: HirFileId,
) -> Option<AnyDiagnostic> {
    Some(match *infer_diagnostic {
        InferenceDiagnostic::AssignmentNotAReference { left_side, actual } => {
            let pointer = source_map.expression_to_source(left_side).ok()?.clone();
            let source = InFile::new(file_id, pointer);
            AnyDiagnostic::AssignmentNotAReference {
                lhs: source,
                actual,
            }
        },
        InferenceDiagnostic::TypeMismatch {
            expr,
            ref expected,
            actual,
        } => {
            let pointer = source_map.expression_to_source(expression).ok()?.clone();
            let source = InFile::new(file_id, pointer);
            AnyDiagnostic::TypeMismatch {
                expr: source,
                expected: expected.clone(),
                actual,
            }
        },
        InferenceDiagnostic::NoSuchField {
            expression,
            ref name,
            ty,
        } => {
            let pointer = source_map.expression_to_source(expression).ok()?.clone();
            let source = InFile::new(file_id, pointer);

            AnyDiagnostic::NoSuchField {
                expr: source,
                name: name.clone(),
                ty,
            }
        },
        InferenceDiagnostic::ArrayAccessInvalidType { expression, ty } => {
            let pointer = source_map.expression_to_source(expression).ok()?.clone();
            let source = InFile::new(file_id, pointer);

            AnyDiagnostic::ArrayAccessInvalidType { expr: source, ty }
        },
        InferenceDiagnostic::UnresolvedName {
            expression,
            ref name,
        } => {
            let pointer = source_map.expression_to_source(expression).ok()?.clone();
            let source = InFile::new(file_id, pointer);

            AnyDiagnostic::UnresolvedName {
                expr: source,
                name: name.clone(),
            }
        },
        InferenceDiagnostic::InvalidConstructionType { expression, ty } => {
            let pointer = source_map.expression_to_source(expression).ok()?.clone();
            let source = InFile::new(file_id, pointer);

            AnyDiagnostic::InvalidConstructionType { expr: source, ty }
        },
        InferenceDiagnostic::NoConstructor {
            expr,
            ty,
            ref builtins,
            ref parameters,
        } => {
            let pointer = source_map.expression_to_source(expression).ok()?.clone();
            let source = InFile::new(file_id, pointer);

            AnyDiagnostic::NoConstructor {
                expr: source,
                builtins: *builtins,
                ty,
                parameters: parameters.clone(),
            }
        },
        InferenceDiagnostic::FunctionCallArgCountMismatch {
            expr,
            n_expected,
            n_actual,
        } => {
            let pointer = source_map.expression_to_source(expression).ok()?.clone();
            let source = InFile::new(file_id, pointer);

            AnyDiagnostic::FunctionCallArgCountMismatch {
                expr: source,
                n_expected,
                n_actual,
            }
        },
        InferenceDiagnostic::NoBuiltinOverload {
            expr,
            builtin,
            ref parameters,
            name,
        } => {
            let pointer = source_map.expression_to_source(expression).ok()?.clone();
            let source = InFile::new(file_id, pointer);

            AnyDiagnostic::NoBuiltinOverload {
                expr: source,
                builtin,
                name,
                parameters: parameters.clone(),
            }
        },
        InferenceDiagnostic::AddrOfNotRef { expression, actual } => {
            let pointer = source_map.expression_to_source(expression).ok()?.clone();
            let source = InFile::new(file_id, pointer);

            AnyDiagnostic::AddrOfNotRef {
                expr: source,
                actual,
            }
        },
        InferenceDiagnostic::DerefNotAPtr { expression, actual } => {
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
                    let source = GlobalVariable { id }.source(db.upcast())?;
                    SyntaxNodePtr::new(source.value.ty()?.syntax())
                },
                hir_ty::infer::TypeContainer::GlobalConstant(id) => {
                    let source = GlobalConstant { id }.source(db.upcast())?;
                    SyntaxNodePtr::new(source.value.ty()?.syntax())
                },
                hir_ty::infer::TypeContainer::Override(id) => {
                    let source = Override { id }.source(db.upcast())?;
                    SyntaxNodePtr::new(source.value.ty()?.syntax())
                },
                hir_ty::infer::TypeContainer::FunctionParameter(_, binding) => {
                    let binding = source_map.binding_to_source(binding).ok()?;
                    binding.syntax_node_pointer()
                },
                hir_ty::infer::TypeContainer::FunctionReturn(id) => {
                    let source = Function { id }.source(db.upcast())?;
                    SyntaxNodePtr::new(source.value.return_type()?.syntax())
                },
                hir_ty::infer::TypeContainer::VariableStatement(statement) => {
                    let statement = source_map.statement_to_source(statement).ok()?;
                    statement.syntax_node_pointer()
                },
                hir_ty::infer::TypeContainer::TypeAlias(id) => {
                    let source = TypeAlias { id }.source(db.upcast())?;
                    SyntaxNodePtr::new(source.value.type_decl()?.syntax())
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
        GlobalVariableDiagnostic::MissingStorageClass => AnyDiagnostic::MissingStorageClass { var },
        GlobalVariableDiagnostic::StorageClassError(error) => {
            AnyDiagnostic::InvalidStorageClass { var, error }
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
                expr: source,
                op: *op,
                sequence_permitted: false,
            })
        },
        PrecedenceDiagnostic::SequencesAllowed(expression, op) => {
            let pointer = source_map.expression_to_source(*expression).ok()?.clone();
            let source = InFile::new(file_id, pointer);
            Some(AnyDiagnostic::PrecedenceParensRequired {
                expr: source,
                op: *op,
                sequence_permitted: true,
            })
        },
    }
}
