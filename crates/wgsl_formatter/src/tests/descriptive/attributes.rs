pub mod comments;

use expect_test::expect;

use crate::test_util::{check, check_comments};

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
        builtin
        (
        magic,
        thing
        )
        fn main() {}
        ",
        expect![[r#"
            @builtin(magic, thing)
            fn main() {}
        "#]],
    );
}

#[test]
pub fn format_comments_in_attr_simple() {
    check_comments(
        "
        ## @ ## fragment ## fn ## main() {}
        ",
        expect![[r#"
            /* 0 */
            @ /* 1 */ fragment /* 2 */
            fn /* 3 */ main() {}
        "#]],
        expect![[r#"
            // 0
            @ // 1
            fragment // 2
            fn // 3
            main() {}
        "#]],
    );
}

#[test]
pub fn format_comments_in_attr_with_multiple_arguments() {
    check_comments(
        "
        ## @ ## attr ## ( ## 0 ## , ## 0 ## , ## 0 ## ) ## fn ## main() {}
        ",
        expect![[r#"
            /* 0 */
            @ /* 1 */ attr /* 2 */ (
                /* 3 */ 0, /* 4 */ /* 5 */
                0, /* 6 */ /* 7 */
                0, /* 8 */
            ) /* 9 */
            fn /* 10 */ main() {}
        "#]],
        expect![[r#"
            // 0
            @ // 1
            attr // 2
            (
                // 3
                0, // 4
                // 5
                0, // 6
                // 7
                0, // 8
            ) // 9
            fn // 10
            main() {}
        "#]],
    );
}

#[test]
pub fn format_comments_in_attr_with_text_arguments() {
    check_comments(
        "
        ## @ ## builtin ## ( ## magic ## , ## thing ## ) ## fn ## main() {}
        ",
        expect![[r#"
            /* 0 */
            @ /* 1 */ builtin /* 2 */ (/* 3 */ magic, /* 4 */ /* 5 */ thing /* 6 */) /* 7 */
            fn /* 8 */ main() {}
        "#]],
        expect![[r#"
            // 0
            @ // 1
            builtin // 2
            (
                // 3
                magic, // 4
                // 5
                thing, // 6
            ) // 7
            fn // 8
            main() {}
        "#]],
    );
}
