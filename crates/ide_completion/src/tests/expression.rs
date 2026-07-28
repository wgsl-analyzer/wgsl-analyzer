//! Completion tests for expressions.
use expect_test::{Expect, expect};

use crate::{
    CompletionConfig,
    tests::{BASE_ITEMS_FIXTURE, TEST_CONFIG, check, completion_list_with_config},
};

fn check_with_config(
    config: &CompletionConfig,
    wa_fixture: &str,
    expect: &Expect,
) {
    let actual = completion_list_with_config(
        config,
        &format!("{BASE_ITEMS_FIXTURE}{wa_fixture}"),
        true,
        None,
    );
    expect.assert_eq(&actual);
}
