use expect_test::expect;

use crate::{
    FormattingOptions,
    test_util::{CheckOptions, check_with_options},
};

#[test]
pub fn format_var_declaration_dont_break_var_template_args() {
    check_with_options(
        "
        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
        @group(constants::BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB) @binding(0)
        var<storage, read_write> dddddddddd_dddddddddddd_dddddddd_dddddddd_ddddddddd_ddddddd: disp::DispatchIndirectArgsAtomic;
        ",
        &expect![[r#"
            //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
            @group(constants::BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB) @binding(0)
            var<storage, read_write> dddddddddd_dddddddddddd_dddddddd_dddddddd_ddddddddd_ddddddd: disp::DispatchIndirectArgsAtomic;
        "#]],
        &CheckOptions {
            assert_line_width: None,
            formatting: FormattingOptions {
                max_line_width: 80,
                ..Default::default()
            },
        },
        parser::Edition::LATEST,
    );
}

#[test]
pub fn format_var_declaration_do_break_type_template_args() {
    check_with_options(
        "
        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
        @group(constants::BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB) @binding(0)
        var<storage, read_write> dddddddddd_dddddddddddd_dddddddd_dddddddddddd_dddddddddddd_ddddddd: array<fffffffffffffffff, ggggggggggggggg>;
        ",
        &expect![[r#"
            //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
            @group(constants::BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB) @binding(0)
            var<storage, read_write> dddddddddd_dddddddddddd_dddddddd_dddddddddddd_dddddddddddd_ddddddd: array<
                    fffffffffffffffff,
                    ggggggggggggggg,
                >;
        "#]],
        &CheckOptions {
            assert_line_width: None,
            formatting: FormattingOptions {
                max_line_width: 80,
                ..Default::default()
            },
        },
        parser::Edition::LATEST,
    );
}
