use expect_test::expect;

use crate::test_util::check;

#[test]
#[ignore = "TODO"]
fn format_condcomp_statement_without_indentation() {
    // Conditional compound statements do not create a scope - and as such should not be indented
    //https://discord.com/channels/1289346613185351722/1341941812675481680/1537181486279557160
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
        "#]],
    );
}

#[test]
#[ignore = "TODO"]
fn format_condcomp_statement_nested_one_level_indentation() {
    // Conditional compound statements do not create a scope - and as such should not be indented
    //https://discord.com/channels/1289346613185351722/1341941812675481680/1537181486279557160
    check(
        "
        fn main() {
            @if(true)
            {{
                var x: u32;
            }}
            @elif(false)
            {{
                var x: u32;
            }}
            @else
            {{
                var x: u32;
            }}
            return x;
        }
        ",
        expect![[r#"
            fn main() {
                @if(true)
                {{
                var x: u32;
                }}
                @elif(false)
                {{
                var x: u32;
                }}
                @else
                {{
                var x: u32;
                }}
                return x;
            }
        "#]],
    );
}
