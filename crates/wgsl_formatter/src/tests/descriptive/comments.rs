use expect_test::expect;
use parser::Edition;

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
        Edition::LATEST,
    );
}

#[test]
fn format_fn_multiline_header_inline_comments_1() {
    check_with_options(
        "/*000*/ fn /*aaa*/ main /*bbb*/(/*ccc*/ a /*ddd*/ : /*eee*/ b /*fff*/ ) /*ggg*/  -> /*hhh*/ f32 /*iii*/ {} /*jjj*/",
        &expect![[r#"
            /*000*/
            fn /*aaa*/ main /*bbb*/ (
                /*ccc*/
                a: /*ddd*/ /*eee*/ b, /*fff*/
            ) /*ggg*/ -> /*hhh*/ f32 /*iii*/ {} /*jjj*/
        "#]],
        &FormattingOptions {
            width: 50,
            ..Default::default()
        },
        Edition::LATEST,
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
                b,
                // fff
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
