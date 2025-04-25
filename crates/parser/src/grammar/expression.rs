// mostly copied from https://github.com/arzg/eldiro/blob/f3d588f8a76e2e4317c1d77ae2758b0781bb5af3/crates/parser/src/grammar.rs

use crate::{marker::CompletedMarker, syntax_kind::SyntaxKind};

use super::*;

pub(crate) fn expression(p: &mut Parser<'_, '_>) {
    if p.at_set(super::STATEMENT_RECOVER_SET) {
        return;
    }
    expression_binding_power(p, 0);
}

fn expression_binding_power(
    p: &mut Parser<'_, '_>,
    minimum_binding_power: u8,
) -> Option<CompletedMarker> {
    let mut left_side = left_side(p)?;

    loop {
        // postfix ops
        if let Some(postfix_op) = postfix_op(p) {
            let (left_binding_power, ()) = postfix_op.binding_power();
            if left_binding_power < minimum_binding_power {
                break;
            }

            let m = left_side.precede(p);
            match postfix_op {
                PostfixOp::Call => {
                    // Calls cannot be made on arbitrary expressions, merely on only a few versions
                    // We have this as an error
                    function_param_list(p);
                    left_side = m.complete(p, SyntaxKind::InvalidFunctionCall);
                },
                PostfixOp::Index => {
                    array_index(p);
                    left_side = m.complete(p, SyntaxKind::IndexExpression);
                },
                PostfixOp::Field => {
                    p.bump();
                    name_ref(p);
                    left_side = m.complete(p, SyntaxKind::FieldExpression);
                },
            }

            continue;
        }

        let infix_op = match binary_operator(p) {
            Some(op) => op,
            None => break,
        };

        let (left_binding_power, right_binding_power) = infix_op.binding_power();

        if left_binding_power < minimum_binding_power {
            break;
        }

        // Eat the operator's token.

        match infix_op {
            BinaryOperator::ShiftLeft => p.bump_compound(SyntaxKind::ShiftLeft),
            BinaryOperator::ShiftRight => p.bump_compound(SyntaxKind::ShiftRight),
            _ => {
                p.bump();
            },
        }

        let m = left_side.precede(p);
        let parsed_rhs = expression_binding_power(p, right_binding_power).is_some();
        left_side = m.complete(p, SyntaxKind::InfixExpression);

        if !parsed_rhs {
            break;
        }
    }

    Some(left_side)
}

fn function_param_list(p: &mut Parser<'_, '_>) {
    list(
        p,
        SyntaxKind::ParenthesisLeft,
        SyntaxKind::ParenthesisRight,
        SyntaxKind::Comma,
        SyntaxKind::FunctionParameterList,
        |p| {
            expression_binding_power(p, 0);
        },
    );
}

fn array_index(p: &mut Parser<'_, '_>) {
    p.expect(SyntaxKind::BracketLeft);
    expression_binding_power(p, 0);
    p.expect(SyntaxKind::BracketRight);
}

fn left_side(p: &mut Parser<'_, '_>) -> Option<CompletedMarker> {
    let cm = if p.at_set(TOKENSET_LITERAL) {
        literal(p)
    } else if p.at(SyntaxKind::Identifier) {
        let m = p.start();
        name_ref(p);
        if p.at(SyntaxKind::ParenthesisLeft) {
            function_param_list(p);
            // Function call, may be a type initialiser too
            m.complete(p, SyntaxKind::FunctionCall)
        } else {
            m.complete(p, SyntaxKind::PathExpression)
        }
    } else if p.at(SyntaxKind::Bitcast) {
        bitcast_expression(p)
    } else if p.at_set(TYPE_SET) {
        let m = p.start();
        super::type_declaration(p).unwrap();
        if p.at(SyntaxKind::ParenthesisLeft) {
            function_param_list(p);
        } else {
            p.error_no_bump(&[SyntaxKind::ParenthesisLeft]);
        }
        m.complete(p, SyntaxKind::TypeInitializer)
    } else if p.at_set(PREFIX_OP_SET) {
        prefix_expression(p)
    } else if p.at(SyntaxKind::ParenthesisLeft) {
        parenthesis_expression(p)
    } else {
        p.error();
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
    fn binding_power(&self) -> (u8, u8) {
        match self {
            BinaryOperator::ShortCircuitOr => (0, 1),
            BinaryOperator::ShortCircuitAnd => (2, 3),
            BinaryOperator::Or => (4, 5),
            BinaryOperator::Xor => (5, 6),
            BinaryOperator::And => (7, 8),
            BinaryOperator::Equals => (9, 10),
            BinaryOperator::LessThan
            | BinaryOperator::GreaterThan
            | BinaryOperator::LessThanEqual
            | BinaryOperator::GreaterThanEqual
            | BinaryOperator::NotEqual => (11, 12),
            BinaryOperator::ShiftLeft | BinaryOperator::ShiftRight => (13, 14),
            BinaryOperator::Add | BinaryOperator::Subtract => (15, 16),
            BinaryOperator::Multiply | BinaryOperator::Divide | BinaryOperator::Modulo => (17, 18),
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
    fn binding_power(&self) -> ((), u8) {
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
    fn binding_power(&self) -> (u8, ()) {
        match self {
            Self::Call | Self::Field | Self::Index => (21, ()),
        }
    }
}

fn postfix_op(p: &mut Parser<'_, '_>) -> Option<PostfixOp> {
    if p.at(SyntaxKind::Period) {
        Some(PostfixOp::Field)
    } else if p.at(SyntaxKind::ParenthesisLeft) {
        Some(PostfixOp::Call)
    } else if p.at(SyntaxKind::BracketLeft) {
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

pub(crate) fn literal(p: &mut Parser<'_, '_>) -> CompletedMarker {
    assert!(p.at_set(TOKENSET_LITERAL));

    let m = p.start();
    p.bump();
    m.complete(p, SyntaxKind::Literal)
}

fn bitcast_expression(p: &mut Parser<'_, '_>) -> CompletedMarker {
    assert!(p.at(SyntaxKind::Bitcast));
    let m = p.start();
    p.bump();
    if !p.eat(SyntaxKind::LessThan) {
        p.error_expected_no_bump(&[SyntaxKind::LessThan]);
        if p.at(SyntaxKind::ParenthesisLeft) {
            parenthesis_expression(p);
        }
        return m.complete(p, SyntaxKind::BitcastExpression);
    }
    let _ = super::type_declaration(p);
    p.expect(SyntaxKind::GreaterThan);

    if !p.at(SyntaxKind::ParenthesisLeft) {
        p.error_expected_no_bump(&[SyntaxKind::ParenthesisLeft]);
        return m.complete(p, SyntaxKind::BitcastExpression);
    }
    parenthesis_expression(p);
    return m.complete(p, SyntaxKind::BitcastExpression);
}

fn prefix_expression(p: &mut Parser<'_, '_>) -> CompletedMarker {
    let m = p.start();
    let op = if p.at(SyntaxKind::Minus) {
        PrefixOp::Negate
    } else if p.at(SyntaxKind::Bang) {
        PrefixOp::Not
    } else if p.at(SyntaxKind::And) {
        PrefixOp::Reference
    } else if p.at(SyntaxKind::Star) {
        PrefixOp::Dereference
    } else if p.at(SyntaxKind::Tilde) {
        PrefixOp::BitNot
    } else {
        p.error();
        return m.complete(p, SyntaxKind::PrefixExpression);
    };

    let ((), right_binding_power) = op.binding_power();

    // Eat the operator's token.
    p.bump();

    expression_binding_power(p, right_binding_power);

    m.complete(p, SyntaxKind::PrefixExpression)
}

fn parenthesis_expression(p: &mut Parser<'_, '_>) -> CompletedMarker {
    assert!(p.at(SyntaxKind::ParenthesisLeft));

    let m = p.start();
    p.bump();
    if p.at(SyntaxKind::ParenthesisRight) {
        // TODO: Better kind of error here. Ideally just EXPR
        p.error_expected_no_bump(&[SyntaxKind::ParenthesisExpression]);
        p.bump();
        return m.complete(p, SyntaxKind::ParenthesisExpression);
    }

    expression_binding_power(p, 0);
    p.expect(SyntaxKind::ParenthesisRight);

    m.complete(p, SyntaxKind::ParenthesisExpression)
}
