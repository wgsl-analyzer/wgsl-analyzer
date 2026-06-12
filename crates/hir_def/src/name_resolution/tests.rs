mod imports;
mod item_scopes;

use base_db::EditionedFileId;
use expect_test::{Expect, expect};
use test_fixture::WithFixture as _;

use crate::{item_scope::ItemScope, name_resolution::modules_map_query, test_db::TestDatabase};

fn render_modules_map_with_items(wa_fixture: &str) -> String {
    let database = TestDatabase::with_files(wa_fixture);
    let package = database.fetch_test_package();
    modules_map_query(&database, package).dump_with_items(&database)
}

#[expect(clippy::needless_pass_by_value, reason = "matches expect! macro")]
fn check(
    wa_fixture: &str,
    expect: Expect,
) {
    let actual = render_modules_map_with_items(wa_fixture);
    expect.assert_eq(&actual);
}

fn render_modules_map(wa_fixture: &str) -> String {
    let database = TestDatabase::with_files(wa_fixture);
    let package = database.fetch_test_package();
    modules_map_query(&database, package).dump()
}

#[expect(clippy::needless_pass_by_value, reason = "matches expect! macro")]
fn check_modules(
    wa_fixture: &str,
    expect: Expect,
) {
    let actual = render_modules_map(wa_fixture);
    expect.assert_eq(&actual);
}

fn render_item_scope(wa_fixture: &str) -> String {
    let database = TestDatabase::with_files(wa_fixture);
    let package = database.fetch_test_package();
    let package_data = package.data(&database);

    let mut output = String::new();
    ItemScope::of(
        &database,
        EditionedFileId::new(&database, package_data.root_file_id, package_data.edition),
    )
    .dump(&mut output);
    output
}

#[expect(clippy::needless_pass_by_value, reason = "matches expect! macro")]
fn check_item_scope(
    wa_fixture: &str,
    expect: Expect,
) {
    let actual = render_item_scope(wa_fixture);
    expect.assert_eq(&actual);
}
