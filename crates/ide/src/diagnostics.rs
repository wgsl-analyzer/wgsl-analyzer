use std::ops::Range;

use base_db::{FileRange, TextRange, TextSize};
use hir::{
    diagnostics::{AnyDiagnostic, DiagnosticsConfig, NagaVersion},
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
        DiagnosticMessage { severity, ..self }
    }

    pub fn unused(self) -> Self {
        DiagnosticMessage {
            unused: true,
            ..self
        }
    }
}

trait Naga {
    type Module;
    type ParseError: NagaError;
    type ValidationError: NagaError;

    fn parse(source: &str) -> Result<Self::Module, Self::ParseError>;
    fn validate(module: &Self::Module) -> Result<(), Self::ValidationError>;
}

trait NagaError: std::error::Error {
    fn spans<'a>(&'a self) -> Box<dyn Iterator<Item = (Range<usize>, String)> + 'a>;
    fn has_spans(&self) -> bool;
}

struct Naga08;
impl Naga for Naga08 {
    type Module = naga08::Module;
    type ParseError = naga08::front::wgsl::ParseError;
    type ValidationError = naga08::WithSpan<naga08::valid::ValidationError>;

    fn parse(source: &str) -> Result<Self::Module, Self::ParseError> {
        naga08::front::wgsl::parse_str(source)
    }

    fn validate(module: &Self::Module) -> Result<(), Self::ValidationError> {
        let flags = naga08::valid::ValidationFlags::all();
        let capabilities = naga08::valid::Capabilities::all();
        let mut validator = naga08::valid::Validator::new(flags, capabilities);
        validator.validate(module).map(drop)
    }
}
impl NagaError for naga08::front::wgsl::ParseError {
    fn spans<'a>(&'a self) -> Box<dyn Iterator<Item = (Range<usize>, String)> + 'a> {
        Box::new(
            self.labels()
                .map(|(range, label)| (range, label.to_string())),
        )
    }
    fn has_spans(&self) -> bool {
        self.labels().len() > 0
    }
}
impl NagaError for naga08::WithSpan<naga08::valid::ValidationError> {
    fn spans<'a>(&'a self) -> Box<dyn Iterator<Item = (Range<usize>, String)> + 'a> {
        Box::new(
            self.spans()
                .filter_map(move |(span, label)| Some((span.to_range()?, label.clone()))),
        )
    }
    fn has_spans(&self) -> bool {
        self.spans().len() > 0
    }
}


struct Naga09;
impl Naga for Naga09 {
    type Module = naga09::Module;
    type ParseError = naga09::front::wgsl::ParseError;
    type ValidationError = naga09::WithSpan<naga09::valid::ValidationError>;

    fn parse(source: &str) -> Result<Self::Module, Self::ParseError> {
        naga09::front::wgsl::parse_str(source)
    }

    fn validate(module: &Self::Module) -> Result<(), Self::ValidationError> {
        let flags = naga09::valid::ValidationFlags::all();
        let capabilities = naga09::valid::Capabilities::all();
        let mut validator = naga09::valid::Validator::new(flags, capabilities);
        validator.validate(module).map(drop)
    }
}
impl NagaError for naga09::front::wgsl::ParseError {
    fn spans<'a>(&'a self) -> Box<dyn Iterator<Item = (Range<usize>, String)> + 'a> {
        Box::new(
            self.labels()
                .map(|(range, label)| (range, label.to_string())),
        )
    }
    fn has_spans(&self) -> bool {
        self.labels().len() > 0
    }
}
impl NagaError for naga09::WithSpan<naga09::valid::ValidationError> {
    fn spans<'a>(&'a self) -> Box<dyn Iterator<Item = (Range<usize>, String)> + 'a> {
        Box::new(
            self.spans()
                .filter_map(move |(span, label)| Some((span.to_range()?, label.clone()))),
        )
    }
    fn has_spans(&self) -> bool {
        self.spans().len() > 0
    }
}


struct NagaMain;
impl Naga for NagaMain {
    type Module = nagamain::Module;
    type ParseError = nagamain::front::wgsl::ParseError;
    type ValidationError = nagamain::WithSpan<nagamain::valid::ValidationError>;

    fn parse(source: &str) -> Result<Self::Module, Self::ParseError> {
        nagamain::front::wgsl::parse_str(source)
    }

    fn validate(module: &Self::Module) -> Result<(), Self::ValidationError> {
        let flags = nagamain::valid::ValidationFlags::all();
        let capabilities = nagamain::valid::Capabilities::all();
        let mut validator = nagamain::valid::Validator::new(flags, capabilities);
        validator.validate(module).map(drop)
    }
}
impl NagaError for nagamain::front::wgsl::ParseError {
    fn spans<'a>(&'a self) -> Box<dyn Iterator<Item = (Range<usize>, String)> + 'a> {
        Box::new(
            self.labels()
                .map(|(range, label)| (range, label.to_string())),
        )
    }
    fn has_spans(&self) -> bool {
        self.labels().len() > 0
    }
}
impl NagaError for nagamain::WithSpan<nagamain::valid::ValidationError> {
    fn spans<'a>(&'a self) -> Box<dyn Iterator<Item = (Range<usize>, String)> + 'a> {
        Box::new(
            self.spans()
                .filter_map(move |(span, label)| Some((span.to_range()?, label.clone()))),
        )
    }
    fn has_spans(&self) -> bool {
        self.spans().len() > 0
    }
}

#[allow(dead_code)]
enum NagaErrorPolicy {
    SeparateSpans,
    SmallestSpan,
    Related,
}

impl NagaErrorPolicy {
    fn emit<E: NagaError>(
        &self,
        db: &dyn HirDatabase,
        error: E,
        file_id: FileId,
        full_range: TextRange,
        acc: &mut Vec<AnyDiagnostic>,
    ) {
        let message = err_message_cause_chain("naga: ", &error);

        if !error.has_spans() {
            acc.push(AnyDiagnostic::NagaValidationError {
                file_id: file_id.into(),
                range: full_range,
                message,
                related: Vec::new(),
            });
            return;
        }

        let original_range = |range: std::ops::Range<usize>| {
            let range_in_full = TextRange::new(
                TextSize::from(range.start as u32),
                TextSize::from(range.end as u32),
            );
            db.text_range_from_full(file_id.into(), range_in_full)
        };

        let spans = error.spans().filter_map(|(span, label)| {
            let range = original_range(span).ok()?;
            Some((range, label))
        });

        match *self {
            NagaErrorPolicy::SeparateSpans => {
                spans.for_each(|(range, label)| {
                    acc.push(AnyDiagnostic::NagaValidationError {
                        file_id: file_id.into(),
                        range,
                        message: format!("{}: {}", message, label),
                        related: Vec::new(),
                    });
                });
            }
            NagaErrorPolicy::SmallestSpan => {
                if let Some((range, _)) = spans.min_by_key(|(range, _)| range.len()) {
                    acc.push(AnyDiagnostic::NagaValidationError {
                        file_id: file_id.into(),
                        range,
                        message,
                        related: Vec::new(),
                    });
                }
            }
            NagaErrorPolicy::Related => {
                let related: Vec<_> = spans
                    .map(|(range, message)| (message, FileRange { range, file_id }))
                    .collect();
                let min_range = related
                    .iter()
                    .map(|(_, frange)| frange.range)
                    .min_by_key(|range| range.len())
                    .unwrap_or(full_range);

                acc.push(AnyDiagnostic::NagaValidationError {
                    file_id: file_id.into(),
                    range: min_range,
                    message,
                    related,
                });
            }
        }
    }
}
fn naga_diagnostics<N: Naga>(
    db: &dyn HirDatabase,
    file_id: FileId,
    config: &DiagnosticsConfig,
    acc: &mut Vec<AnyDiagnostic>,
) -> Result<(), ()> {
    let source = match db.resolve_full_source(file_id.into()) {
        Ok(source) => source,
        Err(_) => return Ok(()),
    };

    let full_range = TextRange::new(0.into(), TextSize::from(source.len() as u32 - 1));

    let policy = NagaErrorPolicy::Related;
    match N::parse(&source) {
        Ok(module) => {
            if !config.naga_validation_errors {
                return Ok(());
            }
            if let Err(error) = N::validate(&module) {
                policy.emit(db, error, file_id, full_range, acc);
            }
        }
        Err(error) => {
            if !config.naga_parsing_errors {
                return Ok(());
            }
            policy.emit(db, error, file_id, full_range, acc);
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
        match &config.naga_version {
            NagaVersion::Naga08 => {
                let _ = naga_diagnostics::<Naga08>(db, file_id, config, &mut diagnostics);
            }
            NagaVersion::Naga09 => {
                let _ = naga_diagnostics::<Naga09>(db, file_id, config, &mut diagnostics);
            }
            NagaVersion::NagaMain => {
                let _ = naga_diagnostics::<NagaMain>(db, file_id, config, &mut diagnostics);
            }
        }
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
            AnyDiagnostic::NagaValidationError { message, range, related, .. } => {
                let mut msg = DiagnosticMessage::new(message, range);
                msg.related = related;
                msg
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

fn err_message_cause_chain(prefix: &str, error: &dyn std::error::Error) -> String {
    let mut msg = format!("{}{}", prefix, error);

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
