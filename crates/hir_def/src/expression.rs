use la_arena::Idx;
pub use syntax::ast::operators::*;
use syntax::ast::{self, IncrDecr};

use crate::{
    body::BindingId,
    db::Interned,
    module_data::Name,
    type_ref::{AccessMode, StorageClass, TypeReference, VecDimensionality},
};

pub type ExpressionId = Idx<Expression>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Literal {
    Int(i64, BuiltinInt),
    Uint(u64, BuiltinUint),
    Float(u32, BuiltinFloat), // FIXME: f32 is not Eq
    Bool(bool),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BuiltinFloat {
    F32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BuiltinInt {
    I32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BuiltinUint {
    U32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Callee {
    InferredComponentMatrix {
        rows: VecDimensionality,
        columns: VecDimensionality,
    },
    InferredComponentVec(VecDimensionality),
    InferredComponentArray,
    Name(Name),
    Type(Interned<TypeReference>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Missing,
    BinaryOperation {
        left_side: ExpressionId,
        right_side: ExpressionId,
        operation: BinaryOperation,
    },
    UnaryOp {
        expression: ExpressionId,
        op: UnaryOp,
    },
    Field {
        expression: ExpressionId,
        name: Name,
    },
    Call {
        callee: Callee,
        arguments: Vec<ExpressionId>,
    },
    Index {
        left_side: ExpressionId,
        index: ExpressionId,
    },
    Bitcast {
        expression: ExpressionId,
        ty: Interned<TypeReference>,
    },
    Literal(Literal),
    Path(Name),
}

pub type StatementId = Idx<Statement>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Missing,
    Compound {
        statements: Vec<StatementId>,
    },
    LetStatement {
        binding_id: BindingId,
        type_ref: Option<Interned<TypeReference>>,
        initializer: Option<ExpressionId>,
    },
    ConstStatement {
        binding_id: BindingId,
        type_ref: Option<Interned<TypeReference>>,
        initializer: Option<ExpressionId>,
    },
    VariableStatement {
        binding_id: BindingId,
        type_ref: Option<Interned<TypeReference>>,
        initializer: Option<ExpressionId>,
        storage_class: Option<StorageClass>,
        access_mode: Option<AccessMode>,
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
        op: CompoundOp,
    },
    IncrDecr {
        expression: ExpressionId,
        op: IncrDecr,
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
        case_blocks: Vec<(Vec<ExpressionId>, StatementId)>,
        default_block: Option<StatementId>,
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
    // only function calls are allowed in this position. TODO add diagnostic
    Expression {
        expression: ExpressionId,
    },
}

pub fn parse_literal(lit: ast::LiteralKind) -> Literal {
    match lit {
        ast::LiteralKind::IntLiteral(lit) => {
            let text = lit.text().trim_end_matches('i');
            let (text, negative) = match text.strip_prefix('-') {
                Some(new) => (new, true),
                None => (text, false),
            };
            let mut value = match text.strip_prefix("0x") {
                Some(hex) => i64::from_str_radix(hex, 16),
                None => text.parse(),
            }
            .expect("invalid literal");

            if negative {
                value = -value;
            }

            Literal::Int(value, BuiltinInt::I32)
        },
        ast::LiteralKind::UintLiteral(lit) => {
            let text = lit.text().trim_end_matches('u');
            let value = match text.strip_prefix("0x") {
                Some(hex) => u64::from_str_radix(hex, 16),
                None => text.parse(),
            }
            .expect("invalid literal");

            Literal::Uint(value, BuiltinUint::U32)
        },
        ast::LiteralKind::HexFloatLiteral(_) => Literal::Float(0, BuiltinFloat::F32),
        ast::LiteralKind::DecimalFloatLiteral(lit) => {
            use std::str::FromStr;
            // Float suffixes are not accepted by `f32::from_str`. Ignore them
            let text = lit.text().trim_end_matches(char::is_alphabetic);
            let _value = f32::from_str(text).expect("invalid literal");
            Literal::Float(0, BuiltinFloat::F32)
        },
        ast::LiteralKind::True(_) => Literal::Bool(true),
        ast::LiteralKind::False(_) => Literal::Bool(false),
    }
}

impl Expression {
    pub fn walk_child_expressions(
        &self,
        mut f: impl FnMut(ExpressionId),
    ) {
        match self {
            Expression::BinaryOperation {
                left_side,
                right_side,
                ..
            } => {
                f(*left_side);
                f(*right_side);
            },
            Expression::UnaryOp { expression, .. } => {
                f(*expression);
            },
            Expression::Field { expression, .. } => {
                f(*expression);
            },
            Expression::Call { arguments, .. } => {
                arguments.iter().copied().for_each(f);
            },
            Expression::Index { left_side, index } => {
                f(*left_side);
                f(*index);
            },
            Expression::Bitcast { expression, .. } => {
                f(*expression);
            },
            Expression::Missing => {},
            Expression::Literal(_) => {},
            Expression::Path(_) => {},
        }
    }
}
