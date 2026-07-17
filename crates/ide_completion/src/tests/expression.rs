
//! Completion tests for expressions.
use expect_test::{Expect, expect};

use crate::{
    CompletionConfig,
    tests::{
        BASE_ITEMS_FIXTURE, TEST_CONFIG, check, check_with_base_items,
        completion_list_with_config,
    },
};

fn check_with_config(
    config: CompletionConfig,
    wa_fixture: &str,
    expect: Expect,
) {
    let actual = completion_list_with_config(
        config,
        &format!("{BASE_ITEMS_FIXTURE}{wa_fixture}"),
        true,
        None,
    );
    expect.assert_eq(&actual)
}

#[test]
fn complete_literal_struct_with_a_private_field() {
    // `FooDesc.bar` is private, the completion should not be triggered.
    check_with_base_items(
        r#"
mod _69latrick {
    pub struct FooDesc { pub six: bool, pub neuf: Vec<String>, bar: bool }
    pub fn create_foo(foo_desc: &FooDesc) -> () { () }
}

fn baz() {
    use _69latrick::*;

    let foo = create_foo(&$0);
}
            "#,
        // This should not contain `FooDesc {…}`.
        expect![[r#"
            ct CONST                   Unit
            en Enum                    Enum
            fn baz()                   fn()
            fn create_foo(…)   fn(&FooDesc)
            fn function()              fn()
            ma makro!(…) macro_rules! makro
            md _69latrick
            md module
            sc STATIC                  Unit
            st FooDesc              FooDesc
            st Record                Record
            st Tuple                  Tuple
            st Unit                    Unit
            un Union                  Union
            ev TupleV(…)        TupleV(u32)
            bt u32                      u32
            kw const
            kw crate::
            kw false
            kw for
            kw if
            kw if let
            kw loop
            kw match
            kw mut
            kw raw
            kw return
            kw self::
            kw true
            kw unsafe
            kw while
            kw while let
        "#]],
    )
}
