mod imports;
mod item_scopes;

use expect_test::{Expect, expect};
use itertools::Itertools as _;
use std::fmt::Write as _;
use test_fixture::WithFixture as _;

use crate::{
    item_scope::ItemScope,
    name_resolution::ModulesMap,
    test_db::TestDatabase,
};

fn render_modules_map_with_items(wa_fixture: &str) -> String {
    let db = TestDatabase::with_files(wa_fixture);
    let package = db.fetch_test_package();
    let modules_map = ModulesMap::of(&db, package);
    let sorted_modules: Vec<_> = modules_map
        .modules
        .iter()
        .sorted_by(|(path_a, _), (path_b, _)| path_a.cmp(path_b))
        .collect();

    let mut buffer = String::new();
    for (module_path, module) in sorted_modules {
        let Some(file_id) = module.file else {
            continue;
        };
        _ = writeln!(buffer, "{module_path}");
        ItemScope::of(&db, file_id).dump(&mut buffer);
    }
    buffer
}

#[expect(clippy::needless_pass_by_value, reason = "matches expect! macro")]
fn check(
    wa_fixture: &str,
    expect: Expect,
) {
    let actual = render_modules_map_with_items(wa_fixture);
    expect.assert_eq(&actual);
}
fn render_item_scope(wa_fixture: &str) -> String {
    let (db, file) = TestDatabase::with_single_file(wa_fixture);
    let package = db.fetch_test_package();
    let package_data = package.data(&db);

    let mut output = String::new();
    ItemScope::of(&db, file).dump(&mut output);
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
