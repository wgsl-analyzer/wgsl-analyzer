#![expect(clippy::use_debug, reason = "tests can use debug formatting")]

use std::fmt::Write as _;

use expect_test::expect;

use crate::{
    is_ident,
    lexer::{Token, lex},
};

#[expect(clippy::needless_pass_by_value, reason = "intended API")]
fn check_lex(
    source: &str,
    expect: expect_test::Expect,
) {
    let mut diagnostics = vec![];
    let (tokens, _) = lex(source, edition::Edition::Wgsl, &mut diagnostics);
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
    let (tokens, spans) = lex(source, edition::Edition::Wgsl, &mut diagnostics);
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

#[test]
fn is_ident_simple() {
    check_lex("foo", expect!["[Identifier]"]);
    assert!(is_ident("foo"));

    check_lex("_bar", expect!["[Identifier]"]);
    assert!(is_ident("_bar"));

    check_lex("_0x1", expect!["[Identifier]"]);
    assert!(is_ident("_0x1"));
}

#[test]
fn is_ident_accepts_reserved_idents() {
    check_lex("__foo", expect!["[Identifier]"]);
    assert!(is_ident("__foo"));
}

#[test]
fn is_ident_private() {
    check_lex("private", expect!["[Identifier]"]);
    assert!(is_ident("private"));
}

#[test]
fn is_ident_reject_keyword() {
    check_lex("const", expect!["[Const]"]);
    assert!(!is_ident("const"));
}

#[test]
fn is_ident_reject_reserved_word() {
    check_lex(
        "public",
        expect![[r#"
        [Reserved]
        [Diagnostic { message: "'public' is a reserved word in WGSL", range: 0..6 }, Diagnostic { message: "switch to WESL to use `public`", range: 0..6 }]
    "#]],
    );
    assert!(!is_ident("public"));

    check_lex(
        "await",
        expect![[r#"
            [Reserved]
            [Diagnostic { message: "'await' is a reserved word in WGSL", range: 0..5 }]
        "#]],
    );
    assert!(!is_ident("await"));
}
