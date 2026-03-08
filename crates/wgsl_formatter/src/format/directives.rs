use syntax::{AstNode as _, SyntaxKind, SyntaxNode, ast, ast::SyntaxToken};

use crate::FormattingOptions;
use crate::util::{
    create_whitespace, is_whitespace_with_newline, n_newlines_in_whitespace,
    remove_if_whitespace, set_whitespace_before, set_whitespace_single_after,
};

/// Formats directive and top-level nodes: source file, `enable`, `requires`,
/// and attributes.
#[expect(clippy::wildcard_enum_match_arm, reason = "intentional catch-all dispatcher")]
pub(crate) fn format_directive(
    syntax: &SyntaxNode,
    indentation: usize,
    options: &FormattingOptions,
) -> Option<()> {
    match syntax.kind() {
        SyntaxKind::SourceFile => {
            // Collapse multiple blank lines between top-level items to at most one.
            for child in syntax.children() {
                let Some(first) = child.first_token() else {
                    continue;
                };
                let Some(preceding) = first.prev_token() else {
                    continue;
                };
                if let Some(newline_count) = n_newlines_in_whitespace(&preceding)
                    && newline_count > 2
                {
                    // Replace with exactly 2 newlines (= one blank line).
                    set_whitespace_before(&first, create_whitespace("\n\n"));
                }
            }
        },
        SyntaxKind::EnableDirective => {
            // `enable  f16;` → `enable f16;`
            let first = syntax.first_token()?;
            if first.kind() == SyntaxKind::Enable {
                set_whitespace_single_after(&first);
            }
        },
        SyntaxKind::RequiresDirective => {
            // `requires  x;` → `requires x;`
            let first = syntax.first_token()?;
            if first.kind() == SyntaxKind::Requires {
                set_whitespace_single_after(&first);
            }
        },
        SyntaxKind::Attribute => {
            let attribute = ast::Attribute::cast(syntax.clone())?;

            // Remove whitespace between `@` and the attribute name.
            if let Some(name_token) = attribute.ident_token() {
                remove_if_whitespace(&name_token.prev_token()?); // spellchecker:disable-line
            }

            // Format arguments: remove whitespace before `(`, normalize inside.
            if let Some(arguments) = attribute.parameters() {
                remove_if_whitespace(&arguments.left_parenthesis_token()?.prev_token()?); // spellchecker:disable-line
                super::format_parameters(&arguments, indentation, options)?;
            }

            // Preserve newlines after attributes (e.g. @vertex\nfn),
            // but ensure at least a single space when on the same line.
            if let Some(last) = syntax.last_token()
                && let Some(next) = last.next_token()
                && !is_whitespace_with_newline(&next)
            {
                set_whitespace_single_after(&last);
            }
        },
        _ => return None,
    }
    Some(())
}

#[cfg(test)]
mod tests {
    use expect_test::expect;

    use crate::format::tests::{check, check_str};

    #[test]
    fn format_enable_directive() {
        check(
            "enable  f16;
fn a() {}",
            expect![[r#"
            enable f16;
            fn a() {}"#]],
        );
    }

    #[test]
    fn format_requires_directive() {
        check(
            "requires  unrestricted_pointer_parameters;
fn a() {}",
            expect![[r#"
            requires unrestricted_pointer_parameters;
            fn a() {}"#]],
        );
    }

    #[test]
    fn format_attr_space_between_attrs() {
        check(
            "@group(0)@binding(1) var<storage> data: array<f32>;",
            expect![["@group(0) @binding(1) var<storage> data: array<f32>;"]],
        );
    }

    #[test]
    fn format_attr_space_before_fn() {
        check(
            "@vertex fn vs() -> vec4<f32> { return vec4<f32>(0.0); }",
            expect![["@vertex fn vs() -> vec4<f32> { return vec4<f32>(0.0); }"]],
        );
    }

    #[test]
    fn format_attr_space_before_fn_paren() {
        check(
            "@compute @workgroup_size(64)fn cs_main() {}",
            expect![["@compute @workgroup_size(64) fn cs_main() {}"]],
        );
    }

    #[test]
    fn format_attr_space_before_type() {
        check(
            "fn vs() -> @builtin(position)vec4<f32> { return vec4<f32>(0.0); }",
            expect![["fn vs() -> @builtin(position) vec4<f32> { return vec4<f32>(0.0); }"]],
        );
    }

    #[test]
    fn format_attr_space_before_override() {
        check(
            "@id(1)override threads: u32 = 64;",
            expect![["@id(1) override threads: u32 = 64;"]],
        );
    }

    #[test]
    fn format_attr_preserves_newline_before_fn() {
        check(
            "@vertex
fn vs() -> vec4<f32> { return vec4<f32>(0.0); }",
            expect![[r#"
            @vertex
            fn vs() -> vec4<f32> { return vec4<f32>(0.0); }"#]],
        );
    }

    #[test]
    fn format_attr_preserves_newline_before_var() {
        check(
            "@group(0) @binding(0)
var<uniform> params: Params;",
            expect![[r#"
            @group(0) @binding(0)
            var<uniform> params: Params;"#]],
        );
    }

    #[test]
    fn format_no_newlines_at_start_of_file() {
        check_str("\n\n\nfn a() {}\n", "fn a() {}\n");
    }

    #[test]
    fn format_one_newline_at_end_of_file_when_missing() {
        check_str("fn a() {}", "fn a() {}\n");
    }

    #[test]
    fn format_one_newline_at_end_of_file_when_too_much() {
        check_str("fn a() {}\n\n", "fn a() {}\n");
    }

    #[test]
    fn format_collapse_excess_blank_lines_between_fns() {
        check(
            "fn a() {}\n\n\n\nfn e() {}",
            expect![[r#"
            fn a() {}

            fn e() {}"#]],
        );
    }

    #[test]
    fn format_preserve_single_blank_line_between_fns() {
        check(
            "fn a() {}\n\nfn b() {}",
            expect![[r#"
            fn a() {}

            fn b() {}"#]],
        );
    }

    #[test]
    fn format_collapse_excess_blank_lines_between_structs() {
        check(
            "struct A {\n    a: i32,\n}\n\n\n\n\nstruct B {\n    b: i32,\n}",
            expect![[r#"
            struct A {
                a: i32,
            }

            struct B {
                b: i32,
            }"#]],
        );
    }

    #[test]
    fn format_multiple_attributes_on_var() {
        check(
            "@group(0)@binding(1)var<uniform>  params:  Params;",
            expect!["@group(0) @binding(1) var<uniform> params: Params;"],
        );
    }

    #[test]
    fn format_attribute_on_fn_with_return() {
        check(
            "@vertex fn  main(  )  ->  @builtin(position)  vec4<f32>  { return vec4<f32>(0.0); }",
            expect!["@vertex fn main() -> @builtin(position) vec4<f32> { return vec4<f32>(0.0); }"],
        );
    }

    #[test]
    fn format_comment_after_fn_header() {
        check(
            "fn a() { // comment\n    let x = 1;\n}",
            expect![[r#"
            fn a() { // comment
                let x = 1;
            }"#]],
        );
    }

    #[test]
    fn format_comment_between_statements() {
        check(
            "fn a() {\n    let x = 1;\n    // comment\n    let y = 2;\n}",
            expect![[r#"
            fn a() {
                let x = 1;
                // comment
                let y = 2;
            }"#]],
        );
    }

    #[test]
    fn format_block_comment_inline() {
        check(
            "fn a() { let x = /* comment */ 1; }",
            expect!["fn a() { let x = /* comment */ 1; }"],
        );
    }

    #[test]
    fn format_comment_after_struct_member() {
        check(
            "struct A {\n    x: i32, // x coord\n    y: i32, // y coord\n}",
            expect![[r#"
            struct A {
                x: i32, // x coord
                y: i32, // y coord
            }"#]],
        );
    }

    #[test]
    fn format_comment_before_closing_brace() {
        check(
            "fn a() {\n    let x = 1;\n    // trailing comment\n}",
            expect![[r#"
            fn a() {
                let x = 1;
                // trailing comment
            }"#]],
        );
    }

    #[test]
    fn format_attr_removes_space_after_at() {
        check(
            "@  group(0) var<uniform> data: f32;",
            expect!["@group(0) var<uniform> data: f32;"],
        );
    }

    #[test]
    fn format_attr_normalizes_arg_spacing() {
        check(
            "@group( 0 ) @binding( 1 ) var<uniform> data: f32;",
            expect!["@group(0) @binding(1) var<uniform> data: f32;"],
        );
    }

    #[test]
    fn format_attr_normalizes_multiple_args() {
        check(
            "@compute @workgroup_size( 64 , 1 , 1 ) fn cs() {}",
            expect!["@compute @workgroup_size(64, 1, 1) fn cs() {}"],
        );
    }

    #[test]
    fn format_attr_removes_space_before_parens() {
        check(
            "@group (0) @binding (1) var<uniform> data: f32;",
            expect!["@group(0) @binding(1) var<uniform> data: f32;"],
        );
    }

    #[test]
    fn format_attr_full_cleanup() {
        check(
            "@  group ( 0 )   @  binding ( 1 ) var<uniform> data: f32;",
            expect!["@group(0) @binding(1) var<uniform> data: f32;"],
        );
    }

}
