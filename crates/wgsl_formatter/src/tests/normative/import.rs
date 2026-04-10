use expect_test::expect;

use crate::{
    FormattingOptions,
    test_util::{check, check_with_options},
};

#[test]
pub fn format_import_whole_module() {
    check(
        "
        import a;
        ",
        expect![["
            fn main() {}
        "]],
    );
}
#[test]
pub fn format_import_path_single_simple() {
    check(
        "
        import a::b::c;
        ",
        expect![["
            fn main() {}
        "]],
    );
}
#[test]
pub fn format_import_relative_package_single_simple() {
    check(
        "
        import package::b::c;
        ",
        expect![["
            fn main() {}
        "]],
    );
}
#[test]
pub fn format_import_relative_super_single_simple() {
    check(
        "
        import super::b::c;
        ",
        expect![["
            fn main() {}
        "]],
    );
}
#[test]
pub fn format_import_relative_super_multilevel_single_simple() {
    check(
        "
        import super::super::super::super::c;
        ",
        expect![["
            fn main() {}
        "]],
    );
}
#[test]
pub fn format_import_path_multiple_same_prefix() {
    check(
        "
        import a::b::c;
        import a::b::d;
        ",
        expect![["
            fn main() {}
        "]],
    );
}
#[test]
pub fn format_import_path_multiple_different_prefix() {
    check(
        "
        import a::b::c;
        import d::e::f;
        ",
        expect![["
            fn main() {}
        "]],
    );
}
#[test]
pub fn format_import_path_single_with_rename() {
    check(
        "
        import a::b::c as d;
        ",
        expect![["
            fn main() {}
        "]],
    );
}
#[test]
pub fn format_import_collection_simple() {
    check(
        "
        import a::{b,
        c,
        d};
        ",
        expect![["
            fn main() {}
        "]],
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
        expect![["
            fn main() {}
        "]],
    );
}
#[test]
pub fn format_import_collection_break_on_long_items() {
    check_with_options(
        "
        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
        import aaaaaaaaaaaa::{aaaaaaaaaaaa, bbbbbbbbbbbbb, ccccccccccc, ddddddddddddd, eeeeeeeee, fffffff, gggggggg, hhhhhhhhhhh};
        ",
        &expect![["
            fn main() {}
        "]],
        &FormattingOptions {
            max_line_width: 80,
            ..Default::default()
        },
        parser::Edition::LATEST
    );
}
#[test]
pub fn format_import_collection_long_items_prefer_break_in_collection() {
    check_with_options(
        "
        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
        import aaaaaaaaaaaa::{aaaaaaaaaaaa::bbbbbbbbbbbbbbb::ccccccccccccc::ddddddddddddddd, eeeeeeeee, ffffffff::gggggggggggg};
        ",
        &expect![["
            fn main() {}
        "]],
        &FormattingOptions {
            max_line_width: 80,
            ..Default::default()
        },
        parser::Edition::LATEST
    );
}
#[test]
pub fn format_import_path_single_simple_long_items() {
    check_with_options(
        "
        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
        import aaaaaaaaaaaa::bbbbbbbbbbbbbbbbb::cccccccccccc::ddddddddddddd::eeeeeeeeeeee::fffffffffffff::gggggggggg;
        ",
        &expect![["
            fn main() {}
        "]],
        &FormattingOptions {
            max_line_width: 80,
            ..Default::default()
        },
        parser::Edition::LATEST
    );
}
