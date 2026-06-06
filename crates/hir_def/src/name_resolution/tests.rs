use expect_test::{Expect, expect};
use test_fixture::WithFixture as _;

use crate::{name_resolution::modules_map_query, test_db::TestDatabase};

fn render_modules_map(wa_fixture: &str) -> String {
    let database = TestDatabase::with_files(wa_fixture);
    let package = database.fetch_test_package();
    modules_map_query(&database, package).dump()
}

#[expect(clippy::needless_pass_by_value, reason = "matches expect! macro")]
fn check(
    wa_fixture: &str,
    expect: Expect,
) {
    let actual = render_modules_map(wa_fixture);
    expect.assert_eq(&actual);
}

#[test]
fn crate_modules_map_smoke_test() {
    check(
        r#"
//- /shaders.wesl
use package::foo::bar::g;

//- /shaders/foo.wesl
fn f() {}

//- /shaders/foo/bar.wesl
fn g() {}
"#,
        expect![[r#"
            package
            package::foo
            package::foo::bar
        "#]],
    );
}
