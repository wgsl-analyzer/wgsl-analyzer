use base_db::{FileRange, TextRange, TextSize};
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
    pub unused: bool,
    pub severity: Severity,
    pub related: Vec<(String, FileRange)>,
}

#[derive(Clone, Copy)]
pub enum Severity {
    Error,
    WeakWarning,
}

impl DiagnosticMessage {
    pub fn new(message: String, range: TextRange) -> Self {
        Self {
            message,
            range,
            severity: Severity::Error,
            unused: false,
            related: Vec::new(),
        }
    }

    pub fn with_severity(self, severity: Severity) -> Self {
        DiagnosticMessage {
            severity: severity,
            ..self
        }
    }

    pub fn unused(self) -> Self {
        DiagnosticMessage {
            unused: true,
            ..self
        }
    }
}

fn naga_diagnostics(
    db: &dyn HirDatabase,
    file_id: FileId,
    config: &DiagnosticsConfig,
    acc: &mut Vec<AnyDiagnostic>,
) -> Result<(), ()> {
    let source = match db.resolve_full_source(file_id.into()) {
        Ok(source) => source,
        Err(_) => return Ok(()),
    };

    let original_range = |range: std::ops::Range<usize>| {
        let range_in_full = TextRange::new(
            TextSize::from(range.start as u32),
            TextSize::from(range.end as u32),
        );
        db.text_range_from_full(file_id.into(), range_in_full)
    };

    match naga::front::wgsl::parse_str(&source) {
        Ok(module) => {
            if !config.naga_validation_errors {
                return Ok(());
            }

            let flags = naga::valid::ValidationFlags::all();
            let capabilities = naga::valid::Capabilities::all();
            let mut validator = naga::valid::Validator::new(flags, capabilities);
            let error = match validator.validate(&module) {
                Err(e) => e,
                _ => return Ok(()),
            };

            let full_range = || TextRange::new(0.into(), TextSize::from(source.len() as u32 - 1));

            let message = err_message_cause_chain(&error);

            if error.spans().len() == 0 {
                acc.push(AnyDiagnostic::NagaValidationError {
                    file_id: file_id.into(),
                    range: full_range(),
                    message: message,
                })
            } else {
                error
                    .spans()
                    .filter_map(|(span, label)| {
                        let range = span
                            .to_range()
                            .map(|range| original_range(range))
                            .transpose()
                            .ok()?
                            .unwrap_or_else(full_range);
                        Some((range, label))
                    })
                    .for_each(|(range, label)| {
                        acc.push(AnyDiagnostic::NagaValidationError {
                            file_id: file_id.into(),
                            range,
                            message: format!("{}: {}", message, label),
                        })
                    });
            }
        }
        Err(error) => {
            if !config.naga_parsing_errors {
                return Ok(());
            }

            let message = err_message_cause_chain(&error);
            error
                .labels()
                .filter_map(|(span, label)| Some((original_range(span).ok()?, label)))
                .for_each(|(range, label)| {
                    acc.push(AnyDiagnostic::NagaValidationError {
                        file_id: file_id.into(),
                        range,
                        message: format!("{}: {}", message, label),
                    })
                });
        }
    }

    Ok(())
}

pub fn diagnostics(
    db: &dyn HirDatabase,
    config: &DiagnosticsConfig,
    file_id: FileId,
) -> Vec<DiagnosticMessage> {
    let (parse, unconfigured) = db.parse_with_unconfigured(file_id);

    let mut diagnostics = Vec::new();

    diagnostics.extend(
        parse
            .errors()
            .iter()
            .map(|error| AnyDiagnostic::ParseError {
                message: error.message(),
                range: error.range,
                file_id: file_id.into(),
            }),
    );

    diagnostics.extend(
        unconfigured
            .iter()
            .map(|unconfigured| AnyDiagnostic::UnconfiguredCode {
                def: unconfigured.def.clone(),
                range: unconfigured.range,
                file_id: file_id.into(),
            }),
    );

    let sema = Semantics::new(db);

    if config.type_errors {
        sema.module(file_id)
            .diagnostics(db, config, &mut diagnostics);
    }

    if config.naga_parsing_errors || config.naga_validation_errors {
        let _ = naga_diagnostics(db, file_id, config, &mut diagnostics);
    }

    diagnostics.into_iter().map(|diagnostic| {
        let file_id = diagnostic.file_id();
        let root = db.parse_or_resolve(file_id).unwrap().syntax();
         match diagnostic {
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
                    .overloads()
                    .map(|(_, overload)| ty::pretty::pretty_type(db, overload.ty))
                    .join("\n");

                let name = match name {
                    Some(name) => name,
                    None => builtin.name(),
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
                    .unwrap_or_else(|| NodeOrToken::Node(var_decl.syntax()));

                let frange = original_file_range(db.upcast(), var.file_id, &source);
                DiagnosticMessage::new(
                    "missing storage class on global variable".to_string(),
                    frange.range,
                )
            }
            AnyDiagnostic::InvalidStorageClass { var, error } => {
                let var_decl = var.value.to_node(&root);
                let source = var_decl
                    .var_token()
                    .map(NodeOrToken::Token)
                    .unwrap_or_else(|| NodeOrToken::Node(var_decl.syntax()));
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
                DiagnosticMessage::new("unresolved import".to_string(), frange.range)
            }
            AnyDiagnostic::NagaValidationError { message, range, .. } => {
                DiagnosticMessage::new(message, range)
            }
            AnyDiagnostic::ParseError { message, range, .. } => {
                DiagnosticMessage::new(message, range)
            }
            AnyDiagnostic::UnconfiguredCode { def, range, .. } => DiagnosticMessage::new(
                format!(
                    "code is inactive due to `#ifdef` directives: `{}` is not enabled",
                    def
                ),
                range,
            )
            .with_severity(Severity::WeakWarning)
            .unused(),
        }
    }).collect()
}

fn err_message_cause_chain(error: &dyn std::error::Error) -> String {
    let mut msg = error.to_string();

    let mut e = error.source();
    if e.is_some() {
        msg.push_str(": ");
    }

    while let Some(source) = e {
        msg.push_str(&source.to_string());
        e = source.source();
    }

    msg
}
