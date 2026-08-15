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
                IfAttribute@9..18
                  AttributeOperator@9..10 "@"
                  If@10..12 "if"
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
                    IfAttribute@20..29
                      AttributeOperator@20..21 "@"
                      If@21..23 "if"
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
