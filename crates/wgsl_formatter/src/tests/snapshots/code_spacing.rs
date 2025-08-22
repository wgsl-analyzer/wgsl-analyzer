use expect_test::expect;

use crate::test_util::{check, check_tabs};

#[test]
fn spacing_between_fn_headers_1() {
    check(
        "
        fn a() {}
        fn b() {}

        fn c() {}fn d() {}


        fn e() {}
        ",
        expect![["
            fn a() {}
            fn b() {}

            fn c() {}
            fn d() {}

            fn e() {}
            "]],
    );
}

#[test]
fn spacing_between_empty_struct_defs() {
    check(
        "
        struct A {}
        struct B {}

        struct C {}


        struct D {}
        ",
        expect![["
            struct A {}
            struct B {}

            struct C {}

            struct D {}
            "]],
    );
}

#[test]
fn spacing_between_module_variables() {
    check(
        "
        var<workgroup> a: u32;
        var<workgroup> b: u32;

        var<workgroup> c: u32;


        var<workgroup> d: u32;
        ",
        expect![["
            var<workgroup> a: u32;
            var<workgroup> b: u32;

            var<workgroup> c: u32;

            var<workgroup> d: u32;
            "]],
    );
}

#[test]
fn spacing_between_module_constants() {
    check(
        "
        const a: u32 = 0u;
        const b: u32 = 0u;

        const c: u32 = 0u;


        const d: u32 = 0u;
        ",
        expect![["
            const a: u32 = 0u;
            const b: u32 = 0u;

            const c: u32 = 0u;

            const d: u32 = 0u;
            "]],
    );
}

#[test]
fn spacing_between_module_overrides() {
    check(
        "
        override a: u32 = 0u;
        override b: u32 = 0u;

        override c: u32 = 0u;


        override d: u32 = 0u;
        ",
        expect![["
            override a: u32 = 0u;
            override b: u32 = 0u;

            override c: u32 = 0u;

            override d: u32 = 0u;
            "]],
    );
}

#[test]
fn spacing_between_type_aliases() {
    check(
        "
        alias A = array<i32, 5>;
        alias B = array<i32, 5>;

        alias C = array<i32, 5>;


        alias D = array<i32, 5>;
        ",
        expect![["
            alias A = array<i32, 5>;
            alias B = array<i32, 5>;

            alias C = array<i32, 5>;

            alias D = array<i32, 5>;
            "]],
    );
}

#[test]
fn spacing_between_mixed_module_items_1() {
    //https://discord.com/channels/1289346613185351722/1341941812675481680/1407083514322616342
    // - "Rustfmt doesn't enforce blank lines between module items, so we won't either"
    check(
        "
        alias A = array<i32, 5>;
        alias B = array<i32, 5>;
        fn c() {}
        fn d() {}
        struct E {
            f: u32;
        }
        fn f() {}
        type G = i32;
        ",
        expect![["
            alias A = array<i32, 5>;
            alias B = array<i32, 5>;
            fn c() {}
            fn d() {}
            struct E {
                f: u32;
            }
            fn f() {}
            type G = i32;
            "]],
    );
}

#[test]
fn spacing_between_nonempty_fn_decls() {
    //https://discord.com/channels/1289346613185351722/1341941812675481680/1407083514322616342
    // - "Rustfmt doesn't enforce blank lines between module items, so we won't either"
    check(
        "
        fn a() {
            callOtherFn();
        }
        fn b() {
            callOtherFn();
        }

        fn c() {
            callOtherFn();
        }


        fn d() {
            callOtherFn();
        }
        ",
        expect![["
            fn a() {
                callOtherFn();
            }
            fn b() {
                callOtherFn();
            }

            fn c() {
                callOtherFn();
            }

            fn d() {
                callOtherFn();
            }
            "]],
    );
}

#[test]
fn spacing_between_nonempty_struct_defs() {
    //https://discord.com/channels/1289346613185351722/1341941812675481680/1407083514322616342
    // - "Rustfmt doesn't enforce blank lines between module items, so we won't either"
    check(
        "
        struct A {
            a: i32,
        }
        struct B {
            b: i32,
        }

        struct C {
            c: i32,
        }


        struct D {
            d: i32,
        }
        ",
        expect![["
            struct A {
                a: i32,
            }
            struct B {
                b: i32,
            }

            struct C {
                c: i32,
            }

            struct D {
                d: i32,
            }
            "]],
    );
}

#[test]
fn no_newlines_at_at_start_of_file() {
    //Do not use expect! here, because it trims newlines and tabs and as such obscures the test case.
    check("\n\n\nfn a() {}\n", "fn a() {}\n");
}

#[test]
fn one_newline_at_end_of_file_when_missing() {
    //Do not use expect! here, because it trims newlines and tabs and as such obscures the test case.
    check("fn a() {}", "fn a() {}\n");
}

#[test]
fn one_newline_at_end_of_file_when_too_much() {
    //Do not use expect! here, because it trims newlines and tabs and as such obscures the test case.
    check("fn a() {}\n\n", "fn a() {}\n");
}
