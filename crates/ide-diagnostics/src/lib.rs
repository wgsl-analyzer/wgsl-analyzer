mod naga;
#[cfg(test)]
mod tests;
mod tint;

use std::fmt::Display;

use base_db::{EditionedFileId, FileRange, TextRange};
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
use paths::AbsPathBuf;
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
    pub naga_parsing_enabled: bool,
    pub naga_validation_enabled: bool,
    pub naga_version: NagaVersion,
    pub tint_enabled: bool,
    pub tint_path: Option<AbsPathBuf>,
}

impl Default for DiagnosticsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            semantic_enabled: true,
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
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::WgslAnalyzer => write!(f, "wgsl-analyzer"),
            Self::Naga => write!(f, "naga"),
            Self::Tint => write!(f, "tint"),
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
    database: &RootDatabase,
    config: &DiagnosticsConfig,
    file_id: FileId,
) -> Vec<Diagnostic> {
    let file_id = EditionedFileId::from_file(database, file_id);
    let parse = file_id.parse(database);

    let mut diagnostics = Vec::new();

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

    let semantics = Semantics::new(database);

    if config.semantic_enabled {
        semantics
            .module(file_id)
            .semantic_diagnostics(database, &mut diagnostics);
    }

    let edition = file_id.edition(database);
    if edition == Edition::Wgsl && (config.naga_parsing_enabled || config.naga_validation_enabled) {
        match &config.naga_version {
            NagaVersion::Naga27 => {
                naga_diagnostics::<Naga27>(database, file_id, config, &mut diagnostics);
            },
            NagaVersion::Naga28 => {
                naga_diagnostics::<Naga28>(database, file_id, config, &mut diagnostics);
            },
            NagaVersion::Naga29 => {
                naga_diagnostics::<Naga29>(database, file_id, config, &mut diagnostics);
            },
            NagaVersion::NagaMain => {
                naga_diagnostics::<NagaMain>(database, file_id, config, &mut diagnostics);
            },
        }
    }

    if edition == Edition::Wgsl && config.tint_enabled {
        // TODO: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/998
        // Clean this up by turning external tool integrations into flycheck.
        // This "." is a hack to avoid adding a working_dir to the interface of ide-diagnostics.
        tint_diagnostics(database, file_id, config, ".", &mut diagnostics);
    }

    diagnostics
        .into_iter()
        .map(|diagnostic| {
            let file_id = diagnostic.file_id();
            let root = file_id.parse(database).syntax();
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
                    operation,
                    sequence_permitted,
                } => {
                    let source = expression.value.to_node(&root);
                    let frange = original_file_range(database, file_id, source.syntax());
                    let symbol = operation.symbol();
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
                    path,
                } => {
                    let source = expression.value.to_node(&root);
                    let frange = original_file_range(database, expression.file_id, source.syntax());
                    Diagnostic::new(
                        DiagnosticCode("23"),
                        format!("{actual} {} is not a {expected}", path.mod_path()),
                        frange.range,
                    )
                },
                AnyDiagnostic::InvalidIdentifier { name, range, .. } => Diagnostic::new(
                    DiagnosticCode("24"),
                    format!("`{}` is not a valid name for an identifier", name.as_str()),
                    range,
                ),
                AnyDiagnostic::UnresolvedImport { id, name } => {
                    let source = id.value.to_node(&root);
                    let frange = original_file_range(database, id.file_id, source.syntax());
                    Diagnostic::new(
                        DiagnosticCode("25"),
                        format!("unresolved import `{}`", name.as_str()),
                        frange.range,
                    )
                },
                AnyDiagnostic::TooManySupers { id } => {
                    let source = id.value.to_node(&root);
                    let frange = original_file_range(database, id.file_id, source.syntax());
                    Diagnostic::new(
                        DiagnosticCode("26"),
                        "too many leading `super` keywords".to_owned(),
                        frange.range,
                    )
                },
                AnyDiagnostic::DetachedFile { id } => {
                    let source = id.value.to_node(&root);
                    let frange = original_file_range(database, id.file_id, source.syntax());
                    Diagnostic::new(
                        DiagnosticCode("27"),
                        "file is detached. Include it with a wesl.toml".to_owned(),
                        frange.range,
                    )
                },
                AnyDiagnostic::NameConflict {
                    item,
                    name: previous,
                } => {
                    let source = item.value.to_node(&root);
                    let frange = original_file_range(database, item.file_id, source.syntax());
                    Diagnostic::new(
                        DiagnosticCode("28"),
                        format!("Duplicate identifier `{}`", previous.as_str()),
                        frange.range,
                    )
                },
            }
        })
        .collect()
}
