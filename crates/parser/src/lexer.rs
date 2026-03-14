use std::ops::Range;

use super::parser::{Diagnostic, Span};
use crate::parser::to_range;

#[expect(
    clippy::upper_case_acronyms,
    reason = "Lelwel generated code emits Token::EOF"
)]
#[expect(clippy::doc_paragraphs_missing_punctuation, reason = "false positive")]
#[derive(logos::Logos, Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[repr(u16)]
pub enum Token {
    EOF,
    EOFAttribute,
    EOFExpression,
    EOFStatement,
    EOFTypeSpecifier,
    /// `enable`
    Enable,
    /// `requires`
    Requires,
    /// `fn`
    Fn,
    /// `alias`
    Alias,
    /// `struct`
    Struct,
    /// `var`
    Var,
    /// `const_assert`
    ConstAssert,
    /// `if`
    If,
    /// `for`
    For,
    /// `else`
    Else,
    /// `loop`
    Loop,
    /// `break`
    Break,
    /// `while`
    While,
    /// `return`
    Return,
    /// `switch`
    Switch,
    /// `discard`
    Discard,
    /// `continuing`
    Continuing,
    /// `const`
    Const,
    /// `case`
    Case,
    /// `default`
    Default,
    /// `override`
    Override,
    /// `continue`
    Continue,
    /// `let`
    Let,
    /// `true`
    True,
    /// `false`
    False,
    /// `diagnostic`
    Diagnostic,
    #[token(";")]
    Semi,
    #[token("(")]
    LPar,
    #[token(")")]
    RPar,
    #[token(",")]
    Comma,
    #[token("=")]
    Eq,
    #[token(":")]
    Colon,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("->")]
    Arrow,
    #[token("<")]
    Lt,
    /// > A template parameter is an expression,
    /// > and therefore does not start with
    /// > either a '<' (U+003C) or a '=' (U+003D) code point.
    ///
    /// Source <https://www.w3.org/TR/WGSL/#template-list-discovery>
    #[token("<=")]
    LtEq,
    #[token("<<")]
    ShiftLeft,
    #[token("<<=")]
    ShiftLeftEq,
    #[token(">")]
    Gt,
    /// Ambiguous with shift right assign
    GtEq,
    /// Ambiguous: Can happen in a template `a<b<c>>`
    ShiftRight,
    /// Ambiguous: Can happen in a template `a<b> >= 2`
    ShiftRightEq,
    TemplateStart,
    TemplateEnd,
    #[token(".")]
    Dot,
    #[token("@")]
    At,
    #[token("[")]
    LBrak,
    #[token("]")]
    RBrak,
    #[token("&")]
    And,
    #[token("!")]
    Excl,
    #[token("*")]
    Star,
    #[token("-")]
    Minus,
    #[token("~")]
    Tilde,
    #[token("+")]
    Plus,
    #[token("==")]
    Eq2,
    #[token("|")]
    Pipe,
    #[token("&&")]
    And2,
    #[token("/")]
    Slash,
    #[token("^")]
    Caret,
    #[token("||")]
    Pipe2,
    #[token("!=")]
    ExclEq,
    #[token("%")]
    Percent,
    #[token("_")]
    Underscore,
    #[token("&=")]
    AndEq,
    #[token("*=")]
    StarEq,
    #[token("+=")]
    PlusEq,
    #[token("|=")]
    PipeEq,
    #[token("-=")]
    MinusEq,
    #[token("/=")]
    SlashEq,
    #[token("^=")]
    CaretEq,
    #[token("%=")]
    PercentEq,
    #[token("++")]
    Plus2,
    #[token("--")]
    Minus2,
    /// `import`
    Import,
    /// `package`
    Package,
    /// `super`
    Super,
    /// `as`
    As,
    #[token("::")]
    DoubleColon,
    /// A WGSL reserved word (https://www.w3.org/TR/WGSL/#reserved-words)
    Reserved,
    /// WGSL identifiers, parsing it ourselves
    Ident,
    #[regex(r"0[fh]")]
    #[regex(r"[1-9][0-9]*[fh]")]
    // We need priorities because some cases (for example, 1.2) would match both
    #[regex(r"[0-9]*\.[0-9]+([eE][+-]?[0-9]+)?[fh]?", priority = 5)]
    #[regex(r"[0-9]+\.[0-9]*([eE][+-]?[0-9]+)?[fh]?")]
    #[regex(r"[0-9]+[eE][+-]?[0-9]+[fh]?")]
    #[regex(
        r"0[xX][0-9a-fA-F]*\.[0-9a-fA-F]+([pP][+-]?[0-9]+[fh]?)?",
        priority = 9
    )]
    #[regex(r"0[xX][0-9a-fA-F]+\.[0-9a-fA-F]*([pP][+-]?[0-9]+[fh]?)?")]
    #[regex(r"0[xX][0-9a-fA-F]+[pP][+-]?[0-9]+[fh]?")]
    FloatLiteral,
    #[regex(r"0[iu]?")]
    #[regex(r"[1-9][0-9]*[iu]?")]
    #[regex(r"0[xX][0-9a-fA-F]+[iu]?")]
    IntLiteral,
    /// Source: <https://www.w3.org/TR/WGSL/#blankspace-and-line-breaks>
    #[regex("[\x20\x09\x0A-\x0D\u{0085}\u{200E}\u{200F}\u{2028}\u{2029}]+")]
    Blankspace,
    #[token("//", lex_line_ending_comment)]
    LineEndingComment,
    #[token("/*", lex_block_comment)]
    BlockComment,

    Error,
}

/// A line-ending comment is a kind of comment consisting of the two code points `//` (U+002F followed by U+002F)
/// and the code points that follow, up until but not including:
/// - the next line break, or
/// - the end of the program.
fn lex_line_ending_comment(lexer: &mut logos::Lexer<'_, Token>) {
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

fn lex_block_comment(lexer: &mut logos::Lexer<'_, Token>) -> Option<()> {
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

/// Returns `true` if the given word is a WGSL reserved word.
/// See <https://www.w3.org/TR/WGSL/#reserved-words>.
fn is_reserved_word(word: &str) -> bool {
    matches!(
        word,
        "NULL"
            | "Self"
            | "abstract"
            | "active"
            | "alignas"
            | "alignof"
            | "asm"
            | "asm_fragment"
            | "async"
            | "attribute"
            | "auto"
            | "await"
            | "become"
            | "cast"
            | "catch"
            | "class"
            | "co_await"
            | "co_return"
            | "co_yield"
            | "coherent"
            | "column_major"
            | "common"
            | "compile"
            | "compile_fragment"
            | "concept"
            | "const_cast"
            | "consteval"
            | "constexpr"
            | "constinit"
            | "crate"
            | "debugger"
            | "decltype"
            | "delete"
            | "demote"
            | "demote_to_helper"
            | "do"
            | "dynamic_cast"
            | "enum"
            | "explicit"
            | "export"
            | "extends"
            | "extern"
            | "external"
            | "fallthrough"
            | "filter"
            | "final"
            | "finally"
            | "friend"
            | "from"
            | "fxgroup"
            | "get"
            | "goto"
            | "groupshared"
            | "highp"
            | "impl"
            | "implements"
            | "inline"
            | "instanceof"
            | "interface"
            | "layout"
            | "lowp"
            | "macro"
            | "macro_rules"
            | "match"
            | "mediump"
            | "meta"
            | "mod"
            | "module"
            | "move"
            | "mut"
            | "mutable"
            | "namespace"
            | "new"
            | "nil"
            | "noexcept"
            | "noinline"
            | "nointerpolation"
            | "non_coherent"
            | "noncoherent"
            | "noperspective"
            | "null"
            | "nullptr"
            | "of"
            | "operator"
            | "packoffset"
            | "partition"
            | "pass"
            | "patch"
            | "pixelfragment"
            | "precise"
            | "precision"
            | "premerge"
            | "priv"
            | "protected"
            | "pub"
            | "public"
            | "readonly"
            | "ref"
            | "regardless"
            | "register"
            | "reinterpret_cast"
            | "require"
            | "resource"
            | "restrict"
            | "self"
            | "set"
            | "shared"
            | "sizeof"
            | "smooth"
            | "snorm"
            | "static"
            | "static_assert"
            | "static_cast"
            | "std"
            | "subroutine"
            | "target"
            | "template"
            | "this"
            | "thread_local"
            | "throw"
            | "trait"
            | "try"
            | "type"
            | "typedef"
            | "typeid"
            | "typename"
            | "typeof"
            | "union"
            | "unless"
            | "unorm"
            | "unsafe"
            | "unsized"
            | "use"
            | "using"
            | "varying"
            | "virtual"
            | "volatile"
            | "wgsl"
            | "where"
            | "with"
            | "writeonly"
            | "yield"
    )
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
                    "enable" => Token::Enable,
                    "requires" => Token::Requires,
                    "fn" => Token::Fn,
                    "alias" => Token::Alias,
                    "struct" => Token::Struct,
                    "var" => Token::Var,
                    "const_assert" => Token::ConstAssert,
                    "if" => Token::If,
                    "for" => Token::For,
                    "else" => Token::Else,
                    "loop" => Token::Loop,
                    "break" => Token::Break,
                    "while" => Token::While,
                    "return" => Token::Return,
                    "switch" => Token::Switch,
                    "discard" => Token::Discard,
                    "continuing" => Token::Continuing,
                    "const" => Token::Const,
                    "case" => Token::Case,
                    "default" => Token::Default,
                    "override" => Token::Override,
                    "continue" => Token::Continue,
                    "let" => Token::Let,
                    "true" => Token::True,
                    "false" => Token::False,
                    "diagnostic" => Token::Diagnostic,

                    "import" => Token::Import,
                    "package" => Token::Package,
                    "super" => Token::Super,
                    "as" => Token::As,

                    word if is_reserved_word(word) => {
                        self.diagnostics.push(Diagnostic {
                            message: format!("'{word}' is a reserved word in WGSL"),
                            range: to_range(token_start..token_end),
                        });
                        Token::Reserved
                    },
                    _ => Token::Ident,
                };

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

                        return Some((Token::Ident, token_start..self.inner.span().end));
                    },
                    _ => {
                        return Some((Token::Underscore, token_start..self.inner.span().end));
                    },
                }
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
            Token::Ident | Token::Var => {
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

                if let Some((Token::Lt, _)) = tokens_iter.peek() {
                    let (next_token, next_span) = tokens_iter.next().unwrap();
                    tokens.push(next_token);
                    spans.push(next_span);

                    pending.push((tokens.len() - 1, nesting_depth));
                }
            },
            Token::Gt => {
                if let Some((start_token, _)) = pending.pop_if(|(_, depth)| *depth == nesting_depth)
                {
                    // We found templates!
                    tokens[start_token] = Token::TemplateStart;
                    *tokens.last_mut().unwrap() = Token::TemplateEnd;
                } else {
                    // Patch up >>, >>=, >>==, >=, >==
                    // Precondition: pending.last().depth != nesting_depth
                    match tokens_iter.peek() {
                        Some((Token::Gt, span)) => {
                            // Might be a `>>`
                            *tokens.last_mut().unwrap() = Token::ShiftRight;
                            spans[tokens.len() - 1].end = span.end;
                            tokens_iter.next();
                            match tokens_iter.peek() {
                                Some((Token::Eq, span)) => {
                                    // Is a >>=
                                    *tokens.last_mut().unwrap() = Token::ShiftRightEq;
                                    spans[tokens.len() - 1].end = span.end;
                                    tokens_iter.next();
                                },
                                Some((Token::Eq2, span)) => {
                                    // Is a >>= =
                                    *tokens.last_mut().unwrap() = Token::ShiftRightEq;
                                    let middle = span.start + 1;
                                    spans[tokens.len() - 1].end = middle;
                                    tokens.push(Token::Eq);
                                    spans.push(middle..span.end);
                                    nesting_depth = 0;
                                    pending.clear();
                                    tokens_iter.next();
                                },
                                _ => {},
                            }
                        },
                        Some((Token::Eq, span)) => {
                            // Is a >=
                            *tokens.last_mut().unwrap() = Token::GtEq;
                            spans[tokens.len() - 1].end = span.end;
                            tokens_iter.next();
                        },
                        Some((Token::Eq2, span)) => {
                            // Is a >= =
                            *tokens.last_mut().unwrap() = Token::GtEq;
                            let middle = span.start + 1;
                            spans[tokens.len() - 1].end = middle;
                            tokens.push(Token::Eq);
                            spans.push(middle..span.end);
                            nesting_depth = 0;
                            pending.clear();
                            tokens_iter.next();
                        },
                        _ => {},
                    }
                }
            },
            Token::LPar | Token::LBrak => {
                nesting_depth += 1;
            },
            Token::RPar | Token::RBrak => {
                // Pop Pending stack until its top entry has depth < NestingDepth.
                while pending
                    .pop_if(|(_, depth)| *depth >= nesting_depth)
                    .is_some()
                {}
                nesting_depth = (nesting_depth - 1).max(0);
            },
            Token::Eq | Token::Semi | Token::LBrace | Token::Colon => {
                // These tokens do not appear in expressions,
                // so they aren't in a template
                nesting_depth = 0;
                pending.clear();
            },
            Token::And2 | Token::Pipe2 => {
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
    use logos::Logos as _;

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
            expect![["[LineEndingComment, Blankspace, Ident]"]],
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
                Ident@6..17
                Blankspace@17..19
                LineEndingComment@19..24
                Blankspace@24..26
                Ident@26..27
            "]],
        );
    }

    #[test]
    fn lex_nested_brackets() {
        // Expect: Identifier (a), [, Identifier (a), [, IntLiteral (0), ], ]
        check_lex(
            "a[a[0]]",
            expect!["[Ident, LBrak, Ident, LBrak, IntLiteral, RBrak, RBrak]"],
        );
    }

    #[test]
    fn lex_nested_templates() {
        check_lex_spanned(
            "foo<X>",
            expect![["
            Ident@0..3
            TemplateStart@3..4
            Ident@4..5
            TemplateEnd@5..6
        "]],
        );
        check_lex_spanned(
            "foo<X<Y>>",
            expect![["
                Ident@0..3
                TemplateStart@3..4
                Ident@4..5
                TemplateStart@5..6
                Ident@6..7
                TemplateEnd@7..8
                TemplateEnd@8..9
            "]],
        );
        check_lex_spanned(
            "foo<X<Y<Z>>>",
            expect![["
                Ident@0..3
                TemplateStart@3..4
                Ident@4..5
                TemplateStart@5..6
                Ident@6..7
                TemplateStart@7..8
                Ident@8..9
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
                Ident@0..3
                TemplateStart@3..4
                Ident@4..7
                Comma@7..8
                Ident@8..14
                LPar@14..15
                IntLiteral@15..16
                Comma@16..17
                IntLiteral@17..18
                Comma@18..19
                Ident@19..20
                Gt@20..21
                Ident@21..22
                RPar@22..23
                TemplateEnd@23..24
            "]],
        );
        check_lex_spanned(
            "foo<(B>=C)>a",
            expect![["
                Ident@0..3
                TemplateStart@3..4
                LPar@4..5
                Ident@5..6
                GtEq@6..8
                Ident@8..9
                RPar@9..10
                TemplateEnd@10..11
                Ident@11..12
            "]],
        );
        check_lex_spanned(
            "foo<(B!=C)>a",
            expect![["
                Ident@0..3
                TemplateStart@3..4
                LPar@4..5
                Ident@5..6
                ExclEq@6..8
                Ident@8..9
                RPar@9..10
                TemplateEnd@10..11
                Ident@11..12
            "]],
        );
        check_lex_spanned(
            "foo<(B==C)>a",
            expect![["
                Ident@0..3
                TemplateStart@3..4
                LPar@4..5
                Ident@5..6
                Eq2@6..8
                Ident@8..9
                RPar@9..10
                TemplateEnd@10..11
                Ident@11..12
            "]],
        );
    }

    #[test]
    fn lex_not_templates() {
        check_lex_spanned(
            "foo<d]>",
            expect![["
                Ident@0..3
                Lt@3..4
                Ident@4..5
                RBrak@5..6
                Gt@6..7
            "]],
        );
        check_lex_spanned(
            "foo",
            expect![["
            Ident@0..3
        "]],
        );
        check_lex_spanned(
            "foo<b || c>d",
            expect![["
            Ident@0..3
            Lt@3..4
            Ident@4..5
            Blankspace@5..6
            Pipe2@6..8
            Blankspace@8..9
            Ident@9..10
            Gt@10..11
            Ident@11..12
        "]],
        );
    }

    #[test]
    fn lex_templates_with_symbols() {
        check_lex_spanned(
            "foo<B<<C>",
            expect![["
                Ident@0..3
                TemplateStart@3..4
                Ident@4..5
                ShiftLeft@5..7
                Ident@7..8
                TemplateEnd@8..9
            "]],
        );
        check_lex_spanned(
            "foo<B<=C>",
            expect![["
            Ident@0..3
            TemplateStart@3..4
            Ident@4..5
            LtEq@5..7
            Ident@7..8
            TemplateEnd@8..9
        "]],
        );

        check_lex_spanned(
            "foo<>",
            expect![["
            Ident@0..3
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
                Ident@0..1
                TemplateStart@1..2
                Ident@2..3
                TemplateEnd@3..4
                Gt@4..5
                Ident@5..6
            "]],
        );
        check_lex_spanned(
            "A<B>==C",
            expect![["
                Ident@0..1
                TemplateStart@1..2
                Ident@2..3
                TemplateEnd@3..4
                Eq2@4..6
                Ident@6..7
            "]],
        );
        check_lex_spanned(
            "C<A<B>=C>",
            expect![["
                Ident@0..1
                Lt@1..2
                Ident@2..3
                TemplateStart@3..4
                Ident@4..5
                TemplateEnd@5..6
                Eq@6..7
                Ident@7..8
                Gt@8..9
            "]],
        );
    }

    #[test]
    fn lex_bitcast_template() {
        check_lex_spanned(
            "bitcast<vec4<u32>>(x)",
            expect![["
                Ident@0..7
                TemplateStart@7..8
                Ident@8..12
                TemplateStart@12..13
                Ident@13..16
                TemplateEnd@16..17
                TemplateEnd@17..18
                LPar@18..19
                Ident@19..20
                RPar@20..21
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
                Ident@4..12
                TemplateEnd@12..13
                Blankspace@13..14
                Ident@14..15
                Colon@15..16
                Blankspace@16..17
                Ident@17..20
                Semi@20..21
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
                Ident@9..10
                Colon@10..11
                Blankspace@11..12
                Ident@12..17
                TemplateStart@17..18
                Blankspace@18..35
                Ident@35..38
                Comma@38..39
                Blankspace@39..56
                IntLiteral@56..57
                Comma@57..58
                Blankspace@58..71
                TemplateEnd@71..72
                Semi@72..73
            "#]],
        );
    }

    #[test]
    fn lex_nested_comment() {
        check_lex_spanned(
            "foo /* bar /* // */ baz */",
            expect![["
                Ident@0..3
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
                Ident@0..3
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
