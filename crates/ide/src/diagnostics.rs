use std::{
    error,
    fmt::Display,
    ops::{self, Range},
};

use base_db::{FileRange, TextRange, TextSize};
use hir::{
    HirDatabase, Semantics,
    diagnostics::{AnyDiagnostic, DiagnosticsConfig, NagaVersion},
};
use hir_def::original_file_range;
use hir_ty::ty::{
    self,
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
    pub source: DiagnosticSource,
}

#[derive(Default)]
pub enum DiagnosticSource {
    #[default]
    WgslAnalyzer,
    Naga,
    WeslRs,
}

impl Display for DiagnosticSource {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::WgslAnalyzer => write!(f, "wgsl-analyzer"),
            Self::Naga => write!(f, "naga"),
            Self::WeslRs => write!(f, "wesl-rs"),
        }
    }
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
            source: DiagnosticSource::WgslAnalyzer,
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
    fn spans(&self) -> Box<dyn Iterator<Item = (Option<Range<usize>>, String)> + '_>;
    fn location(&self) -> Option<Range<usize>>;
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
    fn spans(&self) -> Box<dyn Iterator<Item = (Option<Range<usize>>, String)> + '_> {
        Box::new(
            self.labels()
                .map(|(span, label)| (span.to_range(), label.to_owned())),
        )
    }

    fn location(&self) -> Option<Range<usize>> {
        let (range, _) = self.labels().next()?;
        range.to_range()
    }
}

impl NagaError for naga22::WithSpan<naga22::valid::ValidationError> {
    fn spans(&self) -> Box<dyn Iterator<Item = (Option<Range<usize>>, String)> + '_> {
        Box::new(
            self.spans()
                .map(move |(span, label)| (span.to_range(), label.clone())),
        )
    }
    fn location(&self) -> Option<Range<usize>> {
        self.spans().next().and_then(|(span, _)| span.to_range())
    }
}

struct Naga27;
impl Naga for Naga27 {
    type Module = naga27::Module;
    type ParseError = naga27::front::wgsl::ParseError;
    type ValidationError = naga27::WithSpan<naga27::valid::ValidationError>;

    fn parse(source: &str) -> Result<Self::Module, Self::ParseError> {
        naga27::front::wgsl::parse_str(source)
    }

    fn validate(module: &Self::Module) -> Result<(), Self::ValidationError> {
        let flags = naga27::valid::ValidationFlags::all();
        let capabilities = naga27::valid::Capabilities::all();
        let mut validator = naga27::valid::Validator::new(flags, capabilities);
        validator.validate(module).map(drop)
    }
}

impl NagaError for naga27::front::wgsl::ParseError {
    fn spans(&self) -> Box<dyn Iterator<Item = (Option<Range<usize>>, String)> + '_> {
        Box::new(
            self.labels()
                .map(|(span, label)| (span.to_range(), label.to_owned())),
        )
    }
    fn location(&self) -> Option<Range<usize>> {
        let (span, _) = self.labels().next()?;
        span.to_range()
    }
}

impl NagaError for naga27::WithSpan<naga27::valid::ValidationError> {
    fn spans(&self) -> Box<dyn Iterator<Item = (Option<Range<usize>>, String)> + '_> {
        Box::new(
            self.spans()
                .map(move |(span, label)| (span.to_range(), label.clone())),
        )
    }
    fn location(&self) -> Option<Range<usize>> {
        self.spans().next().and_then(|(span, _)| span.to_range())
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
    fn spans(&self) -> Box<dyn Iterator<Item = (Option<Range<usize>>, String)> + '_> {
        Box::new(
            self.labels()
                .map(|(span, label)| (span.to_range(), label.to_owned())),
        )
    }
    fn location(&self) -> Option<Range<usize>> {
        let (span, _) = self.labels().next()?;
        span.to_range()
    }
}

impl NagaError for nagamain::WithSpan<nagamain::valid::ValidationError> {
    fn spans(&self) -> Box<dyn Iterator<Item = (Option<Range<usize>>, String)> + '_> {
        Box::new(
            self.spans()
                .map(move |(span, label)| (span.to_range(), label.clone())),
        )
    }
    fn location(&self) -> Option<Range<usize>> {
        self.spans().next().and_then(|(span, _)| span.to_range())
    }
}

fn emit<Error: NagaError>(
    error: &Error,
    file_id: FileId,
    full_range: TextRange,
    accumulator: &mut Vec<AnyDiagnostic>,
) {
    let message = error_message_cause_chain(&error);
    let original_range = |range: ops::Range<usize>| {
        TextRange::new(
            TextSize::from(u32::try_from(range.start).expect("indexes are small numbers")),
            TextSize::from(u32::try_from(range.end).expect("indexes are small numbers")),
        )
    };
    let location = error.location().map_or(full_range, original_range);

    let spans = error.spans().filter_map(|(span, label)| {
        let range = original_range(span?);
        Some((range, label))
    });

    let related: Vec<_> = spans
        .map(|(range, message)| (message, FileRange { range, file_id }))
        .collect();

    accumulator.push(AnyDiagnostic::NagaValidationError {
        file_id: file_id.into(),
        range: location,
        message,
        related,
    });
}

fn naga_diagnostics<N: Naga>(
    database: &dyn HirDatabase,
    file_id: FileId,
    config: &DiagnosticsConfig,
    accumulator: &mut Vec<AnyDiagnostic>,
) {
    let source = database.file_text(file_id);
    let full_range = TextRange::up_to(TextSize::of(source.as_str()));

    match N::parse(&source) {
        Ok(module) => {
            if !config.naga_validation_errors {
                return;
            }
            if let Err(error) = N::validate(&module) {
                emit(&error, file_id, full_range, accumulator);
            }
        },
        Err(error) => {
            if !config.naga_parsing_errors {
                return;
            }
            emit(&error, file_id, full_range, accumulator);
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
    let parse = database.parse(file_id);

    let mut diagnostics = Vec::new();

    diagnostics.extend(
        parse
            .errors()
            .iter()
            .map(|error| AnyDiagnostic::ParseError {
                message: error.message.clone(),
                range: error.range,
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
            NagaVersion::Naga27 => {
                naga_diagnostics::<Naga27>(database, file_id, config, &mut diagnostics);
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
            let root = database.parse_or_resolve(file_id).syntax();
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

                    let name = name.unwrap_or_else(|| builtin.name());

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
                AnyDiagnostic::MissingAddressSpace { variable } => {
                    let variable_declaration = variable.value.to_node(&root);
                    let source = variable_declaration.var_token().map_or_else(
                        || NodeOrToken::Node(variable_declaration.syntax()),
                        NodeOrToken::Token,
                    );

                    let frange = original_file_range(database, variable.file_id, &source);
                    Diagnostic::new(
                        DiagnosticCode("11"),
                        "missing address space on global variable".to_owned(),
                        frange.range,
                    )
                },
                AnyDiagnostic::InvalidAddressSpace { variable, error } => {
                    let variable_declaration = variable.value.to_node(&root);
                    let source = variable_declaration.var_token().map_or_else(
                        || NodeOrToken::Node(variable_declaration.syntax()),
                        NodeOrToken::Token,
                    );
                    let frange = original_file_range(database, variable.file_id, &source);
                    Diagnostic::new(DiagnosticCode("12"), format!("{error}"), frange.range)
                },
                AnyDiagnostic::InvalidTypeSpecifier {
                    type_specifier,
                    error,
                } => {
                    let source = type_specifier.value.to_node(&root);
                    let frange =
                        original_file_range(database, type_specifier.file_id, source.syntax());
                    Diagnostic::new(DiagnosticCode("13"), format!("{error}"), frange.range)
                },
                AnyDiagnostic::InvalidIdentExpression { expression, error } => {
                    let source = expression.value.to_node(&root);
                    let frange = original_file_range(database, expression.file_id, source.syntax());
                    Diagnostic::new(DiagnosticCode("14"), format!("{error}"), frange.range)
                },
                AnyDiagnostic::NagaValidationError {
                    message,
                    range,
                    related,
                    ..
                } => {
                    let mut message = Diagnostic::new(DiagnosticCode("15"), message, range);
                    message.related = related;
                    message.source = DiagnosticSource::Naga;
                    message
                },
                AnyDiagnostic::ParseError { message, range, .. } => {
                    Diagnostic::new(DiagnosticCode("16"), message, range)
                },
                AnyDiagnostic::NoConstructor {
                    expression,
                    builtins,
                    r#type,
                    parameters,
                } => {
                    let source = expression.value.to_node(&root).syntax().clone();

                    let parameters = parameters
                        .iter()
                        .map(|r#type| ty::pretty::pretty_type(database, *r#type))
                        .join(", ");

                    let mut possible = Vec::with_capacity(32);
                    let builtin_specific = builtins.lookup(database);
                    possible.extend(builtin_specific.overloads().map(|(_, overload)| {
                        pretty_fn(database, &overload.r#type.lookup(database))
                    }));

                    let possible = possible.join("\n");

                    let frange = original_file_range(database, expression.file_id, source.syntax());
                    Diagnostic::new(
                        DiagnosticCode("18"),
                        format!(
                            "no overload of constructor `{}` found for given \
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
                AnyDiagnostic::CyclicType { name, range, .. } => Diagnostic::new(
                    DiagnosticCode("20"),
                    format!("cyclic type {}", name.as_str()),
                    range,
                ),
                AnyDiagnostic::UnexpectedTemplateArgument { expression } => {
                    let source = expression.value.to_node(&root);
                    let frange = original_file_range(database, expression.file_id, source.syntax());
                    Diagnostic::new(
                        DiagnosticCode("21"),
                        "unexpected template argument".to_owned(),
                        frange.range,
                    )
                },
                AnyDiagnostic::WgslError {
                    expression,
                    message,
                } => {
                    let source = expression.value.to_node(&root);
                    let frange = original_file_range(database, expression.file_id, source.syntax());
                    let mut message = Diagnostic::new(DiagnosticCode("22"), message, frange.range);
                    message.source = DiagnosticSource::WeslRs;
                    message
                },
                AnyDiagnostic::ExpectedLoweredKind {
                    expression,
                    expected,
                    actual,
                    name,
                } => {
                    let source = expression.value.to_node(&root);
                    let frange = original_file_range(database, expression.file_id, source.syntax());
                    Diagnostic::new(
                        DiagnosticCode("23"),
                        format!("{actual} {} is not a {expected}", name.as_str()),
                        frange.range,
                    )
                },
            }
        })
        .collect()
}

fn error_message_cause_chain(error: &dyn error::Error) -> String {
    let mut message = error.to_string();

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
