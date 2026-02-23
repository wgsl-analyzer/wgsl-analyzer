use expect_test::expect;

use crate::test_util::check;

#[test]
pub fn format_attr_simple() {
    check(
        "
        @
        fragment
        fn main() {}
        ",
        expect![[r#"
            @fragment
            fn main() {}
        "#]],
    );
}

#[test]
pub fn format_attr_with_argument() {
    check(
        "
        @
        attr
        (
        0
        )
        fn main() {}
        ",
        expect![[r#"
            @fragment
            fn main() {}
        "#]],
    );
}

#[test]
pub fn format_attr_with_multiple_arguments() {
    check(
        "
        @
        attr
        (
        0,
        0,
        0
        )
        fn main() {}
        ",
        expect![[r#"
            @fragment
            fn main() {}
        "#]],
    );
}

#[test]
pub fn format_attr_with_text_arguments() {
    check(
        "
        @
        builtin
        (
        magic,
        thing
        )
        fn main() {}
        ",
        expect![[r#"
            @fragment
            fn main() {}
        "#]],
    );
}
