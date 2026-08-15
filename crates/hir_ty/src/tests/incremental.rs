use base_db::{EditionedFileId, SourceDatabase as _};
use expect_test::{Expect, expect};
use hir_def::{
    db::{DefinitionWithBodyId, ModuleDefinitionId},
    item_tree::ItemTree,
};
use test_fixture::WithFixture as _;

use crate::{infer::InferenceResult, test_db::TestDatabase, tests::module_definitions};

#[test]
fn typing_whitespace_inside_a_function_should_not_invalidate_types() {
    let (mut db, position) = TestDatabase::with_position(
        "
//- /package.wesl
fn foo() {
    let a = $01 + 1;
}
    ",
    );
    let file_id = EditionedFileId::from_file(&db, position.file_id);
    execute_assert_events(
        &db,
        || {
            let module_info = ItemTree::of(&db, file_id);
            let definitions = module_definitions(&db, file_id, module_info);
            for definition in definitions {
                if let ModuleDefinitionId::Function(id) = definition {
                    let inference_results =
                        InferenceResult::of(&db, DefinitionWithBodyId::Function(id));
                    assert!(inference_results.diagnostics().is_empty());
                }
            }
        },
        &[("InferenceResult::of", 1)],
        expect_test::expect![[r#"
            [
                "ItemTree::of_",
                "EditionedFileId::parse_",
                "AstIdMap::of_",
                "InferenceResult::of_",
                "ItemScope::of_",
                "Body::of_",
                "Body::with_source_map_",
                "FunctionSignature::of_",
                "FunctionSignature::with_source_map_",
                "ExprScopes::of_",
            ]
        "#]],
    );

    let new_text = "
fn foo() {
    let a = 1
    +
    1;
}";

    db.set_file_text(position.file_id, new_text);

    execute_assert_events(
        &db,
        || {
            let module_info = ItemTree::of(&db, file_id);
            let definitions = module_definitions(&db, file_id, module_info);
            for definition in definitions {
                if let ModuleDefinitionId::Function(id) = definition {
                    let inference_results =
                        InferenceResult::of(&db, DefinitionWithBodyId::Function(id));
                    assert!(inference_results.diagnostics().is_empty());
                }
            }
        },
        &[("InferenceResult::of", 0)],
        expect_test::expect![[r#"
            [
                "EditionedFileId::parse_",
                "ItemTree::of_",
                "AstIdMap::of_",
                "Body::with_source_map_",
                "Body::of_",
                "FunctionSignature::with_source_map_",
            ]
        "#]],
    );
}

#[test]
fn typing_inside_a_function_should_not_invalidate_types_in_another() {
    let (mut db, position) = TestDatabase::with_position(
        "
//- /package.wesl
fn foo() -> f32 {
    return 1.0 + 2.0;
}
fn bar() -> i32 {
    return $01 + 1;
}
fn baz() -> i32 {
    return 1 + 1;
}",
    );
    let file_id = EditionedFileId::from_file(&db, position.file_id);
    execute_assert_events(
        &db,
        || {
            let module_info = ItemTree::of(&db, file_id);
            let definitions = module_definitions(&db, file_id, module_info);
            for definition in definitions {
                if let ModuleDefinitionId::Function(id) = definition {
                    let inference_results =
                        InferenceResult::of(&db, DefinitionWithBodyId::Function(id));
                    assert!(inference_results.diagnostics().is_empty());
                }
            }
        },
        &[("InferenceResult::of", 3)],
        expect_test::expect![[r#"
            [
                "ItemTree::of_",
                "EditionedFileId::parse_",
                "AstIdMap::of_",
                "InferenceResult::of_",
                "ItemScope::of_",
                "Body::of_",
                "Body::with_source_map_",
                "FunctionSignature::of_",
                "FunctionSignature::with_source_map_",
                "ExprScopes::of_",
                "InferenceResult::of_",
                "Body::of_",
                "Body::with_source_map_",
                "FunctionSignature::of_",
                "FunctionSignature::with_source_map_",
                "ExprScopes::of_",
                "InferenceResult::of_",
                "Body::of_",
                "Body::with_source_map_",
                "FunctionSignature::of_",
                "FunctionSignature::with_source_map_",
                "ExprScopes::of_",
            ]
        "#]],
    );

    let new_text = "
fn foo() -> f32 {
    return 1.0 + 2.0;
}
fn bar() -> i32 {
    return 1 + 1;
}
fn baz() -> i32 {
    return 1 + 1;
}";

    db.set_file_text(position.file_id, new_text);

    execute_assert_events(
        &db,
        || {
            let module_info = ItemTree::of(&db, file_id);
            let definitions = module_definitions(&db, file_id, module_info);
            for definition in definitions {
                if let ModuleDefinitionId::Function(id) = definition {
                    let inference_results =
                        InferenceResult::of(&db, DefinitionWithBodyId::Function(id));
                    assert!(inference_results.diagnostics().is_empty());
                }
            }
        },
        &[("InferenceResult::of", 0)],
        expect_test::expect![[r#"
            [
                "EditionedFileId::parse_",
                "ItemTree::of_",
                "AstIdMap::of_",
                "Body::with_source_map_",
                "Body::of_",
                "FunctionSignature::with_source_map_",
                "FunctionSignature::of_",
                "Body::with_source_map_",
                "Body::of_",
                "FunctionSignature::with_source_map_",
                "FunctionSignature::of_",
                "Body::with_source_map_",
                "Body::of_",
                "FunctionSignature::with_source_map_",
                "FunctionSignature::of_",
            ]
        "#]],
    );
}

/// Executes a function and checks if the most important events happened exactly n times.
/// Also checks the full list of events, which may change as the implementation changes.
#[expect(clippy::needless_pass_by_value, reason = "matches expect! macro")]
fn execute_assert_events<Callback>(
    db: &TestDatabase,
    callback: Callback,
    required: &[(&str, usize)],
    expect: Expect,
) where
    Callback: FnOnce(),
{
    let (executed, events) = db.log_executed(callback);
    expect.assert_debug_eq(&executed);
    for (event, count) in required {
        let actual_count = executed.iter().filter(|it| it.contains(event)).count();
        assert_eq!(
            actual_count,
            *count,
            "Expected {event} to be executed {count} times, but only got {actual_count}:\n \
             Executed: {executed:#?}\n \
             Event log: {events:#?}",
            events = events
                .iter()
                .filter(|event| !matches!(event.kind, salsa::EventKind::WillCheckCancellation))
                .map(|event| { format!("{:?}", event.kind) })
                .collect::<Vec<_>>(),
        );
    }
}
