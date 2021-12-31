use base_db::TextRange;
use hir::{
    diagnostics::{AnyDiagnostic, DiagnosticsConfig},
    HirDatabase, Semantics,
};
use hir_def::original_file_range;
use hir_ty::ty;
use itertools::Itertools;
use rowan::NodeOrToken;
use syntax::AstNode;
use vfs::FileId;

pub struct DiagnosticMessage {
    pub message: String,
    pub range: TextRange,
}

impl DiagnosticMessage {
    pub fn new(message: String, range: TextRange) -> Self {
        Self { message, range }
    }
}

pub fn diagnostics(
    db: &dyn HirDatabase,
    config: &DiagnosticsConfig,
    file_id: FileId,
) -> Vec<DiagnosticMessage> {
    let parse = db.parse(file_id);

    let mut diagnostics: Vec<_> = parse
        .errors()
        .iter()
        .map(|error| DiagnosticMessage {
            message: error.message(),
            range: error.range,
        })
        .collect();

    let sema = Semantics::new(db);

    let mut all = Vec::new();

    if config.show_type_errors {
        sema.module(file_id).diagnostics(db, config, &mut all);
    }

    for diagnostic in all {
        let file_id = diagnostic.file_id();
        let root = db.parse_or_resolve(file_id).unwrap().syntax();
        let msg = match diagnostic {
            AnyDiagnostic::AssignmentNotAReference { lhs, actual } => {
                let source = lhs.value.to_node(&root);
                let actual = ty::pretty::pretty_type(db, actual);
                let frange = original_file_range(db.upcast(), lhs.file_id, source.syntax());
                DiagnosticMessage::new(
                    format!(
                        "left hand side of assignment should be a reference, found {}",
                        actual
                    ),
                    frange.range,
                )
            }
            AnyDiagnostic::TypeMismatch {
                expr,
                expected,
                actual,
            } => {
                let source = expr.value.to_node(&root);
                let expected = ty::pretty::pretty_type_expectation(db, expected);
                let actual = ty::pretty::pretty_type(db, actual);
                let frange = original_file_range(db.upcast(), expr.file_id, source.syntax());
                DiagnosticMessage::new(
                    format!("expected {}, found {}", expected, actual),
                    frange.range,
                )
            }
            AnyDiagnostic::NoSuchField { expr, name, ty } => {
                let source = expr.value.to_node(&root).syntax().parent().unwrap();
                let ty = ty::pretty::pretty_type(db, ty);
                let frange = original_file_range(db.upcast(), expr.file_id, source.syntax());
                DiagnosticMessage::new(
                    format!("no field `{}` on type {}", name.as_ref(), ty),
                    frange.range,
                )
            }
            AnyDiagnostic::ArrayAccessInvalidType { expr, ty } => {
                let source = expr.value.to_node(&root);
                let ty = ty::pretty::pretty_type(db, ty);
                let frange = original_file_range(db.upcast(), expr.file_id, source.syntax());
                DiagnosticMessage::new(format!("can't index into type {}", ty), frange.range)
            }
            AnyDiagnostic::UnresolvedName { expr, name } => {
                let source = expr.value.to_node(&root);
                let frange = original_file_range(db.upcast(), expr.file_id, source.syntax());
                DiagnosticMessage::new(
                    format!("cannot find `{}` in this scope", name.as_str()),
                    frange.range,
                )
            }
            AnyDiagnostic::InvalidCallType { expr, ty } => {
                let source = expr.value.to_node(&root);
                let ty = ty::pretty::pretty_type(db, ty);
                let frange = original_file_range(db.upcast(), expr.file_id, source.syntax());
                DiagnosticMessage::new(
                    format!("can't call expression of type {}", ty),
                    frange.range,
                )
            }
            AnyDiagnostic::FunctionCallArgCountMismatch {
                expr,
                n_expected,
                n_actual,
            } => {
                let source = expr.value.to_node(&root).syntax().parent().unwrap();
                let frange = original_file_range(db.upcast(), expr.file_id, source.syntax());
                DiagnosticMessage::new(
                    format!("expected {} parameters, found {}", n_expected, n_actual),
                    frange.range,
                )
            }
            AnyDiagnostic::NoBuiltinOverload {
                expr,
                builtin,
                parameters,
                name,
            } => {
                let source = expr.value.to_node(&root).syntax().parent().unwrap();
                let builtin = builtin.lookup(db);

                let parameters = parameters
                    .iter()
                    .map(|ty| ty::pretty::pretty_type(db, *ty))
                    .join(", ");

                let possible = builtin
                    .overloads
                    .iter()
                    .map(|overload| ty::pretty::pretty_type(db, overload.ty))
                    .join("\n");

                let name = match name {
                    Some(name) => name,
                    None => builtin.name.as_str(),
                };

                let frange = original_file_range(db.upcast(), expr.file_id, source.syntax());
                DiagnosticMessage::new(
                    format!(
                        "no overload of `{}` found for given arguments. Found ({}), expected one of:\n{}",
                        name,
                        parameters,
                        possible
                    ),
                    frange.range,
                )
            }
            AnyDiagnostic::AddrOfNotRef { expr, actual } => {
                let source = expr.value.to_node(&root);
                let ty = ty::pretty::pretty_type(db, actual);
                let frange = original_file_range(db.upcast(), expr.file_id, source.syntax());
                DiagnosticMessage::new(format!("expected a reference, found {}", ty), frange.range)
            }
            AnyDiagnostic::DerefNotPtr { expr, actual } => {
                let source = expr.value.to_node(&root);
                let ty = ty::pretty::pretty_type(db, actual);
                let frange = original_file_range(db.upcast(), expr.file_id, source.syntax());
                DiagnosticMessage::new(
                    format!("cannot dereference expression of type {}", ty),
                    frange.range,
                )
            }
            AnyDiagnostic::MissingStorageClass { var } => {
                let var_decl = var.value.to_node(&root);
                let source = var_decl
                    .var_token()
                    .map(NodeOrToken::Token)
                    .unwrap_or(NodeOrToken::Node(var_decl.syntax()));

                let frange = original_file_range(db.upcast(), var.file_id, &source);
                DiagnosticMessage::new(
                    format!("missing storage class on global variable"),
                    frange.range,
                )
            }
            AnyDiagnostic::InvalidStorageClass { var, error } => {
                let var_decl = var.value.to_node(&root);
                let source = var_decl
                    .var_token()
                    .map(NodeOrToken::Token)
                    .unwrap_or(NodeOrToken::Node(var_decl.syntax()));
                let frange = original_file_range(db.upcast(), var.file_id, &source);
                DiagnosticMessage::new(format!("{}", error), frange.range)
            }
            AnyDiagnostic::InvalidType {
                file_id: _,
                location,
                error,
            } => {
                let source = location.to_node(&root);
                let frange = original_file_range(db.upcast(), file_id, source.syntax());
                DiagnosticMessage::new(format!("{}", error), frange.range)
            }
            AnyDiagnostic::UnresolvedImport { import } => {
                let source = import.value.to_node(&root);
                let frange = original_file_range(db.upcast(), file_id, source.syntax());
                DiagnosticMessage::new(format!("unresolved import"), frange.range)
            }
        };
        diagnostics.push(msg);
    }

    diagnostics
}
