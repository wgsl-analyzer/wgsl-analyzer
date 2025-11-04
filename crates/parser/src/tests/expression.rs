use expect_test::{Expect, expect};

use crate::ParseEntryPoint;

#[expect(clippy::needless_pass_by_value, reason = "intended API")]
fn check(
    input: &str,
    expected_tree: Expect,
) {
    crate::check_entrypoint(input, ParseEntryPoint::Expression, &expected_tree);
}

#[test]
fn parse_number() {
    check(
        "123",
        expect![[r#"
                SourceFile@0..3
                  Literal@0..3
                    IntLiteral@0..3 "123""#]],
    );
}

#[test]
fn parse_number_preceded_by_whitespace() {
    check(
        "   9876",
        expect![[r#"
                SourceFile@0..7
                  Blankspace@0..3 "   "
                  Literal@3..7
                    IntLiteral@3..7 "9876""#]],
    );
}

#[test]
fn parse_number_followed_by_whitespace() {
    check(
        "999   ",
        expect![[r#"
                SourceFile@0..6
                  Literal@0..3
                    IntLiteral@0..3 "999"
                  Blankspace@3..6 "   ""#]],
    );
}

#[test]
fn parse_number_surrounded_by_whitespace() {
    check(
        " 123     ",
        expect![[r#"
                SourceFile@0..9
                  Blankspace@0..1 " "
                  Literal@1..4
                    IntLiteral@1..4 "123"
                  Blankspace@4..9 "     ""#]],
    );
}

#[test]
fn parse_variable_ref() {
    check(
        "counter",
        expect![[r#"
            SourceFile@0..7
              IdentExpression@0..7
                NameReference@0..7
                  Identifier@0..7 "counter""#]],
    );
}

#[test]
fn parse_variable_ref_no_comment() {
    check(
        "counter // not part of it",
        expect![[r#"
            SourceFile@0..25
              IdentExpression@0..7
                NameReference@0..7
                  Identifier@0..7 "counter"
              Blankspace@7..8 " "
              LineEndingComment@8..25 "// not part of it""#]],
    );
}

#[test]
fn parse_variable_ref_no_comment2() {
    check(
        "counter /* not part of it */",
        expect![[r#"
            SourceFile@0..28
              IdentExpression@0..7
                NameReference@0..7
                  Identifier@0..7 "counter"
              Blankspace@7..8 " "
              BlockComment@8..28 "/* not part of it */""#]],
    );
}

#[test]
fn parse_simple_infix_expression() {
    check(
        "1+2",
        expect![[r#"
                SourceFile@0..3
                  InfixExpression@0..3
                    Literal@0..1
                      IntLiteral@0..1 "1"
                    Plus@1..2 "+"
                    Literal@2..3
                      IntLiteral@2..3 "2""#]],
    );
}

#[test]
fn parse_left_associative_infix_expression() {
    check(
        "1+2+3+4",
        expect![[r#"
                SourceFile@0..7
                  InfixExpression@0..7
                    InfixExpression@0..5
                      InfixExpression@0..3
                        Literal@0..1
                          IntLiteral@0..1 "1"
                        Plus@1..2 "+"
                        Literal@2..3
                          IntLiteral@2..3 "2"
                      Plus@3..4 "+"
                      Literal@4..5
                        IntLiteral@4..5 "3"
                    Plus@5..6 "+"
                    Literal@6..7
                      IntLiteral@6..7 "4""#]],
    );
}

#[test]
fn parse_infix_expression_with_mixed_binding_power() {
    // See https://www.w3.org/TR/WGSL/#operator-precedence-associativity
    // "a left-to-right association will infer ((a + b) + c) from a + b + c expression"
    check(
        "1+2*3-4",
        expect![[r#"
                SourceFile@0..7
                  InfixExpression@0..7
                    InfixExpression@0..5
                      Literal@0..1
                        IntLiteral@0..1 "1"
                      Plus@1..2 "+"
                      InfixExpression@2..5
                        Literal@2..3
                          IntLiteral@2..3 "2"
                        Star@3..4 "*"
                        Literal@4..5
                          IntLiteral@4..5 "3"
                    Minus@5..6 "-"
                    Literal@6..7
                      IntLiteral@6..7 "4""#]],
    );
}

#[test]
fn parse_infix_expression_with_whitespace() {
    check(
        " 1 +   2* 3 ",
        expect![[r#"
                SourceFile@0..12
                  Blankspace@0..1 " "
                  InfixExpression@1..11
                    Literal@1..2
                      IntLiteral@1..2 "1"
                    Blankspace@2..3 " "
                    Plus@3..4 "+"
                    Blankspace@4..7 "   "
                    InfixExpression@7..11
                      Literal@7..8
                        IntLiteral@7..8 "2"
                      Star@8..9 "*"
                      Blankspace@9..10 " "
                      Literal@10..11
                        IntLiteral@10..11 "3"
                  Blankspace@11..12 " ""#]],
    );
}

#[test]
fn do_not_parse_operator_if_getting_rhs_failed() {
    check(
        "(1+",
        expect![[r#"
                SourceFile@0..3
                  ParenthesisExpression@0..3
                    ParenthesisLeft@0..1 "("
                    InfixExpression@1..3
                      Literal@1..2
                        IntLiteral@1..2 "1"
                      Plus@2..3 "+"

                error at 3..3: invalid syntax, expected one of: '&', '!', 'false', <floating point literal>, <identifier>, <integer literal>, '(', '-', '*', '~', 'true'"#]],
    );
}

#[test]
fn parse_negation() {
    check(
        "-10",
        expect![[r#"
                SourceFile@0..3
                  PrefixExpression@0..3
                    Minus@0..1 "-"
                    Literal@1..3
                      IntLiteral@1..3 "10""#]],
    );
}

#[test]
fn negation_has_higher_binding_power_than_binary_operators() {
    check(
        "-20+20",
        expect![[r#"
                SourceFile@0..6
                  InfixExpression@0..6
                    PrefixExpression@0..3
                      Minus@0..1 "-"
                      Literal@1..3
                        IntLiteral@1..3 "20"
                    Plus@3..4 "+"
                    Literal@4..6
                      IntLiteral@4..6 "20""#]],
    );
}

#[test]
fn parse_nested_parentheses() {
    check(
        "((((((10))))))",
        expect![[r#"
                SourceFile@0..14
                  ParenthesisExpression@0..14
                    ParenthesisLeft@0..1 "("
                    ParenthesisExpression@1..13
                      ParenthesisLeft@1..2 "("
                      ParenthesisExpression@2..12
                        ParenthesisLeft@2..3 "("
                        ParenthesisExpression@3..11
                          ParenthesisLeft@3..4 "("
                          ParenthesisExpression@4..10
                            ParenthesisLeft@4..5 "("
                            ParenthesisExpression@5..9
                              ParenthesisLeft@5..6 "("
                              Literal@6..8
                                IntLiteral@6..8 "10"
                              ParenthesisRight@8..9 ")"
                            ParenthesisRight@9..10 ")"
                          ParenthesisRight@10..11 ")"
                        ParenthesisRight@11..12 ")"
                      ParenthesisRight@12..13 ")"
                    ParenthesisRight@13..14 ")""#]],
    );
}

#[test]
fn parentheses_affect_precedence() {
    check(
        "5*(2+1)",
        expect![[r#"
                SourceFile@0..7
                  InfixExpression@0..7
                    Literal@0..1
                      IntLiteral@0..1 "5"
                    Star@1..2 "*"
                    ParenthesisExpression@2..7
                      ParenthesisLeft@2..3 "("
                      InfixExpression@3..6
                        Literal@3..4
                          IntLiteral@3..4 "2"
                        Plus@4..5 "+"
                        Literal@5..6
                          IntLiteral@5..6 "1"
                      ParenthesisRight@6..7 ")""#]],
    );
}

#[test]
fn parse_unclosed_parentheses() {
    check(
        "(foo",
        expect![[r#"
            SourceFile@0..4
              ParenthesisExpression@0..4
                ParenthesisLeft@0..1 "("
                IdentExpression@1..4
                  NameReference@1..4
                    Identifier@1..4 "foo"

            error at 4..4: invalid syntax, expected: ')'"#]],
    );
}

#[test]
fn parse_expression_complex() {
    check(
        "1 + 2 == 3 || 4 < 5 / 2 == 0",
        expect![[r#"
                SourceFile@0..28
                  InfixExpression@0..28
                    InfixExpression@0..10
                      InfixExpression@0..5
                        Literal@0..1
                          IntLiteral@0..1 "1"
                        Blankspace@1..2 " "
                        Plus@2..3 "+"
                        Blankspace@3..4 " "
                        Literal@4..5
                          IntLiteral@4..5 "2"
                      Blankspace@5..6 " "
                      EqualEqual@6..8 "=="
                      Blankspace@8..9 " "
                      Literal@9..10
                        IntLiteral@9..10 "3"
                    Blankspace@10..11 " "
                    OrOr@11..13 "||"
                    Blankspace@13..14 " "
                    InfixExpression@14..28
                      InfixExpression@14..23
                        Literal@14..15
                          IntLiteral@14..15 "4"
                        Blankspace@15..16 " "
                        LessThan@16..17 "<"
                        Blankspace@17..18 " "
                        InfixExpression@18..23
                          Literal@18..19
                            IntLiteral@18..19 "5"
                          Blankspace@19..20 " "
                          ForwardSlash@20..21 "/"
                          Blankspace@21..22 " "
                          Literal@22..23
                            IntLiteral@22..23 "2"
                      Blankspace@23..24 " "
                      EqualEqual@24..26 "=="
                      Blankspace@26..27 " "
                      Literal@27..28
                        IntLiteral@27..28 "0""#]],
    );
}

#[test]
fn parse_expression_field() {
    check(
        "a.b.c",
        expect![[r#"
            SourceFile@0..5
              FieldExpression@0..5
                FieldExpression@0..3
                  IdentExpression@0..1
                    NameReference@0..1
                      Identifier@0..1 "a"
                  Period@1..2 "."
                  Identifier@2..3 "b"
                Period@3..4 "."
                Identifier@4..5 "c""#]],
    );
}

#[test]
fn parse_expression_field_mix_ops() {
    check(
        "vec.xy + 2 * other.zw",
        expect![[r#"
            SourceFile@0..21
              InfixExpression@0..21
                FieldExpression@0..6
                  IdentExpression@0..3
                    NameReference@0..3
                      Identifier@0..3 "vec"
                  Period@3..4 "."
                  Identifier@4..6 "xy"
                Blankspace@6..7 " "
                Plus@7..8 "+"
                Blankspace@8..9 " "
                InfixExpression@9..21
                  Literal@9..10
                    IntLiteral@9..10 "2"
                  Blankspace@10..11 " "
                  Star@11..12 "*"
                  Blankspace@12..13 " "
                  FieldExpression@13..21
                    IdentExpression@13..18
                      NameReference@13..18
                        Identifier@13..18 "other"
                    Period@18..19 "."
                    Identifier@19..21 "zw""#]],
    );
}

#[test]
fn parse_expression_function_call() {
    check(
        "pow(2, 3)",
        expect![[r#"
            SourceFile@0..9
              FunctionCall@0..9
                IdentExpression@0..3
                  NameReference@0..3
                    Identifier@0..3 "pow"
                Arguments@3..9
                  ParenthesisLeft@3..4 "("
                  Literal@4..5
                    IntLiteral@4..5 "2"
                  Comma@5..6 ","
                  Blankspace@6..7 " "
                  Literal@7..8
                    IntLiteral@7..8 "3"
                  ParenthesisRight@8..9 ")""#]],
    );
}

#[test]
fn parse_expression_function_call_mixed() {
    check(
        "pow(srgb + 14.0, 3.0) * 2.0",
        expect![[r#"
            SourceFile@0..27
              InfixExpression@0..27
                FunctionCall@0..21
                  IdentExpression@0..3
                    NameReference@0..3
                      Identifier@0..3 "pow"
                  Arguments@3..21
                    ParenthesisLeft@3..4 "("
                    InfixExpression@4..15
                      IdentExpression@4..8
                        NameReference@4..8
                          Identifier@4..8 "srgb"
                      Blankspace@8..9 " "
                      Plus@9..10 "+"
                      Blankspace@10..11 " "
                      Literal@11..15
                        FloatLiteral@11..15 "14.0"
                    Comma@15..16 ","
                    Blankspace@16..17 " "
                    Literal@17..20
                      FloatLiteral@17..20 "3.0"
                    ParenthesisRight@20..21 ")"
                Blankspace@21..22 " "
                Star@22..23 "*"
                Blankspace@23..24 " "
                Literal@24..27
                  FloatLiteral@24..27 "2.0""#]],
    );
}

#[test]
fn parse_vec3_initializer() {
    check(
        "vec3<f32>(1.0)",
        expect![[r#"
            SourceFile@0..14
              FunctionCall@0..14
                IdentExpression@0..9
                  NameReference@0..4
                    Identifier@0..4 "vec3"
                  TemplateList@4..9
                    TemplateStart@4..5 "<"
                    IdentExpression@5..8
                      NameReference@5..8
                        Identifier@5..8 "f32"
                    TemplateEnd@8..9 ">"
                Arguments@9..14
                  ParenthesisLeft@9..10 "("
                  Literal@10..13
                    FloatLiteral@10..13 "1.0"
                  ParenthesisRight@13..14 ")""#]],
    );
}

#[test]
fn parse_vec3_initializer_inferred() {
    check(
        "vec3(1.0)",
        expect![[r#"
            SourceFile@0..9
              FunctionCall@0..9
                IdentExpression@0..4
                  NameReference@0..4
                    Identifier@0..4 "vec3"
                Arguments@4..9
                  ParenthesisLeft@4..5 "("
                  Literal@5..8
                    FloatLiteral@5..8 "1.0"
                  ParenthesisRight@8..9 ")""#]],
    );
}

#[test]
fn parse_bool_literal() {
    check(
        "true",
        expect![[r#"
                SourceFile@0..4
                  Literal@0..4
                    True@0..4 "true""#]],
    );
    check(
        "false",
        expect![[r#"
                SourceFile@0..5
                  Literal@0..5
                    False@0..5 "false""#]],
    );
}

#[test]
fn parse_decimal_float_literal() {
    check(
        "0.e+4f",
        expect![[r#"
                SourceFile@0..6
                  Literal@0..6
                    FloatLiteral@0..6 "0.e+4f""#]],
    );
    check(
        "01.",
        expect![[r#"
                SourceFile@0..3
                  Literal@0..3
                    FloatLiteral@0..3 "01.""#]],
    );
    check(
        ".01",
        expect![[r#"
                SourceFile@0..3
                  Literal@0..3
                    FloatLiteral@0..3 ".01""#]],
    );
    check(
        "12.34",
        expect![[r#"
                SourceFile@0..5
                  Literal@0..5
                    FloatLiteral@0..5 "12.34""#]],
    );
    check(
        ".0f",
        expect![[r#"
                SourceFile@0..3
                  Literal@0..3
                    FloatLiteral@0..3 ".0f""#]],
    );
    check(
        "0h",
        expect![[r#"
                SourceFile@0..2
                  Literal@0..2
                    FloatLiteral@0..2 "0h""#]],
    );
    check(
        "1e-3",
        expect![[r#"
                SourceFile@0..4
                  Literal@0..4
                    FloatLiteral@0..4 "1e-3""#]],
    );
}

#[test]
fn parse_hex_int_literal() {
    check(
        "0x123",
        expect![[r#"
                SourceFile@0..5
                  Literal@0..5
                    IntLiteral@0..5 "0x123""#]],
    );
    check(
        "0X123u",
        expect![[r#"
                SourceFile@0..6
                  Literal@0..6
                    IntLiteral@0..6 "0X123u""#]],
    );
    check(
        "0x3f",
        expect![[r#"
                SourceFile@0..4
                  Literal@0..4
                    IntLiteral@0..4 "0x3f""#]],
    );
}

#[test]
fn parse_hex_float_literal() {
    check(
        "0xa.fp+2",
        expect![[r#"
                SourceFile@0..8
                  Literal@0..8
                    FloatLiteral@0..8 "0xa.fp+2""#]],
    );
    check(
        "0x1P+4f",
        expect![[r#"
                SourceFile@0..7
                  Literal@0..7
                    FloatLiteral@0..7 "0x1P+4f""#]],
    );
    check(
        "0X.3",
        expect![[r#"
                SourceFile@0..4
                  Literal@0..4
                    FloatLiteral@0..4 "0X.3""#]],
    );
    check(
        "0x3p+2h",
        expect![[r#"
                SourceFile@0..7
                  Literal@0..7
                    FloatLiteral@0..7 "0x3p+2h""#]],
    );
    check(
        "0X1.fp-4",
        expect![[r#"
                SourceFile@0..8
                  Literal@0..8
                    FloatLiteral@0..8 "0X1.fp-4""#]],
    );
    check(
        "0x3.2p+2h",
        expect![[r#"
                SourceFile@0..9
                  Literal@0..9
                    FloatLiteral@0..9 "0x3.2p+2h""#]],
    );
}

#[test]
fn parse_prefix_expression() {
    check(
        "- 3 + 3",
        expect![[r#"
                SourceFile@0..7
                  InfixExpression@0..7
                    PrefixExpression@0..3
                      Minus@0..1 "-"
                      Blankspace@1..2 " "
                      Literal@2..3
                        IntLiteral@2..3 "3"
                    Blankspace@3..4 " "
                    Plus@4..5 "+"
                    Blankspace@5..6 " "
                    Literal@6..7
                      IntLiteral@6..7 "3""#]],
    );
}

#[test]
fn parse_index() {
    check(
        "a.b[3+2]",
        expect![[r#"
            SourceFile@0..8
              IndexExpression@0..8
                FieldExpression@0..3
                  IdentExpression@0..1
                    NameReference@0..1
                      Identifier@0..1 "a"
                  Period@1..2 "."
                  Identifier@2..3 "b"
                BracketLeft@3..4 "["
                InfixExpression@4..7
                  Literal@4..5
                    IntLiteral@4..5 "3"
                  Plus@5..6 "+"
                  Literal@6..7
                    IntLiteral@6..7 "2"
                BracketRight@7..8 "]""#]],
    );
}

#[test]
fn parse_modulo_comparison() {
    check(
        "n % 2u == 0u",
        expect![[r#"
            SourceFile@0..12
              InfixExpression@0..12
                InfixExpression@0..6
                  IdentExpression@0..1
                    NameReference@0..1
                      Identifier@0..1 "n"
                  Blankspace@1..2 " "
                  Modulo@2..3 "%"
                  Blankspace@3..4 " "
                  Literal@4..6
                    IntLiteral@4..6 "2u"
                Blankspace@6..7 " "
                EqualEqual@7..9 "=="
                Blankspace@9..10 " "
                Literal@10..12
                  IntLiteral@10..12 "0u""#]],
    );
}

#[test]
fn prefix_expressions() {
    check(
        "!~*&foo",
        expect![[r#"
            SourceFile@0..7
              PrefixExpression@0..7
                Bang@0..1 "!"
                PrefixExpression@1..7
                  Tilde@1..2 "~"
                  PrefixExpression@2..7
                    Star@2..3 "*"
                    PrefixExpression@3..7
                      And@3..4 "&"
                      IdentExpression@4..7
                        NameReference@4..7
                          Identifier@4..7 "foo""#]],
    );
}

#[test]
fn bitcast() {
    check(
        "bitcast<u32>(x)",
        expect![[r#"
            SourceFile@0..15
              FunctionCall@0..15
                IdentExpression@0..12
                  NameReference@0..7
                    Identifier@0..7 "bitcast"
                  TemplateList@7..12
                    TemplateStart@7..8 "<"
                    IdentExpression@8..11
                      NameReference@8..11
                        Identifier@8..11 "u32"
                    TemplateEnd@11..12 ">"
                Arguments@12..15
                  ParenthesisLeft@12..13 "("
                  IdentExpression@13..14
                    NameReference@13..14
                      Identifier@13..14 "x"
                  ParenthesisRight@14..15 ")""#]],
    );
}

#[test]
fn bitcast_vector() {
    check(
        "bitcast<vec4<u32>>(x)",
        expect![[r#"
            SourceFile@0..21
              FunctionCall@0..21
                IdentExpression@0..18
                  NameReference@0..7
                    Identifier@0..7 "bitcast"
                  TemplateList@7..18
                    TemplateStart@7..8 "<"
                    IdentExpression@8..17
                      NameReference@8..12
                        Identifier@8..12 "vec4"
                      TemplateList@12..17
                        TemplateStart@12..13 "<"
                        IdentExpression@13..16
                          NameReference@13..16
                            Identifier@13..16 "u32"
                        TemplateEnd@16..17 ">"
                    TemplateEnd@17..18 ">"
                Arguments@18..21
                  ParenthesisLeft@18..19 "("
                  IdentExpression@19..20
                    NameReference@19..20
                      Identifier@19..20 "x"
                  ParenthesisRight@20..21 ")""#]],
    );
}

#[test]
fn bitcast_no_template() {
    check(
        "bitcast(x)",
        expect![[r#"
            SourceFile@0..10
              FunctionCall@0..10
                IdentExpression@0..7
                  NameReference@0..7
                    Identifier@0..7 "bitcast"
                Arguments@7..10
                  ParenthesisLeft@7..8 "("
                  IdentExpression@8..9
                    NameReference@8..9
                      Identifier@8..9 "x"
                  ParenthesisRight@9..10 ")""#]],
    );
}
#[test]
fn bitcast_in_expression() {
    check(
        "1 + -bitcast<u32>(x) + 1",
        expect![[r#"
            SourceFile@0..24
              InfixExpression@0..24
                InfixExpression@0..20
                  Literal@0..1
                    IntLiteral@0..1 "1"
                  Blankspace@1..2 " "
                  Plus@2..3 "+"
                  Blankspace@3..4 " "
                  PrefixExpression@4..20
                    Minus@4..5 "-"
                    FunctionCall@5..20
                      IdentExpression@5..17
                        NameReference@5..12
                          Identifier@5..12 "bitcast"
                        TemplateList@12..17
                          TemplateStart@12..13 "<"
                          IdentExpression@13..16
                            NameReference@13..16
                              Identifier@13..16 "u32"
                          TemplateEnd@16..17 ">"
                      Arguments@17..20
                        ParenthesisLeft@17..18 "("
                        IdentExpression@18..19
                          NameReference@18..19
                            Identifier@18..19 "x"
                        ParenthesisRight@19..20 ")"
                Blankspace@20..21 " "
                Plus@21..22 "+"
                Blankspace@22..23 " "
                Literal@23..24
                  IntLiteral@23..24 "1""#]],
    );
}

#[test]
fn deref_field() {
    check(
        "*a.b",
        expect![[r#"
            SourceFile@0..4
              PrefixExpression@0..4
                Star@0..1 "*"
                FieldExpression@1..4
                  IdentExpression@1..2
                    NameReference@1..2
                      Identifier@1..2 "a"
                  Period@2..3 "."
                  Identifier@3..4 "b""#]],
    );
}
#[test]
fn deref_field_paren() {
    check(
        "(*a).b",
        expect![[r#"
            SourceFile@0..6
              FieldExpression@0..6
                ParenthesisExpression@0..4
                  ParenthesisLeft@0..1 "("
                  PrefixExpression@1..3
                    Star@1..2 "*"
                    IdentExpression@2..3
                      NameReference@2..3
                        Identifier@2..3 "a"
                  ParenthesisRight@3..4 ")"
                Period@4..5 "."
                Identifier@5..6 "b""#]],
    );
}

#[test]
fn shift_right() {
    check(
        "2 >> 3",
        expect![[r#"
            SourceFile@0..6
              InfixExpression@0..6
                Literal@0..1
                  IntLiteral@0..1 "2"
                Blankspace@1..2 " "
                ShiftRight@2..4 ">>"
                Blankspace@4..5 " "
                Literal@5..6
                  IntLiteral@5..6 "3""#]],
    );
}

#[test]
fn shift_multiple() {
    check(
        "2 >> 3 + 2 << 4",
        expect![[r#"
            SourceFile@0..15
              InfixExpression@0..15
                InfixExpression@0..10
                  Literal@0..1
                    IntLiteral@0..1 "2"
                  Blankspace@1..2 " "
                  ShiftRight@2..4 ">>"
                  Blankspace@4..5 " "
                  InfixExpression@5..10
                    Literal@5..6
                      IntLiteral@5..6 "3"
                    Blankspace@6..7 " "
                    Plus@7..8 "+"
                    Blankspace@8..9 " "
                    Literal@9..10
                      IntLiteral@9..10 "2"
                Blankspace@10..11 " "
                ShiftLeft@11..13 "<<"
                Blankspace@13..14 " "
                Literal@14..15
                  IntLiteral@14..15 "4""#]],
    );
}
