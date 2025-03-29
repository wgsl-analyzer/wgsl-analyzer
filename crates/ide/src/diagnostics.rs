use std::ops::Range;

use base_db::{FileRange, TextRange, TextSize};
use hir::{
    HirDatabase, Semantics,
    diagnostics::{AnyDiagnostic, DiagnosticsConfig, NagaVersion},
};
use hir_def::original_file_range;
use hir_ty::ty::{
    self, Ty, VecSize,
    pretty::{pretty_fn, pretty_type},
};
use itertools::Itertools;
use rowan::NodeOrToken;
use syntax::AstNode;
use vfs::FileId;

pub struct Diagnostic {
    pub code: DiagnosticCode,
    pub message: String,
    pub range: TextRange,
    pub unused: bool,
    pub severity: Severity,
    pub related: Vec<(String, FileRange)>,
}

pub struct DiagnosticCode(&'static str);

impl DiagnosticCode {
    pub fn url(&self) -> String {
        self.0.to_string()
    }

    pub fn as_str(&self) -> &'static str {
        self.0
    }
}

#[derive(Clone, Copy)]
pub enum Severity {
    Error,
    WeakWarning,
}

impl Diagnostic {
    pub fn new(
        code: DiagnosticCode,
        message: String,
        range: TextRange,
    ) -> Self {
        Self {
            code,
            message,
            range,
            severity: Severity::Error,
            unused: false,
            related: Vec::new(),
        }
    }

    pub fn with_severity(
        self,
        severity: Severity,
    ) -> Self {
        Diagnostic { severity, ..self }
    }

    pub fn unused(self) -> Self {
        Diagnostic {
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

struct Naga14;
impl Naga for Naga14 {
    type Module = naga14::Module;
    type ParseError = naga14::front::wgsl::ParseError;
    type ValidationError = naga14::WithSpan<naga14::valid::ValidationError>;

    fn parse(source: &str) -> Result<Self::Module, Self::ParseError> {
        naga14::front::wgsl::parse_str(source)
    }

    fn validate(module: &Self::Module) -> Result<(), Self::ValidationError> {
        let flags = naga14::valid::ValidationFlags::all();
        let capabilities = naga14::valid::Capabilities::all();
        let mut validator = naga14::valid::Validator::new(flags, capabilities);
        validator.validate(module).map(drop)
    }
}

impl NagaError for naga14::front::wgsl::ParseError {
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

impl NagaError for naga14::WithSpan<naga14::valid::ValidationError> {
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

struct Naga19;
impl Naga for Naga19 {
    type Module = naga19::Module;
    type ParseError = naga19::front::wgsl::ParseError;
    type ValidationError = naga19::WithSpan<naga19::valid::ValidationError>;

    fn parse(source: &str) -> Result<Self::Module, Self::ParseError> {
        naga19::front::wgsl::parse_str(source)
    }

    fn validate(module: &Self::Module) -> Result<(), Self::ValidationError> {
        let flags = naga19::valid::ValidationFlags::all();
        let capabilities = naga19::valid::Capabilities::all();
        let mut validator = naga19::valid::Validator::new(flags, capabilities);
        validator.validate(module).map(drop)
    }
}

impl NagaError for naga19::front::wgsl::ParseError {
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

impl NagaError for naga19::WithSpan<naga19::valid::ValidationError> {
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

struct Naga22;
impl Naga for Naga22 {
    type Module = naga22::Module;
    type ParseError = naga22::front::wgsl::ParseError;
    type ValidationError = naga22::WithSpan<naga22::valid::ValidationError>;

    fn parse(source: &str) -> Result<Self::Module, Self::ParseError> {
        naga22::front::wgsl::parse_str(source)
    }

    fn validate(module: &Self::Module) -> Result<(), Self::ValidationError> {
        let flags = naga22::valid::ValidationFlags::all();
        let capabilities = naga22::valid::Capabilities::all();
        let mut validator = naga22::valid::Validator::new(flags, capabilities);
        validator.validate(module).map(drop)
    }
}

impl NagaError for naga22::front::wgsl::ParseError {
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

impl NagaError for naga22::WithSpan<naga22::valid::ValidationError> {
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
        let message = error_message_cause_chain("naga: ", &error);

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
            },
            NagaErrorPolicy::SmallestSpan => {
                if let Some((range, _)) = spans.min_by_key(|(range, _)| range.len()) {
                    acc.push(AnyDiagnostic::NagaValidationError {
                        file_id: file_id.into(),
                        range,
                        message,
                        related: Vec::new(),
                    });
                }
            },
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
            },
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
        },
        Err(error) => {
            if !config.naga_parsing_errors {
                return Ok(());
            }
            policy.emit(db, error, file_id, full_range, acc);
        },
    }

    Ok(())
}

pub fn diagnostics(
    db: &dyn HirDatabase,
    config: &DiagnosticsConfig,
    file_id: FileId,
) -> Vec<Diagnostic> {
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
            NagaVersion::Naga22 => {
                let _ = naga_diagnostics::<Naga22>(db, file_id, config, &mut diagnostics);
            },
            NagaVersion::Naga19 => {
                let _ = naga_diagnostics::<Naga19>(db, file_id, config, &mut diagnostics);
            },
            NagaVersion::Naga14 => {
                let _ = naga_diagnostics::<Naga14>(db, file_id, config, &mut diagnostics);
            },
            NagaVersion::NagaMain => {
                let _ = naga_diagnostics::<NagaMain>(db, file_id, config, &mut diagnostics);
            },
        }
    }

    diagnostics
        .into_iter()
        .map(|diagnostic| {
            let file_id = diagnostic.file_id();
            let root = db.parse_or_resolve(file_id).unwrap().syntax();
            match diagnostic {
                AnyDiagnostic::AssignmentNotAReference { left_side, actual } => {
                    let source = left_side.value.to_node(&root);
                    let actual = ty::pretty::pretty_type(db, actual);
                    let frange =
                        original_file_range(db.upcast(), left_side.file_id, source.syntax());
                    Diagnostic::new(
                        DiagnosticCode("1"),
                        format!(
                            "left hand side of assignment should be a reference, found {}",
                            actual
                        ),
                        frange.range,
                    )
                },
                AnyDiagnostic::TypeMismatch {
                    expression,
                    expected,
                    actual,
                } => {
                    let source = expression.value.to_node(&root);
                    let expected = ty::pretty::pretty_type_expectation(db, expected);
                    let actual = ty::pretty::pretty_type(db, actual);
                    let frange =
                        original_file_range(db.upcast(), expression.file_id, source.syntax());
                    Diagnostic::new(
                        DiagnosticCode("2"),
                        format!("expected {}, found {}", expected, actual),
                        frange.range,
                    )
                },
                AnyDiagnostic::NoSuchField {
                    expression,
                    name,
                    ty,
                } => {
                    let source = expression.value.to_node(&root).syntax().parent().unwrap();
                    let ty = ty::pretty::pretty_type(db, ty);
                    let frange =
                        original_file_range(db.upcast(), expression.file_id, source.syntax());
                    Diagnostic::new(
                        DiagnosticCode("3"),
                        format!("no field `{}` on type {}", name.as_ref(), ty),
                        frange.range,
                    )
                },
                AnyDiagnostic::ArrayAccessInvalidType { expression, ty } => {
                    let source = expression.value.to_node(&root);
                    let ty = ty::pretty::pretty_type(db, ty);
                    let frange =
                        original_file_range(db.upcast(), expression.file_id, source.syntax());
                    Diagnostic::new(
                        DiagnosticCode("4"),
                        format!("cannot index into type {}", ty),
                        frange.range,
                    )
                },
                AnyDiagnostic::UnresolvedName { expression, name } => {
                    let source = expression.value.to_node(&root);
                    let frange =
                        original_file_range(db.upcast(), expression.file_id, source.syntax());
                    Diagnostic::new(
                        DiagnosticCode("5"),
                        format!("cannot find `{}` in this scope", name.as_str()),
                        frange.range,
                    )
                },
                AnyDiagnostic::InvalidConstructionType { expression, ty } => {
                    let source = expression.value.to_node(&root);
                    let ty = ty::pretty::pretty_type(db, ty);
                    let frange =
                        original_file_range(db.upcast(), expression.file_id, source.syntax());
                    Diagnostic::new(
                        DiagnosticCode("6"),
                        format!("cannot construct value of type {}", ty),
                        frange.range,
                    )
                },
                AnyDiagnostic::FunctionCallArgCountMismatch {
                    expression,
                    n_expected,
                    n_actual,
                } => {
                    let source = expression.value.to_node(&root).syntax().parent().unwrap();
                    let frange =
                        original_file_range(db.upcast(), expression.file_id, source.syntax());
                    Diagnostic::new(
                        DiagnosticCode("7"),
                        format!("expected {} parameters, found {}", n_expected, n_actual),
                        frange.range,
                    )
                },
                AnyDiagnostic::NoBuiltinOverload {
                    expression,
                    builtin,
                    parameters,
                    name,
                } => {
                    let source = expression.value.to_node(&root).syntax().parent().unwrap();
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

                    let frange =
                        original_file_range(db.upcast(), expression.file_id, source.syntax());
                    Diagnostic::new(
                        DiagnosticCode("8"),
                        format!(
                            "no overload of `{}` found for given arguments.\
                        Found ({}), expected one of:\n{}",
                            name, parameters, possible
                        ),
                        frange.range,
                    )
                },
                AnyDiagnostic::AddressOfNotReference { expression, actual } => {
                    let source = expression.value.to_node(&root);
                    let ty = ty::pretty::pretty_type(db, actual);
                    let frange =
                        original_file_range(db.upcast(), expression.file_id, source.syntax());
                    Diagnostic::new(
                        DiagnosticCode("9"),
                        format!("expected a reference, found {}", ty),
                        frange.range,
                    )
                },
                AnyDiagnostic::DerefNotPointer { expression, actual } => {
                    let source = expression.value.to_node(&root);
                    let ty = ty::pretty::pretty_type(db, actual);
                    let frange =
                        original_file_range(db.upcast(), expression.file_id, source.syntax());
                    Diagnostic::new(
                        DiagnosticCode("10"),
                        format!("cannot dereference expression of type {}", ty),
                        frange.range,
                    )
                },
                AnyDiagnostic::MissingStorageClass { var } => {
                    let var_decl = var.value.to_node(&root);
                    let source = var_decl
                        .var_token()
                        .map(NodeOrToken::Token)
                        .unwrap_or_else(|| NodeOrToken::Node(var_decl.syntax()));

                    let frange = original_file_range(db.upcast(), var.file_id, &source);
                    Diagnostic::new(
                        DiagnosticCode("11"),
                        "missing storage class on global variable".to_string(),
                        frange.range,
                    )
                },
                AnyDiagnostic::InvalidStorageClass { var, error } => {
                    let var_decl = var.value.to_node(&root);
                    let source = var_decl
                        .var_token()
                        .map(NodeOrToken::Token)
                        .unwrap_or_else(|| NodeOrToken::Node(var_decl.syntax()));
                    let frange = original_file_range(db.upcast(), var.file_id, &source);
                    Diagnostic::new(DiagnosticCode("12"), format!("{}", error), frange.range)
                },
                AnyDiagnostic::InvalidType {
                    file_id: _,
                    location,
                    error,
                } => {
                    let source = location.to_node(&root);
                    let frange = original_file_range(db.upcast(), file_id, source.syntax());
                    Diagnostic::new(DiagnosticCode("13"), format!("{}", error), frange.range)
                },
                AnyDiagnostic::UnresolvedImport { import } => {
                    let source = import.value.to_node(&root);
                    let frange = original_file_range(db.upcast(), file_id, source.syntax());
                    Diagnostic::new(
                        DiagnosticCode("14"),
                        "unresolved import".to_string(),
                        frange.range,
                    )
                },
                AnyDiagnostic::NagaValidationError {
                    message,
                    range,
                    related,
                    ..
                } => {
                    let mut message = Diagnostic::new(DiagnosticCode("15"), message, range);
                    message.related = related;
                    message
                },
                AnyDiagnostic::ParseError { message, range, .. } => {
                    Diagnostic::new(DiagnosticCode("16"), message, range)
                },
                AnyDiagnostic::UnconfiguredCode { def, range, .. } => Diagnostic::new(
                    DiagnosticCode("17"),
                    format!(
                        "code is inactive due to `#ifdef` directives: `{}` is not enabled",
                        def
                    ),
                    range,
                )
                .with_severity(Severity::WeakWarning)
                .unused(),
                AnyDiagnostic::NoConstructor {
                    expression,
                    builtins: [specific, general],
                    ty,
                    parameters,
                } => {
                    let source = expression.value.to_node(&root).syntax().clone();

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

                    let frange =
                        original_file_range(db.upcast(), expression.file_id, source.syntax());
                    Diagnostic::new(
                        DiagnosticCode("18"),
                        format!(
                            "no overload of constructor `{}` found for given\
                            arguments. Found ({parameters}), expected one of:\n{possible}",
                            pretty_type(db, ty),
                        ),
                        frange.range,
                    )
                },
                AnyDiagnostic::PrecedenceParensRequired {
                    expression,
                    operation: op,
                    sequence_permitted,
                } => {
                    let source = expression.value.to_node(&root);
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
                    Diagnostic::new(DiagnosticCode("19"), message, frange.range)
                },
            }
        })
        .collect()
}

fn size_compatible(
    target: VecSize,
    overload: VecSize,
) -> bool {
    match overload {
        VecSize::Two | VecSize::Three | VecSize::Four => overload == target,
        VecSize::BoundVar(_) => true,
    }
}

fn convert_compatible(
    db: &dyn HirDatabase,
    target: Ty,
    overload: Ty,
) -> bool {
    let target_kind = target.kind(db);
    let overload_kind = overload.kind(db);
    match (target_kind, overload_kind) {
        (ty::TyKind::Vector(tg), ty::TyKind::Vector(ov)) => {
            size_compatible(tg.size, ov.size) && convert_compatible(db, tg.inner, ov.inner)
        },
        (ty::TyKind::Matrix(tg), ty::TyKind::Matrix(ov)) => {
            size_compatible(tg.columns, ov.columns)
                && size_compatible(tg.rows, ov.rows)
                && convert_compatible(db, tg.inner, ov.inner)
        },
        (ty::TyKind::Scalar(s1), ty::TyKind::Scalar(s2)) => s1 == s2,
        _ => false,
    }
}

fn error_message_cause_chain(
    prefix: &str,
    error: &dyn std::error::Error,
) -> String {
    let mut message = format!("{}{}", prefix, error);

    let mut e = error.source();
    if e.is_some() {
        message.push_str(": ");
    }

    while let Some(source) = e {
        message.push_str(&source.to_string());
        e = source.source();
    }

    message
}
