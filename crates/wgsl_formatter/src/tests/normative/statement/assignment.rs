use expect_test::expect;

use crate::{
    FormattingOptions,
    test_util::{check, check_with_options},
};

#[test]
fn format_assignment_statement_parenthesis() {
    check(
        "fn main() {
        (a) = (b + c)
        ;
        }",
        expect![[r#"
            fn main() {
                a = b + c;
            }
        "#]],
    );
}

#[test]
fn format_long_assignment_statement_gets_indented_correctly() {
    check_with_options(
        "
        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
        fn main() {
            a = long_name_function_aaaaaaaaaa() + long_name_function_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa();
        }",
        expect![[r#"
            //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
            fn main() {
                a = long_name_function_aaaaaaaaaa()
                    + long_name_function_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa();
            }
        "#]],
        &FormattingOptions {
            max_line_width: 80,
            ..Default::default()
        }.into(),
    );
}

#[test]
fn format_long_phony_statement_gets_indented_correctly() {
    check_with_options(
        "
        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
        fn main() {
            _ = long_name_function_aaaaaaaaaa() + long_name_function_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa();
        }",
        expect![[r#"
            //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
            fn main() {
                _ = long_name_function_aaaaaaaaaa()
                    + long_name_function_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa();
            }
        "#]],
        &FormattingOptions {
            max_line_width: 80,
            ..Default::default()
        }.into(),
    );
}

#[test]
fn format_assignment_with_long_function_call_prefers_breaking_function_call() {
    check_with_options(
        "
        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
        fn main() {
            aaa.bbbbbbbbbbbbb =
                to_long_to_fitaaaaa_aa_aaaa(bbb_bbbbb_bbbb_bbbbb(cccccc.cccccccccccccc), vec4<f32>(ddddddddddddddd, 1.0));

            aaa.bbbbbbbbbbbbb =
                could_fit_on_a_line_aa_aaaa(1.0, 2.0, vec4<f32>(ddddddddddddddd, 1.0));
        }",
        expect![[r#"
            //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
            fn main() {
                aaa.bbbbbbbbbbbbb = to_long_to_fitaaaaa_aa_aaaa(
                        bbb_bbbbb_bbbb_bbbbb(cccccc.cccccccccccccc),
                        vec4<f32>(ddddddddddddddd, 1.0),
                    );

                aaa.bbbbbbbbbbbbb = could_fit_on_a_line_aa_aaaa(
                        1.0,
                        2.0,
                        vec4<f32>(ddddddddddddddd, 1.0),
                    );
            }
        "#]],
        &FormattingOptions {
            max_line_width: 80,
            ..Default::default()
        }.into(),
    );
}

#[test]
fn format_assignment_with_single_arg_function_call_prefers_breaking_assignment_statement() {
    check_with_options(
        "
        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
        fn main() {
            aaa.bbbbbbbbbbbbb =
                i32(bbb_bbbb_bbbbb(cccccc.cccccccccccccc).fffffffffffffffffffffffffff);
        }",
        expect![[r#"
            //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
            fn main() {
                aaa.bbbbbbbbbbbbb =
                    i32(bbb_bbbb_bbbbb(cccccc.cccccccccccccc).fffffffffffffffffffffffffff);
            }
        "#]],
        &FormattingOptions {
            max_line_width: 80,
            ..Default::default()
        }
        .into(),
    );
}

#[test]
fn format_assignment_with_long_field_expr_prefers_breaking_field_expr() {
    check_with_options(
        "
        fn main() {

        aaaaaaaaaa::bbbbbbbbbbbbbbbbbb[cccccc::dddddddddddddddddddddddddd] = ffffff.gggggggggggg;
                }
        ",
        expect![[r#"
            fn main() {
                aaaaaaaaaa::bbbbbbbbbbbbbbbbbb[cccccc::dddddddddddddddddddddddddd] = ffffff
                        .gggggggggggg;
            }
        "#]],
        &FormattingOptions {
            max_line_width: 80,
            ..Default::default()
        }
        .into(),
    );
}
