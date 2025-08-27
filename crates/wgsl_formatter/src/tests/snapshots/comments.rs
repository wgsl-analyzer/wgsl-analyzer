use expect_test::expect;

use crate::test_util::check;

#[test]
fn format_fn_header_inline_comments_1() {
    check(
        "fn /*aaa*/ main /*bbb*/(/*ccc*/ a /*ddd*/ : /*eee*/ b /*fff*/ ) /*ggg*/  -> /*hhh*/ f32 /*iii*/ {} /*jjj*/",
        expect![[r#"
            fn /*aaa*/ main /*bbb*/ (/*ccc*/ a: /*ddd*/ /*eee*/ b /*fff*/) /*ggg*/ -> /*hhh*/ f32 /*iii*/ {}
        "#]],
    );
}

#[test]
fn format_fn_header_line_comments_1() {
    check(
        "fn
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
        {}",
        expect![["
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
