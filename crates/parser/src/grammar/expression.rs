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
        SyntaxKind::FunctionParameters,
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
    SyntaxKind::IntLiteral,
    SyntaxKind::FloatLiteral,
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
}
