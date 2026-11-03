use expect_test::expect;

use crate::tests::check;

#[test]
fn module_compound_nested() {
    check(
        "
        @if(true) { { } }
        ",
        expect![[r#"
            SourceFile@0..35
              Blankspace@0..9 "\n        "
              AttributeList@9..18
                Attribute@9..18
                  AttributeOperator@9..10 "@"
                  Identifier@10..12 "if"
                  AttributeArguments@12..18
                    ParenthesisLeft@12..13 "("
                    Literal@13..17
                      True@13..17 "true"
                    ParenthesisRight@17..18 ")"
              Blankspace@18..19 " "
              GlobalCompoundDeclaration@19..26
                BraceLeft@19..20 "{"
                Blankspace@20..21 " "
                GlobalCompoundDeclaration@21..24
                  BraceLeft@21..22 "{"
                  Blankspace@22..23 " "
                  BraceRight@23..24 "}"
                Blankspace@24..25 " "
                BraceRight@25..26 "}"
              Blankspace@26..35 "\n        ""#]],
    );
}

#[test]
fn function_compound_nested() {
    check(
        "
        fn foo() { @if(true) { { var x = 0; } } }
        ",
        expect![[r#"
            SourceFile@0..59
              Blankspace@0..9 "\n        "
              FunctionDeclaration@9..50
                Fn@9..11 "fn"
                Blankspace@11..12 " "
                Name@12..15
                  Identifier@12..15 "foo"
                FunctionParameters@15..17
                  ParenthesisLeft@15..16 "("
                  ParenthesisRight@16..17 ")"
                Blankspace@17..18 " "
                CompoundStatement@18..50
                  BraceLeft@18..19 "{"
                  Blankspace@19..20 " "
                  AttributeList@20..29
                    Attribute@20..29
                      AttributeOperator@20..21 "@"
                      Identifier@21..23 "if"
                      AttributeArguments@23..29
                        ParenthesisLeft@23..24 "("
                        Literal@24..28
                          True@24..28 "true"
                        ParenthesisRight@28..29 ")"
                  Blankspace@29..30 " "
                  CompoundStatement@30..48
                    BraceLeft@30..31 "{"
                    Blankspace@31..32 " "
                    CompoundStatement@32..46
                      BraceLeft@32..33 "{"
                      Blankspace@33..34 " "
                      VariableDeclaration@34..44
                        Var@34..37 "var"
                        Blankspace@37..38 " "
                        Name@38..39
                          Identifier@38..39 "x"
                        Blankspace@39..40 " "
                        Equal@40..41 "="
                        Blankspace@41..42 " "
                        Literal@42..43
                          IntLiteral@42..43 "0"
                        Semicolon@43..44 ";"
                      Blankspace@44..45 " "
                      BraceRight@45..46 "}"
                    Blankspace@46..47 " "
                    BraceRight@47..48 "}"
                  Blankspace@48..49 " "
                  BraceRight@49..50 "}"
              Blankspace@50..59 "\n        ""#]],
    );
}

#[test]
fn for_initializer_attribute() {
    check(
        "fn main() { for (@if(SLOW) var i = 0; false; i++) {} }",
        expect![[r#"
            SourceFile@0..54
              FunctionDeclaration@0..54
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..7
                  Identifier@3..7 "main"
                FunctionParameters@7..9
                  ParenthesisLeft@7..8 "("
                  ParenthesisRight@8..9 ")"
                Blankspace@9..10 " "
                CompoundStatement@10..54
                  BraceLeft@10..11 "{"
                  Blankspace@11..12 " "
                  ForStatement@12..52
                    For@12..15 "for"
                    Blankspace@15..16 " "
                    ParenthesisLeft@16..17 "("
                    ForInitializer@17..36
                      AttributeList@17..26
                        Attribute@17..26
                          AttributeOperator@17..18 "@"
                          Identifier@18..20 "if"
                          AttributeArguments@20..26
                            ParenthesisLeft@20..21 "("
                            IdentExpression@21..25
                              Path@21..25
                                Identifier@21..25 "SLOW"
                            ParenthesisRight@25..26 ")"
                      Blankspace@26..27 " "
                      VariableDeclaration@27..36
                        Var@27..30 "var"
                        Blankspace@30..31 " "
                        Name@31..32
                          Identifier@31..32 "i"
                        Blankspace@32..33 " "
                        Equal@33..34 "="
                        Blankspace@34..35 " "
                        Literal@35..36
                          IntLiteral@35..36 "0"
                    Semicolon@36..37 ";"
                    Blankspace@37..38 " "
                    ForCondition@38..43
                      Literal@38..43
                        False@38..43 "false"
                    Semicolon@43..44 ";"
                    Blankspace@44..45 " "
                    ForContinuingPart@45..48
                      IncrementDecrementStatement@45..48
                        IdentExpression@45..46
                          Path@45..46
                            Identifier@45..46 "i"
                        PlusPlus@46..48 "++"
                    ParenthesisRight@48..49 ")"
                    Blankspace@49..50 " "
                    CompoundStatement@50..52
                      BraceLeft@50..51 "{"
                      BraceRight@51..52 "}"
                  Blankspace@52..53 " "
                  BraceRight@53..54 "}""#]],
    );
}

#[test]
fn for_update_attribute() {
    check(
        "fn main() { for (var i = 0; false; @if(SLOW) i++) {} }",
        expect![[r#"
            SourceFile@0..54
              FunctionDeclaration@0..54
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..7
                  Identifier@3..7 "main"
                FunctionParameters@7..9
                  ParenthesisLeft@7..8 "("
                  ParenthesisRight@8..9 ")"
                Blankspace@9..10 " "
                CompoundStatement@10..54
                  BraceLeft@10..11 "{"
                  Blankspace@11..12 " "
                  ForStatement@12..52
                    For@12..15 "for"
                    Blankspace@15..16 " "
                    ParenthesisLeft@16..17 "("
                    ForInitializer@17..26
                      VariableDeclaration@17..26
                        Var@17..20 "var"
                        Blankspace@20..21 " "
                        Name@21..22
                          Identifier@21..22 "i"
                        Blankspace@22..23 " "
                        Equal@23..24 "="
                        Blankspace@24..25 " "
                        Literal@25..26
                          IntLiteral@25..26 "0"
                    Semicolon@26..27 ";"
                    Blankspace@27..28 " "
                    ForCondition@28..33
                      Literal@28..33
                        False@28..33 "false"
                    Semicolon@33..34 ";"
                    Blankspace@34..35 " "
                    ForContinuingPart@35..48
                      AttributeList@35..44
                        Attribute@35..44
                          AttributeOperator@35..36 "@"
                          Identifier@36..38 "if"
                          AttributeArguments@38..44
                            ParenthesisLeft@38..39 "("
                            IdentExpression@39..43
                              Path@39..43
                                Identifier@39..43 "SLOW"
                            ParenthesisRight@43..44 ")"
                      Blankspace@44..45 " "
                      IncrementDecrementStatement@45..48
                        IdentExpression@45..46
                          Path@45..46
                            Identifier@45..46 "i"
                        PlusPlus@46..48 "++"
                    ParenthesisRight@48..49 ")"
                    Blankspace@49..50 " "
                    CompoundStatement@50..52
                      BraceLeft@50..51 "{"
                      BraceRight@51..52 "}"
                  Blankspace@52..53 " "
                  BraceRight@53..54 "}""#]],
    );
}

#[test]
fn for_initializer_and_update_attributes() {
    check(
        "fn main() { for (@if(SLOW) var i = 0; false; @if(SLOW) i++) {} }",
        expect![[r#"
            SourceFile@0..64
              FunctionDeclaration@0..64
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..7
                  Identifier@3..7 "main"
                FunctionParameters@7..9
                  ParenthesisLeft@7..8 "("
                  ParenthesisRight@8..9 ")"
                Blankspace@9..10 " "
                CompoundStatement@10..64
                  BraceLeft@10..11 "{"
                  Blankspace@11..12 " "
                  ForStatement@12..62
                    For@12..15 "for"
                    Blankspace@15..16 " "
                    ParenthesisLeft@16..17 "("
                    ForInitializer@17..36
                      AttributeList@17..26
                        Attribute@17..26
                          AttributeOperator@17..18 "@"
                          Identifier@18..20 "if"
                          AttributeArguments@20..26
                            ParenthesisLeft@20..21 "("
                            IdentExpression@21..25
                              Path@21..25
                                Identifier@21..25 "SLOW"
                            ParenthesisRight@25..26 ")"
                      Blankspace@26..27 " "
                      VariableDeclaration@27..36
                        Var@27..30 "var"
                        Blankspace@30..31 " "
                        Name@31..32
                          Identifier@31..32 "i"
                        Blankspace@32..33 " "
                        Equal@33..34 "="
                        Blankspace@34..35 " "
                        Literal@35..36
                          IntLiteral@35..36 "0"
                    Semicolon@36..37 ";"
                    Blankspace@37..38 " "
                    ForCondition@38..43
                      Literal@38..43
                        False@38..43 "false"
                    Semicolon@43..44 ";"
                    Blankspace@44..45 " "
                    ForContinuingPart@45..58
                      AttributeList@45..54
                        Attribute@45..54
                          AttributeOperator@45..46 "@"
                          Identifier@46..48 "if"
                          AttributeArguments@48..54
                            ParenthesisLeft@48..49 "("
                            IdentExpression@49..53
                              Path@49..53
                                Identifier@49..53 "SLOW"
                            ParenthesisRight@53..54 ")"
                      Blankspace@54..55 " "
                      IncrementDecrementStatement@55..58
                        IdentExpression@55..56
                          Path@55..56
                            Identifier@55..56 "i"
                        PlusPlus@56..58 "++"
                    ParenthesisRight@58..59 ")"
                    Blankspace@59..60 " "
                    CompoundStatement@60..62
                      BraceLeft@60..61 "{"
                      BraceRight@61..62 "}"
                  Blankspace@62..63 " "
                  BraceRight@63..64 "}""#]],
    );
}

#[test]
fn for_without_attributes() {
    check(
        "fn main() { for (var i = 0; false; i++) {} }",
        expect![[r#"
            SourceFile@0..44
              FunctionDeclaration@0..44
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..7
                  Identifier@3..7 "main"
                FunctionParameters@7..9
                  ParenthesisLeft@7..8 "("
                  ParenthesisRight@8..9 ")"
                Blankspace@9..10 " "
                CompoundStatement@10..44
                  BraceLeft@10..11 "{"
                  Blankspace@11..12 " "
                  ForStatement@12..42
                    For@12..15 "for"
                    Blankspace@15..16 " "
                    ParenthesisLeft@16..17 "("
                    ForInitializer@17..26
                      VariableDeclaration@17..26
                        Var@17..20 "var"
                        Blankspace@20..21 " "
                        Name@21..22
                          Identifier@21..22 "i"
                        Blankspace@22..23 " "
                        Equal@23..24 "="
                        Blankspace@24..25 " "
                        Literal@25..26
                          IntLiteral@25..26 "0"
                    Semicolon@26..27 ";"
                    Blankspace@27..28 " "
                    ForCondition@28..33
                      Literal@28..33
                        False@28..33 "false"
                    Semicolon@33..34 ";"
                    Blankspace@34..35 " "
                    ForContinuingPart@35..38
                      IncrementDecrementStatement@35..38
                        IdentExpression@35..36
                          Path@35..36
                            Identifier@35..36 "i"
                        PlusPlus@36..38 "++"
                    ParenthesisRight@38..39 ")"
                    Blankspace@39..40 " "
                    CompoundStatement@40..42
                      BraceLeft@40..41 "{"
                      BraceRight@41..42 "}"
                  Blankspace@42..43 " "
                  BraceRight@43..44 "}""#]],
    );
}

#[test]
fn for_empty_initializer_and_update() {
    check(
        "fn main() { for (; false; ) {} }",
        expect![[r#"
            SourceFile@0..32
              FunctionDeclaration@0..32
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..7
                  Identifier@3..7 "main"
                FunctionParameters@7..9
                  ParenthesisLeft@7..8 "("
                  ParenthesisRight@8..9 ")"
                Blankspace@9..10 " "
                CompoundStatement@10..32
                  BraceLeft@10..11 "{"
                  Blankspace@11..12 " "
                  ForStatement@12..30
                    For@12..15 "for"
                    Blankspace@15..16 " "
                    ParenthesisLeft@16..17 "("
                    Semicolon@17..18 ";"
                    Blankspace@18..19 " "
                    ForCondition@19..24
                      Literal@19..24
                        False@19..24 "false"
                    Semicolon@24..25 ";"
                    Blankspace@25..26 " "
                    ParenthesisRight@26..27 ")"
                    Blankspace@27..28 " "
                    CompoundStatement@28..30
                      BraceLeft@28..29 "{"
                      BraceRight@29..30 "}"
                  Blankspace@30..31 " "
                  BraceRight@31..32 "}""#]],
    );
}
