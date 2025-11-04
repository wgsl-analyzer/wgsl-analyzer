#![expect(clippy::use_debug, reason = "tests")]

mod simple;
use expect_test::Expect;
use hir_def::{
    HasSource as _, HirFileId,
    body::{Body, BodySourceMap},
    database::{
        DefDatabase as _, DefinitionWithBodyId, InternDatabase as _, Location, Lookup as _,
    },
    expression_store::SyntheticSyntax,
    module_data::ModuleItem,
};
use std::fmt::Write as _;
use syntax::{AstNode as _, SyntaxNode};
use triomphe::Arc;

use crate::{
    database::HirDatabase as _,
    infer::{InferenceDiagnostic, InferenceResult},
    test_db::{TestDatabase, single_file_db},
    ty::{
        Type,
        pretty::{
            TypeVerbosity, pretty_type_expectation_with_verbosity, pretty_type_with_verbosity,
        },
    },
};

fn infer(ra_fixture: &str) -> String {
    let (database, file_id) = single_file_db(ra_fixture);
    let file_id = HirFileId::from(file_id);
    let root = database.parse_or_resolve(file_id).syntax();
    let mut buffer = String::new();
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
            let pretty = pretty_type_with_verbosity(&database, *r#type, TypeVerbosity::Compact);
            writeln!(buffer, "{range:?} '{}': {pretty}", ellipsize(text, 15));
        }

        // It'd be nicer if the diagnostics were sorted with the types.
        // But this is good enough for unit tests
        for diagnostic in inference_result.diagnostics() {
            match diagnostic {
                InferenceDiagnostic::TypeMismatch {
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
                            &database,
                            expected.clone(),
                            TypeVerbosity::Compact
                        ),
                        pretty_type_with_verbosity(&database, *actual, TypeVerbosity::Compact)
                    );
                },
                InferenceDiagnostic::AssignmentNotAReference { .. }
                | InferenceDiagnostic::NoSuchField { .. }
                | InferenceDiagnostic::ArrayAccessInvalidType { .. }
                | InferenceDiagnostic::UnresolvedName { .. }
                | InferenceDiagnostic::InvalidConstructionType { .. }
                | InferenceDiagnostic::FunctionCallArgCountMismatch { .. }
                | InferenceDiagnostic::NoBuiltinOverload { .. }
                | InferenceDiagnostic::NoConstructor { .. }
                | InferenceDiagnostic::AddressOfNotReference { .. }
                | InferenceDiagnostic::DerefNotAPointer { .. }
                | InferenceDiagnostic::InvalidType { .. }
                | InferenceDiagnostic::CyclicType { .. }
                | InferenceDiagnostic::UnexpectedTemplateArgument { .. }
                | InferenceDiagnostic::WgslError { .. }
                | InferenceDiagnostic::ExpectedLoweredKind { .. } => {
                    writeln!(buffer, "{diagnostic:?}");
                },
            }
        }
    };
    let module_info = database.module_info(file_id);
    let mut definitions = module_definitions(&database, file_id, &module_info);
    definitions.sort_by_key(|definition| text_size(*definition, &database));
    for definition in definitions {
        let (body, source_map) = database.body_with_source_map(definition);
        let infer = database.infer(definition);
        infer_def(infer, body, source_map);
    }
    buffer.truncate(buffer.trim_end().len());
    buffer
}

fn text_size(
    definition: DefinitionWithBodyId,
    database: &TestDatabase,
) -> base_db::TextSize {
    match definition {
        DefinitionWithBodyId::Function(item) => item
            .lookup(database)
            .source(database)
            .value
            .syntax()
            .text_range()
            .start(),
        DefinitionWithBodyId::GlobalConstant(item) => item
            .lookup(database)
            .source(database)
            .value
            .syntax()
            .text_range()
            .start(),
        DefinitionWithBodyId::GlobalVariable(item) => item
            .lookup(database)
            .source(database)
            .value
            .syntax()
            .text_range()
            .start(),
        DefinitionWithBodyId::Override(item) => item
            .lookup(database)
            .source(database)
            .value
            .syntax()
            .text_range()
            .start(),
    }
}

fn module_definitions(
    db: &TestDatabase,
    file_id: HirFileId,
    module_info: &Arc<hir_def::module_data::ModuleInfo>,
) -> Vec<DefinitionWithBodyId> {
    module_info
        .items()
        .iter()
        .filter_map(|item| {
            Some(match item {
                ModuleItem::Function(id) => {
                    let loc = Location::new(file_id, *id);
                    DefinitionWithBodyId::Function(db.intern_function(loc))
                },
                ModuleItem::GlobalVariable(id) => {
                    let loc = Location::new(file_id, *id);
                    DefinitionWithBodyId::GlobalVariable(db.intern_global_variable(loc))
                },
                ModuleItem::GlobalConstant(id) => {
                    let loc = Location::new(file_id, *id);
                    DefinitionWithBodyId::GlobalConstant(db.intern_global_constant(loc))
                },
                ModuleItem::Override(id) => {
                    let loc = Location::new(file_id, *id);
                    DefinitionWithBodyId::Override(db.intern_override(loc))
                },
                ModuleItem::TypeAlias(_) | ModuleItem::Struct(_) => return None,
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
    ra_fixture: &str,
    expect: Expect,
) {
    let mut actual = infer(ra_fixture);
    actual.push('\n');
    expect.assert_eq(&actual);
}
