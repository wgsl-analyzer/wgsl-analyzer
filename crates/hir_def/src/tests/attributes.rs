use crate::{
    expression_store::pretty::{print_function, print_struct},
    name_resolution::modules_map_query,
    test_db::TestDatabase,
};
use expect_test::{Expect, expect};
use test_fixture::WithFixture as _;

use super::super::*;

#[expect(clippy::needless_pass_by_value, reason = "matches expect! macro")]
fn lower_and_print(
    wa_fixture: &str,
    expect: Expect,
) {
    let database = TestDatabase::with_files(wa_fixture);
    let package = database.fetch_test_package();
    let map = modules_map_query(&database, package);
    let dump = map.dump_with_items(&database);
    expect.assert_eq(&dump);
}

#[test]
fn structs() {
    lower_and_print(
        r"
@if(true)
struct Foo { field: u32 }
@elif(false)
struct Foo { field: i32 }
",
        expect![[r#"
            struct S {...}
            struct S(...)
            ;
            struct S;
            struct S<'a, 'b, T, const C: usize = 3, X = ()>
            where
                T: Clone,
                X: Default,
                for<'a, 'c> fn() -> i32: for<'b> Trait::<'a, Item = Boo>
            ;
            #[repr(C)]
            #[repr(pack(1))]
            struct S {...}
        "#]],
    );
}
