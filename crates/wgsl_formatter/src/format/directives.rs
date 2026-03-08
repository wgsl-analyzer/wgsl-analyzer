use syntax::{AstNode as _, SyntaxKind, SyntaxNode, ast, ast::SyntaxToken};

use crate::FormattingOptions;
use crate::util::{
    create_whitespace, is_whitespace_with_newline, n_newlines_in_whitespace, remove_if_whitespace,
    remove_whitespace_around_double_colon, set_whitespace_before, set_whitespace_single_after,
    set_whitespace_single_before,
};

/// Formats directive and top-level nodes: source file, `enable`, `requires`,
/// and attributes.
#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "intentional catch-all dispatcher"
)]
pub(crate) fn format_directive(
    syntax: &SyntaxNode,
    indentation: usize,
    options: &FormattingOptions,
) -> Option<()> {
    match syntax.kind() {
        SyntaxKind::SourceFile => format_source_file(syntax),
        SyntaxKind::EnableDirective => format_enable_directive(syntax)?,
        SyntaxKind::RequiresDirective => format_requires_directive(syntax)?,
        SyntaxKind::Attribute => format_attribute(syntax, indentation, options)?,
        SyntaxKind::ImportStatement => format_import_statement(syntax)?,
        SyntaxKind::ImportPath
        | SyntaxKind::ImportPackageRelative
        | SyntaxKind::ImportSuperRelative => format_import_path(syntax),
        SyntaxKind::ImportCollection => format_import_collection(syntax)?,
        SyntaxKind::ImportItem => format_import_item(syntax),
        _ => return None,
    }
    Some(())
}

/// Collapses multiple blank lines between top-level items to at most one.
fn format_source_file(syntax: &SyntaxNode) {
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
}

/// Formats `enable  f16;` → `enable f16;`.
fn format_enable_directive(syntax: &SyntaxNode) -> Option<()> {
    let first = syntax.first_token()?;
    if first.kind() == SyntaxKind::Enable {
        set_whitespace_single_after(&first);
    }
    Some(())
}

/// Formats `requires  x;` → `requires x;`.
fn format_requires_directive(syntax: &SyntaxNode) -> Option<()> {
    let first = syntax.first_token()?;
    if first.kind() == SyntaxKind::Requires {
        set_whitespace_single_after(&first);
    }
    Some(())
}

/// Formats attributes: removes whitespace between `@` and the name,
/// normalizes argument parentheses, and ensures proper spacing after.
fn format_attribute(
    syntax: &SyntaxNode,
    indentation: usize,
    options: &FormattingOptions,
) -> Option<()> {
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
    Some(())
}

/// Formats `import  package::foo::{ bar , baz };`
/// → `import package::foo::{bar, baz};`.
///
/// Normalizes the space after `import` and removes whitespace around `::` tokens
/// between the `import` keyword and the first path/collection node.
fn format_import_statement(syntax: &SyntaxNode) -> Option<()> {
    let first = syntax.first_token()?;
    if first.kind() == SyntaxKind::Import {
        set_whitespace_single_after(&first);
    }
    // Remove whitespace around `::` tokens that are direct children.
    remove_whitespace_around_double_colon(syntax);
    Some(())
}

/// Formats an `ImportPath` node: `foo :: bar` → `foo::bar`.
fn format_import_path(syntax: &SyntaxNode) {
    remove_whitespace_around_double_colon(syntax);
}

/// Formats an `ImportCollection` node: `{ foo , bar , baz }` → `{foo, bar, baz}`
/// on a single line, or preserves the multi-line layout.
fn format_import_collection(syntax: &SyntaxNode) -> Option<()> {
    // Check if this collection spans multiple lines.
    let is_multiline = syntax.text().to_string().contains('\n');

    for child in syntax.children_with_tokens() {
        match &child {
            rowan::NodeOrToken::Token(token) if token.kind() == SyntaxKind::BraceLeft => {
                if !is_multiline {
                    // Single-line: remove space after `{`.
                    remove_if_whitespace(&token.next_token()?);
                }
            },
            rowan::NodeOrToken::Token(token) if token.kind() == SyntaxKind::BraceRight => {
                if !is_multiline {
                    // Single-line: remove space before `}`.
                    remove_if_whitespace(&token.prev_token()?); // spellchecker:disable-line
                }
            },
            rowan::NodeOrToken::Token(token) if token.kind() == SyntaxKind::Comma => {
                // Remove space before comma.
                remove_if_whitespace(&token.prev_token()?); // spellchecker:disable-line
                // Ensure single space after comma (single-line only).
                if !is_multiline {
                    set_whitespace_single_after(token);
                }
            },
            rowan::NodeOrToken::Token(_) | rowan::NodeOrToken::Node(_) => {},
        }
    }
    // Also clean up `::` inside the collection node itself.
    remove_whitespace_around_double_colon(syntax);
    Some(())
}

/// Formats an `ImportItem` node: `foo  as  bar` → `foo as bar`.
fn format_import_item(syntax: &SyntaxNode) {
    for child in syntax.children_with_tokens() {
        if let rowan::NodeOrToken::Token(token) = &child
            && token.kind() == SyntaxKind::As
        {
            // Ensure single space before and after `as`.
            set_whitespace_single_before(token);
            set_whitespace_single_after(token);
        }
    }
}

#[cfg(test)]
mod tests {
    use expect_test::expect;

    use crate::format::tests::{check, check_str, check_wesl};

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

    // ── WESL import formatting ──────────────────────────────────────────

    #[test]
    fn format_import_simple() {
        check_wesl("import  package::foo;", expect!["import package::foo;"]);
    }

    #[test]
    fn format_import_path_spacing() {
        check_wesl(
            "import  package :: utils :: math;",
            expect!["import package::utils::math;"],
        );
    }

    #[test]
    fn format_import_collection_single_line() {
        check_wesl(
            "import  package :: { foo , bar , baz };",
            expect!["import package::{foo, bar, baz};"],
        );
    }

    #[test]
    fn format_import_alias() {
        check_wesl(
            "import  foo :: bar  as  qux;",
            expect!["import foo::bar as qux;"],
        );
    }

    #[test]
    fn format_import_super() {
        check_wesl(
            "import  super :: super :: thing;",
            expect!["import super::super::thing;"],
        );
    }

    #[test]
    fn format_import_nested_collection_multiline() {
        check_wesl(
            "import bevy_pbr::{
  forward_io::VertexOutput,
  pbr_types::{PbrInput  as  PbrOutput, pbr_input_new},
  pbr_bindings,
};",
            expect![[r#"import bevy_pbr::{
  forward_io::VertexOutput,
  pbr_types::{PbrInput as PbrOutput, pbr_input_new},
  pbr_bindings,
};"#]],
        );
    }

    #[test]
    fn format_import_already_clean() {
        check_wesl("import package::foo;", expect!["import package::foo;"]);
    }
}
