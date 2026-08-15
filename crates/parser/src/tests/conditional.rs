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
              AttributeList@1..10
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
              AttributeList@22..34
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
              AttributeList@1..4
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
              AttributeList@16..21
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
              AttributeList@1..6
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
              AttributeList@18..23
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
              AttributeList@1..10
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
              AttributeList@22..27
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

            error at 27..28: invalid syntax, expected one of: 'alias', '{', 'const', 'const_assert', 'diagnostic', 'enable', 'fn', 'import', 'let', 'override', 'requires', 'struct', 'var'"#]],
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
        default: { return; }
    }
}
        ",
        expect![[r#"
            SourceFile@0..131
              Blankspace@0..1 "\n"
              FunctionDeclaration@1..122
                Fn@1..3 "fn"
                Blankspace@3..4 " "
                Name@4..7
                  Identifier@4..7 "foo"
                FunctionParameters@7..9
                  ParenthesisLeft@7..8 "("
                  ParenthesisRight@8..9 ")"
                Blankspace@9..10 " "
                CompoundStatement@10..122
                  BraceLeft@10..11 "{"
                  Blankspace@11..16 "\n    "
                  SwitchStatement@16..120
                    Switch@16..22 "switch"
                    Blankspace@22..23 " "
                    Literal@23..27
                      True@23..27 "true"
                    Blankspace@27..28 " "
                    SwitchBody@28..120
                      BraceLeft@28..29 "{"
                      Blankspace@29..38 "\n        "
                      AttributeList@38..47
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
                      SwitchBodyCase@94..114
                        Default@94..101 "default"
                        Colon@101..102 ":"
                        Blankspace@102..103 " "
                        CompoundStatement@103..114
                          BraceLeft@103..104 "{"
                          Blankspace@104..105 " "
                          ReturnStatement@105..112
                            Return@105..111 "return"
                            Semicolon@111..112 ";"
                          Blankspace@112..113 " "
                          BraceRight@113..114 "}"
                      Blankspace@114..119 "\n    "
                      BraceRight@119..120 "}"
                  Blankspace@120..121 "\n"
                  BraceRight@121..122 "}"
              Blankspace@122..131 "\n        ""#]],
    );
}

#[test]
fn small_fragment_shader() {
    check_with_edition(
        edition::Edition::Wesl2025Unstable,
        "
@fragment
fn main(@builtin(position) pos: vec4f) -> @location(0) vec4f {
    var x: f32;
    @if (true)
    {
        var x = 1i;
    }
    return vec4(vec3(x), 1);
}
        ",
        expect![[r#"
            SourceFile@0..176
              Blankspace@0..1 "\n"
              AttributeList@1..10
                FragmentAttribute@1..10
                  AttributeOperator@1..2 "@"
                  Fragment@2..10 "fragment"
              Blankspace@10..11 "\n"
              FunctionDeclaration@11..167
                Fn@11..13 "fn"
                Blankspace@13..14 " "
                Name@14..18
                  Identifier@14..18 "main"
                FunctionParameters@18..49
                  ParenthesisLeft@18..19 "("
                  AttributeList@19..37
                    BuiltinAttribute@19..37
                      AttributeOperator@19..20 "@"
                      Builtin@20..27 "builtin"
                      ParenthesisLeft@27..28 "("
                      BuiltinValueName@28..36
                        Identifier@28..36 "position"
                      ParenthesisRight@36..37 ")"
                  Blankspace@37..38 " "
                  Parameter@38..48
                    Name@38..41
                      Identifier@38..41 "pos"
                    Colon@41..42 ":"
                    Blankspace@42..43 " "
                    TypeSpecifier@43..48
                      Path@43..48
                        Identifier@43..48 "vec4f"
                  ParenthesisRight@48..49 ")"
                Blankspace@49..50 " "
                ReturnType@50..71
                  Arrow@50..52 "->"
                  Blankspace@52..53 " "
                  AttributeList@53..65
                    LocationAttribute@53..65
                      AttributeOperator@53..54 "@"
                      Location@54..62 "location"
                      ParenthesisLeft@62..63 "("
                      Literal@63..64
                        IntLiteral@63..64 "0"
                      ParenthesisRight@64..65 ")"
                  Blankspace@65..66 " "
                  TypeSpecifier@66..71
                    Path@66..71
                      Identifier@66..71 "vec4f"
                Blankspace@71..72 " "
                CompoundStatement@72..167
                  BraceLeft@72..73 "{"
                  Blankspace@73..78 "\n    "
                  VariableDeclaration@78..89
                    Var@78..81 "var"
                    Blankspace@81..82 " "
                    Name@82..83
                      Identifier@82..83 "x"
                    Colon@83..84 ":"
                    Blankspace@84..85 " "
                    TypeSpecifier@85..88
                      Path@85..88
                        Identifier@85..88 "f32"
                    Semicolon@88..89 ";"
                  Blankspace@89..94 "\n    "
                  AttributeList@94..104
                    IfAttribute@94..104
                      AttributeOperator@94..95 "@"
                      If@95..97 "if"
                      Blankspace@97..98 " "
                      ParenthesisLeft@98..99 "("
                      Literal@99..103
                        True@99..103 "true"
                      ParenthesisRight@103..104 ")"
                  Blankspace@104..109 "\n    "
                  CompoundStatement@109..136
                    BraceLeft@109..110 "{"
                    Blankspace@110..119 "\n        "
                    VariableDeclaration@119..130
                      Var@119..122 "var"
                      Blankspace@122..123 " "
                      Name@123..124
                        Identifier@123..124 "x"
                      Blankspace@124..125 " "
                      Equal@125..126 "="
                      Blankspace@126..127 " "
                      Literal@127..129
                        IntLiteral@127..129 "1i"
                      Semicolon@129..130 ";"
                    Blankspace@130..135 "\n    "
                    BraceRight@135..136 "}"
                  Blankspace@136..141 "\n    "
                  ReturnStatement@141..165
                    Return@141..147 "return"
                    Blankspace@147..148 " "
                    FunctionCall@148..164
                      IdentExpression@148..152
                        Path@148..152
                          Identifier@148..152 "vec4"
                      Arguments@152..164
                        ParenthesisLeft@152..153 "("
                        FunctionCall@153..160
                          IdentExpression@153..157
                            Path@153..157
                              Identifier@153..157 "vec3"
                          Arguments@157..160
                            ParenthesisLeft@157..158 "("
                            IdentExpression@158..159
                              Path@158..159
                                Identifier@158..159 "x"
                            ParenthesisRight@159..160 ")"
                        Comma@160..161 ","
                        Blankspace@161..162 " "
                        Literal@162..163
                          IntLiteral@162..163 "1"
                        ParenthesisRight@163..164 ")"
                    Semicolon@164..165 ";"
                  Blankspace@165..166 "\n"
                  BraceRight@166..167 "}"
              Blankspace@167..176 "\n        ""#]],
    );
}
