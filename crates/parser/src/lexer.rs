use std::ops::Range;

use super::parser::{Diagnostic, Span};
use crate::{SyntaxKind, parser::to_range};

pub(crate) type Token = SyntaxKind;

#[derive(Default, Clone)]
pub struct LexerExtras {
    pub after_at: bool,
    pub after_interpolate: bool,
}

/// A line-ending comment is a kind of comment consisting of the two code points `//` (U+002F followed by U+002F)
/// and the code points that follow, up until but not including:
/// - the next line break, or
/// - the end of the program.
pub(crate) fn lex_line_ending_comment(lexer: &mut logos::Lexer<'_, SyntaxKind>) {
    let remainder = lexer.remainder();

    // see blankspace and line breaks: https://www.w3.org/TR/WGSL/#blankspace-and-line-breaks
    let line_end = remainder
        .char_indices()
        .find(|(_, character)| is_line_ending_comment_end(*character))
        .map_or(remainder.len(), |(index, _)| index);
    lexer.bump(line_end);
}

/// See: <https://www.w3.org/TR/WGSL/#blankspace-and-line-breaks>
/// The comment does not include the line break.
fn is_line_ending_comment_end(character: char) -> bool {
    [
        '\u{000A}', // line feed
        '\u{000B}', // vertical tab
        '\u{000C}', // form feed
        '\u{000D}', // carriage return when not also followed by line feed or carriage return followed by line feed
        '\u{0085}', // next line
        '\u{2028}', // line separator
        '\u{2029}', // paragraph separator
    ]
    .contains(&character)
}

pub(crate) fn lex_block_comment(lexer: &mut logos::Lexer<'_, SyntaxKind>) -> Option<()> {
    let mut depth = 1;
    let slice = lexer.remainder();
    let mut index = 0;
    let bytes = slice.as_bytes();
    while index + 1 < bytes.len() {
        if bytes[index] == b'/' && bytes[index + 1] == b'*' {
            depth += 1;
            index += 2;
        } else if bytes[index] == b'*' && bytes[index + 1] == b'/' {
            depth -= 1;
            index += 2;
            if depth == 0 {
                lexer.bump(index);
                return Some(());
            }
        } else {
            index += 1;
        }
    }
    // If we reach here, the comment was unterminated; consume the rest.
    lexer.bump(index);
    None
}

pub fn lex(
    source: &str,
    diagnostics: &mut Vec<Diagnostic>,
) -> (Vec<Token>, Vec<Range<usize>>) {
    collect_with_templates(WgslLexer {
        inner: <Token as logos::Logos>::lexer(source),
        diagnostics,
    })
}

struct WgslLexer<'source, 'diagnostics> {
    inner: logos::Lexer<'source, Token>,
    diagnostics: &'diagnostics mut Vec<Diagnostic>,
}

impl Iterator for WgslLexer<'_, '_> {
    type Item = (Token, Span);

    #[expect(
        clippy::too_many_lines,
        clippy::cognitive_complexity,
        reason = "match arms with control flow, hard to refactor"
    )]
    fn next(&mut self) -> Option<Self::Item> {
        // Parse WGSL identifiers.
        // Avoiding Logos here for compile time reasons.
        // ([_\p{XID_Start}][\p{XID_Continue}]+) | [\p{XID_Start}]
        let token_start = self.inner.span().end;

        let mut characters = self.inner.remainder().chars();
        match characters.next() {
            Some(first_char) if unicode_ident::is_xid_start(first_char) => {
                // An ident that may have more characters
                self.inner.bump(first_char.len_utf8());

                while let Some(next_char) = characters.next()
                    && unicode_ident::is_xid_continue(next_char)
                {
                    self.inner.bump(next_char.len_utf8());
                }

                // Check for all keywords
                let token_end = self.inner.span().end;
                let token_type = match &self.inner.source()[token_start..token_end] {
                    "alias" => Token::Alias,
                    "break" => Token::Break,
                    "case" => Token::Case,
                    "const_assert" => Token::ConstantAssert,
                    "const" => Token::Const,
                    "continue" => Token::Continue,
                    "continuing" => Token::Continuing,
                    "default" => Token::Default,
                    "diagnostic" => Token::Diagnostic,
                    "discard" => Token::Discard,
                    "else" => Token::Else,
                    "enable" => Token::Enable,
                    "false" => Token::False,
                    "fn" => Token::Fn,
                    "for" => Token::For,
                    "if" => Token::If,
                    "let" => Token::Let,
                    "loop" => Token::Loop,
                    "override" => Token::Override,
                    "requires" => Token::Requires,
                    "return" => Token::Return,
                    "struct" => Token::Struct,
                    "switch" => Token::Switch,
                    "true" => Token::True,
                    "var" => Token::Var,
                    "while" => Token::While,

                    // These WGSL reserved words are keywords in WESL
                    "import" => Token::Import,
                    "package" => Token::Package,
                    "super" => Token::Super,
                    "as" => Token::As,

                    // Context-dependent attribute keywords
                    "align" if self.inner.extras.after_at => Token::Align,
                    "binding" if self.inner.extras.after_at => Token::Binding,
                    "blend_src" if self.inner.extras.after_at => Token::BlendSrc,
                    "builtin" if self.inner.extras.after_at => Token::Builtin,
                    "group" if self.inner.extras.after_at => Token::Group,
                    "id" if self.inner.extras.after_at => Token::Id,
                    "interpolate" if self.inner.extras.after_at => {
                        self.inner.extras.after_interpolate = true;
                        Token::Interpolate
                    },
                    "invariant" if self.inner.extras.after_at => Token::Invariant,
                    "location" if self.inner.extras.after_at => Token::Location,
                    "must_use" if self.inner.extras.after_at => Token::MustUse,
                    "size" if self.inner.extras.after_at => Token::Size,
                    "workgroup_size" if self.inner.extras.after_at => Token::WorkgroupSize,
                    "vertex" if self.inner.extras.after_at => Token::Vertex,
                    "fragment" if self.inner.extras.after_at => Token::Fragment,
                    "compute" if self.inner.extras.after_at => Token::Compute,

                    // Context-dependent attribute arguments
                    "flat" if self.inner.extras.after_interpolate => Token::Flat,
                    "linear" if self.inner.extras.after_interpolate => Token::Linear,
                    "perspective" if self.inner.extras.after_interpolate => Token::Perspective,
                    "center" if self.inner.extras.after_interpolate => Token::Center,
                    "centroid" if self.inner.extras.after_interpolate => Token::Centroid,
                    "sample" if self.inner.extras.after_interpolate => Token::Sample,
                    "first" if self.inner.extras.after_interpolate => Token::First,
                    "either" if self.inner.extras.after_interpolate => Token::Either,

                    _ => Token::Identifier,
                };
                self.inner.extras.after_at = false;
                return Some((token_type, token_start..token_end));
            },
            Some('_') => {
                // An ident that must have more characters
                self.inner.bump('_'.len_utf8());

                match characters.next() {
                    Some(next_char) if unicode_ident::is_xid_continue(next_char) => {
                        self.inner.bump(next_char.len_utf8());
                        while let Some(next_char) = characters.next()
                            && unicode_ident::is_xid_continue(next_char)
                        {
                            self.inner.bump(next_char.len_utf8());
                        }

                        return Some((Token::Identifier, token_start..self.inner.span().end));
                    },
                    _ => {
                        return Some((Token::Underscore, token_start..self.inner.span().end));
                    },
                }
            },
            Some(')') => {
                self.inner.extras.after_interpolate = false;
            },
            _ => (), // Not an ident
        }

        // For everything else, just ask Logos
        self.inner.next().map(|token| {
            let span = self.inner.span();

            if let Ok(token) = token {
                (token, span)
            } else {
                self.diagnostics.push(Diagnostic {
                    message: "unexpected tokens".to_owned(),
                    range: to_range(span.clone()),
                });
                (Token::Error, span)
            }
        })
    }
}

/// Mutate tokens to be templates using <https://www.w3.org/TR/WGSL/#template-list-discovery>.
/// `<` and `>` tokens can be turned into template starts.
/// A pair of `>` `>` can start with a template end, or be a right shift.
/// Same goes for `>` `=` and `>` `>` `=`.
///
/// Meanwhile `<<` and `<<=` are unambiguously handled in the lexer,
/// since a template cannot start with those.
#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "Tries to mirror the algorithm as specified in the spec. Listing all tokens makes it less clear."
)]
fn collect_with_templates(
    tokens_iter: impl Iterator<Item = (Token, Span)>
) -> (Vec<Token>, Vec<Range<usize>>) {
    let mut tokens_iter = tokens_iter.peekable();
    let mut nesting_depth = 0;
    let mut pending = vec![];
    let mut tokens = vec![];
    let mut spans = vec![];

    while let Some((token, span)) = tokens_iter.next() {
        tokens.push(token);
        spans.push(span);
        match token {
            Token::Identifier | Token::Var => {
                // Skip to next non-whitespace token
                while let Some((
                    Token::Blankspace | Token::LineEndingComment | Token::BlockComment,
                    _,
                )) = tokens_iter.peek()
                {
                    let (next_token, next_span) = tokens_iter.next().unwrap();
                    tokens.push(next_token);
                    spans.push(next_span);
                }

                if let Some((Token::LessThan, _)) = tokens_iter.peek() {
                    let (next_token, next_span) = tokens_iter.next().unwrap();
                    tokens.push(next_token);
                    spans.push(next_span);

                    pending.push((tokens.len() - 1, nesting_depth));
                }
            },
            Token::GreaterThan => {
                if let Some((start_token, _)) = pending.pop_if(|(_, depth)| *depth == nesting_depth)
                {
                    // We found templates!
                    tokens[start_token] = Token::TemplateStart;
                    *tokens.last_mut().unwrap() = Token::TemplateEnd;
                } else {
                    // Patch up >>, >>=, >>==, >=, >==
                    // Precondition: pending.last().depth != nesting_depth
                    match tokens_iter.peek() {
                        Some((Token::GreaterThan, span)) => {
                            // Might be a `>>`
                            *tokens.last_mut().unwrap() = Token::ShiftRight;
                            spans[tokens.len() - 1].end = span.end;
                            tokens_iter.next();
                            match tokens_iter.peek() {
                                Some((Token::Equal, span)) => {
                                    // Is a >>=
                                    *tokens.last_mut().unwrap() = Token::ShiftRightEqual;
                                    spans[tokens.len() - 1].end = span.end;
                                    tokens_iter.next();
                                },
                                Some((Token::EqualEqual, span)) => {
                                    // Is a >>= =
                                    *tokens.last_mut().unwrap() = Token::ShiftRightEqual;
                                    let middle = span.start + 1;
                                    spans[tokens.len() - 1].end = middle;
                                    tokens.push(Token::Equal);
                                    spans.push(middle..span.end);
                                    nesting_depth = 0;
                                    pending.clear();
                                    tokens_iter.next();
                                },
                                _ => {},
                            }
                        },
                        Some((Token::Equal, span)) => {
                            // Is a >=
                            *tokens.last_mut().unwrap() = Token::GreaterThanEqual;
                            spans[tokens.len() - 1].end = span.end;
                            tokens_iter.next();
                        },
                        Some((Token::EqualEqual, span)) => {
                            // Is a >= =
                            *tokens.last_mut().unwrap() = Token::GreaterThanEqual;
                            let middle = span.start + 1;
                            spans[tokens.len() - 1].end = middle;
                            tokens.push(Token::Equal);
                            spans.push(middle..span.end);
                            nesting_depth = 0;
                            pending.clear();
                            tokens_iter.next();
                        },
                        _ => {},
                    }
                }
            },
            Token::ParenthesisLeft | Token::BracketLeft => {
                nesting_depth += 1;
            },
            Token::ParenthesisRight | Token::BracketRight => {
                // Pop Pending stack until its top entry has depth < NestingDepth.
                while pending
                    .pop_if(|(_, depth)| *depth >= nesting_depth)
                    .is_some()
                {}
                nesting_depth = (nesting_depth - 1).max(0);
            },
            Token::Equal | Token::Semicolon | Token::BraceLeft | Token::Colon => {
                // These tokens do not appear in expressions,
                // so they aren't in a template
                nesting_depth = 0;
                pending.clear();
            },
            Token::AndAnd | Token::OrOr => {
                while pending
                    .pop_if(|(_, depth)| *depth >= nesting_depth)
                    .is_some()
                {}
            },
            _ => {},
        }
    }

    (tokens, spans)
}

#[cfg(test)]
mod tests {
    #![expect(clippy::use_debug, reason = "tests can use debug formatting")]

    use std::fmt::Write as _;

    use expect_test::expect;

    use super::{Token, lex};

    #[expect(clippy::needless_pass_by_value, reason = "intended API")]
    fn check_lex(
        source: &str,
        expect: expect_test::Expect,
    ) {
        let mut diagnostics = vec![];
        let (tokens, _) = lex(source, &mut diagnostics);
        let mut expected = format!("{tokens:?}");
        if !diagnostics.is_empty() {
            writeln!(expected, "\n{diagnostics:?}");
        }
        expect.assert_eq(&expected);
    }

    #[expect(clippy::needless_pass_by_value, reason = "intended API")]
    fn check_lex_spanned(
        source: &str,
        expect: expect_test::Expect,
    ) {
        let mut diagnostics = Vec::new();
        let (tokens, spans) = lex(source, &mut diagnostics);
        let mut tokens_with_spans: String =
            tokens
                .into_iter()
                .zip(spans)
                .fold(String::new(), |mut output, (token, span)| {
                    _ = writeln!(output, "{token:?}@{}..{}", span.start, span.end);
                    output
                });
        for diagnostic in diagnostics {
            _ = writeln!(
                tokens_with_spans,
                "Error: {}@{}..{}",
                diagnostic.message,
                u32::from(diagnostic.range.start()),
                u32::from(diagnostic.range.end())
            );
        }
        expect.assert_eq(&tokens_with_spans);
    }

    #[test]
    fn lex_decimal_float() {
        check_lex("10.0", expect![["[FloatLiteral]"]]);
        check_lex("-10.0", expect![["[Minus, FloatLiteral]"]]);
        check_lex("1e9f", expect![["[FloatLiteral]"]]);
        check_lex("-0.0e7", expect!["[Minus, FloatLiteral]"]);
        check_lex(".1", expect![["[FloatLiteral]"]]);
        check_lex("1.", expect![["[FloatLiteral]"]]);
    }

    #[test]
    fn lex_hex_float() {
        check_lex("0x0.0", expect![["[FloatLiteral]"]]);
        check_lex("0X1p9", expect![["[FloatLiteral]"]]);
        check_lex("-0x0.0", expect![["[Minus, FloatLiteral]"]]);
        check_lex("0xff.13p13", expect![["[FloatLiteral]"]]);
    }

    #[test]
    fn lex_comment() {
        check_lex(
            "// test asdf\nnot_comment",
            expect![["[LineEndingComment, Blankspace, Identifier]"]],
        );
    }

    #[test]
    fn lex_odd_whitespace_comment() {
        check_lex_spanned(
            "\n\r//\r\nnot_comment\r\n//foo\n\ra",
            expect![["
                Blankspace@0..2
                LineEndingComment@2..4
                Blankspace@4..6
                Identifier@6..17
                Blankspace@17..19
                LineEndingComment@19..24
                Blankspace@24..26
                Identifier@26..27
            "]],
        );
    }

    #[test]
    fn lex_nested_brackets() {
        // Expect: Identifier (a), [, Identifier (a), [, IntLiteral (0), ], ]
        check_lex(
            "a[a[0]]",
            expect![
                "[Identifier, BracketLeft, Identifier, BracketLeft, IntLiteral, BracketRight, BracketRight]"
            ],
        );
    }

    #[test]
    fn lex_nested_templates() {
        check_lex_spanned(
            "foo<X>",
            expect![["
            Identifier@0..3
            TemplateStart@3..4
            Identifier@4..5
            TemplateEnd@5..6
        "]],
        );
        check_lex_spanned(
            "foo<X<Y>>",
            expect![["
                Identifier@0..3
                TemplateStart@3..4
                Identifier@4..5
                TemplateStart@5..6
                Identifier@6..7
                TemplateEnd@7..8
                TemplateEnd@8..9
            "]],
        );
        check_lex_spanned(
            "foo<X<Y<Z>>>",
            expect![["
                Identifier@0..3
                TemplateStart@3..4
                Identifier@4..5
                TemplateStart@5..6
                Identifier@6..7
                TemplateStart@7..8
                Identifier@8..9
                TemplateEnd@9..10
                TemplateEnd@10..11
                TemplateEnd@11..12
            "]],
        );
    }

    #[test]
    fn lex_template_with_brackets() {
        // cases from the WGSL spec
        check_lex_spanned(
            "foo<i32,select(2,3,a>b)>",
            expect![["
                Identifier@0..3
                TemplateStart@3..4
                Identifier@4..7
                Comma@7..8
                Identifier@8..14
                ParenthesisLeft@14..15
                IntLiteral@15..16
                Comma@16..17
                IntLiteral@17..18
                Comma@18..19
                Identifier@19..20
                GreaterThan@20..21
                Identifier@21..22
                ParenthesisRight@22..23
                TemplateEnd@23..24
            "]],
        );
        check_lex_spanned(
            "foo<(B>=C)>a",
            expect![["
                Identifier@0..3
                TemplateStart@3..4
                ParenthesisLeft@4..5
                Identifier@5..6
                GreaterThanEqual@6..8
                Identifier@8..9
                ParenthesisRight@9..10
                TemplateEnd@10..11
                Identifier@11..12
            "]],
        );
        check_lex_spanned(
            "foo<(B!=C)>a",
            expect![["
                Identifier@0..3
                TemplateStart@3..4
                ParenthesisLeft@4..5
                Identifier@5..6
                NotEqual@6..8
                Identifier@8..9
                ParenthesisRight@9..10
                TemplateEnd@10..11
                Identifier@11..12
            "]],
        );
        check_lex_spanned(
            "foo<(B==C)>a",
            expect![["
                Identifier@0..3
                TemplateStart@3..4
                ParenthesisLeft@4..5
                Identifier@5..6
                EqualEqual@6..8
                Identifier@8..9
                ParenthesisRight@9..10
                TemplateEnd@10..11
                Identifier@11..12
            "]],
        );
    }

    #[test]
    fn lex_not_templates() {
        check_lex_spanned(
            "foo<d]>",
            expect![["
                Identifier@0..3
                LessThan@3..4
                Identifier@4..5
                BracketRight@5..6
                GreaterThan@6..7
            "]],
        );
        check_lex_spanned(
            "foo",
            expect![["
            Identifier@0..3
        "]],
        );
        check_lex_spanned(
            "foo<b || c>d",
            expect![["
            Identifier@0..3
            LessThan@3..4
            Identifier@4..5
            Blankspace@5..6
            OrOr@6..8
            Blankspace@8..9
            Identifier@9..10
            GreaterThan@10..11
            Identifier@11..12
        "]],
        );
    }

    #[test]
    fn lex_templates_with_symbols() {
        check_lex_spanned(
            "foo<B<<C>",
            expect![["
                Identifier@0..3
                TemplateStart@3..4
                Identifier@4..5
                ShiftLeft@5..7
                Identifier@7..8
                TemplateEnd@8..9
            "]],
        );
        check_lex_spanned(
            "foo<B<=C>",
            expect![["
            Identifier@0..3
            TemplateStart@3..4
            Identifier@4..5
            LessThanEqual@5..7
            Identifier@7..8
            TemplateEnd@8..9
        "]],
        );

        check_lex_spanned(
            "foo<>",
            expect![["
            Identifier@0..3
            TemplateStart@3..4
            TemplateEnd@4..5
        "]],
        );
    }

    #[test]
    fn lex_templates_with_ends() {
        check_lex_spanned(
            "A<B>>C",
            expect![["
                Identifier@0..1
                TemplateStart@1..2
                Identifier@2..3
                TemplateEnd@3..4
                GreaterThan@4..5
                Identifier@5..6
            "]],
        );
        check_lex_spanned(
            "A<B>==C",
            expect![["
                Identifier@0..1
                TemplateStart@1..2
                Identifier@2..3
                TemplateEnd@3..4
                EqualEqual@4..6
                Identifier@6..7
            "]],
        );
        check_lex_spanned(
            "C<A<B>=C>",
            expect![["
                Identifier@0..1
                LessThan@1..2
                Identifier@2..3
                TemplateStart@3..4
                Identifier@4..5
                TemplateEnd@5..6
                Equal@6..7
                Identifier@7..8
                GreaterThan@8..9
            "]],
        );
    }

    #[test]
    fn lex_bitcast_template() {
        check_lex_spanned(
            "bitcast<vec4<u32>>(x)",
            expect![["
                Identifier@0..7
                TemplateStart@7..8
                Identifier@8..12
                TemplateStart@12..13
                Identifier@13..16
                TemplateEnd@16..17
                TemplateEnd@17..18
                ParenthesisLeft@18..19
                Identifier@19..20
                ParenthesisRight@20..21
            "]],
        );
    }

    #[test]
    fn lex_var_template() {
        check_lex_spanned(
            "var<function> x: u32;",
            expect![["
                Var@0..3
                TemplateStart@3..4
                Identifier@4..12
                TemplateEnd@12..13
                Blankspace@13..14
                Identifier@14..15
                Colon@15..16
                Blankspace@16..17
                Identifier@17..20
                Semicolon@20..21
            "]],
        );
    }

    #[test]
    fn lex_template_trailing_comment() {
        check_lex_spanned(
            "override x: array<
                u32,
                2,
            >;",
            expect![[r#"
                Override@0..8
                Blankspace@8..9
                Identifier@9..10
                Colon@10..11
                Blankspace@11..12
                Identifier@12..17
                TemplateStart@17..18
                Blankspace@18..35
                Identifier@35..38
                Comma@38..39
                Blankspace@39..56
                IntLiteral@56..57
                Comma@57..58
                Blankspace@58..71
                TemplateEnd@71..72
                Semicolon@72..73
            "#]],
        );
    }

    #[test]
    fn lex_nested_comment() {
        check_lex_spanned(
            "foo /* bar /* // */ baz */",
            expect![["
                Identifier@0..3
                Blankspace@3..4
                BlockComment@4..26
            "]],
        );
    }

    #[test]
    fn lex_unclosed_comment() {
        check_lex_spanned(
            "foo /*",
            expect![["
                Identifier@0..3
                Blankspace@3..4
                Error@4..6
                Error: unexpected tokens@4..6
            "]],
        );
    }

    #[test]
    fn lex_leading_zeros() {
        check_lex_spanned(
            "007",
            expect![[r#"
                IntLiteral@0..1
                IntLiteral@1..2
                IntLiteral@2..3
            "#]],
        );
    }
}
