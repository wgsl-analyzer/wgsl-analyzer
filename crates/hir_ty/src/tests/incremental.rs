use base_db::{EditionedFileId, SourceDatabase as _};
use expect_test::{Expect, expect};
use hir_def::database::{DefDatabase as _, DefinitionWithBodyId, ModuleDefinitionId};
use test_fixture::WithFixture as _;

use crate::{
    database::HirDatabase as _, infer::InferenceResult, test_db::TestDatabase,
    tests::module_definitions,
};

#[test]
fn typing_whitespace_inside_a_function_should_not_invalidate_types() {
    let (mut database, position) = TestDatabase::with_position(
        "
//- /package.wesl
fn foo() {
    let a = $01 + 1;
}
    ",
    );
    let file_id = EditionedFileId::from_file(&database, position.file_id);
    execute_assert_events(
        &database,
        || {
            let module_info = database.item_tree(file_id);
            let definitions = module_definitions(&database, file_id, &module_info);
            for definition in definitions {
                if let ModuleDefinitionId::Function(id) = definition {
                    let inference_results =
                        InferenceResult::of(&database, DefinitionWithBodyId::Function(id));
                    assert!(inference_results.diagnostics().is_empty());
                }
            }
        },
        &[("InferenceResult::of", 1)],
        expect_test::expect![[r#"
            [
                "item_tree_shim",
                "parse",
                "ast_id_map_shim",
                "InferenceResult::of_",
                "ItemScope::of_",
                "body_shim",
                "body_with_source_map_shim",
                "function_data_shim",
                "expression_scopes_shim",
            ]
        "#]],
    );

    let new_text = "
fn foo() {
    let a = 1
    +
    1;
}";

    database.set_file_text(position.file_id, new_text);

    execute_assert_events(
        &database,
        || {
            let module_info = database.item_tree(file_id);
            let definitions = module_definitions(&database, file_id, &module_info);
            for definition in definitions {
                if let ModuleDefinitionId::Function(id) = definition {
                    let inference_results =
                        InferenceResult::of(&database, DefinitionWithBodyId::Function(id));
                    assert!(inference_results.diagnostics().is_empty());
                }
            }
        },
        &[("InferenceResult::of", 0)],
        expect_test::expect![[r#"
            [
                "parse",
                "item_tree_shim",
                "ast_id_map_shim",
                "body_with_source_map_shim",
                "body_shim",
                "function_data_shim",
            ]
        "#]],
    );
}

#[test]
#[expect(
    clippy::should_panic_without_expect,
    reason = "we need to improve our incrementality, see: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/1178"
)]
#[should_panic]
fn typing_inside_a_function_should_not_invalidate_types_in_another() {
    let (mut database, position) = TestDatabase::with_position(
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
    let file_id = EditionedFileId::from_file(&database, position.file_id);
    execute_assert_events(
        &database,
        || {
            let module_info = database.item_tree(file_id);
            let definitions = module_definitions(&database, file_id, &module_info);
            for definition in definitions {
                if let ModuleDefinitionId::Function(id) = definition {
                    let inference_results =
                        InferenceResult::of(&database, DefinitionWithBodyId::Function(id));
                    assert!(inference_results.diagnostics().is_empty());
                }
            }
        },
        &[("InferenceResult::of", 3)],
        expect_test::expect![[r#"
            [
                "item_tree_shim",
                "parse",
                "ast_id_map_shim",
                "InferenceResult::of_",
                "ItemScope::of_",
                "body_shim",
                "body_with_source_map_shim",
                "function_data_shim",
                "expression_scopes_shim",
                "InferenceResult::of_",
                "body_shim",
                "body_with_source_map_shim",
                "function_data_shim",
                "expression_scopes_shim",
                "InferenceResult::of_",
                "body_shim",
                "body_with_source_map_shim",
                "function_data_shim",
                "expression_scopes_shim",
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

    database.set_file_text(position.file_id, new_text);

    execute_assert_events(
        &database,
        || {
            let module_info = database.item_tree(file_id);
            let definitions = module_definitions(&database, file_id, &module_info);
            for definition in definitions {
                if let ModuleDefinitionId::Function(id) = definition {
                    let inference_results =
                        InferenceResult::of(&database, DefinitionWithBodyId::Function(id));
                    assert!(inference_results.diagnostics().is_empty());
                }
            }
        },
        &[("InferenceResult::of", 0)],
        expect_test::expect![[r#"
            [
                "parse",
                "item_tree_shim",
                "ast_id_map_shim",
                "body_with_source_map_shim",
                "body_shim",
                "function_data_shim",
                "InferenceResult::of_",
                "body_with_source_map_shim",
                "body_shim",
                "function_data_shim",
                "InferenceResult::of_",
                "body_with_source_map_shim",
                "body_shim",
                "function_data_shim",
                "InferenceResult::of_",
            ]
        "#]],
    );
}

/// Executes a function and checks if the most important events happened exactly n times.
/// Also checks the full list of events, which may change as the implementation changes.
#[expect(clippy::needless_pass_by_value, reason = "matches expect! macro")]
fn execute_assert_events<Callback>(
    database: &TestDatabase,
    callback: Callback,
    required: &[(&str, usize)],
    expect: Expect,
) where
    Callback: FnOnce(),
{
    let (executed, events) = database.log_executed(callback);
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
