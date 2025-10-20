mod simple;

use base_db::SourceDatabase;
use expect_test::Expect;
use hir_def::{
    AstIdMap, HasSource, HirFileId, InFile,
    body::{Body, BodySourceMap},
    database::{DefDatabase, DefinitionWithBodyId, InternDatabase, Location, Lookup},
    expression_store::SyntheticSyntax,
    module_data::ModuleItem,
};
use stdx::format_to;
use syntax::{AstNode, SyntaxNode};
use triomphe::Arc;
use wgsl_types::ty::Ty;

use crate::{
    database::HirDatabase,
    infer::InferenceResult,
    test_db::{TestDB, single_file_db},
    ty::{
        Type,
        pretty::{pretty_type_expectation_with_verbosity, pretty_type_with_verbosity},
    },
};

fn infer(ra_fixture: &str) -> String {
    let (db, file_id) = single_file_db(ra_fixture);
    let file_id = HirFileId::from(file_id);

    let root = db.parse_or_resolve(file_id).unwrap().syntax();

    let mut buf = String::new();

    let mut infer_def = |inference_result: Arc<InferenceResult>,
                         _body: Arc<Body>,
                         body_source_map: Arc<BodySourceMap>| {
        let mut types: Vec<(SyntaxNode, &Type)> = Vec::new();

        for (binding, ty) in inference_result.type_of_binding.iter() {
            let node = match body_source_map.binding_to_source(binding) {
                Ok(sp) => sp.to_node(&root).syntax().clone(),
                Err(SyntheticSyntax) => continue,
            };
            types.push((node.clone(), ty));
        }

        for (expr, ty) in inference_result.type_of_expression.iter() {
            let node = match body_source_map.expression_to_source(expr) {
                Ok(sp) => sp.to_node(&root).syntax().clone(),
                Err(SyntheticSyntax) => continue,
            };
            types.push((node.clone(), ty));
        }

        // sort ranges for consistency
        types.sort_by_key(|(node, _)| {
            let range = node.text_range();
            (range.start(), range.end())
        });
        for (node, ty) in types {
            let (range, text) = (
                node.text_range(),
                node.text().to_string().replace('\n', " "),
            );
            format_to!(
                buf,
                "{:?} '{}': {}\n",
                range,
                ellipsize(text, 15),
                pretty_type_with_verbosity(&db, *ty, crate::ty::pretty::TypeVerbosity::Compact)
            );
        }

        // It'd be nicer if the diagnostics were sorted with the types.
        // But this is good enough for unit tests
        for diagnostic in inference_result.diagnostics() {
            match diagnostic {
                crate::infer::InferenceDiagnostic::TypeMismatch {
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
                    format_to!(
                        buf,
                        "{:?} '{}': expected {} but got {}\n",
                        range,
                        ellipsize(text, 15),
                        pretty_type_expectation_with_verbosity(
                            &db,
                            expected.clone(),
                            crate::ty::pretty::TypeVerbosity::Compact
                        ),
                        pretty_type_with_verbosity(
                            &db,
                            *actual,
                            crate::ty::pretty::TypeVerbosity::Compact
                        )
                    );
                },
                _ => {
                    format_to!(buf, "{:?}\n", diagnostic);
                },
            }
        }
    };

    let module_info = db.module_info(file_id);

    let mut defs: Vec<DefinitionWithBodyId> = module_info
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
        .collect();

    defs.sort_by_key(|def| match def {
        DefinitionWithBodyId::Function(it) => {
            let loc = it.lookup(&db);
            loc.source(&db).value.syntax().text_range().start()
        },
        DefinitionWithBodyId::GlobalConstant(it) => {
            let loc = it.lookup(&db);
            loc.source(&db).value.syntax().text_range().start()
        },
        DefinitionWithBodyId::GlobalVariable(it) => {
            let loc = it.lookup(&db);
            loc.source(&db).value.syntax().text_range().start()
        },
        DefinitionWithBodyId::Override(it) => {
            let loc = it.lookup(&db);
            loc.source(&db).value.syntax().text_range().start()
        },
    });
    for def in defs {
        let (body, source_map) = db.body_with_source_map(def);
        let infer = db.infer(def);
        infer_def(infer, body, source_map);
    }

    buf.truncate(buf.trim_end().len());

    buf
}

fn ellipsize(
    mut text: String,
    max_len: usize,
) -> String {
    if text.len() <= max_len {
        return text;
    }
    let ellipsis = "...";
    let e_len = ellipsis.len();
    let mut prefix_len = (max_len - e_len) / 2;
    while !text.is_char_boundary(prefix_len) {
        prefix_len += 1;
    }
    let mut suffix_len = max_len - e_len - prefix_len;
    while !text.is_char_boundary(text.len() - suffix_len) {
        suffix_len += 1;
    }
    text.replace_range(prefix_len..text.len() - suffix_len, ellipsis);
    text
}

fn check_infer(
    ra_fixture: &str,
    expect: Expect,
) {
    let mut actual = infer(ra_fixture);
    actual.push('\n');
    expect.assert_eq(&actual);
}
