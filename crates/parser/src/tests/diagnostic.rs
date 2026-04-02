use expect_test::expect;

use crate::tests::check;

#[test]
fn parse_diagnostic_attribute() {
    check(
        "
        @diagnostic(off, bla)
        fn a() {}
        ",
        expect![[r#"
            SourceFile@0..57
              Blankspace@0..9 "\n        "
              FunctionDeclaration@9..48
                DiagnosticAttribute@9..30
                  AttributeOperator@9..10 "@"
                  Diagnostic@10..20 "diagnostic"
                  DiagnosticControl@20..30
                    ParenthesisLeft@20..21 "("
                    SeverityControlName@21..24
                      Identifier@21..24 "off"
                    Comma@24..25 ","
                    Blankspace@25..26 " "
                    DiagnosticRuleName@26..29
                      Identifier@26..29 "bla"
                    ParenthesisRight@29..30 ")"
                Blankspace@30..39 "\n        "
                Fn@39..41 "fn"
                Blankspace@41..42 " "
                Name@42..43
                  Identifier@42..43 "a"
                FunctionParameters@43..45
                  ParenthesisLeft@43..44 "("
                  ParenthesisRight@44..45 ")"
                Blankspace@45..46 " "
                CompoundStatement@46..48
                  BraceLeft@46..47 "{"
                  BraceRight@47..48 "}"
              Blankspace@48..57 "\n        ""#]],
    );
}

#[test]
fn parse_diagnostic_directive() {
    check(
        "
        diagnostic(off, bla);
        ",
        expect![[r#"
            SourceFile@0..39
              Blankspace@0..9 "\n        "
              DiagnosticDirective@9..30
                Diagnostic@9..19 "diagnostic"
                DiagnosticControl@19..29
                  ParenthesisLeft@19..20 "("
                  SeverityControlName@20..23
                    Identifier@20..23 "off"
                  Comma@23..24 ","
                  Blankspace@24..25 " "
                  DiagnosticRuleName@25..28
                    Identifier@25..28 "bla"
                  ParenthesisRight@28..29 ")"
                Semicolon@29..30 ";"
              Blankspace@30..39 "\n        ""#]],
    );
}
