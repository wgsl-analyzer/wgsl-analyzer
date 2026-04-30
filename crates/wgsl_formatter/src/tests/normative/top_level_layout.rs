use expect_test::expect;

use crate::test_util::check;

// TODO(MonaMayrhofer) Rename this file to top-level items

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
fn spacing_between_struct_defs() {
    check(
        "
        struct A {
          a: i32
        }
        struct B {
          b: i32
        }

        struct C {
          c: i32
        }




        struct D {
          d: i32
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
            f: u32,
        }
        fn f() {}
        ",
        expect![["
            alias A = array<i32, 5>;
            alias B = array<i32, 5>;
            fn c() {}
            fn d() {}
            struct E {
                f: u32,
            }
            fn f() {}
            "]],
    );
}

#[test]
fn spacing_between_nonempty_fn_decls() {
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

#[test]
fn format_line_comments_around_nonempty_function_declaration() {
    check(
        "
        // Alone

        // Line Before
        fn a() { let a = 1; } // Should be broken into new line
        // Line After

        // Alone
        ",
        expect![[r#"
            // Alone

            // Line Before
            fn a() {
                let a = 1;
            }
            // Should be broken into new line
            // Line After

            // Alone
        "#]],
    );
}

#[test]
fn format_block_comments_around_nonempty_function_declaration() {
    check(
        "
        /* Alone */

        /* Line Before */
        fn a() { let a = 1; } /* Should be broken into new line */
        /* Line After */

        /* Alone */
        ",
        expect![[r#"
            /* Alone */

            /* Line Before */
            fn a() {
                let a = 1;
            }
            /* Should be broken into new line */
            /* Line After */

            /* Alone */
        "#]],
    );
}

#[test]
fn format_block_comments_around_nonempty_struct_definition() {
    check(
        "
        /* Alone */

        /* Line Before */
        struct A { item: u32 } /* Should be broken into new line */
        /* Line After */

        /* Alone */
        ",
        expect![[r#"
            /* Alone */

            /* Line Before */
            struct A {
                item: u32,
            }
            /* Should be broken into new line */
            /* Line After */

            /* Alone */
        "#]],
    );
}

#[test]
fn format_line_comments_around_empty_function_declaration() {
    check(
        "
        // Alone

        // Line Before
        fn a() {} // Can be kept on the same line
        // Line After

        // Alone
        ",
        expect![[r#"
            // Alone

            // Line Before
            fn a() {} // Can be kept on the same line
            // Line After

            // Alone
        "#]],
    );
}

#[test]
fn format_line_comments_around_global_declaration() {
    check(
        "
        // Alone

        // Line Before
        const a: i32 = 1; // Should be kept on the same line
        // Line After

        // Alone
        ",
        expect![[r#"
            // Alone

            // Line Before
            const a: i32 = 1; // Should be kept on the same line
            // Line After

            // Alone
        "#]],
    );
}

#[test]
fn comment_after_toplevel_declaration_should_stay_on_same_line() {
    check(
        "
        override a = 1; // This is one
        ",
        expect![[r#"
            override a = 1; // This is one
        "#]],
    );
}
