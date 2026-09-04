use expect_test::expect;

use crate::test_util::check;

#[test]
fn format_condcomp_if_attribute_following_global_compound_declaration_does_not_get_merged() {
    check(
        "
        @if(true)
        {
        fn a() {}
        }
        @if(false)
        {
        fn a() {}
        }
        @else
        {
        fn a() {}
        }
        ",
        expect![[r#"
            @if(true) {
            fn a() {}
            }
            @if(false) {
            fn a() {}
            } @else {
            fn a() {}
            }
        "#]],
    );
}

#[test]
fn format_condcomp_attribute_with_global_compound_declaration_gets_merged() {
    check(
        "
        @if(true)
        {
        fn a() {}
        }
        @elif(false)
        {
        fn a() {}
        }
        @else
        {
        fn a() {}
        }
        ",
        expect![[r#"
            @if(true) {
            fn a() {}
            } @elif(false) {
            fn a() {}
            } @else {
            fn a() {}
            }
        "#]],
    );
}

#[test]
fn format_condcomp_if_attribute_following_compound_does_not_get_merged() {
    check(
        "
        fn main() {
        @if(true)
        {
            var x: u32;
        }
        @if(false)
        {
            var x: u32;
        }
        @else
        {
            var x: u32;
        }
            return x;
        }
        ",
        expect![[r#"
            fn main() {
            @if(true) {
                var x: u32;
            }
            @if(false) {
                var x: u32;
            } @else {
                var x: u32;
            }
                return x;
            }
        "#]],
    );
}

#[test]
fn format_condcomp_attribute_with_compound_gets_merged() {
    check(
        "
        fn main() {
        @if(true)
        {
            var x: u32;
        }
        @elif(false)
        {
            var x: u32;
        }
        @else
        {
            var x: u32;
        }
            return x;
        }
        ",
        expect![[r#"
            fn main() {
            @if(true) {
                var x: u32;
            } @elif(false) {
                var x: u32;
            } @else {
                var x: u32;
            }
                return x;
            }
        "#]],
    );
}

#[test]
fn format_condcomp_attribute_with_compound_gets_detented() {
    check(
        "
        fn main() {
            @if(true) {
            var x: u32;
            } @elif(false) {
            var x: u32;
            } @else {
            var x: u32;
            }
            return x;
        }
        ",
        expect![[r#"
            fn main() {
            @if(true) {
                var x: u32;
            } @elif(false) {
                var x: u32;
            } @else {
                var x: u32;
            }
                return x;
            }
        "#]],
    );
}

#[test]
fn format_condcomp_attribute_without_compound_gets_indetented() {
    // Conditional compound statements do not create a scope - and as such should not be indented
    //https://discord.com/channels/1289346613185351722/1341941812675481680/1537181486279557160
    check(
        "
        fn main() {
        @if(true)
        var x: u32;
        @elif(false)
        var x: u32;
        @else
        var x: u32;
            return x;
        }
        ",
        expect![[r#"
            fn main() {
                @if(true)
                var x: u32;
                @elif(false)
                var x: u32;
                @else
                var x: u32;
                return x;
            }
        "#]],
    );
}
