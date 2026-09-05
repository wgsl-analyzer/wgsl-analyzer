use expect_test::expect;

use crate::{
    FormattingOptions,
    test_util::{CheckOptions, assert_out_of_scope, check, check_with_options},
};

#[test]
pub fn format_import_whole_module() {
    check(
        "
        import a;
        ",
        expect![[r#"
            import a;
        "#]],
    );
}

#[test]
pub fn assert_trailing_colcol_is_parse_error() {
    assert_out_of_scope(
        "import a::b::;",
        "The formatter does not care about stripping trailing colcols because it should be a parse error.",
    );
}

#[test]
pub fn format_import_path_single_simple_() {
    check(
        "
        import a::b::c::d::e::f;
        ",
        expect![[r#"
            import a::b::c::d::e::f;
        "#]],
    );
}
#[test]
pub fn format_import_relative_package_single_simple() {
    check(
        "
        import package::b::c;
        ",
        expect![[r#"
            import package::b::c;
        "#]],
    );
}
#[test]
pub fn format_import_relative_super_single_simple() {
    check(
        "
        import super::b::c;
        ",
        expect![[r#"
            import super::b::c;
        "#]],
    );
}
#[test]
pub fn format_import_relative_super_multilevel_single_simple() {
    check(
        "
        import super::super::super::super::c;
        ",
        expect![[r#"
            import super::super::super::super::c;
        "#]],
    );
}
#[test]
pub fn format_import_path_multiple_same_prefix_does_not_get_collapsed() {
    check(
        "
        import a::b::c;
        import a::b::d;
        ",
        expect![[r#"
            import a::b::c;
            import a::b::d;
        "#]],
    );
}
#[test]
pub fn format_import_path_multiple_different_prefix() {
    check(
        "
        import a::b::c;
        import d::e::f;
        ",
        expect![[r#"
            import a::b::c;
            import d::e::f;
        "#]],
    );
}
#[test]
pub fn format_import_path_single_with_rename() {
    check(
        "
        import a::b::c as d;
        ",
        expect![[r#"
            import a::b::c as d;
        "#]],
    );
}
#[test]
pub fn format_import_collection_simple_gets_ordered() {
    check(
        "
        import a::{ZZZ,
    YYY,
        XXX};
        ",
        expect![[r#"
            import a::{XXX, YYY, ZZZ};
        "#]],
    );
}

#[test]
pub fn format_import_collection_order_items_alphabetical() {
    check(
        "
        import a::{a::x, a::m, a::a};
        ",
        expect![[r#"
            import a::{a::a, a::m, a::x};
        "#]],
    );
}

#[test]
pub fn format_import_collection_order_paths_alphabetical() {
    check(
        "
        import a::{a::x::a, a::m::a, a::a::a};
        ",
        expect![[r#"
            import a::{a::a::a, a::m::a, a::x::a};
        "#]],
    );
}

#[test]
pub fn format_import_collection_order_item_before_path() {
    check(
        "
        import a::{a::x, a::a, a::m::a};
        ",
        expect![[r#"
            import a::{a::a, a::x, a::m::a};
        "#]],
    );
}

#[test]
pub fn format_import_collection_order_path_before_collection() {
    check(
        "
        import a::{a::x::a, a::a::a, a::{b,c}};
        ",
        expect![[r#"
            import a::{a::a::a, a::x::a, a::{b, c}};
        "#]],
    );
}

#[test]
pub fn format_import_collection_remove_trailing_comma_when_singleline() {
    check(
        "
        import a::{b,
        c,
        d,};
        ",
        expect![[r#"
            import a::{b, c, d};
        "#]],
    );
}
#[test]
pub fn format_import_collection_break_on_long_items() {
    check_with_options(
        "
        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
        import aaaaaaaaaaaa::{aaaaaaaaaaaa, bbbbbbbbbbbbb, ccccccccccc, ddddddddddddd, eeeeeeeee, fffffff, gggggggg, hhhhhhhhhhh};
        ",
        expect![[r#"
            //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
            import aaaaaaaaaaaa::{
                aaaaaaaaaaaa,
                bbbbbbbbbbbbb,
                ccccccccccc,
                ddddddddddddd,
                eeeeeeeee,
                fffffff,
                gggggggg,
                hhhhhhhhhhh
            };
        "#]],
        &FormattingOptions {
            max_line_width: 80,
            ..Default::default()
        }.into(),
    );
}
#[test]
pub fn format_import_path_does_not_get_broken_into_lines() {
    // We decided that paths should not be broken up.
    // https://discord.com/channels/1289346613185351722/1341941812675481680/1540082081240064040
    check_with_options(
        "
        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
        import aaaaaaaaaaaa::bbbbbbbbbbbbbbbbb::cccccccccccc::ddddddddddddd::eeeeeeeeeeee::fffffffffffff::gggggggggg;
        ",
        expect![[r#"
            //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
            import aaaaaaaaaaaa::bbbbbbbbbbbbbbbbb::cccccccccccc::ddddddddddddd::eeeeeeeeeeee::fffffffffffff::gggggggggg;
        "#]],
        &CheckOptions {
            assert_line_width: None,
            formatting: FormattingOptions {
                max_line_width: 80,
                ..Default::default()
            },
            ..Default::default()
        },
    );
}

#[test]
pub fn immediately_nested_import_collections_dont_parse() {
    // The logic for sorting imports relies on this not parsing.
    assert_out_of_scope(
        "
        import a::{{a::a, b::b}, {c::c, d::d}};
        ",
        "ImportCollections immediately within ImportCollections are not supported.",
    );

    assert_out_of_scope(
        "
        import a::{{b}, {c}};
        ",
        "ImportCollections immediately within ImportCollections are not supported.",
    );
}

#[test]
pub fn format_import_collection_items_are_all_split_if_multiline() {
    check(
        "
import package::tracer_plugin::renderer::buffers::{
    a as b, c as d, e as f, ffffffffffffffffff, aaaaaaaaaaaaa,
    aaaaaaaaaaaaaaa::bbbbbbbbbbbbbbbbbbbbbb as cccccca, gggggggggggg
};
        ",
        expect![[r#"
            import package::tracer_plugin::renderer::buffers::{
                a as b,
                aaaaaaaaaaaaa,
                c as d,
                e as f,
                ffffffffffffffffff,
                gggggggggggg,
                aaaaaaaaaaaaaaa::bbbbbbbbbbbbbbbbbbbbbb as cccccca
            };
        "#]],
    );
}

#[test]
pub fn format_import_collection_items_are_kept_on_one_line_if_they_fit() {
    check_with_options(
        "
import package::tracer_plugin::renderer::buffers::{
keeps_inline::{AaaaaAaaa, BbbbbbBbbbbbBbb, CccCcc},
    aaaaaaaaaaaaaaa::bbbbbbbbbbbbbbbbbbbbbb as cccccca, gggggggggggg
};
        ",
        expect![[r#"
            import package::tracer_plugin::renderer::buffers::{
                gggggggggggg,
                aaaaaaaaaaaaaaa::bbbbbbbbbbbbbbbbbbbbbb as cccccca,
                keeps_inline::{AaaaaAaaa, BbbbbbBbbbbbBbb, CccCcc}
            };
        "#]],
        &FormattingOptions {
            max_line_width: 80,
            ..Default::default()
        }
        .into(),
    );
}
