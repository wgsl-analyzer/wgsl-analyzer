use expect_test::expect;

use crate::test_util::check;

#[test]
fn format_global_compdec_simple() {
    check(
        "
        fn a() {}
        fn b() {}
        {
        fn c() {}fn d() {}
        }
        fn e() {}
        ",
        expect![[r#"
            fn a() {}
            fn b() {}
            {
                fn c() {}
                fn d() {}
            }
            fn e() {}
        "#]],
    );
}

#[test]
fn format_global_compdec_is_unfolded() {
    check(
        "
        fn a() {}
        fn b() {}
        { fn c() {}fn d() {} }
        fn e() {}
        ",
        expect![[r#"
            fn a() {}
            fn b() {}
            {
                fn c() {}
                fn d() {}
            }
            fn e() {}
        "#]],
    );
}

#[test]
fn format_global_compdec_gets_indent() {
    check(
        "
        fn a() {}
        fn b() {}
        {
            fn c() {}fn d() {}
        }
        fn e() {}
        ",
        expect![[r#"
            fn a() {}
            fn b() {}
            {
                fn c() {}
                fn d() {}
            }
            fn e() {}
        "#]],
    );
}

#[test]
fn format_global_compdec_respects_spacing_around() {
    check(
        "
        fn a() {}
        { fn b() {} }
        fn c() {}

        { fn d() {} }
        fn e() {}

        { fn f() {} }

        fn g() {}
        ",
        expect![[r#"
            fn a() {}
            {
                fn b() {}
            }
            fn c() {}

            {
                fn d() {}
            }
            fn e() {}

            {
                fn f() {}
            }

            fn g() {}
        "#]],
    );
}

#[test]
fn format_global_compdec_work_with_attributes() {
    check(
        "
        fn a() {}
        @if(true)
        { fn b() {} }
        fn c() {}

        @else
        { fn b() {} }
        ",
        expect![[r#"
            fn a() {}
            @if(true) {
                fn b() {}
            }
            fn c() {}

            @else {
                fn b() {}
            }
        "#]],
    );
}

#[test]
fn format_global_compdec_removes_extra_lines_at_start_and_end() {
    check(
        "
        {




        fn a() {}
        fn b() {}

        fn c() {}


        }
        ",
        expect![[r#"
            {
                fn a() {}
                fn b() {}

                fn c() {}
            }
        "#]],
    );
}

#[test]
fn format_global_compdec_leaves_line_comments_alone() {
    check(
        "
        { // Same Line
        fn a () {}
        }

        {
        // Next
        fn a () {}
        }

        {
        } // Same Line

        {
        }
        // Next Line
        ",
        expect![[r#"
            { // Same Line
                fn a() {}
            }

            {
                // Next
                fn a() {}
            }

            {
            } // Same Line

            {
            }
            // Next Line
        "#]],
    );
}

#[test]
fn format_global_compdec_leaves_block_comments_alone() {
    check(
        "
        { /* Same Line */
        fn a () {}
        }

        {
        /* Next */
        fn a () {}
        }

        {
        } /* Same Line */

        {
        }
        /* Next Line */
        ",
        expect![[r#"
            { /* Same Line */
                fn a() {}
            }

            {
                /* Next */
                fn a() {}
            }

            {
            } /* Same Line */

            {
            }
            /* Next Line */
        "#]],
    );
}

#[test]
fn format_global_compdec_does_not_beautify_nested_compdecs() {
    // For now this is expected behavior - however this is simply because we think noone will
    // want these nested. If need arises - feel free to change this test.
    check(
        "
        {{{
        fn a() {}
        }}}
        ",
        expect![[r#"
            {
                {
                    {
                        fn a() {}
                    }
                }
            }
        "#]],
    );
}
