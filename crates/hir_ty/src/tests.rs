#![expect(clippy::use_debug, reason = "tests")]

mod big;
mod builtins;
mod imports;
mod incremental;
mod layout;
mod operators;
mod simple;
use std::{fmt::Write as _, ops::ControlFlow};

use base_db::{EditionedFileId, ExtensionsConfigInput, Intern as _, Lookup as _};
use expect_test::Expect;
use hir_def::{
    HasSource as _,
    body::{Body, BodySourceMap},
    db::{DefinitionWithBodyId, Location, ModuleDefinitionId},
    expression::ExpressionId,
    expression_store::{
        ExpressionSourceMap, ExpressionStore, ExpressionStoreOwnerId, ExpressionStoreSource,
        SyntheticSyntax,
    },
    item_tree::{ItemTree, ModuleItemId, Name},
    signature::{StructSignature, TypeAliasSignature},
    type_specifier::{self, TypeSpecifierId},
};
use itertools::Itertools as _;
use syntax::{AstNode as _, Diagnostic, ExtensionsConfig, SyntaxNode};
use test_fixture::WithFixture as _;
use triomphe::Arc;

use crate::{
    db::HirDatabase as _,
    diagnostics::{self, InferenceDiagnostic, InferenceDiagnosticKind},
    infer::{InferenceResult, TypeExpectation},
    lower::{LoweredKind, TypeContainer, TypeLoweringError},
    test_db::TestDatabase,
    ty::{
        Type,
        pretty::{
            TypeVerbosity, pretty_type_expectation_with_verbosity, pretty_type_with_verbosity,
        },
    },
};

fn infer(
    extensions: ExtensionsConfig,
    wa_fixture: &str,
) -> String {
    let (mut db, files) = TestDatabase::with_many_files(wa_fixture);
    ExtensionsConfigInput::update_extensions(&mut db, extensions);
    let mut buffer = String::new();

    for (index, file_id) in files.into_iter().enumerate() {
        if index > 0 {
            buffer.push_str("---\n");
        }
        InferPrinter::new(&db, file_id).infer_file(&mut buffer);
    }
    buffer.truncate(buffer.trim_end().len());
    buffer
}

struct InferPrinter<'db> {
    db: &'db TestDatabase,
    file_id: EditionedFileId,
    root: SyntaxNode,
}

impl<'db> InferPrinter<'db> {
    fn new(
        db: &'db TestDatabase,
        file_id: EditionedFileId,
    ) -> Self {
        let parse = file_id.parse(db);
        assert_eq!(<&[Diagnostic]>::default(), parse.errors());
        let root = parse.syntax();
        Self { db, file_id, root }
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
        let pretty = pretty_type_with_verbosity(self.db, r#type, TypeVerbosity::Full);
        writeln!(buffer, "{range:?} '{}': {pretty}", ellipsize(text, 15)).unwrap();
    }

    fn print_diagnostic(
        &self,
        diagnostic: &InferenceDiagnostic,
        source_map: &ExpressionSourceMap,
        buffer: &mut String,
    ) {
        use InferenceDiagnosticKind as IDK;
        match &diagnostic.kind {
            IDK::TypeMismatch {
                expression,
                expected,
                actual,
            } => {
                self.print_type_mismatch(source_map, buffer, *expression, *expected, *actual);
            },
            IDK::AssignmentNotAReference { .. }
            | IDK::AddressOfNotReference { .. }
            | IDK::AddressOfNotReference { .. }
            | IDK::DerefNotAPointer { .. }
            | IDK::CyclicType { .. }
            | IDK::UnexpectedTemplateArgument { .. }
            | IDK::WgslError { .. } => {
                self.print_todo_bad_diagnostic(diagnostic, buffer);
            },
            IDK::NoBuiltinOverload {
                builtin,
                expression,
                name,
                parameters,
            } => {
                self.print_no_builtin_overload(source_map, buffer, *expression, *name, parameters);
            },
            IDK::UnexpectedLoweredKind {
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
            IDK::InvalidType { error } => {
                self.print_invalid_type(source_map, buffer, error);
            },
            IDK::NoConstructor {
                builtins,
                expression,
                parameters,
                r#type,
            } => {
                self.print_no_constructor(source_map, buffer, *builtins, *expression, parameters);
            },
            IDK::NoSuchField {
                expression,
                name,
                r#type,
            } => {
                self.print_no_such_field(source_map, buffer, *expression, name, *r#type);
            },
            IDK::StoreTypeMustBeStorable { actual, expression } => {
                self.print_store_type_must_be_storable(source_map, buffer, *actual, *expression);
            },
            IDK::ArrayAccessInvalidType { expression, r#type } => {
                self.print_array_access_invalid(source_map, buffer, *expression, *r#type);
            },
            IDK::UnexpectedReturnValue { actual, expression } => {
                self.print_unexpected_return_value(source_map, buffer, *actual, *expression);
            },
            IDK::NotConstructible { expression, r#type } => {
                self.print_not_constructible(source_map, buffer, *expression, *r#type);
            },
            IDK::FunctionCallArgCountMismatch {
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

    fn print_no_builtin_overload(
        &self,
        source_map: &ExpressionSourceMap,
        buffer: &mut String,
        expression: ExpressionId,
        name: Option<&'static str>,
        parameters: &[Type],
    ) {
        let Some((range, text)) = self.get_expression_range_text(source_map, expression) else {
            return;
        };
        writeln!(
            buffer,
            "{range:?} '{}': no built-in overload of `{}` with parameters: ({})",
            ellipsize(text, 15),
            name.unwrap_or("<missing>"),
            parameters
                .iter()
                .map(|r#type| pretty_type_with_verbosity(self.db, *r#type, TypeVerbosity::Full))
                .join(", ")
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
            pretty_type_expectation_with_verbosity(self.db, expected, TypeVerbosity::Full),
            pretty_type_with_verbosity(self.db, actual, TypeVerbosity::Full)
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
            error.kind,
        )
        .unwrap();
    }

    fn print_no_constructor(
        &self,
        source_map: &ExpressionSourceMap,
        buffer: &mut String,
        builtins: crate::builtins::BuiltinId,
        expression: ExpressionId,
        parameters: &[Type],
    ) {
        let Some((range, text)) = self.get_expression_range_text(source_map, expression) else {
            return;
        };
        writeln!(
            buffer,
            "{range:?} '{}': no constructor for builtin `{}` with parameters `{}`",
            ellipsize(text, 15),
            builtins.lookup(self.db).name(),
            join_display(
                parameters
                    .iter()
                    .map(|parameter| pretty_type_with_verbosity(
                        self.db,
                        *parameter,
                        TypeVerbosity::Full
                    ))
            ),
        )
        .unwrap();
    }

    fn print_todo_bad_diagnostic(
        &self,
        diagnostic: &InferenceDiagnostic,
        buffer: &mut String,
    ) {
        writeln!(
            buffer,
            "[{:?}] {:?} in {:?}",
            self.file_id, diagnostic.kind, diagnostic.source
        )
        .unwrap();
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
            pretty_type_with_verbosity(self.db, r#type, TypeVerbosity::Full),
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
            pretty_type_with_verbosity(self.db, actual, TypeVerbosity::Full),
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
            pretty_type_with_verbosity(self.db, r#type, TypeVerbosity::Full),
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
            pretty_type_with_verbosity(self.db, actual, TypeVerbosity::Full),
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
            pretty_type_with_verbosity(self.db, r#type, TypeVerbosity::Full),
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
                ModuleItemId::ImportStatement(id) => return None,
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

#[expect(clippy::needless_pass_by_value, reason = "Matches expect! macro")]
fn check_infer(
    extensions: ExtensionsConfig,
    wa_fixture: &str,
    expect: Expect,
) {
    let mut actual = infer(extensions, wa_fixture);
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
