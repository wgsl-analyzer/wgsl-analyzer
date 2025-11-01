use la_arena::Idx;
pub use syntax::ast::operators::*;
use syntax::ast::{self, IncrementDecrement};

use crate::{
    body::BindingId,
    module_data::Name,
    type_specifier::{IdentExpression, TypeSpecifier, TypeSpecifierId},
};

pub type ExpressionId = Idx<Expression>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Literal {
    Int(u64, BuiltinInt),
    Float(u64, BuiltinFloat),
    Bool(bool),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BuiltinFloat {
    F16,
    F32,
    Abstract,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BuiltinInt {
    I32,
    U32,
    Abstract,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Missing,
    BinaryOperation {
        left_side: ExpressionId,
        right_side: ExpressionId,
        operation: BinaryOperation,
    },
    UnaryOperator {
        expression: ExpressionId,
        op: UnaryOperator,
    },
    Field {
        expression: ExpressionId,
        name: Name,
    },
    Call {
        ident_expression: IdentExpression,
        arguments: Vec<ExpressionId>,
    },
    Index {
        left_side: ExpressionId,
        index: ExpressionId,
    },
    Literal(Literal),
    IdentExpression(IdentExpression),
}

pub type StatementId = Idx<Statement>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Missing,
    Compound {
        statements: Vec<StatementId>,
    },
    Let {
        binding_id: BindingId,
        type_ref: Option<TypeSpecifierId>,
        initializer: Option<ExpressionId>,
    },
    Const {
        binding_id: BindingId,
        type_ref: Option<TypeSpecifierId>,
        initializer: Option<ExpressionId>,
    },
    Variable {
        binding_id: BindingId,
        type_ref: Option<TypeSpecifierId>,
        initializer: Option<ExpressionId>,
        template_parameters: Vec<ExpressionId>,
    },
    Return {
        expression: Option<ExpressionId>,
    },
    Assignment {
        left_side: ExpressionId,
        right_side: ExpressionId,
    },
    CompoundAssignment {
        left_side: ExpressionId,
        right_side: ExpressionId,
        op: AssignmentOperator,
    },
    PhonyAssignment {
        right_side: ExpressionId,
    },
    IncrDecr {
        expression: ExpressionId,
        op: IncrementDecrement,
    },
    If {
        condition: ExpressionId,
        block: StatementId,
        else_if_blocks: Vec<StatementId>,
        else_block: Option<StatementId>,
    },
    For {
        initializer: Option<StatementId>,
        condition: Option<ExpressionId>,
        continuing_part: Option<StatementId>,
        block: StatementId,
    },
    While {
        condition: ExpressionId,
        block: StatementId,
    },
    Switch {
        expression: ExpressionId,
        case_blocks: Vec<(Vec<SwitchCaseSelector>, StatementId)>,
    },
    Loop {
        body: StatementId,
    },
    Discard,
    Break,
    Continue,
    Continuing {
        block: StatementId,
    },
    BreakIf {
        condition: ExpressionId,
    },
    Assert {
        expression: ExpressionId,
    },
    Expression {
        expression: ExpressionId,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SwitchCaseSelector {
    Expression(ExpressionId),
    Default,
}

/// Parses a literal from the given `ast::LiteralKind`.
///
/// # Panics
///
/// Panics if the literal is invalid.
#[must_use]
pub fn parse_literal(literal: ast::LiteralKind) -> Literal {
    match literal {
        ast::LiteralKind::IntLiteral(literal) => {
            let (text, suffix) = split_number_suffix(literal.text());
            let value = match text.strip_prefix("0x").or_else(|| text.strip_prefix("0X")) {
                Some(hex) => u64::from_str_radix(hex, 16),
                None => text.parse::<u64>(),
            }
            .expect("invalid literal");
            let int_variant = match suffix {
                Some('u') => BuiltinInt::U32,
                Some('i') => BuiltinInt::I32,
                _ => BuiltinInt::Abstract,
            };
            Literal::Int(value, int_variant)
        },
        ast::LiteralKind::FloatLiteral(literal) => {
            use std::str::FromStr as _;
            // Float suffixes are not accepted by `f32::from_str`. Ignore them
            let (text, suffix) = split_number_suffix(literal.text());
            let value = match text.strip_prefix("0x").or_else(|| text.strip_prefix("0X")) {
                Some(_hex) => Ok(0_f64), // TODO: Hex floats need to be handled
                None => f64::from_str(text),
            }
            .expect("invalid literal");
            // future reference: naga has li and lu suffix for 64bits literals.
            let float_variant = match suffix {
                Some('f') => BuiltinFloat::F32,
                Some('h') => BuiltinFloat::F16,
                _ => BuiltinFloat::Abstract,
            };
            Literal::Float(value.to_bits(), float_variant)
        },
        ast::LiteralKind::True(_) => Literal::Bool(true),
        ast::LiteralKind::False(_) => Literal::Bool(false),
    }
}

fn split_number_suffix(number: &str) -> (&str, Option<char>) {
    if let Some(last_char) = number.chars().next_back()
        && last_char.is_alphabetic()
    {
        (
            &number[0..(number.len() - last_char.len_utf8())],
            Some(last_char),
        )
    } else {
        (number, None)
    }
}

impl Expression {
    pub fn walk_child_expressions<Function: FnMut(ExpressionId)>(
        &self,
        mut function: Function,
    ) {
        match self {
            Self::BinaryOperation {
                left_side,
                right_side,
                ..
            } => {
                function(*left_side);
                function(*right_side);
            },
            Self::UnaryOperator { expression, .. } | Self::Field { expression, .. } => {
                function(*expression);
            },
            Self::Call {
                ident_expression,
                arguments,
                ..
            } => {
                ident_expression
                    .template_parameters
                    .iter()
                    .copied()
                    .chain(arguments.iter().copied())
                    .for_each(function);
            },
            Self::Index { left_side, index } => {
                function(*left_side);
                function(*index);
            },
            Self::IdentExpression(IdentExpression {
                template_parameters,
                ..
            }) => {
                template_parameters.iter().copied().for_each(function);
            },
            Self::Missing | Self::Literal(_) => {},
        }
    }
}
