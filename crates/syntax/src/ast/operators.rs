#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum UnaryOp {
    Minus,
    Not,
    Ref,
    Deref,
    BitNot,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum BinaryOp {
    LogicOp(LogicOp),
    ArithOp(ArithOp),
    CmpOp(CmpOp),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum LogicOp {
    And,
    Or,
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ArithOp {
    Add,
    Mul,
    Sub,
    Div,
    Shl,
    Shr,
    BitXor,
    BitOr,
    BitAnd,
    Modulo,
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum CmpOp {
    Eq { negated: bool },
    Ord { ordering: Ordering, strict: bool },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Ordering {
    Less,
    Greater,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum CompoundOp {
    Add,
    Mul,
    Sub,
    Div,
    Shl,
    Shr,
    Modulo,
    BitAnd,
    BitOr,
    BitXor,
}

impl From<CompoundOp> for BinaryOp {
    fn from(op: CompoundOp) -> Self {
        match op {
            CompoundOp::Add => BinaryOp::ArithOp(ArithOp::Add),
            CompoundOp::Mul => BinaryOp::ArithOp(ArithOp::Mul),
            CompoundOp::Sub => BinaryOp::ArithOp(ArithOp::Sub),
            CompoundOp::Div => BinaryOp::ArithOp(ArithOp::Div),
            CompoundOp::Shl => BinaryOp::ArithOp(ArithOp::Shl),
            CompoundOp::Shr => BinaryOp::ArithOp(ArithOp::Shr),
            CompoundOp::Modulo => BinaryOp::ArithOp(ArithOp::Modulo),
            CompoundOp::BitAnd => BinaryOp::ArithOp(ArithOp::BitAnd),
            CompoundOp::BitOr => BinaryOp::ArithOp(ArithOp::BitOr),
            CompoundOp::BitXor => BinaryOp::ArithOp(ArithOp::BitXor),
        }
    }
}
