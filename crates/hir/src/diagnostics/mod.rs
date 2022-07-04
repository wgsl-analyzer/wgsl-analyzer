pub mod global_variable;

use base_db::{FileRange, TextRange};
use hir_def::{body::BodySourceMap, module_data::Name, HirFileId, InFile};
use hir_ty::{
    builtins::BuiltinId,
    infer::{InferenceDiagnostic, TypeExpectation, TypeLoweringError},
    ty::Ty,
    validate::StorageClassError,
    HirDatabase,
};
use syntax::{
    ast,
    ptr::{AstPtr, SyntaxNodePtr},
    AstNode,
};

use crate::{Function, GlobalConstant, GlobalVariable, HasSource, TypeAlias};

use self::global_variable::GlobalVariableDiagnostic;

pub struct DiagnosticsConfig {
    pub type_errors: bool,
    pub naga_parsing_errors: bool,
    pub naga_validation_errors: bool,
    pub naga_version: NagaVersion,
}

#[derive(Debug)]
pub enum NagaVersion {
    Naga08,
    Naga09,
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
    InvalidCallType {
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
    DerefNotPtr {
        expr: InFile<AstPtr<ast::Expr>>,
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

    NagaValidationError {
        file_id: HirFileId,
        range: TextRange,
        message: String,
        related: Vec<(String, FileRange)>,
    },
}

impl AnyDiagnostic {
    pub fn file_id(&self) -> HirFileId {
        match self {
            AnyDiagnostic::AssignmentNotAReference { lhs, .. } => lhs.file_id,
            AnyDiagnostic::TypeMismatch { expr, .. } => expr.file_id,
            AnyDiagnostic::NoSuchField { expr, .. } => expr.file_id,
            AnyDiagnostic::ArrayAccessInvalidType { expr, .. } => expr.file_id,
            AnyDiagnostic::UnresolvedName { expr, .. } => expr.file_id,
            AnyDiagnostic::InvalidCallType { expr, .. } => expr.file_id,
            AnyDiagnostic::FunctionCallArgCountMismatch { expr, .. } => expr.file_id,
            AnyDiagnostic::NoBuiltinOverload { expr, .. } => expr.file_id,
            AnyDiagnostic::AddrOfNotRef { expr, .. } => expr.file_id,
            AnyDiagnostic::DerefNotPtr { expr, .. } => expr.file_id,
            AnyDiagnostic::MissingStorageClass { var } => var.file_id,
            AnyDiagnostic::InvalidStorageClass { var, .. } => var.file_id,
            AnyDiagnostic::InvalidType { file_id, .. } => *file_id,
            AnyDiagnostic::UnresolvedImport { import, .. } => import.file_id,
            AnyDiagnostic::NagaValidationError { file_id, .. } => *file_id,
            AnyDiagnostic::ParseError { file_id, .. } => *file_id,
            AnyDiagnostic::UnconfiguredCode { file_id, .. } => *file_id,
        }
    }
}

pub(crate) fn any_diag_from_infer_diag(
    db: &dyn HirDatabase,
    infer_diag: &InferenceDiagnostic,
    source_map: &BodySourceMap,
    file_id: HirFileId,
) -> Option<AnyDiagnostic> {
    Some(match *infer_diag {
        InferenceDiagnostic::AssignmentNotAReference { lhs, actual } => {
            let ptr = source_map.expr_to_source(lhs).ok()?.clone();
            let source = InFile::new(file_id, ptr);
            AnyDiagnostic::AssignmentNotAReference {
                lhs: source,
                actual,
            }
        }
        InferenceDiagnostic::TypeMismatch {
            expr,
            ref expected,
            actual,
        } => {
            let ptr = source_map.expr_to_source(expr).ok()?.clone();
            let source = InFile::new(file_id, ptr);
            AnyDiagnostic::TypeMismatch {
                expr: source,
                expected: expected.clone(),
                actual,
            }
        }
        InferenceDiagnostic::NoSuchField { expr, ref name, ty } => {
            let ptr = source_map.expr_to_source(expr).ok()?.clone();
            let source = InFile::new(file_id, ptr);

            AnyDiagnostic::NoSuchField {
                expr: source,
                name: name.clone(),
                ty,
            }
        }
        InferenceDiagnostic::ArrayAccessInvalidType { expr, ty } => {
            let ptr = source_map.expr_to_source(expr).ok()?.clone();
            let source = InFile::new(file_id, ptr);

            AnyDiagnostic::ArrayAccessInvalidType { expr: source, ty }
        }
        InferenceDiagnostic::UnresolvedName { expr, ref name } => {
            let ptr = source_map.expr_to_source(expr).ok()?.clone();
            let source = InFile::new(file_id, ptr);

            AnyDiagnostic::UnresolvedName {
                expr: source,
                name: name.clone(),
            }
        }
        InferenceDiagnostic::InvalidCallType { expr, ty } => {
            let ptr = source_map.expr_to_source(expr).ok()?.clone();
            let source = InFile::new(file_id, ptr);

            AnyDiagnostic::InvalidCallType { expr: source, ty }
        }
        InferenceDiagnostic::FunctionCallArgCountMismatch {
            expr,
            n_expected,
            n_actual,
        } => {
            let ptr = source_map.expr_to_source(expr).ok()?.clone();
            let source = InFile::new(file_id, ptr);

            AnyDiagnostic::FunctionCallArgCountMismatch {
                expr: source,
                n_expected,
                n_actual,
            }
        }
        InferenceDiagnostic::NoBuiltinOverload {
            expr,
            builtin,
            ref parameters,
            name,
        } => {
            let ptr = source_map.expr_to_source(expr).ok()?.clone();
            let source = InFile::new(file_id, ptr);

            AnyDiagnostic::NoBuiltinOverload {
                expr: source,
                builtin,
                name,
                parameters: parameters.clone(),
            }
        }
        InferenceDiagnostic::AddrOfNotRef { expr, actual } => {
            let ptr = source_map.expr_to_source(expr).ok()?.clone();
            let source = InFile::new(file_id, ptr);

            AnyDiagnostic::AddrOfNotRef {
                expr: source,
                actual,
            }
        }
        InferenceDiagnostic::DerefNotAPtr { expr, actual } => {
            let ptr = source_map.expr_to_source(expr).ok()?.clone();
            let source = InFile::new(file_id, ptr);

            AnyDiagnostic::DerefNotPtr {
                expr: source,
                actual,
            }
        }
        InferenceDiagnostic::InvalidType {
            ref container,
            ref error,
        } => {
            let location = match *container {
                hir_ty::infer::TypeContainer::Expr(expr) => {
                    let expr = source_map.expr_to_source(expr).ok()?;
                    expr.syntax_node_ptr()
                }
                hir_ty::infer::TypeContainer::GlobalVar(id) => {
                    let source = GlobalVariable { id }.source(db.upcast())?;
                    SyntaxNodePtr::new(source.value.ty()?.syntax())
                }
                hir_ty::infer::TypeContainer::GlobalConstant(id) => {
                    let source = GlobalConstant { id }.source(db.upcast())?;
                    SyntaxNodePtr::new(source.value.ty()?.syntax())
                }
                hir_ty::infer::TypeContainer::FunctionParameter(_, binding) => {
                    let binding = source_map.binding_to_source(binding).ok()?;
                    binding.syntax_node_ptr()
                }
                hir_ty::infer::TypeContainer::FunctionReturn(id) => {
                    let source = Function { id }.source(db.upcast())?;
                    SyntaxNodePtr::new(source.value.return_type()?.syntax())
                }
                hir_ty::infer::TypeContainer::VariableStatement(stmt) => {
                    let stmt = source_map.stmt_to_source(stmt).ok()?;
                    stmt.syntax_node_ptr()
                }
                hir_ty::infer::TypeContainer::TypeAlias(id) => {
                    let source = TypeAlias { id }.source(db.upcast())?;
                    SyntaxNodePtr::new(source.value.type_decl()?.syntax())
                }
            };
            AnyDiagnostic::InvalidType {
                file_id,
                location,
                error: error.clone(),
            }
        }
    })
}

pub(crate) fn any_diag_from_global_var(
    var_diag: GlobalVariableDiagnostic,
    var: InFile<AstPtr<ast::GlobalVariableDecl>>,
) -> AnyDiagnostic {
    match var_diag {
        GlobalVariableDiagnostic::MissingStorageClass => AnyDiagnostic::MissingStorageClass { var },
        GlobalVariableDiagnostic::StorageClassError(error) => {
            AnyDiagnostic::InvalidStorageClass { var, error }
        }
    }
}
