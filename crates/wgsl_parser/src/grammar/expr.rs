// mostly copied from https://github.com/arzg/eldiro/blob/f3d588f8a76e2e4317c1d77ae2758b0781bb5af3/crates/parser/src/grammar.rs

use super::*;

pub fn expr(p: &mut Parser) {
    if p.at_set(super::STATEMENT_RECOVER_SET) {
        return;
    }
    expr_binding_power(p, 0);
}

fn expr_binding_power(p: &mut Parser, minimum_binding_power: u8) -> Option<CompletedMarker> {
    let mut lhs = lhs(p)?;

    loop {
        // postfix ops
        if let Some(postfix_op) = postfix_op(p) {
            let (left_binding_power, ()) = postfix_op.binding_power();
            if left_binding_power < minimum_binding_power {
                break;
            }

            let m = lhs.precede(p);
            match postfix_op {
                PostfixOp::Call => {
                    function_param_list(p);
                    lhs = m.complete(p, SyntaxKind::FunctionCall);
                }
                PostfixOp::Index => {
                    array_index(p);
                    lhs = m.complete(p, SyntaxKind::IndexExpr);
                }
                PostfixOp::Field => {
                    p.bump();
                    name_ref(p);
                    lhs = m.complete(p, SyntaxKind::FieldExpr);
                }
            }

            continue;
        }

        let infix_op = match binary_op(p) {
            Some(op) => op,
            None => break,
        };

        let (left_binding_power, right_binding_power) = infix_op.binding_power();

        if left_binding_power < minimum_binding_power {
            break;
        }

        // Eat the operator's token.

        match infix_op {
            BinaryOp::ShiftLeft => p.bump_compound(SyntaxKind::ShiftLeft),
            BinaryOp::ShiftRight => p.bump_compound(SyntaxKind::ShiftRight),
            _ => {
                p.bump();
            }
        }

        let m = lhs.precede(p);
        let parsed_rhs = expr_binding_power(p, right_binding_power).is_some();
        lhs = m.complete(p, SyntaxKind::InfixExpr);

        if !parsed_rhs {
            break;
        }
    }

    Some(lhs)
}

fn function_param_list(p: &mut Parser) {
    list(
        p,
        SyntaxKind::ParenLeft,
        SyntaxKind::ParenRight,
        SyntaxKind::Comma,
        SyntaxKind::FunctionParamList,
        |p| {
            expr_binding_power(p, 0);
        },
    );
}
fn array_index(p: &mut Parser) {
    p.expect(SyntaxKind::BracketLeft);
    expr_binding_power(p, 0);
    p.expect(SyntaxKind::BracketRight);
}

fn lhs(p: &mut Parser) -> Option<CompletedMarker> {
    let cm = if p.at_set(TOKENSET_LITERAL) {
        literal(p)
    } else if p.at(SyntaxKind::Ident) {
        let m = p.start();
        name_ref(p);
        m.complete(p, SyntaxKind::PathExpr)
    } else if p.at(SyntaxKind::Bitcast) {
        bitcast_expr(p)
    } else if p.at_set(TYPE_SET) {
        let type_decl = super::type_decl(p).unwrap();
        type_decl
            .precede(p)
            .complete(p, SyntaxKind::TypeInitializer)
    } else if p.at_set(PREFIX_OP_SET) {
        prefix_expr(p)
    } else if p.at(SyntaxKind::ParenLeft) {
        paren_expr(p)
    } else {
        p.error();
        return None;
    };

    Some(cm)
}

enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
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
fn binary_op(p: &mut Parser) -> Option<BinaryOp> {
    let op = if p.at(SyntaxKind::Plus) {
        Some(BinaryOp::Add)
    } else if p.at(SyntaxKind::Minus) {
        Some(BinaryOp::Sub)
    } else if p.at(SyntaxKind::Star) {
        Some(BinaryOp::Mul)
    } else if p.at(SyntaxKind::ForwardSlash) {
        Some(BinaryOp::Div)
    } else if p.at(SyntaxKind::Or) {
        Some(BinaryOp::Or)
    } else if p.at(SyntaxKind::And) {
        Some(BinaryOp::And)
    } else if p.at(SyntaxKind::OrOr) {
        Some(BinaryOp::ShortCircuitOr)
    } else if p.at(SyntaxKind::AndAnd) {
        Some(BinaryOp::ShortCircuitAnd)
    } else if p.at(SyntaxKind::Xor) {
        Some(BinaryOp::Xor)
    } else if p.at_compound(SyntaxKind::LessThan, SyntaxKind::LessThan) {
        Some(BinaryOp::ShiftLeft)
    } else if p.at_compound(SyntaxKind::GreaterThan, SyntaxKind::GreaterThan) {
        Some(BinaryOp::ShiftRight)
    } else if p.at(SyntaxKind::GreaterThan) {
        Some(BinaryOp::GreaterThan)
    } else if p.at(SyntaxKind::LessThan) {
        Some(BinaryOp::LessThan)
    } else if p.at(SyntaxKind::GreaterThanEqual) {
        Some(BinaryOp::GreaterThanEqual)
    } else if p.at(SyntaxKind::LessThanEqual) {
        Some(BinaryOp::LessThanEqual)
    } else if p.at(SyntaxKind::NotEqual) {
        Some(BinaryOp::NotEqual)
    } else if p.at(SyntaxKind::EqualEqual) {
        Some(BinaryOp::Equals)
    } else if p.at(SyntaxKind::Modulo) {
        Some(BinaryOp::Modulo)
    } else {
        None
    };
    p.set_expected(vec![SyntaxKind::BinaryOperator]);
    op
}

impl BinaryOp {
    fn binding_power(&self) -> (u8, u8) {
        match self {
            BinaryOp::ShortCircuitOr => (0, 1),
            BinaryOp::ShortCircuitAnd => (2, 3),
            BinaryOp::Or => (4, 5),
            BinaryOp::Xor => (5, 6),
            BinaryOp::And => (7, 8),
            BinaryOp::Equals => (9, 10),
            BinaryOp::LessThan
            | BinaryOp::GreaterThan
            | BinaryOp::LessThanEqual
            | BinaryOp::GreaterThanEqual
            | BinaryOp::NotEqual => (11, 12),
            BinaryOp::ShiftLeft | BinaryOp::ShiftRight => (13, 14),
            BinaryOp::Add | BinaryOp::Sub => (15, 16),
            BinaryOp::Mul | BinaryOp::Div | BinaryOp::Modulo => (17, 18),
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
    Neg,
    Not,
    Ref,
    Deref,
    BitNot,
}

impl PrefixOp {
    fn binding_power(&self) -> ((), u8) {
        match self {
            Self::Neg | Self::Not | Self::Ref | Self::Deref | Self::BitNot => ((), 20),
        }
    }
}

enum PostfixOp {
    Call,
    Index,
    Field,
}
impl PostfixOp {
    fn binding_power(&self) -> (u8, ()) {
        match self {
            Self::Call | Self::Field | Self::Index => (21, ()),
        }
    }
}
fn postfix_op(p: &mut Parser) -> Option<PostfixOp> {
    if p.at(SyntaxKind::Period) {
        Some(PostfixOp::Field)
    } else if p.at(SyntaxKind::ParenLeft) {
        Some(PostfixOp::Call)
    } else if p.at(SyntaxKind::BracketLeft) {
        Some(PostfixOp::Index)
    } else {
        None
    }
}

pub const TOKENSET_LITERAL: &[SyntaxKind] = &[
    SyntaxKind::IntLiteral,
    SyntaxKind::UintLiteral,
    SyntaxKind::HexFloatLiteral,
    SyntaxKind::DecimalFloatLiteral,
    SyntaxKind::True,
    SyntaxKind::False,
];
pub fn literal(p: &mut Parser) -> CompletedMarker {
    assert!(p.at_set(TOKENSET_LITERAL));

    let m = p.start();
    p.bump();
    m.complete(p, SyntaxKind::Literal)
}

fn bitcast_expr(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(SyntaxKind::Bitcast));
    let m = p.start();
    p.bump();
    if !p.eat(SyntaxKind::LessThan) {
        p.error_expected_no_bump(&[SyntaxKind::LessThan]);
        if p.at(SyntaxKind::ParenLeft) {
            paren_expr(p);
        }
        return m.complete(p, SyntaxKind::BitcastExpr);
    }
    let _ = super::type_decl(p);
    p.expect(SyntaxKind::GreaterThan);

    if !p.at(SyntaxKind::ParenLeft) {
        p.error_expected_no_bump(&[SyntaxKind::ParenLeft]);
        return m.complete(p, SyntaxKind::BitcastExpr);
    }
    paren_expr(p);
    return m.complete(p, SyntaxKind::BitcastExpr);
}

fn prefix_expr(p: &mut Parser) -> CompletedMarker {
    let m = p.start();
    let op = if p.at(SyntaxKind::Minus) {
        PrefixOp::Neg
    } else if p.at(SyntaxKind::Bang) {
        PrefixOp::Not
    } else if p.at(SyntaxKind::And) {
        PrefixOp::Ref
    } else if p.at(SyntaxKind::Star) {
        PrefixOp::Deref
    } else if p.at(SyntaxKind::Tilde) {
        PrefixOp::BitNot
    } else {
        p.error();
        return m.complete(p, SyntaxKind::PrefixExpr);
    };

    let ((), right_binding_power) = op.binding_power();

    // Eat the operator's token.
    p.bump();

    expr_binding_power(p, right_binding_power);

    m.complete(p, SyntaxKind::PrefixExpr)
}

fn paren_expr(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(SyntaxKind::ParenLeft));

    let m = p.start();
    p.bump();
    if p.at(SyntaxKind::ParenRight) {
        // TODO: Better kind of error here. Ideally just EXPR
        p.error_expected_no_bump(&[SyntaxKind::ParenExpr]);
        p.bump();
        return m.complete(p, SyntaxKind::ParenExpr);
    }

    expr_binding_power(p, 0);
    p.expect(SyntaxKind::ParenRight);

    m.complete(p, SyntaxKind::ParenExpr)
}

#[cfg(test)]
mod tests {
    use crate::ParseEntryPoint;
    use expect_test::{expect, Expect};

    fn check(input: &str, expected_tree: Expect) {
        crate::check_entrypoint(input, ParseEntryPoint::Expression, expected_tree);
    }

    #[test]
    fn parse_number() {
        check(
            "123",
            expect![[r#"
                Literal@0..3
                  IntLiteral@0..3 "123""#]],
        );
    }

    #[test]
    fn parse_number_preceded_by_whitespace() {
        check(
            "   9876",
            expect![[r#"
                Literal@0..7
                  Whitespace@0..3 "   "
                  IntLiteral@3..7 "9876""#]],
        );
    }

    #[test]
    fn parse_number_followed_by_whitespace() {
        check(
            "999   ",
            expect![[r#"
                Literal@0..6
                  IntLiteral@0..3 "999"
                  Whitespace@3..6 "   ""#]],
        );
    }

    #[test]
    fn parse_number_surrounded_by_whitespace() {
        check(
            " 123     ",
            expect![[r#"
                Literal@0..9
                  Whitespace@0..1 " "
                  IntLiteral@1..4 "123"
                  Whitespace@4..9 "     ""#]],
        );
    }

    #[test]
    fn parse_variable_ref() {
        check(
            "counter",
            expect![[r#"
                PathExpr@0..7
                  NameRef@0..7
                    Ident@0..7 "counter""#]],
        );
    }

    #[test]
    fn parse_simple_infix_expression() {
        check(
            "1+2",
            expect![[r#"
                InfixExpr@0..3
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
                InfixExpr@0..7
                  InfixExpr@0..5
                    InfixExpr@0..3
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
        check(
            "1+2*3-4",
            expect![[r#"
                InfixExpr@0..5
                  Literal@0..1
                    IntLiteral@0..1 "1"
                  Plus@1..2 "+"
                  InfixExpr@2..5
                    Literal@2..3
                      IntLiteral@2..3 "2"
                    Star@3..4 "*"
                    Literal@4..5
                      IntLiteral@4..5 "3""#]],
        );
    }

    #[test]
    fn parse_infix_expression_with_whitespace() {
        check(
            " 1 +   2* 3 ",
            expect![[r#"
                InfixExpr@0..12
                  Literal@0..3
                    Whitespace@0..1 " "
                    IntLiteral@1..2 "1"
                    Whitespace@2..3 " "
                  Plus@3..4 "+"
                  Whitespace@4..7 "   "
                  InfixExpr@7..12
                    Literal@7..8
                      IntLiteral@7..8 "2"
                    Star@8..9 "*"
                    Whitespace@9..10 " "
                    Literal@10..12
                      IntLiteral@10..11 "3"
                      Whitespace@11..12 " ""#]],
        );
    }

    #[test]
    fn do_not_parse_operator_if_gettting_rhs_failed() {
        check(
            "(1+",
            expect![[r#"
                ParenExpr@0..3
                  ParenLeft@0..1 "("
                  InfixExpr@1..3
                    Literal@1..2
                      IntLiteral@1..2 "1"
                    Plus@2..3 "+"

                error at 2..3: expected Ident, Bitcast or ParenLeft
                error at 2..3: expected ParenRight"#]],
        );
    }

    #[test]
    fn parse_negation() {
        check(
            "-10",
            expect![[r#"
                Literal@0..3
                  IntLiteral@0..3 "-10""#]],
        );
    }

    #[test]
    fn negation_has_higher_binding_power_than_binary_operators() {
        check(
            "-20+20",
            expect![[r#"
                InfixExpr@0..6
                  Literal@0..3
                    IntLiteral@0..3 "-20"
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
                ParenExpr@0..14
                  ParenLeft@0..1 "("
                  ParenExpr@1..13
                    ParenLeft@1..2 "("
                    ParenExpr@2..12
                      ParenLeft@2..3 "("
                      ParenExpr@3..11
                        ParenLeft@3..4 "("
                        ParenExpr@4..10
                          ParenLeft@4..5 "("
                          ParenExpr@5..9
                            ParenLeft@5..6 "("
                            Literal@6..8
                              IntLiteral@6..8 "10"
                            ParenRight@8..9 ")"
                          ParenRight@9..10 ")"
                        ParenRight@10..11 ")"
                      ParenRight@11..12 ")"
                    ParenRight@12..13 ")"
                  ParenRight@13..14 ")""#]],
        );
    }

    #[test]
    fn parentheses_affect_precedence() {
        check(
            "5*(2+1)",
            expect![[r#"
                InfixExpr@0..7
                  Literal@0..1
                    IntLiteral@0..1 "5"
                  Star@1..2 "*"
                  ParenExpr@2..7
                    ParenLeft@2..3 "("
                    InfixExpr@3..6
                      Literal@3..4
                        IntLiteral@3..4 "2"
                      Plus@4..5 "+"
                      Literal@5..6
                        IntLiteral@5..6 "1"
                    ParenRight@6..7 ")""#]],
        );
    }

    #[test]
    fn parse_unclosed_parentheses() {
        check(
            "(foo",
            expect![[r#"
                ParenExpr@0..4
                  ParenLeft@0..1 "("
                  PathExpr@1..4
                    NameRef@1..4
                      Ident@1..4 "foo"

                error at 1..4: expected BinaryOperator or ParenRight"#]],
        );
    }

    #[test]
    fn parse_expr_complex() {
        check(
            "1 + 2 == 3 || 4 < 5 / 2 == 0",
            expect![[r#"
            InfixExpr@0..28
              InfixExpr@0..11
                InfixExpr@0..6
                  Literal@0..2
                    IntLiteral@0..1 "1"
                    Whitespace@1..2 " "
                  Plus@2..3 "+"
                  Whitespace@3..4 " "
                  Literal@4..6
                    IntLiteral@4..5 "2"
                    Whitespace@5..6 " "
                EqualEqual@6..8 "=="
                Whitespace@8..9 " "
                Literal@9..11
                  IntLiteral@9..10 "3"
                  Whitespace@10..11 " "
              OrOr@11..13 "||"
              Whitespace@13..14 " "
              InfixExpr@14..28
                InfixExpr@14..24
                  Literal@14..16
                    IntLiteral@14..15 "4"
                    Whitespace@15..16 " "
                  LessThan@16..17 "<"
                  Whitespace@17..18 " "
                  InfixExpr@18..24
                    Literal@18..20
                      IntLiteral@18..19 "5"
                      Whitespace@19..20 " "
                    ForwardSlash@20..21 "/"
                    Whitespace@21..22 " "
                    Literal@22..24
                      IntLiteral@22..23 "2"
                      Whitespace@23..24 " "
                EqualEqual@24..26 "=="
                Whitespace@26..27 " "
                Literal@27..28
                  IntLiteral@27..28 "0""#]],
        );
    }

    #[test]
    fn parse_expr_field() {
        check(
            "a.b.c",
            expect![[r#"
                FieldExpr@0..5
                  FieldExpr@0..3
                    PathExpr@0..1
                      NameRef@0..1
                        Ident@0..1 "a"
                    Period@1..2 "."
                    NameRef@2..3
                      Ident@2..3 "b"
                  Period@3..4 "."
                  NameRef@4..5
                    Ident@4..5 "c""#]],
        );
    }

    #[test]
    fn parse_expr_field_mix_ops() {
        check(
            "vec.xy + 2 * other.zw",
            expect![[r#"
                InfixExpr@0..21
                  FieldExpr@0..7
                    PathExpr@0..3
                      NameRef@0..3
                        Ident@0..3 "vec"
                    Period@3..4 "."
                    NameRef@4..7
                      Ident@4..6 "xy"
                      Whitespace@6..7 " "
                  Plus@7..8 "+"
                  Whitespace@8..9 " "
                  InfixExpr@9..21
                    Literal@9..11
                      IntLiteral@9..10 "2"
                      Whitespace@10..11 " "
                    Star@11..12 "*"
                    Whitespace@12..13 " "
                    FieldExpr@13..21
                      PathExpr@13..18
                        NameRef@13..18
                          Ident@13..18 "other"
                      Period@18..19 "."
                      NameRef@19..21
                        Ident@19..21 "zw""#]],
        );
    }

    #[test]
    fn parse_expr_function_call() {
        check(
            "pow(2, 3)",
            expect![[r#"
                FunctionCall@0..9
                  PathExpr@0..3
                    NameRef@0..3
                      Ident@0..3 "pow"
                  FunctionParamList@3..9
                    ParenLeft@3..4 "("
                    Literal@4..5
                      IntLiteral@4..5 "2"
                    Comma@5..6 ","
                    Whitespace@6..7 " "
                    Literal@7..8
                      IntLiteral@7..8 "3"
                    ParenRight@8..9 ")""#]],
        );
    }

    #[test]
    fn parse_expr_function_call_mixed() {
        check(
            "pow(srgb + 14.0, 3.0) * 2.0",
            expect![[r#"
                InfixExpr@0..27
                  FunctionCall@0..22
                    PathExpr@0..3
                      NameRef@0..3
                        Ident@0..3 "pow"
                    FunctionParamList@3..22
                      ParenLeft@3..4 "("
                      InfixExpr@4..15
                        PathExpr@4..9
                          NameRef@4..9
                            Ident@4..8 "srgb"
                            Whitespace@8..9 " "
                        Plus@9..10 "+"
                        Whitespace@10..11 " "
                        Literal@11..15
                          DecimalFloatLiteral@11..15 "14.0"
                      Comma@15..16 ","
                      Whitespace@16..17 " "
                      Literal@17..20
                        DecimalFloatLiteral@17..20 "3.0"
                      ParenRight@20..21 ")"
                      Whitespace@21..22 " "
                  Star@22..23 "*"
                  Whitespace@23..24 " "
                  Literal@24..27
                    DecimalFloatLiteral@24..27 "2.0""#]],
        );
    }

    #[test]
    fn parse_vec3_initializer() {
        check(
            "vec3<f32>(1.0)",
            expect![[r#"
                FunctionCall@0..14
                  TypeInitializer@0..9
                    Vec3@0..9
                      Vec3@0..4 "vec3"
                      GenericArgList@4..9
                        LessThan@4..5 "<"
                        Float32@5..8
                          Float32@5..8 "f32"
                        GreaterThan@8..9 ">"
                  FunctionParamList@9..14
                    ParenLeft@9..10 "("
                    Literal@10..13
                      DecimalFloatLiteral@10..13 "1.0"
                    ParenRight@13..14 ")""#]],
        );
    }

    #[test]
    fn parse_float_literal() {
        check(
            "0x1f.2",
            expect![[r#"
            Literal@0..6
              HexFloatLiteral@0..6 "0x1f.2""#]],
        );
    }

    #[test]
    fn parse_prefix_expr() {
        check(
            "- 3 + 3",
            expect![[r#"
                InfixExpr@0..7
                  PrefixExpr@0..4
                    Minus@0..1 "-"
                    Whitespace@1..2 " "
                    Literal@2..4
                      IntLiteral@2..3 "3"
                      Whitespace@3..4 " "
                  Plus@4..5 "+"
                  Whitespace@5..6 " "
                  Literal@6..7
                    IntLiteral@6..7 "3""#]],
        );
    }

    #[test]
    fn parse_index() {
        check(
            "a.b[3+2]",
            expect![[r#"
            IndexExpr@0..8
              FieldExpr@0..3
                PathExpr@0..1
                  NameRef@0..1
                    Ident@0..1 "a"
                Period@1..2 "."
                NameRef@2..3
                  Ident@2..3 "b"
              BracketLeft@3..4 "["
              InfixExpr@4..7
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
                InfixExpr@0..12
                  InfixExpr@0..7
                    PathExpr@0..2
                      NameRef@0..2
                        Ident@0..1 "n"
                        Whitespace@1..2 " "
                    Modulo@2..3 "%"
                    Whitespace@3..4 " "
                    Literal@4..7
                      UintLiteral@4..6 "2u"
                      Whitespace@6..7 " "
                  EqualEqual@7..9 "=="
                  Whitespace@9..10 " "
                  Literal@10..12
                    UintLiteral@10..12 "0u""#]],
        );
    }

    #[test]
    fn prefix_exprs() {
        check(
            "!~*&foo",
            expect![[r#"
            PrefixExpr@0..7
              Bang@0..1 "!"
              PrefixExpr@1..7
                Tilde@1..2 "~"
                PrefixExpr@2..7
                  Star@2..3 "*"
                  PrefixExpr@3..7
                    And@3..4 "&"
                    PathExpr@4..7
                      NameRef@4..7
                        Ident@4..7 "foo""#]],
        );
    }

    #[test]
    fn bitcast() {
        check(
            "bitcast<u32>(x)",
            expect![[r#"
                BitcastExpr@0..15
                  Bitcast@0..7 "bitcast"
                  LessThan@7..8 "<"
                  Uint32@8..11
                    Uint32@8..11 "u32"
                  GreaterThan@11..12 ">"
                  ParenExpr@12..15
                    ParenLeft@12..13 "("
                    PathExpr@13..14
                      NameRef@13..14
                        Ident@13..14 "x"
                    ParenRight@14..15 ")""#]],
        );
    }

    #[test]
    fn bitcast_vector() {
        check(
            "bitcast<vec4<u32>>(x)",
            expect![[r#"
                BitcastExpr@0..21
                  Bitcast@0..7 "bitcast"
                  LessThan@7..8 "<"
                  Vec4@8..17
                    Vec4@8..12 "vec4"
                    GenericArgList@12..17
                      LessThan@12..13 "<"
                      Uint32@13..16
                        Uint32@13..16 "u32"
                      GreaterThan@16..17 ">"
                  GreaterThan@17..18 ">"
                  ParenExpr@18..21
                    ParenLeft@18..19 "("
                    PathExpr@19..20
                      NameRef@19..20
                        Ident@19..20 "x"
                    ParenRight@20..21 ")""#]],
        );
    }

    #[test]
    fn bitcast_no_generics() {
        check(
            "bitcast(x)",
            expect![[r#"
                BitcastExpr@0..10
                  Bitcast@0..7 "bitcast"
                  Error@7..7
                  ParenExpr@7..10
                    ParenLeft@7..8 "("
                    PathExpr@8..9
                      NameRef@8..9
                        Ident@8..9 "x"
                    ParenRight@9..10 ")"

                error at 7..8: expected LessThan, but found ParenLeft"#]],
        );
    }
    #[test]
    fn bitcast_in_expr() {
        check(
            "1 + -bitcast<u32>(x) + 1",
            expect![[r#"
                InfixExpr@0..24
                  InfixExpr@0..21
                    Literal@0..2
                      IntLiteral@0..1 "1"
                      Whitespace@1..2 " "
                    Plus@2..3 "+"
                    Whitespace@3..4 " "
                    PrefixExpr@4..21
                      Minus@4..5 "-"
                      BitcastExpr@5..21
                        Bitcast@5..12 "bitcast"
                        LessThan@12..13 "<"
                        Uint32@13..16
                          Uint32@13..16 "u32"
                        GreaterThan@16..17 ">"
                        ParenExpr@17..21
                          ParenLeft@17..18 "("
                          PathExpr@18..19
                            NameRef@18..19
                              Ident@18..19 "x"
                          ParenRight@19..20 ")"
                          Whitespace@20..21 " "
                  Plus@21..22 "+"
                  Whitespace@22..23 " "
                  Literal@23..24
                    IntLiteral@23..24 "1""#]],
        );
    }

    #[test]
    fn deref_field() {
        check(
            "*a.b",
            expect![[r#"
                PrefixExpr@0..4
                  Star@0..1 "*"
                  FieldExpr@1..4
                    PathExpr@1..2
                      NameRef@1..2
                        Ident@1..2 "a"
                    Period@2..3 "."
                    NameRef@3..4
                      Ident@3..4 "b""#]],
        );
    }
    #[test]
    fn deref_field_paren() {
        check(
            "(*a).b",
            expect![[r#"
            FieldExpr@0..6
              ParenExpr@0..4
                ParenLeft@0..1 "("
                PrefixExpr@1..3
                  Star@1..2 "*"
                  PathExpr@2..3
                    NameRef@2..3
                      Ident@2..3 "a"
                ParenRight@3..4 ")"
              Period@4..5 "."
              NameRef@5..6
                Ident@5..6 "b""#]],
        );
    }

    #[test]
    fn shift_right() {
        check(
            "2 >> 3",
            expect![[r#"
            InfixExpr@0..6
              Literal@0..2
                IntLiteral@0..1 "2"
                Whitespace@1..2 " "
              ShiftRight@2..5
                GreaterThan@2..3 ">"
                GreaterThan@3..4 ">"
                Whitespace@4..5 " "
              Literal@5..6
                IntLiteral@5..6 "3""#]],
        );
    }

    #[test]
    fn shift_multiple() {
        check(
            "2 >> 3 + 2 << 4",
            expect![[r#"
            InfixExpr@0..15
              InfixExpr@0..11
                Literal@0..2
                  IntLiteral@0..1 "2"
                  Whitespace@1..2 " "
                ShiftRight@2..5
                  GreaterThan@2..3 ">"
                  GreaterThan@3..4 ">"
                  Whitespace@4..5 " "
                InfixExpr@5..11
                  Literal@5..7
                    IntLiteral@5..6 "3"
                    Whitespace@6..7 " "
                  Plus@7..8 "+"
                  Whitespace@8..9 " "
                  Literal@9..11
                    IntLiteral@9..10 "2"
                    Whitespace@10..11 " "
              ShiftLeft@11..14
                LessThan@11..12 "<"
                LessThan@12..13 "<"
                Whitespace@13..14 " "
              Literal@14..15
                IntLiteral@14..15 "4""#]],
        );
    }
}
