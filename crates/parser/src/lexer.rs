use std::ops::Range;

use edition::Edition;

use super::parser::{Diagnostic, Span};
use crate::{SyntaxKind, parser::to_range};

pub(crate) type Token = SyntaxKind;

#[derive(Default, Clone)]
pub struct LexerExtras {
    pub after_at: bool,
    pub edition: edition::Edition,
    pub extensions: edition::ExtensionsConfig,
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

/// Returns `true` if the given reserved word is upgraded to a keyword in WESL.
fn is_keyword_in_wesl(word: &str) -> bool {
    debug_assert!(
        is_reserved_word(word),
        "this is meant to be called with a reserved word"
    );
    matches!(word, "import" | "package" | "super" | "as" | "public")
}

/// Returns `true` if the given word is a valid identifier in WGSL.
/// Accepts reserved identifiers that start with two `__`.
pub fn is_ident(word: &str) -> bool {
    let mut chars = word.chars();
    let Some(first_char) = chars.next() else {
        return false;
    };
    if !(unicode_ident::is_xid_start(first_char) || (first_char == '_' && word.len() > 1)) {
        return false;
    }
    chars.all(unicode_ident::is_xid_continue) && !is_keyword(word) && !is_reserved_word(word)
}

/// Returns `true` if the given word is a WGSL keyword.
fn is_keyword(word: &str) -> bool {
    matches!(
        word,
        "alias"
            | "break"
            | "case"
            | "const"
            | "const_assert"
            | "continue"
            | "continuing"
            | "default"
            | "diagnostic"
            | "discard"
            | "else"
            | "enable"
            | "false"
            | "fn"
            | "for"
            | "if"
            | "let"
            | "loop"
            | "override"
            | "requires"
            | "return"
            | "struct"
            | "switch"
            | "true"
            | "var"
            | "while"
    )
}

/// Returns `true` if the given word is a WGSL reserved word.
/// See <https://www.w3.org/TR/WGSL/#reserved-words>.
#[expect(clippy::too_many_lines, reason = "just a big matches!")]
fn is_reserved_word(word: &str) -> bool {
    matches!(
        word,
        "NULL"
            | "Self"
            | "abstract"
            | "active"
            | "alignas"
            | "alignof"
            | "as"
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
            | "import"
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
            | "package"
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
            | "super"
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
    edition: Edition,
    diagnostics: &mut Vec<Diagnostic>,
) -> (Vec<Token>, Vec<Range<usize>>) {
    let inner = <Token as logos::Logos>::lexer(source);
    collect_with_templates(WgslLexer {
        inner,
        edition,
        diagnostics,
    })
}

struct WgslLexer<'source, 'diagnostics> {
    inner: logos::Lexer<'source, Token>,
    edition: Edition,
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
                    _ if self.inner.extras.after_at => Token::Identifier,
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
                    "import" if self.edition.at_least_wesl_0_0_1() => Token::Import,
                    "package" if self.edition.at_least_wesl_0_0_1() => Token::Package,
                    "super" if self.edition.at_least_wesl_0_0_1() => Token::Super,
                    "as" if self.edition.at_least_wesl_0_0_1() => Token::As,
                    "public" if self.edition.at_least_wesl_0_0_1() => Token::Public,
                    "private" if self.edition.at_least_wesl_0_0_1() => Token::Private,

                    word if is_reserved_word(word) => {
                        self.diagnostics.push(Diagnostic {
                            message: format!("'{word}' is a reserved word in WGSL"),
                            range: to_range(token_start..token_end),
                        });
                        // upgraded reserved words are already mapped above
                        if is_keyword_in_wesl(word) {
                            self.diagnostics.push(Diagnostic {
                                message: format!("switch to WESL to use `{word}`"),
                                range: to_range(self.inner.span()),
                            });
                        }
                        Token::Reserved
                    },
                    _ => Token::Identifier,
                };
                self.inner.extras.after_at = false;
                return Some((token_type, token_start..token_end));
            },
            Some('_') => {
                // An ident that must have more characters
                self.inner.bump('_'.len_utf8());

                self.inner.extras.after_at = false;
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
fn collect_with_templates<TokensIterator>(
    tokens_iterator: TokensIterator
) -> (Vec<Token>, Vec<Range<usize>>)
where
    TokensIterator: Iterator<Item = (Token, Span)>,
{
    let mut tokens_iterator = tokens_iterator.peekable();
    let mut nesting_depth = 0;
    let mut pending = vec![];
    let mut tokens = vec![];
    let mut spans = vec![];

    while let Some((token, span)) = tokens_iterator.next() {
        tokens.push(token);
        spans.push(span);
        match token {
            Token::Identifier | Token::Private | Token::Var => {
                // Skip to next non-whitespace token
                while let Some((
                    Token::Blankspace | Token::LineEndingComment | Token::BlockComment,
                    _,
                )) = tokens_iterator.peek()
                {
                    let (next_token, next_span) = tokens_iterator.next().unwrap();
                    tokens.push(next_token);
                    spans.push(next_span);
                }

                if let Some((Token::LessThan, _)) = tokens_iterator.peek() {
                    let (next_token, next_span) = tokens_iterator.next().unwrap();
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
                    match tokens_iterator.peek() {
                        Some((Token::GreaterThan, span)) => {
                            // Might be a `>>`
                            *tokens.last_mut().unwrap() = Token::ShiftRight;
                            spans[tokens.len() - 1].end = span.end;
                            tokens_iterator.next();
                            match tokens_iterator.peek() {
                                Some((Token::Equal, span)) => {
                                    // Is a >>=
                                    *tokens.last_mut().unwrap() = Token::ShiftRightEqual;
                                    spans[tokens.len() - 1].end = span.end;
                                    tokens_iterator.next();
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
                                    tokens_iterator.next();
                                },
                                _ => {},
                            }
                        },
                        Some((Token::Equal, span)) => {
                            // Is a >=
                            *tokens.last_mut().unwrap() = Token::GreaterThanEqual;
                            spans[tokens.len() - 1].end = span.end;
                            tokens_iterator.next();
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
                            tokens_iterator.next();
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
