#![expect(clippy::use_debug, reason = "tests")]

mod big;
mod builtins;
mod conditional_compilation;
mod imports;
mod incremental;
mod language_extensions;
mod layout;
mod operators;
mod simple;
mod single_diagnostics_on_errors;
use std::fmt::Write as _;

use base_db::{CapabilitiesInput, EditionedFileId, Intern as _, Lookup as _, TextRange};
use expect_test::Expect;
use hir_def::{
    HasSource as _,
    body::Body,
    db::{DefinitionWithBodyId, Location, ModuleDefinitionId},
    expression::ExpressionId,
    expression_store::{
        ExpressionSourceMap, ExpressionStore, ExpressionStoreOwnerId, ExpressionStoreSource,
        SyntheticSyntax,
    },
    item_tree::{ItemTree, ModuleItemId, Name},
    signature::{StructSignature, TypeAliasSignature},
    type_specifier::TypeSpecifierId,
};
use syntax::{AstNode as _, Capabilities, Diagnostic, SyntaxNode};
use test_fixture::WithFixture as _;

use crate::{
    db::HirDatabase as _,
    diagnostics::{InferenceDiagnostic, InferenceDiagnosticKind},
    infer::{InferenceResult, TypeExpectation},
    lower::{LoweredKind, TypeContainer, TypeLoweringError},
    test_db::TestDatabase,
    ty::{
        Type,
        pretty::{
            TypeVerbosity, pretty_type, pretty_type_expectation_with_verbosity,
            pretty_type_with_verbosity,
        },
    },
};

fn infer(
    capabilities: Capabilities,
    verbosity: TypeVerbosity,
    wa_fixture: &str,
) -> String {
    let (mut db, files) = TestDatabase::with_many_files(wa_fixture);
    CapabilitiesInput::update_capabilities(&mut db, capabilities);
    let mut buffer = String::new();

    for (index, file_id) in files.into_iter().enumerate() {
        if index > 0 {
            buffer.push_str("---\n");
        }
        InferPrinter::new(&db, file_id, verbosity).infer_file(&mut buffer);
    }
    buffer.truncate(buffer.trim_end().len());
    buffer
}

struct InferPrinter<'db> {
    db: &'db TestDatabase,
    file_id: EditionedFileId,
    root: SyntaxNode,
    verbosity: TypeVerbosity,
}

impl<'db> InferPrinter<'db> {
    fn new(
        db: &'db TestDatabase,
        file_id: EditionedFileId,
        verbosity: TypeVerbosity,
    ) -> Self {
        let parse = file_id.parse(db);
        assert_eq!(<&[Diagnostic]>::default(), parse.errors());
        let root = parse.syntax();
        Self {
            db,
            file_id,
            root,
            verbosity,
        }
    }

    fn infer_file(
        &self,
        buffer: &mut String,
    ) {
        let module_info = ItemTree::of(self.db, self.file_id);
        let mut definitions = module_definitions(self.db, self.file_id, module_info);
        definitions.sort_by_key(|definition| text_range_start(*definition, self.db));
        for definition in definitions {
            match definition {
                ModuleDefinitionId::Function(id) => {
                    self.infer_with_body(DefinitionWithBodyId::Function(id), buffer);
                },
                ModuleDefinitionId::GlobalVariable(id) => {
                    self.infer_with_body(DefinitionWithBodyId::GlobalVariable(id), buffer);
                },
                ModuleDefinitionId::GlobalConstant(id) => {
                    self.infer_with_body(DefinitionWithBodyId::GlobalConstant(id), buffer);
                },
                ModuleDefinitionId::GlobalAssertStatement(id) => {
                    self.infer_with_body(DefinitionWithBodyId::GlobalAssertStatement(id), buffer);
                },
                ModuleDefinitionId::Override(id) => {
                    self.infer_with_body(DefinitionWithBodyId::Override(id), buffer);
                },
                ModuleDefinitionId::Struct(id) => {
                    let (_, signature_map) = StructSignature::with_source_map(self.db, id);
                    let (_, diagnostics) = &*self.db.field_types(id);

                    for diagnostic in diagnostics {
                        self.print_diagnostic(diagnostic, signature_map, buffer);
                    }
                },
                ModuleDefinitionId::TypeAlias(id) => {
                    let (_, signature_map) = TypeAliasSignature::with_source_map(self.db, id);
                    let (_, diagnostics) = &*self.db.type_alias_type(id);
                    for diagnostic in diagnostics {
                        self.print_diagnostic(diagnostic, signature_map, buffer);
                    }
                },
            }
        }
    }

    fn infer_with_body(
        &self,
        definition: DefinitionWithBodyId,
        buffer: &mut String,
    ) {
        let (_, signature_map) = ExpressionStore::with_source_map(
            self.db,
            ExpressionStoreOwnerId::Signature(definition),
        );
        let (_, body_source_map) = Body::with_source_map(self.db, definition);
        let inference_result = InferenceResult::of(self.db, definition);

        let mut types: Vec<(SyntaxNode, Type)> = Vec::new();

        for (binding, r#type) in inference_result.type_of_binding.iter() {
            let node = match body_source_map.binding_to_source(binding) {
                Ok(sp) => sp.to_node(&self.root).syntax().clone(),
                Err(SyntheticSyntax) => continue,
            };
            types.push((node.clone(), *r#type));
        }

        for (expr, r#type) in inference_result.type_of_expression.iter() {
            let node = match body_source_map.expression_to_source(expr) {
                Ok(sp) => sp.to_node(&self.root).syntax().clone(),
                Err(SyntheticSyntax) => continue,
            };
            types.push((node.clone(), *r#type));
        }

        // sort ranges for consistency
        types.sort_by_key(|(node, _)| {
            let range = node.text_range();
            (range.start(), range.end())
        });

        for (node, r#type) in types {
            self.print_type(&node, r#type, buffer);
        }

        for diagnostic in inference_result.diagnostics() {
            let source_map = match diagnostic.source {
                ExpressionStoreSource::Body => body_source_map.expression_source_map(),
                ExpressionStoreSource::Signature => signature_map,
            };
            self.print_diagnostic(diagnostic, source_map, buffer);
        }
    }

    fn print_type(
        &self,
        node: &SyntaxNode,
        r#type: Type,
        buffer: &mut String,
    ) {
        let (range, text) = (
            node.text_range(),
            node.text().to_string().replace('\n', " "),
        );
        let pretty = pretty_type_with_verbosity(self.db, r#type, self.verbosity);
        writeln!(buffer, "{range:?} '{}': {pretty}", ellipsize(text, 15)).unwrap();
    }

    #[expect(clippy::too_many_lines, reason = "long match")]
    fn print_diagnostic(
        &self,
        diagnostic: &InferenceDiagnostic,
        source_map: &ExpressionSourceMap,
        buffer: &mut String,
    ) {
        match &diagnostic.kind {
            InferenceDiagnosticKind::TypeMismatch {
                expression,
                expected,
                actual,
            } => {
                debug_assert!(
                    !actual.is_err(self.db),
                    "don't give a diagnostic for downstream issues"
                );
                self.print_type_mismatch(source_map, buffer, *expression, *expected, *actual);
            },
            InferenceDiagnosticKind::AssignmentNotAReference { actual, left_side } => {
                debug_assert!(
                    !actual.is_err(self.db),
                    "don't give a diagnostic for downstream issues"
                );
                self.print_assignment_not_a_reference(source_map, buffer, *actual, *left_side);
            },
            InferenceDiagnosticKind::AssignmentNotWritable { left_side } => {
                self.print_assignment_not_writable(source_map, buffer, *left_side);
            },
            InferenceDiagnosticKind::CyclicType { name, range } => {
                self.print_cyclic_type(buffer, name, *range);
            },
            InferenceDiagnosticKind::WgslError {
                expression,
                message,
            } => {
                self.print_wgsl_error(source_map, buffer, *expression, message);
            },
            InferenceDiagnosticKind::UnexpectedTemplateArgument { expression } => {
                self.print_unexpected_template_argument(source_map, buffer, *expression);
            },
            InferenceDiagnosticKind::UnexpectedLoweredKind {
                actual,
                expected,
                expression,
                path,
            } => {
                self.print_unexpected_lower_kind(
                    source_map,
                    buffer,
                    *actual,
                    *expected,
                    *expression,
                    path,
                );
            },
            InferenceDiagnosticKind::InvalidType { error } => {
                self.print_invalid_type(source_map, buffer, error);
            },
            InferenceDiagnosticKind::NoConstructor {
                expression,
                parameters,
                r#type,
            } => {
                debug_assert!(
                    !r#type.is_err(self.db),
                    "don't give a diagnostic for downstream issues"
                );
                self.print_no_constructor(source_map, buffer, *expression, parameters, *r#type);
            },
            InferenceDiagnosticKind::NoOverload {
                expression,
                parameters,
                name,
            } => {
                self.print_no_overload(source_map, buffer, *expression, parameters, name);
            },
            InferenceDiagnosticKind::NoSuchField {
                expression,
                name,
                r#type,
            } => {
                debug_assert!(
                    !r#type.is_err(self.db),
                    "don't give a diagnostic for downstream issues"
                );
                self.print_no_such_field(source_map, buffer, *expression, name, *r#type);
            },
            InferenceDiagnosticKind::StoreTypeMustBeStorable { actual, expression } => {
                debug_assert!(
                    !actual.is_err(self.db),
                    "don't give a diagnostic for downstream issues"
                );
                self.print_store_type_must_be_storable(source_map, buffer, *actual, *expression);
            },
            InferenceDiagnosticKind::ArrayAccessInvalidType { expression, r#type } => {
                debug_assert!(
                    !r#type.is_err(self.db),
                    "don't give a diagnostic for downstream issues"
                );
                self.print_array_access_invalid(source_map, buffer, *expression, *r#type);
            },
            InferenceDiagnosticKind::UnexpectedReturnValue { actual, expression } => {
                self.print_unexpected_return_value(source_map, buffer, *actual, *expression);
            },
            InferenceDiagnosticKind::NotConstructible { expression, r#type } => {
                debug_assert!(
                    !r#type.is_err(self.db),
                    "don't give a diagnostic for downstream issues"
                );
                self.print_not_constructible(source_map, buffer, *expression, *r#type);
            },
            InferenceDiagnosticKind::FunctionCallArgCountMismatch {
                expression,
                n_actual,
                n_expected,
            } => {
                self.print_function_call_argument_count_mismatch(
                    source_map,
                    buffer,
                    *expression,
                    *n_actual,
                    *n_expected,
                );
            },
        }
    }

    fn print_assignment_not_a_reference(
        &self,
        source_map: &ExpressionSourceMap,
        buffer: &mut String,
        actual: Type,
        left_side: ExpressionId,
    ) {
        let Some((range, text)) = self.get_expression_range_text(source_map, left_side) else {
            return;
        };
        writeln!(
            buffer,
            "{range:?} '{}': cannot assign to non-reference `{}`",
            ellipsize(text, 15),
            pretty_type(self.db, actual),
        )
        .unwrap();
    }

    fn print_assignment_not_writable(
        &self,
        source_map: &ExpressionSourceMap,
        buffer: &mut String,
        left_side: ExpressionId,
    ) {
        let Some((range, text)) = self.get_expression_range_text(source_map, left_side) else {
            return;
        };
        writeln!(
            buffer,
            "{range:?} '{}': cannot assign to value with `read` access mode",
            ellipsize(text, 15),
        )
        .unwrap();
    }

    fn print_unexpected_template_argument(
        &self,
        source_map: &ExpressionSourceMap,
        buffer: &mut String,
        expression: ExpressionId,
    ) {
        let Some((range, text)) = self.get_expression_range_text(source_map, expression) else {
            return;
        };
        writeln!(
            buffer,
            "{range:?} '{}': unexpected template argument `{text}`",
            ellipsize(text.clone(), 15),
        )
        .unwrap();
    }

    fn print_unexpected_lower_kind(
        &self,
        source_map: &ExpressionSourceMap,
        buffer: &mut String,
        actual: LoweredKind,
        expected: LoweredKind,
        expression: ExpressionId,
        path: &hir_def::expression_store::path::Path,
    ) {
        let Some((range, text)) = self.get_expression_range_text(source_map, expression) else {
            return;
        };
        writeln!(
            buffer,
            "{range:?} '{}': expected {expected}, but got {actual} `{}`",
            ellipsize(text, 15),
            path.0
        )
        .unwrap();
    }

    fn print_type_mismatch(
        &self,
        source_map: &ExpressionSourceMap,
        buffer: &mut String,
        expression: ExpressionId,
        expected: TypeExpectation,
        actual: Type,
    ) {
        let Some((range, text)) = self.get_expression_range_text(source_map, expression) else {
            return;
        };
        writeln!(
            buffer,
            "{range:?} '{}': expected {} but got {}",
            ellipsize(text, 15),
            pretty_type_expectation_with_verbosity(self.db, expected, self.verbosity),
            pretty_type_with_verbosity(self.db, actual, self.verbosity)
        )
        .unwrap();
    }

    fn print_invalid_type(
        &self,
        source_map: &ExpressionSourceMap,
        buffer: &mut String,
        error: &TypeLoweringError,
    ) {
        let Some((range, text)) = (match error.container {
            TypeContainer::Expression(expression) => {
                self.get_expression_range_text(source_map, expression)
            },
            TypeContainer::TypeSpecifier(type_specifier) => {
                self.get_type_range_text(source_map, type_specifier)
            },
        }) else {
            return;
        };
        writeln!(
            buffer,
            "{range:?} '{}': {}",
            ellipsize(text, 15),
            error.kind.display(self.db),
        )
        .unwrap();
    }

    fn print_no_constructor(
        &self,
        source_map: &ExpressionSourceMap,
        buffer: &mut String,
        expression: ExpressionId,
        parameters: &[Type],
        r#type: Type,
    ) {
        let Some((range, text)) = self.get_expression_range_text(source_map, expression) else {
            return;
        };
        if parameters.is_empty() {
            writeln!(
                buffer,
                "{range:?} '{}': no overload of constructor `{}` found that takes no arguments",
                ellipsize(text, 15),
                pretty_type(self.db, r#type),
            )
            .unwrap();
        } else {
            let parameters =
                join_display(parameters.iter().map(|parameter| {
                    pretty_type_with_verbosity(self.db, *parameter, self.verbosity)
                }));
            writeln!(
                buffer,
                "{range:?} '{}': no overload of constructor `{}` found for arguments of type ({parameters})",
                ellipsize(text, 15),
                pretty_type(self.db, r#type),
            )
            .unwrap();
        }
    }

    fn print_no_overload(
        &self,
        source_map: &ExpressionSourceMap,
        buffer: &mut String,
        expression: ExpressionId,
        parameters: &[Type],
        name: &Name,
    ) {
        let Some((range, text)) = self.get_expression_range_text(source_map, expression) else {
            return;
        };
        if parameters.is_empty() {
            writeln!(
                buffer,
                "{range:?} '{}': no overload of function `{}` found that takes no arguments",
                ellipsize(text, 15),
                name.as_str(),
            )
            .unwrap();
        } else {
            let parameters =
                join_display(parameters.iter().map(|parameter| {
                    pretty_type_with_verbosity(self.db, *parameter, self.verbosity)
                }));
            writeln!(
                buffer,
                "{range:?} '{}': no overload of function `{}` found for arguments of type ({parameters})",
                ellipsize(text, 15),
                name.as_str(),
            )
            .unwrap();
        }
    }

    #[expect(clippy::unused_self, reason = "intended API")]
    fn print_cyclic_type(
        &self,
        buffer: &mut String,
        name: &Name,
        range: TextRange,
    ) {
        writeln!(
            buffer,
            "{range:?}: cyclic definition for type `{}`",
            name.as_str()
        )
        .unwrap();
    }

    fn print_wgsl_error(
        &self,
        source_map: &ExpressionSourceMap,
        buffer: &mut String,
        expression: ExpressionId,
        message: &str,
    ) {
        let node = match source_map.expression_to_source(expression) {
            Ok(sp) => sp.to_node(&self.root).syntax().clone(),
            Err(SyntheticSyntax) => return,
        };
        let (range, text) = (
            node.text_range(),
            node.text().to_string().replace('\n', " "),
        );
        writeln!(buffer, "{range:?} '{}': {message}", ellipsize(text, 15)).unwrap();
    }

    fn print_no_such_field(
        &self,
        source_map: &ExpressionSourceMap,
        buffer: &mut String,
        expression: ExpressionId,
        name: &Name,
        r#type: Type,
    ) {
        let node = match source_map.expression_to_source(expression) {
            Ok(sp) => sp.to_node(&self.root).syntax().clone(),
            Err(SyntheticSyntax) => return,
        };
        let (range, text) = (
            node.parent().unwrap().text_range(),
            node.parent().unwrap().text().to_string().replace('\n', " "),
        );
        writeln!(
            buffer,
            "{range:?} '{}': no such field `{}` on type `{}`",
            ellipsize(text, 15),
            name.as_str(),
            pretty_type_with_verbosity(self.db, r#type, self.verbosity),
        )
        .unwrap();
    }

    fn print_store_type_must_be_storable(
        &self,
        source_map: &ExpressionSourceMap,
        buffer: &mut String,
        actual: Type,
        expression: ExpressionId,
    ) {
        let Some((range, text)) = self.get_expression_range_text(source_map, expression) else {
            return;
        };
        writeln!(
            buffer,
            "{range:?} '{}': expected storable type but got `{}`",
            ellipsize(text, 15),
            pretty_type_with_verbosity(self.db, actual, self.verbosity),
        )
        .unwrap();
    }

    fn print_array_access_invalid(
        &self,
        source_map: &ExpressionSourceMap,
        buffer: &mut String,
        expression: ExpressionId,
        r#type: Type,
    ) {
        let Some((range, text)) = self.get_expression_range_text(source_map, expression) else {
            return;
        };
        writeln!(
            buffer,
            "{range:?} '{}': cannot index into type {}",
            ellipsize(text, 15),
            pretty_type_with_verbosity(self.db, r#type, self.verbosity),
        )
        .unwrap();
    }

    fn print_unexpected_return_value(
        &self,
        source_map: &ExpressionSourceMap,
        buffer: &mut String,
        actual: Type,
        expression: ExpressionId,
    ) {
        let Some((range, text)) = self.get_expression_range_text(source_map, expression) else {
            return;
        };
        writeln!(
            buffer,
            "{range:?} '{}': unexpected return value of type `{}` in function with no return type",
            ellipsize(text, 15),
            pretty_type_with_verbosity(self.db, actual, self.verbosity),
        )
        .unwrap();
    }

    fn print_not_constructible(
        &self,
        source_map: &ExpressionSourceMap,
        buffer: &mut String,
        expression: ExpressionId,
        r#type: Type,
    ) {
        let Some((range, text)) = self.get_expression_range_text(source_map, expression) else {
            return;
        };
        writeln!(
            buffer,
            "{range:?} '{}': type `{}` is not constructible",
            ellipsize(text, 15),
            pretty_type_with_verbosity(self.db, r#type, self.verbosity),
        )
        .unwrap();
    }

    fn print_function_call_argument_count_mismatch(
        &self,
        source_map: &ExpressionSourceMap,
        buffer: &mut String,
        expression: ExpressionId,
        n_actual: usize,
        n_expected: usize,
    ) {
        let Some((range, text)) = self.get_expression_range_text(source_map, expression) else {
            return;
        };
        writeln!(
            buffer,
            "{range:?} '{}': expected `{n_expected}` arguments, but received `{n_actual}`",
            ellipsize(text, 15),
        )
        .unwrap();
    }

    fn get_expression_range_text(
        &self,
        source_map: &ExpressionSourceMap,
        expression: ExpressionId,
    ) -> Option<(base_db::TextRange, String)> {
        let node = match source_map.expression_to_source(expression) {
            Ok(sp) => sp.to_node(&self.root).syntax().clone(),
            Err(SyntheticSyntax) => return None,
        };
        let (range, text) = (
            node.text_range(),
            node.text().to_string().replace('\n', " "),
        );
        Some((range, text))
    }

    fn get_type_range_text(
        &self,
        source_map: &ExpressionSourceMap,
        r#type: TypeSpecifierId,
    ) -> Option<(base_db::TextRange, String)> {
        let node = match source_map.type_specifier_to_source(r#type) {
            Ok(sp) => sp.to_node(&self.root).syntax().clone(),
            Err(SyntheticSyntax) => return None,
        };
        let (range, text) = (
            node.text_range(),
            node.text().to_string().replace('\n', " "),
        );
        Some((range, text))
    }
}

fn text_range_start(
    definition: ModuleDefinitionId,
    db: &TestDatabase,
) -> base_db::TextSize {
    match definition {
        ModuleDefinitionId::Function(item) => item
            .lookup(db)
            .source(db)
            .value
            .syntax()
            .text_range()
            .start(),
        ModuleDefinitionId::GlobalConstant(item) => item
            .lookup(db)
            .source(db)
            .value
            .syntax()
            .text_range()
            .start(),
        ModuleDefinitionId::GlobalVariable(item) => item
            .lookup(db)
            .source(db)
            .value
            .syntax()
            .text_range()
            .start(),
        ModuleDefinitionId::Override(item) => item
            .lookup(db)
            .source(db)
            .value
            .syntax()
            .text_range()
            .start(),
        ModuleDefinitionId::GlobalAssertStatement(item) => item
            .lookup(db)
            .source(db)
            .value
            .syntax()
            .text_range()
            .start(),
        ModuleDefinitionId::Struct(item) => item
            .lookup(db)
            .source(db)
            .value
            .syntax()
            .text_range()
            .start(),
        ModuleDefinitionId::TypeAlias(item) => item
            .lookup(db)
            .source(db)
            .value
            .syntax()
            .text_range()
            .start(),
    }
}

fn module_definitions(
    db: &TestDatabase,
    file_id: EditionedFileId,
    item_tree: &hir_def::item_tree::ItemTree,
) -> Vec<ModuleDefinitionId> {
    item_tree
        .top_level_items()
        .iter()
        .filter_map(|item| {
            Some(match item {
                ModuleItemId::Function(id) => {
                    ModuleDefinitionId::Function(Location::new(file_id, *id).intern(db))
                },
                ModuleItemId::GlobalVariable(id) => {
                    ModuleDefinitionId::GlobalVariable(Location::new(file_id, *id).intern(db))
                },
                ModuleItemId::GlobalConstant(id) => {
                    ModuleDefinitionId::GlobalConstant(Location::new(file_id, *id).intern(db))
                },
                ModuleItemId::Override(id) => {
                    ModuleDefinitionId::Override(Location::new(file_id, *id).intern(db))
                },
                ModuleItemId::GlobalAssertStatement(id) => {
                    ModuleDefinitionId::GlobalAssertStatement(
                        Location::new(file_id, *id).intern(db),
                    )
                },
                ModuleItemId::TypeAlias(id) => {
                    ModuleDefinitionId::TypeAlias(Location::new(file_id, *id).intern(db))
                },
                ModuleItemId::Struct(id) => {
                    ModuleDefinitionId::Struct(Location::new(file_id, *id).intern(db))
                },
                ModuleItemId::ImportStatement(_) => return None,
            })
        })
        .collect()
}

fn ellipsize(
    mut text: String,
    max_length: usize,
) -> String {
    if text.len() <= max_length {
        return text;
    }
    const ELLIPSIS: &str = "...";
    let e_length = ELLIPSIS.len();
    #[expect(clippy::integer_division, reason = "precision loss is not a concern")]
    let mut prefix_length = (max_length - e_length) / 2;
    while !text.is_char_boundary(prefix_length) {
        prefix_length += 1;
    }
    let mut suffix_length = max_length - e_length - prefix_length;
    while !text.is_char_boundary(text.len() - suffix_length) {
        suffix_length += 1;
    }
    text.replace_range(prefix_length..text.len() - suffix_length, ELLIPSIS);
    text
}

#[expect(clippy::semicolon_if_nothing_returned, reason = "wrapper")]
fn check_infer(
    wa_fixture: &str,
    expect: Expect,
) {
    check_infer_with_verbosity(TypeVerbosity::Full, wa_fixture, expect)
}

#[expect(clippy::semicolon_if_nothing_returned, reason = "wrapper")]
fn check_infer_with_verbosity(
    verbosity: TypeVerbosity,
    wa_fixture: &str,
    expect: Expect,
) {
    check_infer_with_capabilities_verbosity(Capabilities::default(), verbosity, wa_fixture, expect)
}

#[expect(clippy::needless_pass_by_value, reason = "Matches expect! macro")]
fn check_infer_with_capabilities(
    capabilities: Capabilities,
    wa_fixture: &str,
    expect: Expect,
) {
    let mut actual = infer(capabilities, TypeVerbosity::Full, wa_fixture);
    actual.push('\n');
    expect.assert_eq(&actual);
}
#[expect(clippy::needless_pass_by_value, reason = "Matches expect! macro")]
fn check_infer_with_capabilities_verbosity(
    capabilities: Capabilities,
    verbosity: TypeVerbosity,
    wa_fixture: &str,
    expect: Expect,
) {
    let mut actual = infer(capabilities, verbosity, wa_fixture);
    actual.push('\n');
    expect.assert_eq(&actual);
}

fn join_display<I, T>(iter: I) -> String
where
    I: IntoIterator<Item = T>,
    T: std::fmt::Display,
{
    iter.into_iter()
        .map(|item| item.to_string())
        .collect::<Vec<_>>()
        .join(", ")
}
