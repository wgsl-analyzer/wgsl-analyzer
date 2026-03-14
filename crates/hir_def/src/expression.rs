use la_arena::Idx;
pub use syntax::ast::operators::*;
use syntax::ast::{self, IncrementDecrement};

use crate::{
    body::BindingId,
    item_tree::Name,
    type_specifier::{IdentExpression, TypeSpecifierId},
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
        operator: UnaryOperator,
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
        operator: AssignmentOperator,
    },
    PhonyAssignment {
        right_side: ExpressionId,
    },
    IncrDecr {
        expression: ExpressionId,
        operator: IncrementDecrement,
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

/// Parses a literal from the given [`ast::LiteralKind`].
///
/// # Panics
///
/// Panics if the literal is invalid.
#[must_use]
pub fn parse_literal(literal: ast::LiteralKind) -> Literal {
    match literal {
        ast::LiteralKind::IntLiteral(literal) => {
            let (text, int_variant) = split_int_suffix(literal.text());
            let value = match text.strip_prefix("0x").or_else(|| text.strip_prefix("0X")) {
                Some(hex) => u64::from_str_radix(hex, 16),
                None => text.parse::<u64>(),
            }
            .expect("invalid literal");

            Literal::Int(value, int_variant)
        },
        ast::LiteralKind::FloatLiteral(literal) => {
            use std::str::FromStr as _;
            // Float suffixes are not accepted by `f32::from_str`. Ignore them
            let (text, float_variant) = split_float_suffix(literal.text());
            let value = match text.strip_prefix("0x").or_else(|| text.strip_prefix("0X")) {
                Some(hex) => parse_hex_float(hex),
                None => f64::from_str(text).expect("invalid literal"),
            };
            Literal::Float(value.to_bits(), float_variant)
        },
        ast::LiteralKind::True(_) => Literal::Bool(true),
        ast::LiteralKind::False(_) => Literal::Bool(false),
    }
}

fn split_int_suffix(number: &str) -> (&str, BuiltinInt) {
    if number.ends_with('u') {
        (&number[0..(number.len() - 1)], BuiltinInt::U32)
    } else if number.ends_with('i') {
        (&number[0..(number.len() - 1)], BuiltinInt::I32)
    } else {
        (number, BuiltinInt::Abstract)
    }
}

fn split_float_suffix(number: &str) -> (&str, BuiltinFloat) {
    // future reference: naga has li and lu suffix for 64bits literals.
    if number.ends_with('f') {
        (&number[0..(number.len() - 1)], BuiltinFloat::F32)
    } else if number.ends_with('h') {
        (&number[0..(number.len() - 1)], BuiltinFloat::F16)
    } else {
        (number, BuiltinFloat::Abstract)
    }
}

/// Parses a hex float string (without the `0x`/`0X` prefix or type suffix).
///
/// Accepts formats like:
/// - `a.fp+2` (whole.fraction p exponent)
/// - `1p9` (whole p exponent, no fraction)
/// - `.3` (no whole, fraction only, no exponent)
/// - `ff.13p13` (multi-digit whole and fraction)
///
/// # Panics
///
/// Panics if the input is malformed.
fn parse_hex_float(hex: &str) -> f64 {
    // Split on 'p' or 'P' to get mantissa and exponent parts
    let (mantissa_str, exponent) = match hex.find(['p', 'P']) {
        Some(pos) => {
            let exp: i32 = hex[pos + 1..].parse().expect("invalid hex float exponent");
            (&hex[..pos], exp)
        },
        None => (hex, 0),
    };

    // Split mantissa on '.' to get whole and fractional parts
    let (whole_str, frac_str) = match mantissa_str.find('.') {
        Some(pos) => (&mantissa_str[..pos], &mantissa_str[pos + 1..]),
        None => (mantissa_str, ""),
    };

    // Parse the whole part
    let whole: f64 = if whole_str.is_empty() {
        0.0
    } else {
        #[expect(
            clippy::as_conversions,
            reason = "u64 to f64 may lose precision but is correct for float parsing"
        )]
        {
            u64::from_str_radix(whole_str, 16).expect("invalid hex float whole part") as f64
        }
    };

    // Parse the fractional part: each hex digit is worth 1/16 of the previous
    let mut frac = 0.0_f64;
    let mut place = 1.0_f64 / 16.0;
    for ch in frac_str.chars() {
        let digit = ch.to_digit(16).expect("invalid hex float fraction digit");
        frac += f64::from(digit) * place;
        place /= 16.0;
    }

    (whole + frac) * f64::exp2(f64::from(exponent))
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

#[cfg(test)]
#[expect(clippy::float_cmp, reason = "hex float parsing produces exact results")]
mod hex_float_tests {
    use super::parse_hex_float;

    #[test]
    fn simple_whole() {
        assert_eq!(parse_hex_float("1p0"), 1.0);
        assert_eq!(parse_hex_float("ap0"), 10.0);
        assert_eq!(parse_hex_float("ffp0"), 255.0);
    }

    #[test]
    fn with_exponent() {
        assert_eq!(parse_hex_float("1p9"), 512.0);
        assert_eq!(parse_hex_float("1p+4"), 16.0);
        assert_eq!(parse_hex_float("1p-1"), 0.5);
    }

    #[test]
    fn with_fraction() {
        assert_eq!(parse_hex_float("0.0"), 0.0);
        assert_eq!(parse_hex_float("1.0"), 1.0);
        assert_eq!(parse_hex_float("0.8"), 0.5);
        assert_eq!(parse_hex_float("0.4"), 0.25);
    }

    #[test]
    fn fraction_and_exponent() {
        // 0xa.fp+2 = (10 + 15/16) * 4 = 43.75
        assert_eq!(parse_hex_float("a.fp+2"), 43.75);
        // 0x1.fp-4 = (1 + 15/16) * 2^-4 = 0.12109375
        assert_eq!(parse_hex_float("1.fp-4"), 0.121_093_75);
        // 0x3.2p+2 = (3 + 2/16) * 4 = 12.5
        assert_eq!(parse_hex_float("3.2p+2"), 12.5);
    }

    #[test]
    fn no_exponent_with_dot() {
        assert_eq!(parse_hex_float(".3"), 0.1875);
        assert_eq!(parse_hex_float("a."), 10.0);
    }

    #[test]
    fn multi_digit_fraction() {
        // 0xff.13p13 = (255 + 1/16 + 3/256) * 2^13
        let expected = 255.074_218_75 * 8192.0;
        assert_eq!(parse_hex_float("ff.13p13"), expected);
    }

    #[test]
    fn uppercase() {
        assert_eq!(parse_hex_float("1P+4"), 16.0);
        assert_eq!(parse_hex_float("A.FP+2"), 43.75);
    }

    #[test]
    fn wgsl_spec_example() {
        // 0x80.8p-5 = (128 + 8/16) * 2^-5 = 4.015625
        assert_eq!(parse_hex_float("80.8p-5"), 4.015_625);
    }
}
