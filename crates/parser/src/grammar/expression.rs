// mostly copied from https://github.com/arzg/eldiro/blob/f3d588f8a76e2e4317c1d77ae2758b0781bb5af3/crates/parser/src/grammar.rs

use crate::{marker::CompletedMarker, syntax_kind::SyntaxKind};

use super::{Parser, TYPE_SET, list, name_ref};

pub(crate) fn expression(parser: &mut Parser<'_, '_>) {
    if parser.at_set(super::STATEMENT_RECOVER_SET) {
        return;
    }
    expression_binding_power(parser, 0);
}

fn expression_binding_power(
    parser: &mut Parser<'_, '_>,
    minimum_binding_power: u8,
) -> Option<CompletedMarker> {
    let mut left_side = left_side(parser)?;

    loop {
        // postfix ops
        if let Some(postfix_op) = postfix_op(parser) {
            let (left_binding_power, ()) = postfix_op.binding_power();
            if left_binding_power < minimum_binding_power {
                break;
            }

            let marker = left_side.precede(parser);
            match postfix_op {
                PostfixOp::Call => {
                    // Calls cannot be made on arbitrary expressions, merely on only a few versions
                    // We have this as an error
                    function_param_list(parser);
                    left_side = marker.complete(parser, SyntaxKind::InvalidFunctionCall);
                },
                PostfixOp::Index => {
                    array_index(parser);
                    left_side = marker.complete(parser, SyntaxKind::IndexExpression);
                },
                PostfixOp::Field => {
                    parser.bump();
                    name_ref(parser);
                    left_side = marker.complete(parser, SyntaxKind::FieldExpression);
                },
            }

            continue;
        }

        let Some(infix_operator) = binary_operator(parser) else {
            break;
        };

        let (left_binding_power, right_binding_power) = infix_operator.binding_power();

        if left_binding_power < minimum_binding_power {
            break;
        }

        // Eat the operator's token.

        match infix_operator {
            BinaryOperator::ShiftLeft => parser.bump_compound(SyntaxKind::ShiftLeft),
            BinaryOperator::ShiftRight => parser.bump_compound(SyntaxKind::ShiftRight),
            BinaryOperator::Add
            | BinaryOperator::Subtract
            | BinaryOperator::Multiply
            | BinaryOperator::Divide
            | BinaryOperator::Or
            | BinaryOperator::And
            | BinaryOperator::Xor
            | BinaryOperator::ShortCircuitAnd
            | BinaryOperator::ShortCircuitOr
            | BinaryOperator::GreaterThan
            | BinaryOperator::LessThan
            | BinaryOperator::GreaterThanEqual
            | BinaryOperator::LessThanEqual
            | BinaryOperator::NotEqual
            | BinaryOperator::Equals
            | BinaryOperator::Modulo => {
                parser.bump();
            },
        }

        let marker = left_side.precede(parser);
        let parsed_rhs = expression_binding_power(parser, right_binding_power).is_some();
        left_side = marker.complete(parser, SyntaxKind::InfixExpression);

        if !parsed_rhs {
            break;
        }
    }

    Some(left_side)
}

fn function_param_list(parser: &mut Parser<'_, '_>) {
    list(
        parser,
        SyntaxKind::ParenthesisLeft,
        SyntaxKind::ParenthesisRight,
        SyntaxKind::Comma,
        SyntaxKind::FunctionParameterList,
        |parser| {
            expression_binding_power(parser, 0);
        },
    );
}

fn array_index(parser: &mut Parser<'_, '_>) {
    parser.expect(SyntaxKind::BracketLeft);
    expression_binding_power(parser, 0);
    parser.expect(SyntaxKind::BracketRight);
}

fn left_side(parser: &mut Parser<'_, '_>) -> Option<CompletedMarker> {
    let cm = if parser.at_set(TOKENSET_LITERAL) {
        literal(parser)
    } else if parser.at(SyntaxKind::Identifier) {
        let marker = parser.start();
        name_ref(parser);
        if parser.at(SyntaxKind::ParenthesisLeft) {
            function_param_list(parser);
            // Function call, may be a type initialiser too
            marker.complete(parser, SyntaxKind::FunctionCall)
        } else {
            marker.complete(parser, SyntaxKind::PathExpression)
        }
    } else if parser.at(SyntaxKind::Bitcast) {
        bitcast_expression(parser)
    } else if parser.at_set(TYPE_SET) {
        let marker = parser.start();
        super::type_declaration(parser).unwrap();
        if parser.at(SyntaxKind::ParenthesisLeft) {
            function_param_list(parser);
        } else {
            parser.error_no_bump(&[SyntaxKind::ParenthesisLeft]);
        }
        marker.complete(parser, SyntaxKind::TypeInitializer)
    } else if parser.at_set(PREFIX_OP_SET) {
        prefix_expression(parser)
    } else if parser.at(SyntaxKind::ParenthesisLeft) {
        parenthesis_expression(parser)
    } else {
        parser.error();
        return None;
    };

    Some(cm)
}

enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Or,
    And,
    Xor,
    ShortCircuitAnd,
    ShortCircuitOr,
    ShiftRight,
    ShiftLeft,
    GreaterThan,
    LessThan,
    GreaterThanEqual,
    LessThanEqual,
    NotEqual,
    Equals,
    Modulo,
}

fn binary_operator(parser: &mut Parser<'_, '_>) -> Option<BinaryOperator> {
    let operator = if parser.at(SyntaxKind::Plus) {
        Some(BinaryOperator::Add)
    } else if parser.at(SyntaxKind::Minus) {
        Some(BinaryOperator::Subtract)
    } else if parser.at(SyntaxKind::Star) {
        Some(BinaryOperator::Multiply)
    } else if parser.at(SyntaxKind::ForwardSlash) {
        Some(BinaryOperator::Divide)
    } else if parser.at(SyntaxKind::Or) {
        Some(BinaryOperator::Or)
    } else if parser.at(SyntaxKind::And) {
        Some(BinaryOperator::And)
    } else if parser.at(SyntaxKind::OrOr) {
        Some(BinaryOperator::ShortCircuitOr)
    } else if parser.at(SyntaxKind::AndAnd) {
        Some(BinaryOperator::ShortCircuitAnd)
    } else if parser.at(SyntaxKind::Xor) {
        Some(BinaryOperator::Xor)
    } else if parser.at_compound(SyntaxKind::LessThan, SyntaxKind::LessThan) {
        Some(BinaryOperator::ShiftLeft)
    } else if parser.at_compound(SyntaxKind::GreaterThan, SyntaxKind::GreaterThan) {
        Some(BinaryOperator::ShiftRight)
    } else if parser.at(SyntaxKind::GreaterThan) {
        Some(BinaryOperator::GreaterThan)
    } else if parser.at(SyntaxKind::LessThan) {
        Some(BinaryOperator::LessThan)
    } else if parser.at(SyntaxKind::GreaterThanEqual) {
        Some(BinaryOperator::GreaterThanEqual)
    } else if parser.at(SyntaxKind::LessThanEqual) {
        Some(BinaryOperator::LessThanEqual)
    } else if parser.at(SyntaxKind::NotEqual) {
        Some(BinaryOperator::NotEqual)
    } else if parser.at(SyntaxKind::EqualEqual) {
        Some(BinaryOperator::Equals)
    } else if parser.at(SyntaxKind::Modulo) {
        Some(BinaryOperator::Modulo)
    } else {
        None
    };
    parser.set_expected(vec![SyntaxKind::BinaryOperator]);
    operator
}

impl BinaryOperator {
    const fn binding_power(&self) -> (u8, u8) {
        match self {
            Self::ShortCircuitOr => (0, 1),
            Self::ShortCircuitAnd => (2, 3),
            Self::Or => (4, 5),
            Self::Xor => (5, 6),
            Self::And => (7, 8),
            Self::Equals => (9, 10),
            Self::LessThan
            | Self::GreaterThan
            | Self::LessThanEqual
            | Self::GreaterThanEqual
            | Self::NotEqual => (11, 12),
            Self::ShiftLeft | Self::ShiftRight => (13, 14),
            Self::Add | Self::Subtract => (15, 16),
            Self::Multiply | Self::Divide | Self::Modulo => (17, 18),
        }
    }
}

const PREFIX_OP_SET: &[SyntaxKind] = &[
    SyntaxKind::Bang,
    SyntaxKind::Minus,
    SyntaxKind::And,
    SyntaxKind::Star,
    SyntaxKind::Tilde,
];
enum PrefixOp {
    Negate,
    Not,
    Reference,
    Dereference,
    BitNot,
}

impl PrefixOp {
    const fn binding_power(&self) -> ((), u8) {
        match self {
            Self::Negate | Self::Not | Self::Reference | Self::Dereference | Self::BitNot => {
                ((), 20)
            },
        }
    }
}

enum PostfixOp {
    Call,
    Index,
    Field,
}

impl PostfixOp {
    const fn binding_power(&self) -> (u8, ()) {
        match self {
            Self::Call | Self::Field | Self::Index => (21, ()),
        }
    }
}

fn postfix_op(parser: &mut Parser<'_, '_>) -> Option<PostfixOp> {
    if parser.at(SyntaxKind::Period) {
        Some(PostfixOp::Field)
    } else if parser.at(SyntaxKind::ParenthesisLeft) {
        Some(PostfixOp::Call)
    } else if parser.at(SyntaxKind::BracketLeft) {
        Some(PostfixOp::Index)
    } else {
        None
    }
}

pub(crate) const TOKENSET_LITERAL: &[SyntaxKind] = &[
    SyntaxKind::DecimalIntLiteral,
    SyntaxKind::HexIntLiteral,
    SyntaxKind::UnsignedIntLiteral,
    SyntaxKind::HexFloatLiteral,
    SyntaxKind::DecimalFloatLiteral,
    SyntaxKind::True,
    SyntaxKind::False,
];

pub(crate) fn literal(parser: &mut Parser<'_, '_>) -> CompletedMarker {
    assert!(parser.at_set(TOKENSET_LITERAL));

    let marker = parser.start();
    parser.bump();
    marker.complete(parser, SyntaxKind::Literal)
}

fn bitcast_expression(parser: &mut Parser<'_, '_>) -> CompletedMarker {
    assert!(parser.at(SyntaxKind::Bitcast));
    let marker = parser.start();
    parser.bump();
    if !parser.eat(SyntaxKind::LessThan) {
        parser.error_expected_no_bump(&[SyntaxKind::LessThan]);
        if parser.at(SyntaxKind::ParenthesisLeft) {
            parenthesis_expression(parser);
        }
        return marker.complete(parser, SyntaxKind::BitcastExpression);
    }
    _ = super::type_declaration(parser);
    parser.expect(SyntaxKind::GreaterThan);

    if !parser.at(SyntaxKind::ParenthesisLeft) {
        parser.error_expected_no_bump(&[SyntaxKind::ParenthesisLeft]);
        return marker.complete(parser, SyntaxKind::BitcastExpression);
    }
    parenthesis_expression(parser);
    marker.complete(parser, SyntaxKind::BitcastExpression)
}

fn prefix_expression(parser: &mut Parser<'_, '_>) -> CompletedMarker {
    let marker = parser.start();
    let op = if parser.at(SyntaxKind::Minus) {
        PrefixOp::Negate
    } else if parser.at(SyntaxKind::Bang) {
        PrefixOp::Not
    } else if parser.at(SyntaxKind::And) {
        PrefixOp::Reference
    } else if parser.at(SyntaxKind::Star) {
        PrefixOp::Dereference
    } else if parser.at(SyntaxKind::Tilde) {
        PrefixOp::BitNot
    } else {
        parser.error();
        return marker.complete(parser, SyntaxKind::PrefixExpression);
    };

    let ((), right_binding_power) = op.binding_power();

    // Eat the operator's token.
    parser.bump();

    expression_binding_power(parser, right_binding_power);

    marker.complete(parser, SyntaxKind::PrefixExpression)
}

fn parenthesis_expression(parser: &mut Parser<'_, '_>) -> CompletedMarker {
    assert!(parser.at(SyntaxKind::ParenthesisLeft));

    let marker = parser.start();
    parser.bump();
    if parser.at(SyntaxKind::ParenthesisRight) {
        // TODO: Better kind of error here. Ideally just EXPR
        parser.error_expected_no_bump(&[SyntaxKind::ParenthesisExpression]);
        parser.bump();
        return marker.complete(parser, SyntaxKind::ParenthesisExpression);
    }

    expression_binding_power(parser, 0);
    parser.expect(SyntaxKind::ParenthesisRight);

    marker.complete(parser, SyntaxKind::ParenthesisExpression)
}

#[cfg(test)]
mod tests {
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
                Literal@0..3
                  DecimalIntLiteral@0..3 "123""#]],
        );
    }

    #[test]
    fn parse_number_preceded_by_whitespace() {
        check(
            "   9876",
            expect![[r#"
                Literal@0..7
                  Blankspace@0..3 "   "
                  DecimalIntLiteral@3..7 "9876""#]],
        );
    }

    #[test]
    fn parse_number_followed_by_whitespace() {
        check(
            "999   ",
            expect![[r#"
                Literal@0..6
                  DecimalIntLiteral@0..3 "999"
                  Blankspace@3..6 "   ""#]],
        );
    }

    #[test]
    fn parse_number_surrounded_by_whitespace() {
        check(
            " 123     ",
            expect![[r#"
                Literal@0..9
                  Blankspace@0..1 " "
                  DecimalIntLiteral@1..4 "123"
                  Blankspace@4..9 "     ""#]],
        );
    }

    #[test]
    fn parse_variable_ref() {
        check(
            "counter",
            expect![[r#"
                PathExpression@0..7
                  NameReference@0..7
                    Identifier@0..7 "counter""#]],
        );
    }

    #[test]
    fn parse_variable_ref_no_comment() {
        check(
            "counter // not part of it",
            expect![[r#"
                PathExpression@0..25
                  NameReference@0..25
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
                PathExpression@0..28
                  NameReference@0..28
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
                InfixExpression@0..3
                  Literal@0..1
                    DecimalIntLiteral@0..1 "1"
                  Plus@1..2 "+"
                  Literal@2..3
                    DecimalIntLiteral@2..3 "2""#]],
        );
    }

    #[test]
    fn parse_left_associative_infix_expression() {
        check(
            "1+2+3+4",
            expect![[r#"
                InfixExpression@0..7
                  InfixExpression@0..5
                    InfixExpression@0..3
                      Literal@0..1
                        DecimalIntLiteral@0..1 "1"
                      Plus@1..2 "+"
                      Literal@2..3
                        DecimalIntLiteral@2..3 "2"
                    Plus@3..4 "+"
                    Literal@4..5
                      DecimalIntLiteral@4..5 "3"
                  Plus@5..6 "+"
                  Literal@6..7
                    DecimalIntLiteral@6..7 "4""#]],
        );
    }

    #[test]
    fn parse_infix_expression_with_mixed_binding_power() {
        check(
            "1+2*3-4",
            expect![[r#"
                InfixExpression@0..5
                  Literal@0..1
                    DecimalIntLiteral@0..1 "1"
                  Plus@1..2 "+"
                  InfixExpression@2..5
                    Literal@2..3
                      DecimalIntLiteral@2..3 "2"
                    Star@3..4 "*"
                    Literal@4..5
                      DecimalIntLiteral@4..5 "3""#]],
        );
    }

    #[test]
    fn parse_infix_expression_with_whitespace() {
        check(
            " 1 +   2* 3 ",
            expect![[r#"
                InfixExpression@0..12
                  Literal@0..3
                    Blankspace@0..1 " "
                    DecimalIntLiteral@1..2 "1"
                    Blankspace@2..3 " "
                  Plus@3..4 "+"
                  Blankspace@4..7 "   "
                  InfixExpression@7..12
                    Literal@7..8
                      DecimalIntLiteral@7..8 "2"
                    Star@8..9 "*"
                    Blankspace@9..10 " "
                    Literal@10..12
                      DecimalIntLiteral@10..11 "3"
                      Blankspace@11..12 " ""#]],
        );
    }

    #[test]
    fn do_not_parse_operator_if_getting_rhs_failed() {
        check(
            "(1+",
            expect![[r#"
                ParenthesisExpression@0..3
                  ParenthesisLeft@0..1 "("
                  InfixExpression@1..3
                    Literal@1..2
                      DecimalIntLiteral@1..2 "1"
                    Plus@2..3 "+"

                error at 2..3: expected Identifier, Bitcast, or ParenthesisLeft
                error at 2..3: expected ParenthesisRight"#]],
        );
    }

    #[test]
    fn parse_negation() {
        check(
            "-10",
            expect![[r#"
                Literal@0..3
                  DecimalIntLiteral@0..3 "-10""#]],
        );
    }

    #[test]
    fn negation_has_higher_binding_power_than_binary_operators() {
        check(
            "-20+20",
            expect![[r#"
                InfixExpression@0..6
                  Literal@0..3
                    DecimalIntLiteral@0..3 "-20"
                  Plus@3..4 "+"
                  Literal@4..6
                    DecimalIntLiteral@4..6 "20""#]],
        );
    }

    #[test]
    fn parse_nested_parentheses() {
        check(
            "((((((10))))))",
            expect![[r#"
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
                              DecimalIntLiteral@6..8 "10"
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
                InfixExpression@0..7
                  Literal@0..1
                    DecimalIntLiteral@0..1 "5"
                  Star@1..2 "*"
                  ParenthesisExpression@2..7
                    ParenthesisLeft@2..3 "("
                    InfixExpression@3..6
                      Literal@3..4
                        DecimalIntLiteral@3..4 "2"
                      Plus@4..5 "+"
                      Literal@5..6
                        DecimalIntLiteral@5..6 "1"
                    ParenthesisRight@6..7 ")""#]],
        );
    }

    #[test]
    fn parse_unclosed_parentheses() {
        check(
            "(foo",
            expect![[r#"
                ParenthesisExpression@0..4
                  ParenthesisLeft@0..1 "("
                  PathExpression@1..4
                    NameReference@1..4
                      Identifier@1..4 "foo"

                error at 1..4: expected BinaryOperator or ParenthesisRight"#]],
        );
    }

    #[test]
    fn parse_expression_complex() {
        check(
            "1 + 2 == 3 || 4 < 5 / 2 == 0",
            expect![[r#"
                InfixExpression@0..28
                  InfixExpression@0..11
                    InfixExpression@0..6
                      Literal@0..2
                        DecimalIntLiteral@0..1 "1"
                        Blankspace@1..2 " "
                      Plus@2..3 "+"
                      Blankspace@3..4 " "
                      Literal@4..6
                        DecimalIntLiteral@4..5 "2"
                        Blankspace@5..6 " "
                    EqualEqual@6..8 "=="
                    Blankspace@8..9 " "
                    Literal@9..11
                      DecimalIntLiteral@9..10 "3"
                      Blankspace@10..11 " "
                  OrOr@11..13 "||"
                  Blankspace@13..14 " "
                  InfixExpression@14..28
                    InfixExpression@14..24
                      Literal@14..16
                        DecimalIntLiteral@14..15 "4"
                        Blankspace@15..16 " "
                      LessThan@16..17 "<"
                      Blankspace@17..18 " "
                      InfixExpression@18..24
                        Literal@18..20
                          DecimalIntLiteral@18..19 "5"
                          Blankspace@19..20 " "
                        ForwardSlash@20..21 "/"
                        Blankspace@21..22 " "
                        Literal@22..24
                          DecimalIntLiteral@22..23 "2"
                          Blankspace@23..24 " "
                    EqualEqual@24..26 "=="
                    Blankspace@26..27 " "
                    Literal@27..28
                      DecimalIntLiteral@27..28 "0""#]],
        );
    }

    #[test]
    fn parse_expression_field() {
        check(
            "a.b.c",
            expect![[r#"
                FieldExpression@0..5
                  FieldExpression@0..3
                    PathExpression@0..1
                      NameReference@0..1
                        Identifier@0..1 "a"
                    Period@1..2 "."
                    NameReference@2..3
                      Identifier@2..3 "b"
                  Period@3..4 "."
                  NameReference@4..5
                    Identifier@4..5 "c""#]],
        );
    }

    #[test]
    fn parse_expression_field_mix_ops() {
        check(
            "vec.xy + 2 * other.zw",
            expect![[r#"
                InfixExpression@0..21
                  FieldExpression@0..7
                    PathExpression@0..3
                      NameReference@0..3
                        Identifier@0..3 "vec"
                    Period@3..4 "."
                    NameReference@4..7
                      Identifier@4..6 "xy"
                      Blankspace@6..7 " "
                  Plus@7..8 "+"
                  Blankspace@8..9 " "
                  InfixExpression@9..21
                    Literal@9..11
                      DecimalIntLiteral@9..10 "2"
                      Blankspace@10..11 " "
                    Star@11..12 "*"
                    Blankspace@12..13 " "
                    FieldExpression@13..21
                      PathExpression@13..18
                        NameReference@13..18
                          Identifier@13..18 "other"
                      Period@18..19 "."
                      NameReference@19..21
                        Identifier@19..21 "zw""#]],
        );
    }

    #[test]
    fn parse_expression_function_call() {
        check(
            "pow(2, 3)",
            expect![[r#"
                FunctionCall@0..9
                  NameReference@0..3
                    Identifier@0..3 "pow"
                  FunctionParameterList@3..9
                    ParenthesisLeft@3..4 "("
                    Literal@4..5
                      DecimalIntLiteral@4..5 "2"
                    Comma@5..6 ","
                    Blankspace@6..7 " "
                    Literal@7..8
                      DecimalIntLiteral@7..8 "3"
                    ParenthesisRight@8..9 ")""#]],
        );
    }

    #[test]
    fn parse_expression_function_call_mixed() {
        check(
            "pow(srgb + 14.0, 3.0) * 2.0",
            expect![[r#"
                InfixExpression@0..27
                  FunctionCall@0..22
                    NameReference@0..3
                      Identifier@0..3 "pow"
                    FunctionParameterList@3..22
                      ParenthesisLeft@3..4 "("
                      InfixExpression@4..15
                        PathExpression@4..9
                          NameReference@4..9
                            Identifier@4..8 "srgb"
                            Blankspace@8..9 " "
                        Plus@9..10 "+"
                        Blankspace@10..11 " "
                        Literal@11..15
                          DecimalFloatLiteral@11..15 "14.0"
                      Comma@15..16 ","
                      Blankspace@16..17 " "
                      Literal@17..20
                        DecimalFloatLiteral@17..20 "3.0"
                      ParenthesisRight@20..21 ")"
                      Blankspace@21..22 " "
                  Star@22..23 "*"
                  Blankspace@23..24 " "
                  Literal@24..27
                    DecimalFloatLiteral@24..27 "2.0""#]],
        );
    }

    #[test]
    fn parse_vec3_initializer() {
        check(
            "vec3<f32>(1.0)",
            expect![[r#"
                TypeInitializer@0..14
                  Vec3@0..9
                    Vec3@0..4 "vec3"
                    GenericArgumentList@4..9
                      LessThan@4..5 "<"
                      Float32@5..8
                        Float32@5..8 "f32"
                      GreaterThan@8..9 ">"
                  FunctionParameterList@9..14
                    ParenthesisLeft@9..10 "("
                    Literal@10..13
                      DecimalFloatLiteral@10..13 "1.0"
                    ParenthesisRight@13..14 ")""#]],
        );
    }

    #[test]
    fn parse_vec3_initializer_inferred() {
        check(
            "vec3(1.0)",
            expect![[r#"
                TypeInitializer@0..9
                  Vec3@0..4
                    Vec3@0..4 "vec3"
                  FunctionParameterList@4..9
                    ParenthesisLeft@4..5 "("
                    Literal@5..8
                      DecimalFloatLiteral@5..8 "1.0"
                    ParenthesisRight@8..9 ")""#]],
        );
    }

    #[test]
    fn parse_bool_literal() {
        check(
            "true",
            expect![[r#"
                Literal@0..4
                  True@0..4 "true""#]],
        );
        check(
            "false",
            expect![[r#"
                Literal@0..5
                  False@0..5 "false""#]],
        );
    }

    #[test]
    fn parse_decimal_float_literal() {
        check(
            "0.e+4f",
            expect![[r#"
                Literal@0..6
                  DecimalFloatLiteral@0..6 "0.e+4f""#]],
        );
        check(
            "01.",
            expect![[r#"
                Literal@0..3
                  DecimalFloatLiteral@0..3 "01.""#]],
        );
        check(
            ".01",
            expect![[r#"
                Literal@0..3
                  DecimalFloatLiteral@0..3 ".01""#]],
        );
        check(
            "12.34",
            expect![[r#"
                Literal@0..5
                  DecimalFloatLiteral@0..5 "12.34""#]],
        );
        check(
            ".0f",
            expect![[r#"
                Literal@0..3
                  DecimalFloatLiteral@0..3 ".0f""#]],
        );
        check(
            "0h",
            expect![[r#"
                Literal@0..2
                  DecimalFloatLiteral@0..2 "0h""#]],
        );
        check(
            "1e-3",
            expect![[r#"
                Literal@0..4
                  DecimalFloatLiteral@0..4 "1e-3""#]],
        );
    }

    #[test]
    fn parse_hex_int_literal() {
        check(
            "0x123",
            expect![[r#"
                Literal@0..5
                  HexIntLiteral@0..5 "0x123""#]],
        );
        check(
            "0X123u",
            expect![[r#"
                Literal@0..6
                  UnsignedIntLiteral@0..6 "0X123u""#]],
        );
        check(
            "0x3f",
            expect![[r#"
                Literal@0..4
                  HexIntLiteral@0..4 "0x3f""#]],
        );
    }

    #[test]
    fn parse_hex_float_literal() {
        check(
            "0xa.fp+2",
            expect![[r#"
                Literal@0..8
                  HexFloatLiteral@0..8 "0xa.fp+2""#]],
        );
        check(
            "0x1P+4f",
            expect![[r#"
                Literal@0..7
                  HexFloatLiteral@0..7 "0x1P+4f""#]],
        );
        check(
            "0X.3",
            expect![[r#"
                Literal@0..4
                  HexFloatLiteral@0..4 "0X.3""#]],
        );
        check(
            "0x3p+2h",
            expect![[r#"
                Literal@0..7
                  HexFloatLiteral@0..7 "0x3p+2h""#]],
        );
        check(
            "0X1.fp-4",
            expect![[r#"
                Literal@0..8
                  HexFloatLiteral@0..8 "0X1.fp-4""#]],
        );
        check(
            "0x3.2p+2h",
            expect![[r#"
                Literal@0..9
                  HexFloatLiteral@0..9 "0x3.2p+2h""#]],
        );
    }

    #[test]
    fn parse_prefix_expression() {
        check(
            "- 3 + 3",
            expect![[r#"
                InfixExpression@0..7
                  PrefixExpression@0..4
                    Minus@0..1 "-"
                    Blankspace@1..2 " "
                    Literal@2..4
                      DecimalIntLiteral@2..3 "3"
                      Blankspace@3..4 " "
                  Plus@4..5 "+"
                  Blankspace@5..6 " "
                  Literal@6..7
                    DecimalIntLiteral@6..7 "3""#]],
        );
    }

    #[test]
    fn parse_index() {
        check(
            "a.b[3+2]",
            expect![[r#"
                IndexExpression@0..8
                  FieldExpression@0..3
                    PathExpression@0..1
                      NameReference@0..1
                        Identifier@0..1 "a"
                    Period@1..2 "."
                    NameReference@2..3
                      Identifier@2..3 "b"
                  BracketLeft@3..4 "["
                  InfixExpression@4..7
                    Literal@4..5
                      DecimalIntLiteral@4..5 "3"
                    Plus@5..6 "+"
                    Literal@6..7
                      DecimalIntLiteral@6..7 "2"
                  BracketRight@7..8 "]""#]],
        );
    }

    #[test]
    fn parse_modulo_comparison() {
        check(
            "n % 2u == 0u",
            expect![[r#"
                InfixExpression@0..12
                  InfixExpression@0..7
                    PathExpression@0..2
                      NameReference@0..2
                        Identifier@0..1 "n"
                        Blankspace@1..2 " "
                    Modulo@2..3 "%"
                    Blankspace@3..4 " "
                    Literal@4..7
                      UnsignedIntLiteral@4..6 "2u"
                      Blankspace@6..7 " "
                  EqualEqual@7..9 "=="
                  Blankspace@9..10 " "
                  Literal@10..12
                    UnsignedIntLiteral@10..12 "0u""#]],
        );
    }

    #[test]
    fn prefix_expressions() {
        check(
            "!~*&foo",
            expect![[r#"
            PrefixExpression@0..7
              Bang@0..1 "!"
              PrefixExpression@1..7
                Tilde@1..2 "~"
                PrefixExpression@2..7
                  Star@2..3 "*"
                  PrefixExpression@3..7
                    And@3..4 "&"
                    PathExpression@4..7
                      NameReference@4..7
                        Identifier@4..7 "foo""#]],
        );
    }

    #[test]
    fn bitcast() {
        check(
            "bitcast<u32>(x)",
            expect![[r#"
                BitcastExpression@0..15
                  Bitcast@0..7 "bitcast"
                  LessThan@7..8 "<"
                  Uint32@8..11
                    Uint32@8..11 "u32"
                  GreaterThan@11..12 ">"
                  ParenthesisExpression@12..15
                    ParenthesisLeft@12..13 "("
                    PathExpression@13..14
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
                BitcastExpression@0..21
                  Bitcast@0..7 "bitcast"
                  LessThan@7..8 "<"
                  Vec4@8..17
                    Vec4@8..12 "vec4"
                    GenericArgumentList@12..17
                      LessThan@12..13 "<"
                      Uint32@13..16
                        Uint32@13..16 "u32"
                      GreaterThan@16..17 ">"
                  GreaterThan@17..18 ">"
                  ParenthesisExpression@18..21
                    ParenthesisLeft@18..19 "("
                    PathExpression@19..20
                      NameReference@19..20
                        Identifier@19..20 "x"
                    ParenthesisRight@20..21 ")""#]],
        );
    }

    #[test]
    fn bitcast_no_generics() {
        check(
            "bitcast(x)",
            expect![[r#"
                BitcastExpression@0..10
                  Bitcast@0..7 "bitcast"
                  Error@7..7
                  ParenthesisExpression@7..10
                    ParenthesisLeft@7..8 "("
                    PathExpression@8..9
                      NameReference@8..9
                        Identifier@8..9 "x"
                    ParenthesisRight@9..10 ")"

                error at 7..8: expected LessThan, but found ParenthesisLeft"#]],
        );
    }
    #[test]
    fn bitcast_in_expression() {
        check(
            "1 + -bitcast<u32>(x) + 1",
            expect![[r#"
                InfixExpression@0..24
                  InfixExpression@0..21
                    Literal@0..2
                      DecimalIntLiteral@0..1 "1"
                      Blankspace@1..2 " "
                    Plus@2..3 "+"
                    Blankspace@3..4 " "
                    PrefixExpression@4..21
                      Minus@4..5 "-"
                      BitcastExpression@5..21
                        Bitcast@5..12 "bitcast"
                        LessThan@12..13 "<"
                        Uint32@13..16
                          Uint32@13..16 "u32"
                        GreaterThan@16..17 ">"
                        ParenthesisExpression@17..21
                          ParenthesisLeft@17..18 "("
                          PathExpression@18..19
                            NameReference@18..19
                              Identifier@18..19 "x"
                          ParenthesisRight@19..20 ")"
                          Blankspace@20..21 " "
                  Plus@21..22 "+"
                  Blankspace@22..23 " "
                  Literal@23..24
                    DecimalIntLiteral@23..24 "1""#]],
        );
    }

    #[test]
    fn deref_field() {
        check(
            "*a.b",
            expect![[r#"
                PrefixExpression@0..4
                  Star@0..1 "*"
                  FieldExpression@1..4
                    PathExpression@1..2
                      NameReference@1..2
                        Identifier@1..2 "a"
                    Period@2..3 "."
                    NameReference@3..4
                      Identifier@3..4 "b""#]],
        );
    }
    #[test]
    fn deref_field_paren() {
        check(
            "(*a).b",
            expect![[r#"
            FieldExpression@0..6
              ParenthesisExpression@0..4
                ParenthesisLeft@0..1 "("
                PrefixExpression@1..3
                  Star@1..2 "*"
                  PathExpression@2..3
                    NameReference@2..3
                      Identifier@2..3 "a"
                ParenthesisRight@3..4 ")"
              Period@4..5 "."
              NameReference@5..6
                Identifier@5..6 "b""#]],
        );
    }

    #[test]
    fn shift_right() {
        check(
            "2 >> 3",
            expect![[r#"
                InfixExpression@0..6
                  Literal@0..2
                    DecimalIntLiteral@0..1 "2"
                    Blankspace@1..2 " "
                  ShiftRight@2..5
                    GreaterThan@2..3 ">"
                    GreaterThan@3..4 ">"
                    Blankspace@4..5 " "
                  Literal@5..6
                    DecimalIntLiteral@5..6 "3""#]],
        );
    }

    #[test]
    fn shift_multiple() {
        check(
            "2 >> 3 + 2 << 4",
            expect![[r#"
                InfixExpression@0..15
                  InfixExpression@0..11
                    Literal@0..2
                      DecimalIntLiteral@0..1 "2"
                      Blankspace@1..2 " "
                    ShiftRight@2..5
                      GreaterThan@2..3 ">"
                      GreaterThan@3..4 ">"
                      Blankspace@4..5 " "
                    InfixExpression@5..11
                      Literal@5..7
                        DecimalIntLiteral@5..6 "3"
                        Blankspace@6..7 " "
                      Plus@7..8 "+"
                      Blankspace@8..9 " "
                      Literal@9..11
                        DecimalIntLiteral@9..10 "2"
                        Blankspace@10..11 " "
                  ShiftLeft@11..14
                    LessThan@11..12 "<"
                    LessThan@12..13 "<"
                    Blankspace@13..14 " "
                  Literal@14..15
                    DecimalIntLiteral@14..15 "4""#]],
        );
    }
}
