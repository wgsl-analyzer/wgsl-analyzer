use la_arena::Idx;
use syntax::ast::{self, IncrDecr};

use crate::{
    body::BindingId,
    db::Interned,
    module_data::Name,
    type_ref::{AccessMode, StorageClass, TypeRef, VecDimensionality},
};

pub use syntax::ast::operators::*;

pub type ExprId = Idx<Expr>;

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

// impl From<ast::InferredInitializer> for BuiltinInitializer {
//     fn from(initializer: ast::InferredInitializer) -> Self {
//         use ast::InferredInitializerType::*;
//         use VecDimensionality::*;
//         match initializer
//             .ty()
//             .expect("InferredInitializer is only created from a single token, so that token definitely exists")
//         {
//             Array(_) => BuiltinInitializer::Array,
//             Mat2x2(_) => BuiltinInitializer::Matrix {
//                 rows: Two,
//                 columns: Two,
//             },
//             Mat2x3(_) => BuiltinInitializer::Matrix {
//                 rows: Two,
//                 columns: Three,
//             },
//             Mat2x4(_) => BuiltinInitializer::Matrix {
//                 rows: Two,
//                 columns: Four,
//             },
//             Mat3x2(_) => BuiltinInitializer::Matrix {
//                 rows: Three,
//                 columns: Two,
//             },
//             Mat3x3(_) => BuiltinInitializer::Matrix {
//                 rows: Three,
//                 columns: Three,
//             },
//             Mat3x4(_) => BuiltinInitializer::Matrix {
//                 rows: Three,
//                 columns: Four,
//             },
//             Mat4x2(_) => BuiltinInitializer::Matrix {
//                 rows: Four,
//                 columns: Two,
//             },
//             Mat4x3(_) => BuiltinInitializer::Matrix {
//                 rows: Four,
//                 columns: Three,
//             },
//             Mat4x4(_) => BuiltinInitializer::Matrix {
//                 rows: Four,
//                 columns: Four,
//             },
//             Vec2(_) => BuiltinInitializer::Vec(Two),
//             Vec3(_) => BuiltinInitializer::Vec(Three),
//             Vec4(_) => BuiltinInitializer::Vec(Four),
//         }
//     }
// }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Callee {
    InferredComponentMatrix {
        rows: VecDimensionality,
        columns: VecDimensionality,
    },
    InferredComponentVec(VecDimensionality),
    InferredComponentArray,
    Name(Name),
    Type(Interned<TypeRef>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Missing,
    BinaryOp {
        lhs: ExprId,
        rhs: ExprId,
        op: BinaryOp,
    },
    UnaryOp {
        expr: ExprId,
        op: UnaryOp,
    },
    Field {
        expr: ExprId,
        name: Name,
    },
    Call {
        callee: Callee,
        args: Vec<ExprId>,
    },
    Index {
        lhs: ExprId,
        index: ExprId,
    },
    Bitcast {
        expr: ExprId,
        ty: Interned<TypeRef>,
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
        type_ref: Option<Interned<TypeRef>>,
        initializer: Option<ExprId>,
    },
    ConstStatement {
        binding_id: BindingId,
        type_ref: Option<Interned<TypeRef>>,
        initializer: Option<ExprId>,
    },
    VariableStatement {
        binding_id: BindingId,
        type_ref: Option<Interned<TypeRef>>,
        initializer: Option<ExprId>,
        storage_class: Option<StorageClass>,
        access_mode: Option<AccessMode>,
    },
    Return {
        expr: Option<ExprId>,
    },
    Assignment {
        lhs: ExprId,
        rhs: ExprId,
    },
    CompoundAssignment {
        lhs: ExprId,
        rhs: ExprId,
        op: CompoundOp,
    },
    IncrDecr {
        expr: ExprId,
        op: IncrDecr,
    },
    If {
        condition: ExprId,
        block: StatementId,
        else_if_blocks: Vec<StatementId>,
        else_block: Option<StatementId>,
    },
    For {
        initializer: Option<StatementId>,
        condition: Option<ExprId>,
        continuing_part: Option<StatementId>,
        block: StatementId,
    },
    While {
        condition: ExprId,
        block: StatementId,
    },
    Switch {
        expr: ExprId,
        case_blocks: Vec<(Vec<ExprId>, StatementId)>,
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
    Expr {
        expr: ExprId,
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
        }
        ast::LiteralKind::UintLiteral(lit) => {
            let text = lit.text().trim_end_matches('u');
            let value = match text.strip_prefix("0x") {
                Some(hex) => u64::from_str_radix(hex, 16),
                None => text.parse(),
            }
            .expect("invalid literal");

            Literal::Uint(value, BuiltinUint::U32)
        }
        ast::LiteralKind::HexFloatLiteral(_) => Literal::Float(0, BuiltinFloat::F32),
        ast::LiteralKind::DecimalFloatLiteral(lit) => {
            use std::str::FromStr;
            // Float suffixes aren't accepted by `f32::from_str`. Ignore them
            let text = lit.text().trim_end_matches(char::is_alphabetic);
            let _value = f32::from_str(text).expect("invalid literal");
            Literal::Float(0, BuiltinFloat::F32)
        }
        ast::LiteralKind::True(_) => Literal::Bool(true),
        ast::LiteralKind::False(_) => Literal::Bool(false),
    }
}

impl Expr {
    pub fn walk_child_exprs(&self, mut f: impl FnMut(ExprId)) {
        match self {
            Expr::BinaryOp { lhs, rhs, .. } => {
                f(*lhs);
                f(*rhs);
            }
            Expr::UnaryOp { expr, .. } => {
                f(*expr);
            }
            Expr::Field { expr, .. } => {
                f(*expr);
            }
            Expr::Call { args, .. } => {
                args.iter().copied().for_each(f);
            }
            Expr::Index { lhs, index } => {
                f(*lhs);
                f(*index);
            }
            Expr::Bitcast { expr, .. } => {
                f(*expr);
            }
            Expr::Missing => {}
            Expr::Literal(_) => {}
            Expr::Path(_) => {}
        }
    }
}
