use std::ops::Range;

use base_db::{FileRange, TextRange, TextSize};
use hir::{
    diagnostics::{AnyDiagnostic, DiagnosticsConfig, NagaVersion},
    HirDatabase, Semantics,
};
use hir_def::original_file_range;
use hir_ty::ty::{
    self,
    pretty::{pretty_fn, pretty_type},
    Ty, VecSize,
};
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

struct Naga10;
impl Naga for Naga10 {
    type Module = naga10::Module;
    type ParseError = naga10::front::wgsl::ParseError;
    type ValidationError = naga10::WithSpan<naga10::valid::ValidationError>;

    fn parse(source: &str) -> Result<Self::Module, Self::ParseError> {
        naga10::front::wgsl::parse_str(source)
    }

    fn validate(module: &Self::Module) -> Result<(), Self::ValidationError> {
        let flags = naga10::valid::ValidationFlags::all();
        let capabilities = naga10::valid::Capabilities::all();
        let mut validator = naga10::valid::Validator::new(flags, capabilities);
        validator.validate(module).map(drop)
    }
}
impl NagaError for naga10::front::wgsl::ParseError {
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
impl NagaError for naga10::WithSpan<naga10::valid::ValidationError> {
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

struct Naga11;
impl Naga for Naga11 {
    type Module = naga11::Module;
    type ParseError = naga11::front::wgsl::ParseError;
    type ValidationError = naga11::WithSpan<naga11::valid::ValidationError>;

    fn parse(source: &str) -> Result<Self::Module, Self::ParseError> {
        naga11::front::wgsl::parse_str(source)
    }

    fn validate(module: &Self::Module) -> Result<(), Self::ValidationError> {
        let flags = naga11::valid::ValidationFlags::all();
        let capabilities = naga11::valid::Capabilities::all();
        let mut validator = naga11::valid::Validator::new(flags, capabilities);
        validator.validate(module).map(drop)
    }
}
impl NagaError for naga11::front::wgsl::ParseError {
    fn spans<'a>(&'a self) -> Box<dyn Iterator<Item = (Range<usize>, String)> + 'a> {
        Box::new(
            self.labels()
                .flat_map(|(range, label)| Some((range.to_range()?, label.to_string()))),
        )
    }
    fn has_spans(&self) -> bool {
        self.labels().len() > 0
    }
}
impl NagaError for naga11::WithSpan<naga11::valid::ValidationError> {
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

struct Naga13;
impl Naga for Naga13 {
    type Module = naga13::Module;
    type ParseError = naga13::front::wgsl::ParseError;
    type ValidationError = naga13::WithSpan<naga13::valid::ValidationError>;

    fn parse(source: &str) -> Result<Self::Module, Self::ParseError> {
        naga13::front::wgsl::parse_str(source)
    }

    fn validate(module: &Self::Module) -> Result<(), Self::ValidationError> {
        let flags = naga13::valid::ValidationFlags::all();
        let capabilities = naga13::valid::Capabilities::all();
        let mut validator = naga13::valid::Validator::new(flags, capabilities);
        validator.validate(module).map(drop)
    }
}
impl NagaError for naga13::front::wgsl::ParseError {
    fn spans<'a>(&'a self) -> Box<dyn Iterator<Item = (Range<usize>, String)> + 'a> {
        Box::new(
            self.labels()
                .flat_map(|(range, label)| Some((range.to_range()?, label.to_string()))),
        )
    }
    fn has_spans(&self) -> bool {
        self.labels().len() > 0
    }
}
impl NagaError for naga13::WithSpan<naga13::valid::ValidationError> {
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
                .flat_map(|(range, label)| Some((range.to_range()?, label.to_string()))),
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

    let full_range = TextRange::up_to(TextSize::of(&source));

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
            NagaVersion::Naga13 => {
                let _ = naga_diagnostics::<Naga13>(db, file_id, config, &mut diagnostics);
            }
            NagaVersion::Naga11 => {
                let _ = naga_diagnostics::<Naga11>(db, file_id, config, &mut diagnostics);
            }
            NagaVersion::Naga10 => {
                let _ = naga_diagnostics::<Naga10>(db, file_id, config, &mut diagnostics);
            }
            NagaVersion::NagaMain => {
                let _ = naga_diagnostics::<NagaMain>(db, file_id, config, &mut diagnostics);
            }
        }
    }

    diagnostics
        .into_iter()
        .map(|diagnostic| {
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
                AnyDiagnostic::InvalidConstructionType { expr, ty } => {
                    let source = expr.value.to_node(&root);
                    let ty = ty::pretty::pretty_type(db, ty);
                    let frange = original_file_range(db.upcast(), expr.file_id, source.syntax());
                    DiagnosticMessage::new(
                        format!("can't construct value of type {}", ty),
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
                        .map(|(_, overload)| pretty_fn(db, &overload.ty.lookup(db)))
                        .join("\n");

                    let name = match name {
                        Some(name) => name,
                        None => builtin.name(),
                    };

                    let frange = original_file_range(db.upcast(), expr.file_id, source.syntax());
                    DiagnosticMessage::new(
                        format!(
                            "no overload of `{}` found for given arguments.\
                        Found ({}), expected one of:\n{}",
                            name, parameters, possible
                        ),
                        frange.range,
                    )
                }
                AnyDiagnostic::AddrOfNotRef { expr, actual } => {
                    let source = expr.value.to_node(&root);
                    let ty = ty::pretty::pretty_type(db, actual);
                    let frange = original_file_range(db.upcast(), expr.file_id, source.syntax());
                    DiagnosticMessage::new(
                        format!("expected a reference, found {}", ty),
                        frange.range,
                    )
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
                AnyDiagnostic::NagaValidationError {
                    message,
                    range,
                    related,
                    ..
                } => {
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
                AnyDiagnostic::NoConstructor {
                    expr,
                    builtins: [specific, general],
                    ty,
                    parameters,
                } => {
                    let source = expr.value.to_node(&root).syntax().clone();

                    let parameters = parameters
                        .iter()
                        .map(|ty| ty::pretty::pretty_type(db, *ty))
                        .join(", ");

                    let mut possible = Vec::with_capacity(32);
                    let builtin_specific = specific.lookup(db);
                    possible.extend(
                        builtin_specific
                            .overloads()
                            .map(|(_, overload)| pretty_fn(db, &overload.ty.lookup(db))),
                    );
                    let builtin_general = general.lookup(db);
                    possible.extend(
                        builtin_general
                            .overloads()
                            .filter(|(_, overload)| {
                                let function = overload.ty.lookup(db);
                                if let Some(return_ty) = function.return_type {
                                    convert_compatible(db, ty, return_ty)
                                } else {
                                    true
                                }
                            })
                            .map(|(_, overload)| pretty_fn(db, &overload.ty.lookup(db))),
                    );

                    let possible = possible.join("\n");

                    let frange = original_file_range(db.upcast(), expr.file_id, source.syntax());
                    DiagnosticMessage::new(
                        format!(
                            "no overload of constructor `{}` found for given\
                            arguments. Found ({parameters}), expected one of:\n{possible}",
                            pretty_type(db, ty),
                        ),
                        frange.range,
                    )
                }
                AnyDiagnostic::PrecedenceParensRequired {
                    expr,
                    op,
                    sequence_permitted,
                } => {
                    let source = expr.value.to_node(&root);
                    let frange = original_file_range(db.upcast(), file_id, source.syntax());
                    let symbol = op.symbol();
                    let message = if sequence_permitted {
                        format!(
                            "{symbol} sequences may only have unary operands.
                            More complex operands must be this with parenthesized `()`",
                        )
                    } else {
                        format!(
                            "{symbol} expressions may only have unary operands.
                            More complex operands must be this with parenthesized `()`"
                        )
                    };
                    DiagnosticMessage::new(message, frange.range)
                }
            }
        })
        .collect()
}

fn size_compatible(target: VecSize, overload: VecSize) -> bool {
    match overload {
        VecSize::Two | VecSize::Three | VecSize::Four => overload == target,
        VecSize::BoundVar(_) => true,
    }
}

fn convert_compatible(db: &dyn HirDatabase, target: Ty, overload: Ty) -> bool {
    let target_kind = target.kind(db);
    let overload_kind = overload.kind(db);
    match (target_kind, overload_kind) {
        (ty::TyKind::Vector(tg), ty::TyKind::Vector(ov)) => {
            size_compatible(tg.size, ov.size) && convert_compatible(db, tg.inner, ov.inner)
        }
        (ty::TyKind::Matrix(tg), ty::TyKind::Matrix(ov)) => {
            size_compatible(tg.columns, ov.columns)
                && size_compatible(tg.rows, ov.rows)
                && convert_compatible(db, tg.inner, ov.inner)
        }
        (ty::TyKind::Scalar(s1), ty::TyKind::Scalar(s2)) => s1 == s2,
        _ => false,
    }
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
