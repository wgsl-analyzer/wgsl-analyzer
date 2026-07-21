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
              IfAttribute@1..10
                AttributeOperator@1..2 "@"
                If@2..4 "if"
                ParenthesisLeft@4..5 "("
                Literal@5..9
                  True@5..9 "true"
                ParenthesisRight@9..10 ")"
              Blankspace@10..11 "\n"
              FunctionDeclaration@11..21
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
              ElifAttribute@22..34
                AttributeOperator@22..23 "@"
                Elif@23..27 "elif"
                ParenthesisLeft@27..28 "("
                Literal@28..33
                  False@28..33 "false"
                ParenthesisRight@33..34 ")"
              Blankspace@34..35 "\n"
              FunctionDeclaration@35..45
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
@elif
fn foo(){}
        ",
        expect![[r#"
            SourceFile@0..41
              Blankspace@0..1 "\n"
              IfAttribute@1..4
                AttributeOperator@1..2 "@"
                If@2..4 "if"
              Blankspace@4..5 "\n"
              FunctionDeclaration@5..15
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
              ElifAttribute@16..21
                AttributeOperator@16..17 "@"
                Elif@17..21 "elif"
              Blankspace@21..22 "\n"
              FunctionDeclaration@22..32
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

            error at 5..7: invalid syntax, expected: '('
            error at 22..24: invalid syntax, expected: '('"#]],
    );
}

#[test]
fn if_attr_no_expression_is_wrong() {
    check_with_edition(
        edition::Edition::Wesl2025Unstable,
        "
@if()
fn foo(){}
@else
fn foo(){}
        ",
        expect![[r#"
            SourceFile@0..43
              Blankspace@0..1 "\n"
              IfAttribute@1..6
                AttributeOperator@1..2 "@"
                If@2..4 "if"
                ParenthesisLeft@4..5 "("
                ParenthesisRight@5..6 ")"
              Blankspace@6..7 "\n"
              FunctionDeclaration@7..17
                Fn@7..9 "fn"
                Blankspace@9..10 " "
                Name@10..13
                  Identifier@10..13 "foo"
                FunctionParameters@13..15
                  ParenthesisLeft@13..14 "("
                  ParenthesisRight@14..15 ")"
                CompoundStatement@15..17
                  BraceLeft@15..16 "{"
                  BraceRight@16..17 "}"
              Blankspace@17..18 "\n"
              ElseAttribute@18..23
                AttributeOperator@18..19 "@"
                Else@19..23 "else"
              Blankspace@23..24 "\n"
              FunctionDeclaration@24..34
                Fn@24..26 "fn"
                Blankspace@26..27 " "
                Name@27..30
                  Identifier@27..30 "foo"
                FunctionParameters@30..32
                  ParenthesisLeft@30..31 "("
                  ParenthesisRight@31..32 ")"
                CompoundStatement@32..34
                  BraceLeft@32..33 "{"
                  BraceRight@33..34 "}"
              Blankspace@34..43 "\n        "

            error at 5..6: invalid syntax, expected one of: '&', '!', 'false', <floating point literal>, <identifier>, <integer literal>, '-', 'package', '(', '*', 'super', '~', 'true'"#]],
    );
}

#[test]
fn else_attr_arguments_is_wrong() {
    check_with_edition(
        edition::Edition::Wesl2025Unstable,
        "
@if(true)
fn foo(){}
@else(false)
fn foo(){}
        ",
        expect![[r#"
            SourceFile@0..54
              Blankspace@0..1 "\n"
              IfAttribute@1..10
                AttributeOperator@1..2 "@"
                If@2..4 "if"
                ParenthesisLeft@4..5 "("
                Literal@5..9
                  True@5..9 "true"
                ParenthesisRight@9..10 ")"
              Blankspace@10..11 "\n"
              FunctionDeclaration@11..21
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
              ElseAttribute@22..27
                AttributeOperator@22..23 "@"
                Else@23..27 "else"
              Error@27..34
                ParenthesisLeft@27..28 "("
                False@28..33 "false"
                ParenthesisRight@33..34 ")"
              Blankspace@34..35 "\n"
              FunctionDeclaration@35..45
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
              Blankspace@45..54 "\n        "

            error at 27..28: invalid syntax, expected one of: 'alias', '@', 'const', 'const_assert', 'diagnostic', 'enable', 'fn', 'import', 'let', 'override', 'requires', 'struct', 'var'"#]],
    );
}
