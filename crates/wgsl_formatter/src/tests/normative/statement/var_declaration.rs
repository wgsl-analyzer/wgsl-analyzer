use expect_test::expect;

use crate::test_util::check;

#[test]
pub fn format_var_declaration_prefer_break_type_instead_of_path() {
    check(
        "
        @group(constants::BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB) @binding(0)
        var<storage, read_write> dddddddddd_dddddddddddd_dddddddd: disp::DispatchIndirectArgsAtomic;
        ",
        expect![[r#"
            @group(constants::BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB) @binding(0)
            var<storage, read_write> dddddddddd_dddddddddddd_dddddddd: disp::DispatchIndirectArgsAtomic;
        "#]],
    );
}
