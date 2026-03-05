use expect_test::expect;

use crate::{
    FormattingOptions,
    test_util::{check, check_with_options},
};

// TODO (MonaMayrhofer move some of these tests to check_comments())

#[test]
fn format_fn_header_inline_comments_1() {
    check_with_options(
        "/*000*/ fn /*aaa*/ main /*bbb*/(/*ccc*/ a /*ddd*/ : /*eee*/ b /*fff*/ ) /*ggg*/  -> /*hhh*/ f32 /*iii*/ {} /*jjj*/",
        &expect![[r#"
            /*000*/
            fn /*aaa*/ main /*bbb*/ (/*ccc*/ a: /*ddd*/ /*eee*/ b /*fff*/) /*ggg*/ -> /*hhh*/ f32 /*iii*/ {} /*jjj*/
        "#]],
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
        &expect![[r#"
            /*000*/
            fn /*aaa*/ main /*bbb*/ (
                /*ccc*/ a: /*ddd*/ /*eee*/ b, /*fff*/
            ) /*ggg*/ -> /*hhh*/ f32 /*iii*/ {} /*jjj*/
        "#]],
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
        expect![[r#"
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
            {} // jjj
            // kkk
        "#]],
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
fn format_struct_line_comments_1() {
    check(
        "
        // 000
        struct
        // aaa
        Abc
        // bbb
        {
        // ccc
        a
        // ddd
        :
        // eee
        b
        // fff
        ,
        // ggg
        c
        // hhh
        :
        // iii
        d
        // jjj
        }
        // 111
        ",
        expect![["
            // 000
            struct // aaa
            Abc // bbb
            {
                // ccc
                a: // ddd
                // eee
                b, // fff
                // ggg
                c: // hhh
                // iii
                d, // jjj
            }
            // 111
        "]],
    );
}

#[test]
fn format_struct_with_attributes_line_comments_1() {
    check(
        "
        // 000
        struct
        // aaa
        Abc
        // bbb
        {
        // pre_attr_1
        @attribute(0)
        // pre_attr_2
        @attribute(0)
        // ccc
        a
        // ddd
        :
        // eee
        b
        // fff
        ,
        // ggg
        c
        // hhh
        :
        // iii
        d
        // jjj
        }
        // 111
        ",
        expect![["
            // 000
            struct // aaa
            Abc // bbb
            {
                // pre_attr_1
                @attribute(0) // pre_attr_2
                @attribute(0) // ccc
                a: // ddd
                // eee
                b, // fff
                // ggg
                c: // hhh
                // iii
                d, // jjj
            }
            // 111
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
        expect![["
            /* 000 */
            struct /* aaa */ Abc /* bbb */ {
                /* ccc */
                a: /* ddd */ /* eee */ b, /* fff */ /* ggg */
                c: /* hhh */ /* iii */ d, /* jjj */
            }
            /* ggg */
        "]],
    );
}

#[test]
fn format_struct_inline_comments_1() {
    check(
        "/* 000 */struct/* aaa */Abc/* bbb */{/* ccc */a/* ddd */:/* eee */b/* fff */,/* ggg */c/* hhh */:/* iii */d/* jjj */}/* ggg */",
        expect![["
            /* 000 */
            struct /* aaa */ Abc /* bbb */ {
                /* ccc */
                a: /* ddd */ /* eee */ b, /* fff */ /* ggg */
                c: /* hhh */ /* iii */ d, /* jjj */
            }
            /* ggg */
        "]],
    );
}

#[test]
fn format_struct_with_attribute_inline_comments_1() {
    check(
        "/* 000 */struct/* aaa */Abc/* bbb */{/* pre_attr */@attribute(1)/* amidst_attr */@attribute(2)/* ccc */a/* ddd */:/* eee */b/* fff */,/* ggg */c/* hhh */:/* iii */d/* jjj */}/* ggg */",
        expect![["
            /* 000 */
            struct /* aaa */ Abc /* bbb */ {
                /* pre_attr */
                @attribute(1) /* amidst_attr */
                @attribute(2) /* ccc */
                a: /* ddd */ /* eee */ b, /* fff */ /* ggg */
                c: /* hhh */ /* iii */ d, /* jjj */
            }
            /* ggg */
        "]],
    );
}

#[test]
fn format_struct_opening_comments_1() {
    check(
        "
        struct A {
        // This comment should stick to the member
        a: i32
        }
        ",
        expect![["
            struct A {
                // This comment should stick to the member
                a: i32,
            }
        "]],
    );
}

// Following rustfmt, even if the comment is on the same line as the struct
// we assume it belongs to the member. If the user wanted to comment on the
// struct, they should put the comment in front of the struct.
#[test]
fn format_struct_opening_comments_2() {
    check(
        "
        struct A { // This comment should stick to the member
        a: i32
        }
        ",
        expect![["
            struct A {
                // This comment should stick to the member
                a: i32,
            }
        "]],
    );
}

#[test]
fn format_comment_indent_1() {
    check(
        "
        fn main() {

        let a = //a
        /* A */
        // b
        1;


        for(/* A */ var i = 0;
        // Force Multiline
        ; a++) {}
        }
        ",
        expect![[r#"
            fn main() {
                let a = //a
                    /* A */ // b
                    1;

                for(
                    /* A */
                    var i = 0; // Force Multiline
                    ;
                    a++
                ) {}
            }
        "#]],
    );
}

#[test]
fn comment_after_statement_should_stay_on_same_line() {
    check(
        "
        fn main() {
        let a = 1; // This is one
        }
        ",
        expect![[r#"
            fn main() {
                let a = 1; // This is one
            }
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
