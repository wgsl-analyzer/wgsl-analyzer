pub mod comments;

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
            @attr(0)
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
            @attr(0, 0, 0)
            fn main() {}
        "#]],
    );
}

#[test]
pub fn format_attr_with_text_arguments() {
    check(
        "
        @
        thingy
        (
        magic,
        thing
        )
        fn main() {}
        ",
        expect![[r#"
            @thingy(magic, thing)
            fn main() {}
        "#]],
    );
}

#[test]
pub fn format_diagnostic_attr_simple_1() {
    check(
        "
       @bla
       @
       diagnostic
       (off, something)
       @blu
       fn main() {}
       ",
        expect![[r#"
            @bla
            @blu
            @diagnostic(off, something)
            fn main() {}
        "#]],
    );
}

#[test]
pub fn format_forced_linebreak_in_attribute_arguments() {
    check(
        "
        @foo(12, // Force break
        foo, 1 + vubble)
        override a: usize = 0;
        ",
        expect![[r#"
            @foo(
                12, // Force break
                foo,
                1 + vubble,
            )
            override a: usize = 0;
        "#]],
    );
}
