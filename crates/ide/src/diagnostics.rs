use std::{
    collections::HashSet,
    error,
    fmt::Display,
    ops::{self, Range},
};

use base_db::{EditionedFileId, FileRange, TextRange, TextSize};
use hir::{
    HirDatabase, Semantics,
    diagnostics::{AnyDiagnostic, DiagnosticsConfig, NagaVersion},
};
use hir_def::item_tree::{ImportStatement, ItemTreeNode, ModuleItem};
use hir_def::{HirFileId, original_file_range};
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

struct Naga28;
impl Naga for Naga28 {
    type Module = naga28::Module;
    type ParseError = naga28::front::wgsl::ParseError;
    type ValidationError = naga28::WithSpan<naga28::valid::ValidationError>;

    fn parse(source: &str) -> Result<Self::Module, Self::ParseError> {
        naga28::front::wgsl::parse_str(source)
    }

    fn validate(module: &Self::Module) -> Result<(), Self::ValidationError> {
        let flags = naga28::valid::ValidationFlags::all();
        let capabilities = naga28::valid::Capabilities::all();
        let mut validator = naga28::valid::Validator::new(flags, capabilities);
        validator.validate(module).map(drop)
    }
}

impl NagaError for naga28::front::wgsl::ParseError {
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

impl NagaError for naga28::WithSpan<naga28::valid::ValidationError> {
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
    file_id: EditionedFileId,
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
        .map(|(range, message)| {
            (
                message,
                FileRange {
                    range,
                    file_id: file_id.file_id,
                },
            )
        })
        .collect();

    accumulator.push(AnyDiagnostic::NagaValidationError {
        file_id: file_id.into(),
        range: location,
        message,
        related,
    });
}

/// Like `emit`, but remaps byte offsets from a combined source back to the main file.
/// Errors whose locations fall outside the main file's range are silently dropped.
fn emit_with_offset<Error: NagaError>(
    error: &Error,
    file_id: EditionedFileId,
    full_range: TextRange,
    main_file_offset: usize,
    accumulator: &mut Vec<AnyDiagnostic>,
) {
    let message = error_message_cause_chain(&error);

    // Remap a byte range from combined source to main file coordinates.
    // Returns None if the range falls outside the main file.
    let remap_range = |range: ops::Range<usize>| -> Option<TextRange> {
        let start = range.start.checked_sub(main_file_offset)?;
        let end = range.end.checked_sub(main_file_offset)?;
        Some(TextRange::new(
            TextSize::from(u32::try_from(start).ok()?),
            TextSize::from(u32::try_from(end).ok()?),
        ))
    };

    // Try to remap the main error location. If it falls outside the main file,
    // still report it but use the full file range.
    let location = error
        .location()
        .and_then(|loc| remap_range(loc))
        .unwrap_or(full_range);

    let related: Vec<_> = error
        .spans()
        .filter_map(|(span, label)| {
            let range = remap_range(span?)?;
            Some((
                label,
                FileRange {
                    range,
                    file_id: file_id.file_id,
                },
            ))
        })
        .collect();

    accumulator.push(AnyDiagnostic::NagaValidationError {
        file_id: file_id.into(),
        range: location,
        message,
        related,
    });
}

/// Resolve an import statement to the FileId of the imported module file.
///
/// Delegates to `hir_def::resolver::resolve_import_to_file`.
fn resolve_import_to_file(
    database: &dyn HirDatabase,
    anchor_file: FileId,
    import: &ImportStatement,
) -> Option<FileId> {
    hir_def::resolver::resolve_import_to_file(database, anchor_file, import)
}

/// Collect all files transitively imported by the given file.
fn collect_transitive_imports(
    database: &dyn HirDatabase,
    root_file: FileId,
) -> Vec<FileId> {
    let mut visited = HashSet::new();
    let mut queue = vec![root_file];
    let mut result = Vec::new();

    while let Some(file_id) = queue.pop() {
        if !visited.insert(file_id) {
            continue;
        }

        let editioned = database.editioned_file_id(file_id);
        // Only follow imports in WESL files
        if !editioned.edition.at_least_wesl_0_0_1() {
            continue;
        }

        let hir_file = HirFileId::from(editioned);
        let item_tree = database.item_tree(hir_file);

        for item in item_tree.items() {
            if let ModuleItem::ImportStatement(import_id) = item {
                let import = item_tree.get(*import_id);
                if let Some(imported_file) = resolve_import_to_file(database, file_id, import) {
                    if !visited.contains(&imported_file) {
                        queue.push(imported_file);
                    }
                    if imported_file != root_file {
                        result.push(imported_file);
                    }
                }
            }
        }
    }

    result.dedup();
    result
}

/// A mapping from byte offsets in the combined source to the original file.
struct SourceMap {
    /// Each entry: (start_offset_in_combined, length, original_file_id)
    segments: Vec<(usize, usize, FileId)>,
}

impl SourceMap {
    /// Map a byte offset in the combined source back to (file_id, offset_in_file).
    /// Returns None if the offset falls in a gap or outside any segment.
    fn map_offset(
        &self,
        combined_offset: usize,
    ) -> Option<(FileId, usize)> {
        for &(start, len, file_id) in &self.segments {
            if combined_offset >= start && combined_offset < start + len {
                return Some((file_id, combined_offset - start));
            }
        }
        None
    }
}

/// Strip `import` lines from WESL source, replacing them with blank lines
/// to preserve line numbers within the file.
fn strip_import_lines(source: &str) -> String {
    let mut result = String::with_capacity(source.len());
    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("import ") || trimmed == "import" {
            // Replace with a blank line to preserve byte offsets within the file
            result.push('\n');
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }
    // Remove trailing newline if original didn't have one
    if !source.ends_with('\n') && result.ends_with('\n') {
        result.pop();
    }
    result
}

/// Build a combined WGSL source from a WESL file and all its transitive imports.
/// Returns the combined source string, a source map for error remapping,
/// and the byte range in the combined source that corresponds to the main file.
fn build_combined_source(
    database: &dyn HirDatabase,
    main_file: FileId,
) -> (String, SourceMap, ops::Range<usize>) {
    let imported_files = collect_transitive_imports(database, main_file);

    let mut combined = String::new();
    let mut segments = Vec::new();

    // First, add all imported files (dependencies come before the main file)
    for &dep_file in &imported_files {
        let source = database.file_text(dep_file);
        let stripped = strip_import_lines(&source);
        let start = combined.len();
        combined.push_str(&stripped);
        combined.push('\n');
        segments.push((start, stripped.len(), dep_file));
    }

    // Then add the main file
    let main_source = database.file_text(main_file);
    let main_stripped = strip_import_lines(&main_source);
    let main_start = combined.len();
    let main_len = main_stripped.len();
    combined.push_str(&main_stripped);
    segments.push((main_start, main_len, main_file));

    let source_map = SourceMap { segments };
    (combined, source_map, main_start..main_start + main_len)
}

fn naga_diagnostics<N: Naga>(
    database: &dyn HirDatabase,
    file_id: EditionedFileId,
    config: &DiagnosticsConfig,
    accumulator: &mut Vec<AnyDiagnostic>,
) {
    let is_wesl = file_id.edition.at_least_wesl_0_0_1();

    // For WESL files, build a combined source with all imports resolved.
    // For WGSL files, just use the file's source directly.
    let (source, main_file_offset) = if is_wesl {
        let (combined, _source_map, main_range) = build_combined_source(database, file_id.file_id);
        (combined, main_range.start)
    } else {
        let text = database.file_text(file_id.file_id).to_string();
        (text, 0)
    };

    let full_range = TextRange::up_to(TextSize::of(source.as_str()));

    match N::parse(&source) {
        Ok(module) => {
            if !config.naga_validation_errors {
                return;
            }
            if let Err(error) = N::validate(&module) {
                if is_wesl {
                    emit_with_offset(&error, file_id, full_range, main_file_offset, accumulator);
                } else {
                    emit(&error, file_id, full_range, accumulator);
                }
            }
        },
        Err(error) => {
            if !config.naga_parsing_errors {
                return;
            }
            if is_wesl {
                emit_with_offset(&error, file_id, full_range, main_file_offset, accumulator);
            } else {
                emit(&error, file_id, full_range, accumulator);
            }
        },
    }
}

/// # Panics
///
/// Panics if the file is not found in the database.
#[expect(clippy::too_many_lines, reason = "TODO")]
pub fn diagnostics(
    database: &dyn HirDatabase,
    config: &DiagnosticsConfig,
    file_id: FileId,
) -> Vec<Diagnostic> {
    let file_id = database.editioned_file_id(file_id);
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

    // For WESL files, naga receives a combined source with imports resolved.
    // For WGSL files, naga receives the raw source directly.
    if config.naga_parsing_errors || config.naga_validation_errors {
        match &config.naga_version {
            NagaVersion::Naga27 => {
                naga_diagnostics::<Naga27>(database, file_id, config, &mut diagnostics);
            },
            NagaVersion::Naga28 => {
                naga_diagnostics::<Naga28>(database, file_id, config, &mut diagnostics);
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
                AnyDiagnostic::ReservedIdentifier { name, range, .. } => Diagnostic::new(
                    DiagnosticCode("24"),
                    format!(
                        "identifier '{}' is reserved; identifiers starting with '__' are reserved by the WGSL specification",
                        name.as_str()
                    ),
                    range,
                ),
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

#[cfg(test)]
mod tests {
    use expect_test::{Expect, expect};
    use hir::diagnostics::DiagnosticsConfig;
    use itertools::Itertools;
    use std::fmt::Write as _;

    use crate::{diagnostics::Diagnostic, fixture::single_file_db};

    #[expect(clippy::needless_pass_by_value, reason = "Matches expect! macro")]
    #[expect(clippy::use_debug, reason = "useful in tests")]
    fn check_diagnostics(
        source: &str,
        expect: Expect,
    ) {
        let (analysis, file_id) = single_file_db(source);
        let config = DiagnosticsConfig {
            enabled: true,
            type_errors: true,
            naga_parsing_errors: false,
            naga_validation_errors: false,
            ..Default::default()
        };
        let diagnostics = analysis.diagnostics(&config, file_id).unwrap();
        let mut actual = String::new();
        for Diagnostic {
            code,
            message,
            range,
            severity,
            ..
        } in diagnostics
        {
            let severity_text = match severity {
                crate::diagnostics::Severity::Error => "Error",
                crate::diagnostics::Severity::WeakWarning => "Warning",
            };
            writeln!(
                actual,
                "{range:?} {severity_text} {}: {message}",
                code.as_str()
            );
        }

        expect.assert_eq(&actual);
    }

    #[test]
    fn global_var_function_address_space_error() {
        check_diagnostics(
            "var<function> not_allowed_at_module_level: u32;",
            expect![[r#"
                0..3 Error 12: address space is only valid in function-scope
                4..12 Error 21: unexpected template argument
            "#]],
        );
    }

    #[test]
    fn invalid_body() {
        check_diagnostics(
            "fn f() { let x: u32 = 1.0; }",
            expect![[r#"
                22..25 Error 2: expected u32, found float
            "#]],
        );
    }

    #[test]
    fn no_host_shareable_error_for_undefined_struct() {
        // https://github.com/wgsl-analyzer/wgsl-analyzer/issues/722
        // When referencing an undefined struct, we should NOT get a spurious
        // "not host-shareable" diagnostic — only the "unresolved" error.
        check_diagnostics(
            "
@group(0) @binding(0)
var<storage> lines: array<LineSegment>;
",
            expect![[r#"
                48..59 Error 14: `LineSegment` not found in scope
            "#]],
        );
    }

    #[test]
    fn reserved_identifier_double_underscore() {
        // https://github.com/wgsl-analyzer/wgsl-analyzer/issues/681
        // Identifiers starting with "__" are reserved by the WGSL spec.
        check_diagnostics(
            "
fn __my_func() {}
",
            expect![[r#"
                3..12 Error 24: identifier '__my_func' is reserved; identifiers starting with '__' are reserved by the WGSL specification
            "#]],
        );
    }

    #[test]
    fn non_reserved_identifier_single_underscore() {
        // A single underscore prefix should NOT trigger the reserved identifier diagnostic.
        check_diagnostics(
            "
fn _my_func() {}
",
            expect![[r#"
"#]],
        );
    }

    #[test]
    fn incomplete_variable_error() {
        // https://github.com/wgsl-analyzer/wgsl-analyzer/issues/825
        check_diagnostics(
            "
@group(0) @binding(0)
var<storage, read> a: array<f32>;

@group(0) @binding(1) // line 4
var<storage
",
            expect![[r#"
                92..93 Error 16: invalid syntax, expected one of: '@', ',', '=', <identifier>, '{', '}', ')', ';', <template start>
                101..101 Error 16: invalid syntax, expected one of: ':', '=', ';'
                22..25 Error 12: address space is only valid for handle or texture types
                26..33 Error 21: unexpected template argument
                26..33 Error 21: unexpected template argument
                89..92 Error 12: address space is only valid for handle or texture types
            "#]],
        );
    }

    #[test]
    fn subgroup_builtin_requires_enable_diagnostic() {
        // Using a subgroup builtin without `enable subgroups` should produce a diagnostic.
        check_diagnostics(
            "
fn test() {
    let a = subgroupAdd(1u);
}
",
            expect![[r#"
                24..39 Error 22: `subgroupAdd` requires `enable subgroups`
            "#]],
        );
    }

    #[test]
    fn reserved_word_diagnostic() {
        // https://github.com/wgsl-analyzer/wgsl-analyzer/issues/624
        // WGSL reserved words should produce a diagnostic.
        check_diagnostics(
            "
fn test() {
    let enum = 1u;
}
",
            expect![[r#"
                20..24 Error 16: 'enum' is a reserved word in WGSL
            "#]],
        );
    }
}
