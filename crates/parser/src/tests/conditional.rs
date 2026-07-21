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

#[test]
fn on_switch_cases() {
    check_with_edition(
        edition::Edition::Wesl2025Unstable,
        "
fn foo() {
    switch true {
        @if(true)
        case true, false: { return; }
        @elif(false)
        case true, false: { return; }
        @if(true)
        default: { return; }
        @else
        default: { return; }
    }
}
        ",
        expect![[r#"
            SourceFile@0..251
              Blankspace@0..1 "\n"
              FunctionDeclaration@1..242
                Fn@1..3 "fn"
                Blankspace@3..4 " "
                Name@4..7
                  Identifier@4..7 "foo"
                FunctionParameters@7..9
                  ParenthesisLeft@7..8 "("
                  ParenthesisRight@8..9 ")"
                Blankspace@9..10 " "
                CompoundStatement@10..242
                  BraceLeft@10..11 "{"
                  Blankspace@11..16 "\n    "
                  SwitchStatement@16..240
                    Switch@16..22 "switch"
                    Blankspace@22..23 " "
                    Literal@23..27
                      True@23..27 "true"
                    Blankspace@27..28 " "
                    SwitchBody@28..240
                      BraceLeft@28..29 "{"
                      Blankspace@29..38 "\n        "
                      IfAttribute@38..47
                        AttributeOperator@38..39 "@"
                        If@39..41 "if"
                        ParenthesisLeft@41..42 "("
                        Literal@42..46
                          True@42..46 "true"
                        ParenthesisRight@46..47 ")"
                      Blankspace@47..56 "\n        "
                      SwitchBodyCase@56..85
                        Case@56..60 "case"
                        Blankspace@60..61 " "
                        SwitchCaseSelectors@61..72
                          Literal@61..65
                            True@61..65 "true"
                          Comma@65..66 ","
                          Blankspace@66..67 " "
                          Literal@67..72
                            False@67..72 "false"
                        Colon@72..73 ":"
                        Blankspace@73..74 " "
                        CompoundStatement@74..85
                          BraceLeft@74..75 "{"
                          Blankspace@75..76 " "
                          ReturnStatement@76..83
                            Return@76..82 "return"
                            Semicolon@82..83 ";"
                          Blankspace@83..84 " "
                          BraceRight@84..85 "}"
                      Blankspace@85..94 "\n        "
                      ElifAttribute@94..106
                        AttributeOperator@94..95 "@"
                        Elif@95..99 "elif"
                        ParenthesisLeft@99..100 "("
                        Literal@100..105
                          False@100..105 "false"
                        ParenthesisRight@105..106 ")"
                      Blankspace@106..115 "\n        "
                      SwitchBodyCase@115..144
                        Case@115..119 "case"
                        Blankspace@119..120 " "
                        SwitchCaseSelectors@120..131
                          Literal@120..124
                            True@120..124 "true"
                          Comma@124..125 ","
                          Blankspace@125..126 " "
                          Literal@126..131
                            False@126..131 "false"
                        Colon@131..132 ":"
                        Blankspace@132..133 " "
                        CompoundStatement@133..144
                          BraceLeft@133..134 "{"
                          Blankspace@134..135 " "
                          ReturnStatement@135..142
                            Return@135..141 "return"
                            Semicolon@141..142 ";"
                          Blankspace@142..143 " "
                          BraceRight@143..144 "}"
                      Blankspace@144..153 "\n        "
                      IfAttribute@153..162
                        AttributeOperator@153..154 "@"
                        If@154..156 "if"
                        ParenthesisLeft@156..157 "("
                        Literal@157..161
                          True@157..161 "true"
                        ParenthesisRight@161..162 ")"
                      Blankspace@162..171 "\n        "
                      SwitchBodyCase@171..191
                        Default@171..178 "default"
                        Colon@178..179 ":"
                        Blankspace@179..180 " "
                        CompoundStatement@180..191
                          BraceLeft@180..181 "{"
                          Blankspace@181..182 " "
                          ReturnStatement@182..189
                            Return@182..188 "return"
                            Semicolon@188..189 ";"
                          Blankspace@189..190 " "
                          BraceRight@190..191 "}"
                      Blankspace@191..200 "\n        "
                      ElseAttribute@200..205
                        AttributeOperator@200..201 "@"
                        Else@201..205 "else"
                      Blankspace@205..214 "\n        "
                      SwitchBodyCase@214..234
                        Default@214..221 "default"
                        Colon@221..222 ":"
                        Blankspace@222..223 " "
                        CompoundStatement@223..234
                          BraceLeft@223..224 "{"
                          Blankspace@224..225 " "
                          ReturnStatement@225..232
                            Return@225..231 "return"
                            Semicolon@231..232 ";"
                          Blankspace@232..233 " "
                          BraceRight@233..234 "}"
                      Blankspace@234..239 "\n    "
                      BraceRight@239..240 "}"
                  Blankspace@240..241 "\n"
                  BraceRight@241..242 "}"
              Blankspace@242..251 "\n        ""#]],
    );
}
