use expect_test::expect;

use crate::tests::{check, check_with_edition};

#[test]
fn conditional_transpilation_attributes() {
    check_with_edition(
        edition::Edition::Wesl2025Unstable,
        "
@if(true)
fn foo(){}
@elif(false)
fn foo(){}
        ",
        expect![[r#"
            SourceFile@0..54
              Blankspace@0..1 "\n"
              FunctionDeclaration@1..21
                IfAttr@1..10
                  AttributeOperator@1..2 "@"
                  If@2..4 "if"
                  Arguments@4..10
                    ParenthesisLeft@4..5 "("
                    Literal@5..9
                      True@5..9 "true"
                    ParenthesisRight@9..10 ")"
                Blankspace@10..11 "\n"
                Fn@11..13 "fn"
                Blankspace@13..14 " "
                Name@14..17
                  Identifier@14..17 "foo"
                FunctionParameters@17..19
                  ParenthesisLeft@17..18 "("
                  ParenthesisRight@18..19 ")"
                CompoundStatement@19..21
                  BraceLeft@19..20 "{"
                  BraceRight@20..21 "}"
              Blankspace@21..22 "\n"
              FunctionDeclaration@22..45
                ElifAttr@22..34
                  AttributeOperator@22..23 "@"
                  Elif@23..27 "elif"
                  Arguments@27..34
                    ParenthesisLeft@27..28 "("
                    Literal@28..33
                      False@28..33 "false"
                    ParenthesisRight@33..34 ")"
                Blankspace@34..35 "\n"
                Fn@35..37 "fn"
                Blankspace@37..38 " "
                Name@38..41
                  Identifier@38..41 "foo"
                FunctionParameters@41..43
                  ParenthesisLeft@41..42 "("
                  ParenthesisRight@42..43 ")"
                CompoundStatement@43..45
                  BraceLeft@43..44 "{"
                  BraceRight@44..45 "}"
              Blankspace@45..54 "\n        ""#]],
    );
}

#[test]
fn conditional_transpilation_attributes_missing_expression() {
    check_with_edition(
        edition::Edition::Wesl2025Unstable,
        "
@if
fn foo(){}
@else
fn foo(){}
        ",
        expect![[r#"
            SourceFile@0..41
              Blankspace@0..1 "\n"
              FunctionDeclaration@1..15
                IfAttr@1..5
                  AttributeOperator@1..2 "@"
                  If@2..4 "if"
                  Blankspace@4..5 "\n"
                Fn@5..7 "fn"
                Blankspace@7..8 " "
                Name@8..11
                  Identifier@8..11 "foo"
                FunctionParameters@11..13
                  ParenthesisLeft@11..12 "("
                  ParenthesisRight@12..13 ")"
                CompoundStatement@13..15
                  BraceLeft@13..14 "{"
                  BraceRight@14..15 "}"
              Blankspace@15..16 "\n"
              FunctionDeclaration@16..32
                ElseAttr@16..21
                  AttributeOperator@16..17 "@"
                  Else@17..21 "else"
                Blankspace@21..22 "\n"
                Fn@22..24 "fn"
                Blankspace@24..25 " "
                Name@25..28
                  Identifier@25..28 "foo"
                FunctionParameters@28..30
                  ParenthesisLeft@28..29 "("
                  ParenthesisRight@29..30 ")"
                CompoundStatement@30..32
                  BraceLeft@30..31 "{"
                  BraceRight@31..32 "}"
              Blankspace@32..41 "\n        "

            error at 5..7: invalid syntax, expected: '('"#]],
    );
}
