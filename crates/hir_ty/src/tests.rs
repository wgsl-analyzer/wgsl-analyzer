#![expect(clippy::use_debug, reason = "tests")]

mod big;
mod builtins;
mod imports;
mod incremental;
mod layout;
mod simple;
use std::{fmt::Write as _, ops::ControlFlow};

use base_db::{EditionedFileId, Intern as _, Lookup as _};
use expect_test::Expect;
use hir_def::{
    HasSource as _,
    body::{Body, BodySourceMap},
    database::{
        DefDatabase as _, DefinitionWithBodyId, InternDatabase as _, Location, ModuleDefinitionId,
    },
    expression::ExpressionId,
    expression_store::{ExpressionSourceMap, ExpressionStoreSource, SyntheticSyntax},
    item_tree::ModuleItemId,
    type_specifier::{self, TypeSpecifierId},
};
use salsa::Durability;
use syntax::{AstNode as _, ExtensionsConfig, SyntaxNode};
use test_fixture::WithFixture as _;
use triomphe::Arc;

use crate::{
    database::HirDatabase as _,
    diagnostics::{self, InferenceDiagnostic, InferenceDiagnosticKind},
    infer::InferenceResult,
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
    let (mut database, files) = TestDatabase::with_many_files(wa_fixture);
    database.set_extensions_with_durability(extensions, Durability::MEDIUM);
    let mut buffer = String::new();

    for (index, file_id) in files.into_iter().enumerate() {
        if index > 0 {
            buffer.push_str("---\n");
        }
        InferPrinter::new(&database, file_id).infer_file(&mut buffer);
    }
    buffer.truncate(buffer.trim_end().len());
    buffer
}

struct InferPrinter<'db> {
    database: &'db TestDatabase,
    file_id: EditionedFileId,
    root: SyntaxNode,
}

impl<'db> InferPrinter<'db> {
    fn new(
        database: &'db TestDatabase,
        file_id: EditionedFileId,
    ) -> Self {
        let root = file_id.parse(database).syntax();
        Self {
            database,
            file_id,
            root,
        }
    }

    fn infer_file(
        &self,
        buffer: &mut String,
    ) {
        let module_info = self.database.item_tree(self.file_id);
        let mut definitions = module_definitions(self.database, self.file_id, &module_info);
        definitions.sort_by_key(|definition| text_range_start(*definition, self.database));
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
                    let (_, signature_map) = self.database.struct_data(id);
                    let (_, diagnostics) = &*self.database.field_types(id);

                    for diagnostic in diagnostics {
                        self.print_diagnostic(diagnostic, &signature_map, buffer);
                    }
                },
                ModuleDefinitionId::TypeAlias(id) => {
                    let (_, signature_map) = self.database.type_alias_data(id);
                    let (_, diagnostics) = &*self.database.type_alias_type(id);
                    for diagnostic in diagnostics {
                        self.print_diagnostic(diagnostic, &signature_map, buffer);
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
        let (_, signature_map) = self.database.signature_with_source_map(definition);
        let (_, body_source_map) = self.database.body_with_source_map(definition);
        let inference_result = InferenceResult::of(self.database, definition);

        let mut types: Vec<(SyntaxNode, &Type)> = Vec::new();

        for (binding, r#type) in inference_result.type_of_binding.iter() {
            let node = match body_source_map.binding_to_source(binding) {
                Ok(sp) => sp.to_node(&self.root).syntax().clone(),
                Err(SyntheticSyntax) => continue,
            };
            types.push((node.clone(), r#type));
        }

        for (expr, r#type) in inference_result.type_of_expression.iter() {
            let node = match body_source_map.expression_to_source(expr) {
                Ok(sp) => sp.to_node(&self.root).syntax().clone(),
                Err(SyntheticSyntax) => continue,
            };
            types.push((node.clone(), r#type));
        }

        // sort ranges for consistency
        types.sort_by_key(|(node, _)| {
            let range = node.text_range();
            (range.start(), range.end())
        });

        for (node, r#type) in types {
            self.print_type(&node, *r#type, buffer);
        }

        for diagnostic in inference_result.diagnostics() {
            let source_map = match diagnostic.source {
                ExpressionStoreSource::Body => body_source_map.expression_source_map(),
                ExpressionStoreSource::Signature => &signature_map,
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
        let pretty = pretty_type_with_verbosity(self.database, r#type, TypeVerbosity::Full);
        writeln!(buffer, "{range:?} '{}': {pretty}", ellipsize(text, 15)).unwrap();
    }

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
                let Some((range, text)) = self.get_expression_range_text(source_map, *expression)
                else {
                    return;
                };
                writeln!(
                    buffer,
                    "{range:?} '{}': expected {} but got {}",
                    ellipsize(text, 15),
                    pretty_type_expectation_with_verbosity(
                        self.database,
                        expected.clone(),
                        TypeVerbosity::Full
                    ),
                    pretty_type_with_verbosity(self.database, *actual, TypeVerbosity::Full)
                )
                .unwrap();
            },
            InferenceDiagnosticKind::AssignmentNotAReference { .. }
            | InferenceDiagnosticKind::UnresolvedName { .. }
            | InferenceDiagnosticKind::NoBuiltinOverload { .. }
            | InferenceDiagnosticKind::AddressOfNotReference { .. }
            | InferenceDiagnosticKind::AddressOfNotReference { .. }
            | InferenceDiagnosticKind::DerefNotAPointer { .. }
            | InferenceDiagnosticKind::CyclicType { .. }
            | InferenceDiagnosticKind::UnexpectedTemplateArgument { .. }
            | InferenceDiagnosticKind::WgslError { .. }
            | InferenceDiagnosticKind::ExpectedLoweredKind { .. } => {
                writeln!(buffer, "{:?} in {:?}", diagnostic.kind, diagnostic.source).unwrap();
            },
            InferenceDiagnosticKind::InvalidType { error } => {
                let Some((range, text)) = (match error.container {
                    crate::lower::TypeContainer::Expression(expression) => {
                        self.get_expression_range_text(source_map, expression)
                    },
                    crate::lower::TypeContainer::TypeSpecifier(type_specifier) => {
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
            },
            InferenceDiagnosticKind::NoConstructor {
                builtins,
                expression,
                parameters,
                r#type,
            } => {
                let Some((range, text)) = self.get_expression_range_text(source_map, *expression)
                else {
                    return;
                };
                writeln!(
                    buffer,
                    "{range:?} '{}': no constructor for builtin `{}` with parameters `{}`",
                    ellipsize(text, 15),
                    builtins.lookup(self.database).name(),
                    join_display(
                        parameters
                            .iter()
                            .map(|parameter| pretty_type_with_verbosity(
                                self.database,
                                *parameter,
                                TypeVerbosity::Full
                            ))
                    ),
                )
                .unwrap();
            },
            InferenceDiagnosticKind::NoSuchField {
                expression,
                name,
                r#type,
            } => {
                let node = match source_map.expression_to_source(*expression) {
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
                    pretty_type_with_verbosity(self.database, *r#type, TypeVerbosity::Full),
                )
                .unwrap();
            },
            InferenceDiagnosticKind::StoreTypeMustBeStorable { actual, expression } => {
                let Some((range, text)) = self.get_expression_range_text(source_map, *expression)
                else {
                    return;
                };
                writeln!(
                    buffer,
                    "{range:?} '{}': expected storable type but got `{}`",
                    ellipsize(text, 15),
                    pretty_type_with_verbosity(self.database, *actual, TypeVerbosity::Full),
                )
                .unwrap();
            },
            InferenceDiagnosticKind::ArrayAccessInvalidType { expression, r#type } => {
                let Some((range, text)) = self.get_expression_range_text(source_map, *expression)
                else {
                    return;
                };
                writeln!(
                    buffer,
                    "{range:?} '{}': cannot index into type {}",
                    ellipsize(text, 15),
                    pretty_type_with_verbosity(self.database, *r#type, TypeVerbosity::Full),
                )
                .unwrap();
            },
            InferenceDiagnosticKind::UnexpectedReturnValue { actual, expression } => {
                let Some((range, text)) = self.get_expression_range_text(source_map, *expression)
                else {
                    return;
                };
                writeln!(
                    buffer,
                    "{range:?} '{}': unexpected return value of type `{}` in function with no return type",
                    ellipsize(text, 15),
                    pretty_type_with_verbosity(self.database, *actual, TypeVerbosity::Full),
                )
                .unwrap();
            },
            InferenceDiagnosticKind::NotConstructible { expression, r#type } => {
                let Some((range, text)) = self.get_expression_range_text(source_map, *expression)
                else {
                    return;
                };
                writeln!(
                    buffer,
                    "{range:?} '{}': type `{}` is not constructible",
                    ellipsize(text, 15),
                    pretty_type_with_verbosity(self.database, *r#type, TypeVerbosity::Full),
                )
                .unwrap();
            },
            InferenceDiagnosticKind::FunctionCallArgCountMismatch {
                expression,
                n_actual,
                n_expected,
            } => {
                let Some((range, text)) = self.get_expression_range_text(source_map, *expression)
                else {
                    return;
                };
                writeln!(
                    buffer,
                    "{range:?} '{}': expected `{n_expected}` arguments, but received `{n_actual}`",
                    ellipsize(text, 15),
                )
                .unwrap();
            },
        }
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
    database: &TestDatabase,
) -> base_db::TextSize {
    match definition {
        ModuleDefinitionId::Function(item) => item
            .lookup(database)
            .source(database)
            .value
            .syntax()
            .text_range()
            .start(),
        ModuleDefinitionId::GlobalConstant(item) => item
            .lookup(database)
            .source(database)
            .value
            .syntax()
            .text_range()
            .start(),
        ModuleDefinitionId::GlobalVariable(item) => item
            .lookup(database)
            .source(database)
            .value
            .syntax()
            .text_range()
            .start(),
        ModuleDefinitionId::Override(item) => item
            .lookup(database)
            .source(database)
            .value
            .syntax()
            .text_range()
            .start(),
        ModuleDefinitionId::GlobalAssertStatement(item) => item
            .lookup(database)
            .source(database)
            .value
            .syntax()
            .text_range()
            .start(),
        ModuleDefinitionId::Struct(item) => item
            .lookup(database)
            .source(database)
            .value
            .syntax()
            .text_range()
            .start(),
        ModuleDefinitionId::TypeAlias(item) => item
            .lookup(database)
            .source(database)
            .value
            .syntax()
            .text_range()
            .start(),
    }
}

fn module_definitions(
    database: &TestDatabase,
    file_id: EditionedFileId,
    item_tree: &hir_def::item_tree::ItemTree,
) -> Vec<ModuleDefinitionId> {
    item_tree
        .top_level_items()
        .iter()
        .filter_map(|item| {
            Some(match item {
                ModuleItemId::Function(id) => {
                    ModuleDefinitionId::Function(Location::new(file_id, *id).intern(database))
                },
                ModuleItemId::GlobalVariable(id) => {
                    ModuleDefinitionId::GlobalVariable(Location::new(file_id, *id).intern(database))
                },
                ModuleItemId::GlobalConstant(id) => {
                    ModuleDefinitionId::GlobalConstant(Location::new(file_id, *id).intern(database))
                },
                ModuleItemId::Override(id) => {
                    ModuleDefinitionId::Override(Location::new(file_id, *id).intern(database))
                },
                ModuleItemId::GlobalAssertStatement(id) => {
                    ModuleDefinitionId::GlobalAssertStatement(
                        Location::new(file_id, *id).intern(database),
                    )
                },
                ModuleItemId::TypeAlias(id) => {
                    ModuleDefinitionId::TypeAlias(Location::new(file_id, *id).intern(database))
                },
                ModuleItemId::Struct(id) => {
                    ModuleDefinitionId::Struct(Location::new(file_id, *id).intern(database))
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
