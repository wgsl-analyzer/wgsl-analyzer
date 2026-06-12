#![expect(clippy::use_debug, reason = "tests")]

mod builtins;
mod imports;
mod incremental;
mod simple;
use std::fmt::Write as _;

use base_db::{EditionedFileId, Intern as _, Lookup as _};
use expect_test::Expect;
use hir_def::{
    HasSource as _,
    body::{Body, BodySourceMap},
    database::{
        DefDatabase as _, DefinitionWithBodyId, InternDatabase as _, Location, ModuleDefinitionId,
    },
    expression_store::SyntheticSyntax,
    item_tree::ModuleItemId,
};
use salsa::Durability;
use syntax::{AstNode as _, ExtensionsConfig, SyntaxNode};
use test_fixture::WithFixture as _;
use triomphe::Arc;

use crate::{
    database::HirDatabase as _,
    infer::{InferenceDiagnosticKind, InferenceResult},
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

    if files.len() == 1 {
        infer_file(&database, &mut buffer, files[0]);
    } else {
        for file_id in files {
            buffer.push_str("---\n");
            infer_file(&database, &mut buffer, file_id);
        }
    }
    buffer.truncate(buffer.trim_end().len());
    buffer
}

fn infer_file(
    database: &TestDatabase,
    buffer: &mut String,
    file_id: EditionedFileId,
) {
    let root = file_id.parse(database).syntax();
    let mut infer_def = |inference_result: Arc<InferenceResult>,
                         _body: Arc<Body>,
                         body_source_map: Arc<BodySourceMap>| {
        let mut types: Vec<(SyntaxNode, &Type)> = Vec::new();

        for (binding, r#type) in inference_result.type_of_binding.iter() {
            let node = match body_source_map.binding_to_source(binding) {
                Ok(sp) => sp.to_node(&root).syntax().clone(),
                Err(SyntheticSyntax) => continue,
            };
            types.push((node.clone(), r#type));
        }

        for (expr, r#type) in inference_result.type_of_expression.iter() {
            let node = match body_source_map.expression_to_source(expr) {
                Ok(sp) => sp.to_node(&root).syntax().clone(),
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
            let (range, text) = (
                node.text_range(),
                node.text().to_string().replace('\n', " "),
            );
            let pretty = pretty_type_with_verbosity(database, *r#type, TypeVerbosity::Compact);
            writeln!(buffer, "{range:?} '{}': {pretty}", ellipsize(text, 15)).unwrap();
        }

        // It'd be nicer if the diagnostics were sorted with the types.
        // But this is good enough for unit tests
        for diagnostic in inference_result.diagnostics() {
            match &diagnostic.kind {
                InferenceDiagnosticKind::TypeMismatch {
                    expression,
                    expected,
                    actual,
                } => {
                    let node = match body_source_map.expression_to_source(*expression) {
                        Ok(sp) => sp.to_node(&root).syntax().clone(),
                        Err(SyntheticSyntax) => continue,
                    };
                    let (range, text) = (
                        node.text_range(),
                        node.text().to_string().replace('\n', " "),
                    );
                    writeln!(
                        buffer,
                        "{range:?} '{}': expected {} but got {}",
                        ellipsize(text, 15),
                        pretty_type_expectation_with_verbosity(
                            database,
                            expected.clone(),
                            TypeVerbosity::Compact
                        ),
                        pretty_type_with_verbosity(database, *actual, TypeVerbosity::Compact)
                    )
                    .unwrap();
                },
                InferenceDiagnosticKind::AssignmentNotAReference { .. }
                | InferenceDiagnosticKind::NoSuchField { .. }
                | InferenceDiagnosticKind::ArrayAccessInvalidType { .. }
                | InferenceDiagnosticKind::UnresolvedName { .. }
                | InferenceDiagnosticKind::InvalidConstructionType { .. }
                | InferenceDiagnosticKind::FunctionCallArgCountMismatch { .. }
                | InferenceDiagnosticKind::NoBuiltinOverload { .. }
                | InferenceDiagnosticKind::NoConstructor { .. }
                | InferenceDiagnosticKind::AddressOfNotReference { .. }
                | InferenceDiagnosticKind::DerefNotAPointer { .. }
                | InferenceDiagnosticKind::InvalidType { .. }
                | InferenceDiagnosticKind::CyclicType { .. }
                | InferenceDiagnosticKind::UnexpectedTemplateArgument { .. }
                | InferenceDiagnosticKind::WgslError { .. }
                | InferenceDiagnosticKind::ExpectedLoweredKind { .. } => {
                    writeln!(buffer, "{diagnostic:?}").unwrap();
                },
            }
        }
    };
    let module_info = database.item_tree(file_id);
    let mut definitions = module_definitions(database, file_id, &module_info);
    definitions.sort_by_key(|definition| text_range_start(*definition, database));
    for definition in definitions
        .into_iter()
        .filter_map(ModuleDefinitionId::with_body)
    {
        let (body, source_map) = database.body_with_source_map(definition);
        let infer = database.infer(definition);
        infer_def(infer, body, source_map);
    }
}

fn text_range_start(
    definition: ModuleDefinitionId,
    database: &TestDatabase,
) -> base_db::TextSize {
    match definition {
        ModuleDefinitionId::Module(_) => base_db::TextSize::new(0),
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
