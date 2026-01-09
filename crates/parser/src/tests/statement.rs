use expect_test::{Expect, expect};

use crate::ParseEntryPoint;

#[expect(clippy::needless_pass_by_value, reason = "intended API")]
fn check(
    input: &str,
    expected_tree: Expect,
) {
    crate::check_entrypoint(input, ParseEntryPoint::Statement, &expected_tree);
}

#[test]
fn function_call_statement() {
    //TODO This is currently producing a parser error
    check(
        "my_function(1);",
        expect![[r#"
            SourceFile@0..15
              FunctionCallStatement@0..15
                FunctionCall@0..14
                  IdentExpression@0..11
                    NameReference@0..11
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
    //TODO This is currently producing a parser error but should be allowed by wgsl
    //https://www.w3.org/TR/WGSL/#recursive-descent-syntax-statement
    check("my_template_elaborated_function<f32>(1);", expect![[]]);
}
