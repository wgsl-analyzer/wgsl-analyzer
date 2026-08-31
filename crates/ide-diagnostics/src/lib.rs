mod naga;
#[cfg(test)]
mod tests;
mod tint;

use std::{error, fmt::Display};

use base_db::{EditionedFileId, FileRange, Lookup as _, TextRange};
use hir::{
    HirDatabase, Semantics,
    diagnostics::{AnyDiagnostic, Severity},
};
use hir_def::original_file_range;
use hir_ty::ty::{
    self,
    pretty::{pretty_fn, pretty_type},
};
use ide_db::RootDatabase;
use itertools::Itertools as _;
use paths::{AbsPathBuf, Utf8PathBuf};
use rowan::NodeOrToken;
use syntax::{AstNode as _, Edition};
use vfs::FileId;

use crate::{
    naga::{Naga27, Naga28, Naga29, NagaMain, naga_diagnostics},
    tint::tint_diagnostics,
};

#[derive(Clone, Copy, Debug, Default)]
pub enum NagaVersion {
    Naga27,
    Naga28,
    #[default]
    Naga29,
    NagaMain,
}

#[derive(Clone, Debug)]
pub struct DiagnosticsConfig {
    /// Whether native diagnostics are enabled.
    pub enabled: bool,
    pub semantic_enabled: bool,
    pub parse_enabled: bool,
    pub naga_parsing_enabled: bool,
    pub naga_validation_enabled: bool,
    pub naga_version: NagaVersion,
    pub tint_enabled: bool,
    pub tint_path: Option<Utf8PathBuf>,
}

impl DiagnosticsConfig {
    const NONE: Self = Self {
        enabled: false,
        semantic_enabled: false,
        parse_enabled: false,
        naga_parsing_enabled: false,
        naga_validation_enabled: false,
        naga_version: NagaVersion::Naga29, // no const default :(
        tint_enabled: false,
        tint_path: None,
    };
}

impl Default for DiagnosticsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            semantic_enabled: true,
            parse_enabled: true,
            naga_parsing_enabled: true,
            naga_validation_enabled: true,
            naga_version: NagaVersion::default(),
            tint_enabled: false,
            tint_path: None,
        }
    }
}

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
    Tint,
    WeslRs,
}

impl Display for DiagnosticSource {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::WgslAnalyzer => write!(formatter, "wgsl-analyzer"),
            Self::Naga => write!(formatter, "naga"),
            Self::Tint => write!(formatter, "tint"),
            Self::WeslRs => write!(formatter, "wesl-rs"),
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

/// # Panics
///
/// Panics if the file is not found in the database.
#[expect(clippy::too_many_lines, reason = "TODO")]
pub fn diagnostics(
    db: &RootDatabase,
    config: &DiagnosticsConfig,
    file_id: FileId,
) -> Vec<Diagnostic> {
    let file_id = EditionedFileId::from_file(db, file_id);
    let parse = file_id.parse(db);

    let mut diagnostics = Vec::new();

    if config.parse_enabled {
        diagnostics.extend(
            parse
                .errors()
                .iter()
                .map(|error| AnyDiagnostic::ParseError {
                    message: error.message.clone(),
                    range: error.range,
                    file_id,
                }),
        );
    }

    let semantics = Semantics::new(db);

    if config.semantic_enabled {
        semantics
            .module(file_id)
            .semantic_diagnostics(db, &mut diagnostics);
    }

    let edition = file_id.edition(db);
    if edition == Edition::Wgsl && (config.naga_parsing_enabled || config.naga_validation_enabled) {
        match &config.naga_version {
            NagaVersion::Naga27 => {
                naga_diagnostics::<Naga27>(db, file_id, config, &mut diagnostics);
            },
            NagaVersion::Naga28 => {
                naga_diagnostics::<Naga28>(db, file_id, config, &mut diagnostics);
            },
            NagaVersion::Naga29 => {
                naga_diagnostics::<Naga29>(db, file_id, config, &mut diagnostics);
            },
            NagaVersion::NagaMain => {
                naga_diagnostics::<NagaMain>(db, file_id, config, &mut diagnostics);
            },
        }
    }

    if edition == Edition::Wgsl && config.tint_enabled {
        // TODO: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/998
        // Clean this up by turning external tool integrations into flycheck.
        // This "." is a hack to avoid adding a working_dir to the interface of ide-diagnostics.
        tint_diagnostics(db, file_id, config, ".", &mut diagnostics);
    }

    diagnostics
        .into_iter()
        .map(|diagnostic| {
            let file_id = diagnostic.file_id();
            let root = file_id.parse(db).syntax();
            match diagnostic {
                AnyDiagnostic::AssignmentNotAReference { left_side, actual } => {
                    debug_assert!(!actual.is_err(db));
                    let source = left_side.value.to_node(&root);
                    let actual = ty::pretty::pretty_type(db, actual);
                    let frange = original_file_range(db, left_side.file_id, source.syntax());
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
                    let expected_pretty = ty::pretty::pretty_type_expectation(db, expected);
                    let actual_pretty = ty::pretty::pretty_type(db, actual);
                    let frange = original_file_range(db, expression.file_id, source.syntax());
                    //debug_assert!(!actual.is_err(db), "{:?} expected {expected_pretty}, found {actual_pretty}", frange.range);
                    Diagnostic::new(
                        DiagnosticCode("2"),
                        format!("expected {expected_pretty}, found {actual_pretty}"),
                        frange.range,
                    )
                },
                AnyDiagnostic::NoSuchField {
                    expression,
                    name,
                    r#type,
                } => {
                    debug_assert!(!r#type.is_err(db));
                    let source = expression.value.to_node(&root).syntax().parent().unwrap();
                    let r#type = ty::pretty::pretty_type(db, r#type);
                    let frange = original_file_range(db, expression.file_id, source.syntax());
                    Diagnostic::new(
                        DiagnosticCode("3"),
                        format!("no field `{}` on type {type}", name.as_ref()),
                        frange.range,
                    )
                },
                AnyDiagnostic::ArrayAccessInvalidType { expression, r#type } => {
                    debug_assert!(!r#type.is_err(db));
                    let source = expression.value.to_node(&root);
                    let r#type = ty::pretty::pretty_type(db, r#type);
                    let frange = original_file_range(db, expression.file_id, source.syntax());
                    Diagnostic::new(
                        DiagnosticCode("4"),
                        format!("cannot index into type {type}"),
                        frange.range,
                    )
                },
                AnyDiagnostic::AssignmentNotWritable { left_side } => {
                    let source = left_side.value.to_node(&root);
                    let frange = original_file_range(db, left_side.file_id, source.syntax());
                    Diagnostic::new(
                        DiagnosticCode("1"),
                        "cannot assign to value with access mode `read`".to_owned(),
                        frange.range,
                    )
                },
                AnyDiagnostic::NotConstructible { expression, r#type } => {
                    debug_assert!(!r#type.is_err(db));
                    let source = expression.value.to_node(&root);
                    let r#type = ty::pretty::pretty_type(db, r#type);
                    let frange = original_file_range(db, expression.file_id, source.syntax());
                    Diagnostic::new(
                        DiagnosticCode("6"),
                        format!("type `{type}` is not constructible"),
                        frange.range,
                    )
                },
                AnyDiagnostic::FunctionCallArgCountMismatch {
                    expression,
                    n_expected,
                    n_actual,
                } => {
                    let source = expression.value.to_node(&root).syntax().parent().unwrap();
                    let frange = original_file_range(db, expression.file_id, source.syntax());
                    Diagnostic::new(
                        DiagnosticCode("7"),
                        format!("expected {n_expected} parameters, found {n_actual}"),
                        frange.range,
                    )
                },
                AnyDiagnostic::MissingAddressSpace { variable } => {
                    let variable_declaration = variable.value.to_node(&root);
                    let source = variable_declaration.var_token().map_or_else(
                        || NodeOrToken::Node(variable_declaration.syntax()),
                        NodeOrToken::Token,
                    );

                    let frange = original_file_range(db, variable.file_id, &source);
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
                    let frange = original_file_range(db, variable.file_id, &source);
                    Diagnostic::new(DiagnosticCode("12"), format!("{error}"), frange.range)
                },
                AnyDiagnostic::InvalidTypeSpecifier {
                    type_specifier,
                    error,
                } => {
                    let source = type_specifier.value.to_node(&root);
                    let frange =
                        original_file_range(db, type_specifier.file_id, source.syntax());
                    Diagnostic::new(DiagnosticCode("13"), format!("{}", error.display(db)), frange.range)
                },
                AnyDiagnostic::InvalidIdentExpression { expression, error } => {
                    let source = expression.value.to_node(&root);
                    let frange = original_file_range(db, expression.file_id, source.syntax());
                    Diagnostic::new(DiagnosticCode("14"), format!("{}", error.display(db)), frange.range)
                },
                AnyDiagnostic::NagaValidationError {
                    message,
                    range,
                    related,
                    file_id: _
                } => {
                    let mut message = Diagnostic::new(DiagnosticCode("15"), message, range);
                    message.related = related;
                    message.source = DiagnosticSource::Naga;
                    message
                },
                AnyDiagnostic::TintValidationError {
                    file_id,
                    range,
                    message,
                    severity,
                } => {
                    let mut message = Diagnostic::new(DiagnosticCode("15"), message, range);
                    message.severity = severity;
                    message.source = DiagnosticSource::Tint;
                    message
                },
                AnyDiagnostic::ParseError { message, range, file_id: _ } => {
                    Diagnostic::new(DiagnosticCode("16"), message, range)
                },
                AnyDiagnostic::NoConstructor {
                    expression,
                    r#type,
                    parameters,
                } => {
                    debug_assert!(!r#type.is_err(db));
                    let source = expression.value.to_node(&root).syntax().clone();

                    let parameters = parameters
                        .iter()
                        .map(|r#type| {
                            debug_assert!(!r#type.is_err(db));
                            ty::pretty::pretty_type(db, *r#type)
                        })
                        .join(", ");

                    let frange = original_file_range(db, expression.file_id, source.syntax());
                    Diagnostic::new(
                        DiagnosticCode("17"),
                        format!(
                            "no overload of constructor `{}` found for arguments of type ({})",
                            pretty_type(db, r#type),
                            if parameters.is_empty() { "<none>" } else { &parameters }
                        ),
                        frange.range,
                    )
                },
                AnyDiagnostic::NoOverload {
                    expression,
                    name,
                    parameters,
                } => {
                    let source = expression.value.to_node(&root).syntax().clone();
                    let frange = original_file_range(db, expression.file_id, source.syntax());
                    if parameters.is_empty() {
                        Diagnostic::new(
                            DiagnosticCode("18"),
                            format!(
                                "no overload of function `{}` found that takes no arguments",
                                name.as_str()
                            ),
                            frange.range,
                        )
                    } else {
                    let parameters = parameters
                        .iter()
                        .map(|r#type| {
                            debug_assert!(!r#type.is_err(db));
                            ty::pretty::pretty_type(db, *r#type)
                        })
                        .join(", ");

                    Diagnostic::new(
                        DiagnosticCode("18"),
                        format!(
                            "no overload of constructor `{}` found for arguments of type ({})",
                            name.as_str(),
                            if parameters.is_empty() { "<none>" } else { &parameters }
                        ),
                        frange.range,
                    )}
                },
                AnyDiagnostic::PrecedenceParensRequired {
                    expression,
                    operation,
                    sequence_permitted,
                } => {
                    let source = expression.value.to_node(&root);
                    let frange = original_file_range(db, file_id, source.syntax());
                    let symbol = operation.symbol();
                    let message = if sequence_permitted {
                        format!(
                            "{symbol} sequences may only have unary operands. More complex operands must be this with parenthesized `()`",
                        )
                    } else {
                        format!(
                            "{symbol} expressions may only have unary operands. More complex operands must be this with parenthesized `()`"
                        )
                    };
                    Diagnostic::new(DiagnosticCode("19"), message, frange.range)
                },
                AnyDiagnostic::CyclicType { name, range, file_id: _ } => Diagnostic::new(
                    DiagnosticCode("20"),
                    format!("cyclic type {}", name.as_str()),
                    range,
                ),
                AnyDiagnostic::UnexpectedTemplateArgument { expression } => {
                    let source = expression.value.to_node(&root);
                    let frange = original_file_range(db, expression.file_id, source.syntax());
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
                    let frange = original_file_range(db, expression.file_id, source.syntax());
                    let mut message = Diagnostic::new(DiagnosticCode("22"), message, frange.range);
                    message.source = DiagnosticSource::WeslRs;
                    message
                },
                AnyDiagnostic::ExpectedLoweredKind {
                    expression,
                    expected,
                    actual,
                    path,
                } => {
                    let source = expression.value.to_node(&root);
                    let frange = original_file_range(db, expression.file_id, source.syntax());
                    Diagnostic::new(
                        DiagnosticCode("23"),
                        format!("{actual} {} is not a {expected}", path.mod_path()),
                        frange.range,
                    )
                },
                AnyDiagnostic::InvalidIdentifier { name, range, file_id: _ } => Diagnostic::new(
                    DiagnosticCode("24"),
                    format!("`{}` is not a valid name for an identifier", name.as_str()),
                    range,
                ),
                AnyDiagnostic::UnnamedImport { id } => {
                    let source = id.value.to_node(&root);
                    let frange = original_file_range(db, id.file_id, source.syntax());
                    Diagnostic::new(
                        DiagnosticCode("25"),
                        "import without a name".to_owned(),
                        frange.range,
                    )
                },
                AnyDiagnostic::UnresolvedPackage { id, name } => {
                    let source = id.value.to_node(&root);
                    let frange = original_file_range(db, id.file_id, source.syntax());
                    Diagnostic::new(
                        DiagnosticCode("26"),
                        format!("could not find package `{}`", name.as_str()),
                        frange.range,
                    )
                },
                AnyDiagnostic::UnresolvedImport { id } => {
                    let source = id.value.to_node(&root);
                    let frange = original_file_range(db, id.file_id, source.syntax());
                    Diagnostic::new(
                        DiagnosticCode("27"),
                        "could not resolve import".to_owned(),
                        frange.range,
                    )
                },
                AnyDiagnostic::TooManySupers { id } => {
                    let source = id.value.to_node(&root);
                    let frange = original_file_range(db, id.file_id, source.syntax());
                    Diagnostic::new(
                        DiagnosticCode("29"),
                        "too many leading `super` keywords".to_owned(),
                        frange.range,
                    )
                },
                AnyDiagnostic::DetachedFile { id } => {
                    let source = id.value.to_node(&root);
                    let frange = original_file_range(db, id.file_id, source.syntax());
                    Diagnostic::new(
                        DiagnosticCode("30"),
                        "file is detached. Include it with a wesl.toml".to_owned(),
                        frange.range,
                    )
                },
                AnyDiagnostic::NameConflict {
                    item,
                    name: previous,
                } => {
                    let source = item.value.to_node(&root);
                    let frange = original_file_range(db, item.file_id, source.syntax());
                    Diagnostic::new(
                        DiagnosticCode("31"),
                        format!("Duplicate identifier `{}`", previous.as_str()),
                        frange.range,
                    )
                },
                AnyDiagnostic::StoreTypeMustBeStorable { expression, actual } => {
                    debug_assert!(!actual.is_err(db));
                    let source = expression.value.to_node(&root);
                    let r#type = ty::pretty::pretty_type(db, actual);
                    let frange = original_file_range(db, expression.file_id, source.syntax());
                    Diagnostic::new(
                        DiagnosticCode("32"),
                        format!("store type must be storable, found {type}"),
                        frange.range,
                    )
                },
                AnyDiagnostic::UnexpectedReturnValue { expression, actual } => {
                    debug_assert!(!actual.is_err(db));
                    let source = expression.value.to_node(&root);
                    let r#type = ty::pretty::pretty_type(db, actual);
                    let frange = original_file_range(db, expression.file_id, source.syntax());
                    Diagnostic::new(
                        DiagnosticCode("33"),
                        format!("unexpected return value of type `{type}` in function with no return type"),
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
