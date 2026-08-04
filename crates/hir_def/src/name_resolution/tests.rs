mod imports;
mod item_scopes;

use base_db::EditionedFileId;
use expect_test::{Expect, expect};
use itertools::Itertools as _;
use std::fmt::Write as _;
use test_fixture::WithFixture as _;

use crate::{
    item_scope::ItemScope,
    mod_path::{AbsoluteModPath, ModPath},
    test_db::TestDatabase,
};

fn render_modules_map_with_items(wa_fixture: &str) -> String {
    let database = TestDatabase::with_files(wa_fixture);
    let package = database.fetch_test_package();
    let source_root = package.data(&database).source_root(&database);
    let modules: Vec<_> = source_root
        .iter()
        .filter_map(|file_id| {
            let (name, extension) = source_root.path_for_file(file_id)?.name_and_extension()?;
            let file_id = EditionedFileId::try_with_extension(&database, file_id, extension?)?;
            let mod_path = AbsoluteModPath::for_file(&database, package, file_id)?;
            Some(ModuleData { file_id, mod_path })
        })
        .sorted_by(|module_a, module_b| module_a.mod_path.cmp(&module_b.mod_path))
        .collect();

    let mut buffer = String::new();
    for module in modules {
        _ = writeln!(buffer, "{}", &ModPath::from(module.mod_path).to_string());
        ItemScope::of(&database, module.file_id).dump(&mut buffer);
    }
    buffer
}

struct ModuleData {
    file_id: EditionedFileId,
    mod_path: AbsoluteModPath,
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
    let (database, file) = TestDatabase::with_single_file(wa_fixture);
    let package = database.fetch_test_package();
    let package_data = package.data(&database);

    let mut output = String::new();
    ItemScope::of(&database, file).dump(&mut output);
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
