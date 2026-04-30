use expect_test::expect;

use crate::test_util::{check, check_comments};

#[test]
pub fn format_diagnostic_simple_1() {
    check(
        "
       diagnostic
       (off, something);
       ",
        expect![[r#"
            diagnostic(off, something);
        "#]],
    );
}

#[test]
pub fn format_diagnostic_with_dot_simple_1() {
    check(
        "
       diagnostic
       (off, something.something);
       ",
        expect![[r#"
            diagnostic(off, something.something);
        "#]],
    );
}

#[test]
pub fn format_diagnostic_with_dot_and_newline() {
    check(
        "
       diagnostic
       (off, something
       .
       something);
       ",
        expect![[r#"
            diagnostic(off, something.something);
        "#]],
    );
}

#[test]
pub fn format_comments_in_diagnostic_1() {
    check_comments(
        "## diagnostic ## ( ## off ## , ## something ## . ## something ## ) ## ; ##",
        expect![[r#"
            /* 0 */
            diagnostic /* 1 */ ( /* 2 */ off /* 3 */ , /* 4 */ something /* 5 */ . /* 6 */ something /* 7 */ ) /* 8 */ ; /* 9 */
        "#]],
        expect![[r#"
            // 0
            diagnostic // 1
            ( // 2
            off // 3
            , // 4
            something // 5
            . // 6
            something // 7
            ) // 8
            ; // 9
        "#]],
    );
}

#[test]
pub fn format_enable_simple_1() {
    check(
        "
       enable
       thing, bla,
       thingy,;
       ",
        expect![[r#"
            enable thing, bla, thingy;
        "#]],
    );
}

#[test]
pub fn format_enable_long_1() {
    check(
        "
        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
       enable
       thing, blaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,thingyyyyyyyyyyyyyyyyyyyyyyyyy,;
       ",
        expect![[r#"
            //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
            enable
                thing,
                blaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,
                thingyyyyyyyyyyyyyyyyyyyyyyyyy;
        "#]],
    );
}

#[test]
pub fn format_comments_in_enable_1() {
    check_comments(
        "enable ## thing ## , ## bla ## , ## ; ##",
        expect![[r#"
            enable /* 0 */ thing, /* 1 */ /* 2 */ bla /* 3 */ /* 4 */; /* 5 */
        "#]],
        expect![[r#"
            enable
                // 0
                thing, // 1
                // 2
                bla // 3
                // 4
                ; // 5
        "#]],
    );
}

#[test]
pub fn format_requires_simple_1() {
    check(
        "
       requires
       thing, bla,
       thingy,;
       ",
        expect![[r#"
            requires thing, bla, thingy;
        "#]],
    );
}

#[test]
pub fn format_requires_long_1() {
    check(
        "
        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
       requires
       thing, blaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,thingyyyyyyyyyyyyyyyyyyyyyyyyy,;
       ",
        expect![[r#"
            //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
            requires
                thing,
                blaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,
                thingyyyyyyyyyyyyyyyyyyyyyyyyy;
        "#]],
    );
}

#[test]
pub fn format_comments_in_requires_1() {
    check_comments(
        "requires ## thing ## , ## bla ## , ## ; ##",
        expect![[r#"
            requires /* 0 */ thing, /* 1 */ /* 2 */ bla /* 3 */ /* 4 */; /* 5 */
        "#]],
        expect![[r#"
            requires
                // 0
                thing, // 1
                // 2
                bla // 3
                // 4
                ; // 5
        "#]],
    );
}
