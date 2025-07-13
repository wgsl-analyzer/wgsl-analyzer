use super::parser2::{Diagnostic, Span};
use logos::{Lexer, Logos};

#[allow(clippy::upper_case_acronyms)]
#[derive(logos::Logos, Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[repr(u16)]
pub enum Token {
    EOF,
    #[token("enable")]
    Enable,
    #[token("requires")]
    Requires,
    #[token("fn")]
    Fn,
    #[token("alias")]
    Alias,
    #[token("struct")]
    Struct,
    #[token("var")]
    Var,
    #[token("const_assert")]
    ConstAssert,
    #[token("if")]
    If,
    #[token("for")]
    For,
    #[token("else")]
    Else,
    #[token("loop")]
    Loop,
    #[token("break")]
    Break,
    #[token("while")]
    While,
    #[token("return")]
    Return,
    #[token("switch")]
    Switch,
    #[token("discard")]
    Discard,
    #[token("continuing")]
    Continuing,
    #[token("const")]
    Const,
    #[token("case")]
    Case,
    #[token("default")]
    Default,
    #[token("override")]
    Override,
    #[token("continue")]
    Continue,
    #[token("let")]
    Let,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("diagnostic")]
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
    MinusGt,
    #[token("<")]
    Lt,
    #[token(">")]
    Gt,
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
    #[regex(r"([_\p{XID_Start}][\p{XID_Continue}]+)|[\p{XID_Start}]")]
    Ident,
    #[regex(r"0[fh]")]
    #[regex(r"[1-9][0-9]*[fh]")]
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
    #[regex("[\x20\x09\x0A-\x0D\u{0085}\u{200E}\u{200F}\u{2028}\u{2029}]+")]
    Whitespace,
    #[token("//", lex_line_ending_comment)]
    #[token("/*", lex_block_comment)]
    Comment,

    #[error]
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

#[cfg(test)]
mod tests {
    use expect_test::expect;
    use logos::Logos as _;

    use super::Token;

    #[expect(clippy::needless_pass_by_value, reason = "intended API")]
    fn check_lex(
        source: &str,
        expect: expect_test::Expect,
    ) {
        let tokens: Vec<_> = Token::lexer(source).collect();
        expect.assert_eq(&format!("{tokens:?}"));
    }

    #[test]
    fn lex_decimal_float() {
        check_lex("10.0", expect![["[DecimalFloatLiteral]"]]);
        check_lex("-10.0", expect![["[DecimalFloatLiteral]"]]);
        check_lex("1e9f", expect![["[DecimalFloatLiteral]"]]);
        check_lex("-0.0e7", expect![["[DecimalFloatLiteral]"]]);
        check_lex(".1", expect![["[DecimalFloatLiteral]"]]);
        check_lex("1.", expect![["[DecimalFloatLiteral]"]]);
    }

    #[test]
    fn lex_hex_float() {
        check_lex("0x0.0", expect![["[HexFloatLiteral]"]]);
        check_lex("0X1p9", expect![["[HexFloatLiteral]"]]);
        check_lex("-0x0.0", expect![["[HexFloatLiteral]"]]);
        check_lex("0xff.13p13", expect![["[HexFloatLiteral]"]]);
    }

    #[test]
    fn lex_comment() {
        check_lex(
            "// test asdf\nnot_comment",
            expect!["[LineEndingComment, Blankspace, Identifier]"],
        );
    }

    #[test]
    fn lex_nested_brackets() {
        // Expect: Identifier (a), [, Identifier (a), [, DecimalIntLiteral (0), ], ]
        check_lex(
            "a[a[0]]",
            expect![[
                "[Identifier, BracketLeft, Identifier, BracketLeft, DecimalIntLiteral, BracketRight, BracketRight]"
            ]],
        );
    }
}
