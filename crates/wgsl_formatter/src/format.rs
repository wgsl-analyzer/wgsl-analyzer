mod declarations;
mod directives;
mod expressions;
mod statements;

use rowan::NodeOrToken;
use syntax::{AstNode, SyntaxKind, SyntaxNode, ast, ast::SyntaxToken};

use crate::FormattingOptions;
use crate::util::{
    clamp_newlines, create_syntax_token, create_whitespace, indent_before, insert_after_syntax,
    is_whitespace_with_newline, n_newlines_in_whitespace, remove_if_whitespace, remove_token,
    replace_token_with, set_whitespace_before, set_whitespace_single_after,
};

use crate::is_indent_kind;

/// Formats a single syntax node in-place.
///
/// Handles two cross-cutting concerns before dispatching:
/// 1. Preserves newline structure for children of compound statements and
///    switch bodies, adjusting indentation as needed.
/// 2. Removes whitespace before semicolons.
///
/// Then routes the node to the appropriate sub-module handler based on its
/// [`SyntaxKind`] category.
pub(crate) fn format_syntax_node(
    syntax: &SyntaxNode,
    indentation: usize,
    options: &FormattingOptions,
) -> Option<()> {
    if let Some(parent) = syntax.parent() {
        let parent_kind = parent.kind();

        if matches!(
            parent_kind,
            SyntaxKind::CompoundStatement | SyntaxKind::SwitchBody
        ) {
            let start = syntax.first_token()?;

            let n_newlines = n_newlines_in_whitespace(&start.prev_token()?) // spellchecker:disable-line
                .unwrap_or(0)
                .min(2);

            if n_newlines > 0 {
                let indent = if is_indent_kind(syntax) {
                    indentation.saturating_sub(1)
                } else {
                    indentation
                };
                set_whitespace_before(
                    &syntax.first_token()?,
                    create_whitespace(&format!(
                        "{}{}",
                        "\n".repeat(n_newlines),
                        options.indent_symbol.repeat(indent)
                    )),
                );
            }
        }

        // Clamp excessive blank lines to at most one (2 newlines).
        // Walk backwards through trivia tokens to catch whitespace before comments.
        if matches!(
            parent_kind,
            SyntaxKind::SourceFile | SyntaxKind::CompoundStatement | SyntaxKind::SwitchBody
        ) && let Some(start) = syntax.first_token()
        {
            let mut tok = start.prev_token(); // spellchecker:disable-line
            while let Some(current) = tok {
                if !current.kind().is_trivia() {
                    break;
                }
                if let Some(newline_count) = n_newlines_in_whitespace(&current)
                    && newline_count > 2
                {
                    let text = current.text();
                    let clamped = clamp_newlines(text, 2);
                    replace_token_with(&current, create_whitespace(&clamped));
                }
                tok = current.prev_token(); // spellchecker:disable-line
            }
        }
    }

    // Remove whitespace before semicolons in any statement node.
    if let Some(last) = syntax.last_token()
        && last.kind() == SyntaxKind::Semicolon
    {
        remove_if_whitespace(&last.prev_token()?); // spellchecker:disable-line
    }

    // Dispatch to the appropriate handler based on node type.
    let kind = syntax.kind();
    if kind.is_directive() {
        directives::format_directive(syntax, indentation, options);
    } else if kind.is_declaration() {
        declarations::format_declaration(syntax, indentation, options);
    } else if kind.is_statement() {
        statements::format_statement(syntax, indentation, options);
    } else {
        expressions::format_expression(syntax, indentation, options);
    }

    None
}

/// Format a colon token: remove whitespace before, single space after.
pub(super) fn format_colon(colon: Option<&SyntaxToken>) {
    if let Some(colon) = colon {
        let preceding = colon.prev_token(); // spellchecker:disable-line
        if let Some(preceding) = preceding {
            remove_if_whitespace(&preceding);
        }
        set_whitespace_single_after(colon);
    }
}

/// Remove whitespace around template angle brackets: `foo < T >` → `foo<T>`.
pub(super) fn format_template_angles(tmpl: &ast::TemplateList) -> Option<()> {
    let left_angle = tmpl.left_angle_token()?;
    remove_if_whitespace(&left_angle.prev_token()?); // spellchecker:disable-line
    remove_if_whitespace(&left_angle.next_token()?);
    let right_angle = tmpl.right_angle_token()?;
    remove_if_whitespace(&right_angle.prev_token()?); // spellchecker:disable-line
    Some(())
}

/// Formats a parenthesized argument list, handling both single-line and
/// multi-line layouts based on whether a newline follows the opening `(`.
pub(super) fn format_parameters(
    param_list: &ast::Arguments,
    indentation: usize,
    options: &FormattingOptions,
) -> Option<()> {
    let has_newline =
        is_whitespace_with_newline(&param_list.left_parenthesis_token()?.next_token()?);
    format_param_list(
        param_list.arguments(),
        param_list.arguments().count(),
        has_newline,
        indentation + 1,
        options.trailing_commas,
        &options.indent_symbol,
    );
    if has_newline {
        indent_before(&param_list.right_parenthesis_token()?, indentation, options);
    } else {
        remove_if_whitespace(&param_list.right_parenthesis_token()?.prev_token()?); // spellchecker:disable-line
    }
    Some(())
}

/// Generic formatter for comma-separated lists (parameters, arguments, struct members).
///
/// Normalizes whitespace between items, inserts missing commas, and applies
/// the trailing-comma policy on the last item.
pub(super) fn format_param_list<T: AstNode>(
    parameters: syntax::AstChildren<T>,
    count: usize,
    has_newline: bool,
    n_indentations: usize,
    trailing_comma_policy: crate::Policy,
    indent_symbol: &str,
) -> Option<()> {
    let mut first = true;
    for (index, parameter) in parameters.enumerate() {
        let last = index == count - 1;

        let first_token = parameter.syntax().first_token()?;
        let previous_had_newline = first_token
            .prev_token() // spellchecker:disable-line
            .is_some_and(|token| token.kind().is_whitespace() && token.text().contains('\n'));

        let whitespace = match (first, previous_had_newline) {
            (true, false) => create_whitespace(""),
            (_, true) => create_whitespace(&format!("\n{}", indent_symbol.repeat(n_indentations))),
            (false, false) => create_whitespace(" "),
        };

        set_whitespace_before(&first_token, whitespace);

        let last_param_token = parameter.syntax().last_token()?;
        remove_if_whitespace(&last_param_token);

        let mut token_after_parameter = match parameter.syntax().next_sibling_or_token() {
            Some(NodeOrToken::Node(node)) => node.first_token(),
            Some(NodeOrToken::Token(token)) => Some(token),
            None => None,
        };
        while let Some(tok) = &token_after_parameter {
            let kind = tok.kind();
            if kind.is_whitespace() {
                let next = tok.next_token();
                remove_token(tok);
                token_after_parameter = next;
            } else if matches!(
                kind,
                SyntaxKind::BlockComment | SyntaxKind::LineEndingComment
            ) {
                token_after_parameter = tok.next_token();
            } else {
                break;
            }
        }

        if let Some(token_after_parameter) = token_after_parameter {
            let is_comma = token_after_parameter.kind() == SyntaxKind::Comma;
            match (last, is_comma) {
                (true, true) if !has_newline => token_after_parameter.detach(),
                (true, has_comma) => match (trailing_comma_policy, has_comma) {
                    (crate::Policy::Remove, true) => token_after_parameter.detach(),
                    (crate::Policy::Remove, false)
                    | (crate::Policy::Insert, true)
                    | (crate::Policy::Ignore, _) => {},
                    (crate::Policy::Insert, false) => {
                        insert_after_syntax(
                            parameter.syntax(),
                            create_syntax_token(SyntaxKind::Comma, ","),
                        );
                    },
                },
                (false, true) => {},
                (false, false) => {
                    insert_after_syntax(
                        parameter.syntax(),
                        create_syntax_token(SyntaxKind::Comma, ","),
                    );
                },
            }
        }

        first = false;
    }

    Some(())
}

#[cfg(test)]
pub(crate) mod tests {
    #![expect(clippy::print_stderr, reason = "useful in tests")]
    #![expect(clippy::print_stdout, reason = "useful in tests")]
    #![expect(clippy::use_debug, reason = "useful in tests")]

    use std::panic;

    use expect_test::Expect;

    use crate::{FormattingOptions, format_recursive, format_str};

    #[expect(clippy::needless_pass_by_value, reason = "intentional API")]
    pub(crate) fn check(
        before: &str,
        after: Expect,
    ) {
        check_with_options(before, &after, &FormattingOptions::default());
    }

    /// Alias for [`check`] — kept for readability in WESL-specific tests.
    #[expect(clippy::needless_pass_by_value, reason = "intentional API")]
    pub(crate) fn check_wesl(
        before: &str,
        after: Expect,
    ) {
        check_with_options(before, &after, &FormattingOptions::default());
    }

    #[expect(clippy::needless_pass_by_value, reason = "intentional API")]
    pub(crate) fn check_tabs(
        before: &str,
        after: Expect,
    ) {
        let options = FormattingOptions {
            indent_symbol: "\t".to_owned(),
            ..Default::default()
        };
        check_with_options(before, &after, &options);
    }

    #[track_caller]
    pub(crate) fn check_with_options(
        before: &str,
        after: &Expect,
        options: &FormattingOptions,
    ) {
        let syntax = syntax::parse(before.trim_start(), syntax::Edition::LATEST)
            .syntax()
            .clone_for_update();
        format_recursive(&syntax, options);
        eprintln!("{syntax:#?}");

        let new = syntax.to_string();
        after.assert_eq(&new);

        // Check for idempotence.
        let syntax = syntax::parse(new.trim_start(), syntax::Edition::LATEST)
            .syntax()
            .clone_for_update();
        format_recursive(&syntax, options);

        let new_second = syntax.to_string();
        let diff = dissimilar::diff(&new, &new_second);
        let position = panic::Location::caller();
        if new == new_second {
            return;
        }
        println!(
            "\n
\x1b[1m\x1b[91merror\x1b[97m: Formatting Idempotence check failed\x1b[0m
\x1b[1m\x1b[34m-->\x1b[0m {position}
\x1b[1mExpect\x1b[0m:
----
{new}
----

\x1b[1mActual\x1b[0m:
----
{new_second}
----

\x1b[1mDiff\x1b[0m:
----
{}
----
",
            format_chunks(diff)
        );

        panic::resume_unwind(Box::new(()));
    }

    fn format_chunks(chunks: Vec<dissimilar::Chunk<'_>>) -> String {
        let mut buf = String::new();
        for chunk in chunks {
            let formatted = match chunk {
                dissimilar::Chunk::Equal(text) => text.into(),
                dissimilar::Chunk::Delete(text) => format!("\x1b[41m{text}\x1b[0m"),
                dissimilar::Chunk::Insert(text) => format!("\x1b[42m{text}\x1b[0m"),
            };
            buf.push_str(&formatted);
        }
        buf
    }

    /// Like `check`, but uses `format_str` (the public API) instead of `format_recursive`.
    #[track_caller]
    pub(crate) fn check_str(
        before: &str,
        expected: &str,
    ) {
        let actual = format_str(before, &FormattingOptions::default());
        assert_eq!(actual, expected, "format_str output mismatch");
    }
}
