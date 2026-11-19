use expect_test::{Expect, expect};

use crate::ParseEntryPoint;

#[expect(clippy::needless_pass_by_value, reason = "intended API")]
fn check(
    input: &str,
    expected_tree: Expect,
) {
    crate::check_entrypoint(
        input,
        ParseEntryPoint::Statement,
        &expected_tree,
        edition::Edition::LATEST,
    );
}

#[test]
fn function_call_statement() {
    check(
        "my_function(1);",
        expect![[r#"
            SourceFile@0..15
              FunctionCallStatement@0..15
                FunctionCall@0..14
                  IdentExpression@0..11
                    Path@0..11
                      Identifier@0..11 "my_function"
                  Arguments@11..14
                    ParenthesisLeft@11..12 "("
                    Literal@12..13
                      IntLiteral@12..13 "1"
                    ParenthesisRight@13..14 ")"
                Semicolon@14..15 ";""#]],
    );
}

#[test]
fn template_elaborated_function_call_statement() {
    check(
        "my_template_elaborated_function<f32>(1);",
        expect![[r#"
        SourceFile@0..40
          FunctionCallStatement@0..40
            FunctionCall@0..39
              IdentExpression@0..36
                Path@0..31
                  Identifier@0..31 "my_template_elaborate ..."
                TemplateList@31..36
                  TemplateStart@31..32 "<"
                  IdentExpression@32..35
                    Path@32..35
                      Identifier@32..35 "f32"
                  TemplateEnd@35..36 ">"
              Arguments@36..39
                ParenthesisLeft@36..37 "("
                Literal@37..38
                  IntLiteral@37..38 "1"
                ParenthesisRight@38..39 ")"
            Semicolon@39..40 ";""#]],
    );
}

#[test]
fn for_statement_with_function_call() {
    check(
        "for (f1(); false; f1()) {   }",
        expect![[r#"
            SourceFile@0..29
              ForStatement@0..29
                For@0..3 "for"
                Blankspace@3..4 " "
                ParenthesisLeft@4..5 "("
                ForInitializer@5..9
                  FunctionCallStatement@5..9
                    FunctionCall@5..9
                      IdentExpression@5..7
                        Path@5..7
                          Identifier@5..7 "f1"
                      Arguments@7..9
                        ParenthesisLeft@7..8 "("
                        ParenthesisRight@8..9 ")"
                Semicolon@9..10 ";"
                Blankspace@10..11 " "
                ForCondition@11..16
                  Literal@11..16
                    False@11..16 "false"
                Semicolon@16..17 ";"
                Blankspace@17..18 " "
                ForContinuingPart@18..22
                  FunctionCallStatement@18..22
                    FunctionCall@18..22
                      IdentExpression@18..20
                        Path@18..20
                          Identifier@18..20 "f1"
                      Arguments@20..22
                        ParenthesisLeft@20..21 "("
                        ParenthesisRight@21..22 ")"
                ParenthesisRight@22..23 ")"
                Blankspace@23..24 " "
                CompoundStatement@24..29
                  BraceLeft@24..25 "{"
                  Blankspace@25..28 "   "
                  BraceRight@28..29 "}""#]],
    );
}
