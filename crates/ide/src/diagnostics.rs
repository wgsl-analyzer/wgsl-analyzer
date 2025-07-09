use std::{
    error,
    ops::{self, Range},
};

use base_db::{FileRange, TextRange, TextSize};
use hir::{
    HirDatabase, Semantics,
    diagnostics::{AnyDiagnostic, DiagnosticsConfig, NagaVersion},
};
use hir_def::original_file_range;
use hir_ty::ty::{
    self, Type, VecSize,
    pretty::{pretty_fn, pretty_type},
};
use itertools::Itertools as _;
use rowan::NodeOrToken;
use syntax::AstNode as _;
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
    #[must_use]
    pub fn url(&self) -> String {
        self.0.to_owned()
    }

    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        self.0
    }
}

#[derive(Clone, Copy)]
pub enum Severity {
    Error,
    WeakWarning,
}

impl Diagnostic {
    #[must_use]
    pub const fn new(
        code: DiagnosticCode,
        message: String,
        range: TextRange,
    ) -> Self {
        Self {
            code,
            message,
            range,
            unused: false,
            severity: Severity::Error,
            related: Vec::new(),
        }
    }

    #[must_use]
    pub fn with_severity(
        self,
        severity: Severity,
    ) -> Self {
        Self { severity, ..self }
    }

    #[must_use]
    pub fn unused(self) -> Self {
        Self {
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

trait NagaError: error::Error {
    fn spans(&self) -> Box<dyn Iterator<Item = (Range<usize>, String)> + '_>;
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
    fn spans(&self) -> Box<dyn Iterator<Item = (Range<usize>, String)> + '_> {
        Box::new(
            self.labels()
                .filter_map(|(range, label)| Some((range.to_range()?, label.to_owned()))),
        )
    }

    fn has_spans(&self) -> bool {
        self.labels().len() > 0
    }
}

impl NagaError for naga14::WithSpan<naga14::valid::ValidationError> {
    fn spans(&self) -> Box<dyn Iterator<Item = (Range<usize>, String)> + '_> {
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
    fn spans(&'_ self) -> Box<dyn Iterator<Item = (Range<usize>, String)> + '_> {
        Box::new(
            self.labels()
                .filter_map(|(range, label)| Some((range.to_range()?, label.to_owned()))),
        )
    }

    fn has_spans(&self) -> bool {
        self.labels().len() > 0
    }
}

impl NagaError for naga19::WithSpan<naga19::valid::ValidationError> {
    fn spans(&self) -> Box<dyn Iterator<Item = (Range<usize>, String)> + '_> {
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
    fn spans(&self) -> Box<dyn Iterator<Item = (Range<usize>, String)> + '_> {
        Box::new(
            self.labels()
                .filter_map(|(range, label)| Some((range.to_range()?, label.to_owned()))),
        )
    }

    fn has_spans(&self) -> bool {
        self.labels().len() > 0
    }
}

impl NagaError for naga22::WithSpan<naga22::valid::ValidationError> {
    fn spans(&self) -> Box<dyn Iterator<Item = (Range<usize>, String)> + '_> {
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
    fn spans(&self) -> Box<dyn Iterator<Item = (Range<usize>, String)> + '_> {
        Box::new(
            self.labels()
                .filter_map(|(range, label)| Some((range.to_range()?, label.to_owned()))),
        )
    }

    fn has_spans(&self) -> bool {
        self.labels().len() > 0
    }
}

impl NagaError for nagamain::WithSpan<nagamain::valid::ValidationError> {
    fn spans(&self) -> Box<dyn Iterator<Item = (Range<usize>, String)> + '_> {
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
    fn emit<Error: NagaError>(
        &self,
        database: &dyn HirDatabase,
        error: &Error,
        file_id: FileId,
        full_range: TextRange,
        accumulator: &mut Vec<AnyDiagnostic>,
    ) {
        let message = error_message_cause_chain("naga: ", &error);

        if !error.has_spans() {
            accumulator.push(AnyDiagnostic::NagaValidationError {
                file_id: file_id.into(),
                range: full_range,
                message,
                related: Vec::new(),
            });
            return;
        }
        let original_range = |range: ops::Range<usize>| {
            let range_in_full = TextRange::new(
                TextSize::from(u32::try_from(range.start).expect("indexes are small numbers")),
                TextSize::from(u32::try_from(range.end).expect("indexes are small numbers")),
            );
            database.text_range_from_full(file_id.into(), range_in_full)
        };

        let spans = error.spans().filter_map(|(span, label)| {
            let range = original_range(span).ok()?;
            Some((range, label))
        });

        match *self {
            Self::SeparateSpans => {
                spans.for_each(|(range, label)| {
                    accumulator.push(AnyDiagnostic::NagaValidationError {
                        file_id: file_id.into(),
                        range,
                        message: format!("{message}: {label}"),
                        related: Vec::new(),
                    });
                });
            },
            Self::SmallestSpan => {
                if let Some((range, _)) = spans.min_by_key(|(range, _)| range.len()) {
                    accumulator.push(AnyDiagnostic::NagaValidationError {
                        file_id: file_id.into(),
                        range,
                        message,
                        related: Vec::new(),
                    });
                }
            },
            Self::Related => {
                let related: Vec<_> = spans
                    .map(|(range, message)| (message, FileRange { range, file_id }))
                    .collect();
                let min_range = related
                    .iter()
                    .map(|(_, frange)| frange.range)
                    .min_by_key(|range| range.len())
                    .unwrap_or(full_range);

                accumulator.push(AnyDiagnostic::NagaValidationError {
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
    database: &dyn HirDatabase,
    file_id: FileId,
    config: &DiagnosticsConfig,
    accumulator: &mut Vec<AnyDiagnostic>,
) {
    let Ok(source) = database.resolve_full_source(file_id.into()) else {
        return;
    };

    let full_range = TextRange::up_to(TextSize::of(&source));

    let policy = NagaErrorPolicy::Related;
    match N::parse(&source) {
        Ok(module) => {
            if !config.naga_validation_errors {
                return;
            }
            if let Err(error) = N::validate(&module) {
                policy.emit(database, &error, file_id, full_range, accumulator);
            }
        },
        Err(error) => {
            if !config.naga_parsing_errors {
                return;
            }
            policy.emit(database, &error, file_id, full_range, accumulator);
        },
    }
}

/// # Panics
/// Panics if the file is not found in the database.
#[expect(clippy::too_many_lines, reason = "TODO")]
pub fn diagnostics(
    database: &dyn HirDatabase,
    config: &DiagnosticsConfig,
    file_id: FileId,
) -> Vec<Diagnostic> {
    let (parse, unconfigured) = database.parse_with_unconfigured(file_id);

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
                definition: unconfigured.definition.clone(),
                range: unconfigured.range,
                file_id: file_id.into(),
            }),
    );

    let semantics = Semantics::new(database);

    if config.type_errors {
        semantics
            .module(file_id)
            .diagnostics(database, config, &mut diagnostics);
    }

    if config.naga_parsing_errors || config.naga_validation_errors {
        match &config.naga_version {
            NagaVersion::Naga22 => {
                naga_diagnostics::<Naga22>(database, file_id, config, &mut diagnostics);
            },
            NagaVersion::Naga19 => {
                naga_diagnostics::<Naga19>(database, file_id, config, &mut diagnostics);
            },
            NagaVersion::Naga14 => {
                naga_diagnostics::<Naga14>(database, file_id, config, &mut diagnostics);
            },
            NagaVersion::NagaMain => {
                naga_diagnostics::<NagaMain>(database, file_id, config, &mut diagnostics);
            },
        }
    }

    diagnostics
        .into_iter()
        .map(|diagnostic| {
            let file_id = diagnostic.file_id();
            let root = database.parse_or_resolve(file_id).unwrap().syntax();
            match diagnostic {
                AnyDiagnostic::AssignmentNotAReference { left_side, actual } => {
                    let source = left_side.value.to_node(&root);
                    let actual = ty::pretty::pretty_type(database, actual);
                    let frange = original_file_range(database, left_side.file_id, source.syntax());
                    Diagnostic::new(
                        DiagnosticCode("1"),
                        format!(
                            "left hand side of assignment should be a reference, found {actual}"
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
                    let expected = ty::pretty::pretty_type_expectation(database, expected);
                    let actual = ty::pretty::pretty_type(database, actual);
                    let frange = original_file_range(database, expression.file_id, source.syntax());
                    Diagnostic::new(
                        DiagnosticCode("2"),
                        format!("expected {expected}, found {actual}"),
                        frange.range,
                    )
                },
                AnyDiagnostic::NoSuchField {
                    expression,
                    name,
                    r#type,
                } => {
                    let source = expression.value.to_node(&root).syntax().parent().unwrap();
                    let r#type = ty::pretty::pretty_type(database, r#type);
                    let frange = original_file_range(database, expression.file_id, source.syntax());
                    Diagnostic::new(
                        DiagnosticCode("3"),
                        format!("no field `{}` on type {type}", name.as_ref()),
                        frange.range,
                    )
                },
                AnyDiagnostic::ArrayAccessInvalidType { expression, r#type } => {
                    let source = expression.value.to_node(&root);
                    let r#type = ty::pretty::pretty_type(database, r#type);
                    let frange = original_file_range(database, expression.file_id, source.syntax());
                    Diagnostic::new(
                        DiagnosticCode("4"),
                        format!("cannot index into type {type}"),
                        frange.range,
                    )
                },
                AnyDiagnostic::UnresolvedName { expression, name } => {
                    let source = expression.value.to_node(&root);
                    let frange = original_file_range(database, expression.file_id, source.syntax());
                    Diagnostic::new(
                        DiagnosticCode("5"),
                        format!("cannot find `{}` in this scope", name.as_str()),
                        frange.range,
                    )
                },
                AnyDiagnostic::InvalidConstructionType { expression, r#type } => {
                    let source = expression.value.to_node(&root);
                    let r#type = ty::pretty::pretty_type(database, r#type);
                    let frange = original_file_range(database, expression.file_id, source.syntax());
                    Diagnostic::new(
                        DiagnosticCode("6"),
                        format!("cannot construct value of type {type}"),
                        frange.range,
                    )
                },
                AnyDiagnostic::FunctionCallArgCountMismatch {
                    expression,
                    n_expected,
                    n_actual,
                } => {
                    let source = expression.value.to_node(&root).syntax().parent().unwrap();
                    let frange = original_file_range(database, expression.file_id, source.syntax());
                    Diagnostic::new(
                        DiagnosticCode("7"),
                        format!("expected {n_expected} parameters, found {n_actual}"),
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
                    let builtin = builtin.lookup(database);

                    let parameters = parameters
                        .iter()
                        .map(|r#type| ty::pretty::pretty_type(database, *r#type))
                        .join(", ");

                    let possible = builtin
                        .overloads()
                        .map(|(_, overload)| pretty_fn(database, &overload.r#type.lookup(database)))
                        .join("\n");

                    let name = name.map_or_else(|| builtin.name(), |name| name);

                    let frange = original_file_range(database, expression.file_id, source.syntax());
                    Diagnostic::new(
                        DiagnosticCode("8"),
                        format!(
                            "no overload of `{name}` found for given arguments.\
                        Found ({parameters}), expected one of:\n{possible}"
                        ),
                        frange.range,
                    )
                },
                AnyDiagnostic::AddressOfNotReference { expression, actual } => {
                    let source = expression.value.to_node(&root);
                    let r#type = ty::pretty::pretty_type(database, actual);
                    let frange = original_file_range(database, expression.file_id, source.syntax());
                    Diagnostic::new(
                        DiagnosticCode("9"),
                        format!("expected a reference, found {type}"),
                        frange.range,
                    )
                },
                AnyDiagnostic::DerefNotPointer { expression, actual } => {
                    let source = expression.value.to_node(&root);
                    let r#type = ty::pretty::pretty_type(database, actual);
                    let frange = original_file_range(database, expression.file_id, source.syntax());
                    Diagnostic::new(
                        DiagnosticCode("10"),
                        format!("cannot dereference expression of type {type}"),
                        frange.range,
                    )
                },
                AnyDiagnostic::MissingAddressSpace { var } => {
                    let var_decl = var.value.to_node(&root);
                    let source = var_decl
                        .var_token()
                        .map_or_else(|| NodeOrToken::Node(var_decl.syntax()), NodeOrToken::Token);

                    let frange = original_file_range(database, var.file_id, &source);
                    Diagnostic::new(
                        DiagnosticCode("11"),
                        "missing address space on global variable".to_owned(),
                        frange.range,
                    )
                },
                AnyDiagnostic::InvalidAddressSpace { var, error } => {
                    let var_decl = var.value.to_node(&root);
                    let source = var_decl
                        .var_token()
                        .map_or_else(|| NodeOrToken::Node(var_decl.syntax()), NodeOrToken::Token);
                    let frange = original_file_range(database, var.file_id, &source);
                    Diagnostic::new(DiagnosticCode("12"), format!("{error}"), frange.range)
                },
                AnyDiagnostic::InvalidType {
                    file_id,
                    location,
                    error,
                } => {
                    let source = location.to_node(&root);
                    let frange = original_file_range(database, file_id, source.syntax());
                    Diagnostic::new(DiagnosticCode("13"), format!("{error}"), frange.range)
                },
                AnyDiagnostic::UnresolvedImport { import } => {
                    let source = import.value.to_node(&root);
                    let frange = original_file_range(database, file_id, source.syntax());
                    Diagnostic::new(
                        DiagnosticCode("14"),
                        "unresolved import".to_owned(),
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
                AnyDiagnostic::UnconfiguredCode {
                    definition, range, ..
                } => Diagnostic::new(
                    DiagnosticCode("17"),
                    format!(
                        "code is inactive due to `#ifdef` directives: `{definition}` is not enabled"
                    ),
                    range,
                )
                .with_severity(Severity::WeakWarning)
                .unused(),
                AnyDiagnostic::NoConstructor {
                    expression,
                    builtins: [specific, general],
                    r#type,
                    parameters,
                } => {
                    let source = expression.value.to_node(&root).syntax().clone();

                    let parameters = parameters
                        .iter()
                        .map(|r#type| ty::pretty::pretty_type(database, *r#type))
                        .join(", ");

                    let mut possible = Vec::with_capacity(32);
                    let builtin_specific = specific.lookup(database);
                    possible.extend(builtin_specific.overloads().map(|(_, overload)| {
                        pretty_fn(database, &overload.r#type.lookup(database))
                    }));
                    let builtin_general = general.lookup(database);
                    possible.extend(
                        builtin_general
                            .overloads()
                            .filter(|(_, overload)| {
                                let function = overload.r#type.lookup(database);
                                function.return_type.is_none_or(|return_ty| {
                                    convert_compatible(database, r#type, return_ty)
                                })
                            })
                            .map(|(_, overload)| {
                                pretty_fn(database, &overload.r#type.lookup(database))
                            }),
                    );

                    let possible = possible.join("\n");

                    let frange = original_file_range(database, expression.file_id, source.syntax());
                    Diagnostic::new(
                        DiagnosticCode("18"),
                        format!(
                            "no overload of constructor `{}` found for given\
                            arguments. Found ({parameters}), expected one of:\n{possible}",
                            pretty_type(database, r#type),
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
                    let frange = original_file_range(database, file_id, source.syntax());
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
    database: &dyn HirDatabase,
    target: Type,
    overload: Type,
) -> bool {
    let target_kind = target.kind(database);
    let overload_kind = overload.kind(database);
    match (target_kind, overload_kind) {
        (ty::TyKind::Vector(tg), ty::TyKind::Vector(ov)) => {
            size_compatible(tg.size, ov.size)
                && convert_compatible(database, tg.component_type, ov.component_type)
        },
        (ty::TyKind::Matrix(tg), ty::TyKind::Matrix(ov)) => {
            size_compatible(tg.columns, ov.columns)
                && size_compatible(tg.rows, ov.rows)
                && convert_compatible(database, tg.inner, ov.inner)
        },
        (ty::TyKind::Scalar(s1), ty::TyKind::Scalar(s2)) => s1 == s2,
        _ => false,
    }
}

fn error_message_cause_chain(
    prefix: &str,
    error: &dyn error::Error,
) -> String {
    let mut message = format!("{prefix}{error}");

    let mut error = error.source();
    if error.is_some() {
        message.push_str(": ");
    }

    while let Some(source) = error {
        message.push_str(&source.to_string());
        error = source.source();
    }

    message
}
