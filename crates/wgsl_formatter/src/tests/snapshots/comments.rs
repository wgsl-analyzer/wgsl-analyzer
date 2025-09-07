use expect_test::expect;

use crate::{
    FormattingOptions,
    test_util::{check, check_with_options},
};

#[test]
fn format_fn_header_inline_comments_1() {
    check_with_options(
        "/*000*/ fn /*aaa*/ main /*bbb*/(/*ccc*/ a /*ddd*/ : /*eee*/ b /*fff*/ ) /*ggg*/  -> /*hhh*/ f32 /*iii*/ {} /*jjj*/",
        &expect![["
            /*000*/
            fn /*aaa*/ main /*bbb*/ (/*ccc*/ a: /*ddd*/ /*eee*/ b /*fff*/) /*ggg*/ -> /*hhh*/ f32 /*iii*/ {}
            /*jjj*/
        "]],
        &FormattingOptions {
            width: 10000,
            ..Default::default()
        },
    );
}

#[test]
fn format_fn_multiline_header_inline_comments_1() {
    check_with_options(
        "/*000*/ fn /*aaa*/ main /*bbb*/(/*ccc*/ a /*ddd*/ : /*eee*/ b /*fff*/ ) /*ggg*/  -> /*hhh*/ f32 /*iii*/ {} /*jjj*/",
        &expect![["
            /*000*/
            fn /*aaa*/ main /*bbb*/ (
                /*ccc*/ a: /*ddd*/ /*eee*/ b, /*fff*/
            ) /*ggg*/ -> /*hhh*/ f32 /*iii*/ {}
            /*jjj*/
        "]],
        &FormattingOptions {
            width: 50,
            ..Default::default()
        },
    );
}

#[test]
fn format_fn_header_line_comments_1() {
    check(
        "
        // 000
        fn
        // aaa
        main
        // bbb
        (
        // ccc
        a
        // ddd
        :
        // eee
        b
        // fff
        )
        // ggg
        ->
        // hhh
        f32
        // iii
        {} // jjj
        // kkk
        ",
        expect![["
            // 000
            fn // aaa
            main // bbb
            (
                // ccc
                a: // ddd
                // eee
                b, // fff
            ) // ggg
            -> // hhh
            f32 // iii
            {}
            // jjj
            // kkk
        "]],
    );
}

#[test]
fn format_fn_header_line_comments_2() {
    check(
        "fn main(
            a: b, // Comment
            c: d // Comment
        ) -> f32 {}",
        expect![["
            fn main(
                a: b, // Comment
                c: d, // Comment
            ) -> f32 {}
        "]],
    );
}

#[test]
fn format_struct_block_comments_1() {
    check(
        "
        /* 000 */
        struct
        /* aaa */
        Abc
        /* bbb */
        {
        /* ccc */
        a
        /* ddd */
        :
        /* eee */
        b
        /* fff */
        ,
        /* ggg */
        c
        /* hhh */
        :
        /* iii */
        d
        /* jjj */
        }
        /* ggg */
        ",
        expect![[r#"
            /* 000 */
            struct /* aaa */ Abc /* bbb */ {
                a: /* ddd */ /* eee */ b, /* fff */ /* ggg */
                c: /* hhh */ /* iii */ d, /* jjj */
            }
            /* ggg */
        "#]],
    );
}

#[test]
fn format_struct_inline_comments_1() {
    check(
        "/* 000 */struct/* aaa */Abc/* bbb */{/* ccc */a/* ddd */:/* eee */b/* fff */,/* ggg */c/* hhh */:/* iii */d/* jjj */}/* ggg */",
        expect![[r#"
            /* 000 */
            struct /* aaa */ Abc /* bbb */ {
                a: /* ddd */ /* eee */ b, /* fff */ /* ggg */
                c: /* hhh */ /* iii */ d, /* jjj */
            }
            /* ggg */
        "#]],
    );
}
