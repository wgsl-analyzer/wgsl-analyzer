#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum UnaryOp {
    Minus,
    Not,
    Ref,
    Deref,
    BitNot,
}

impl UnaryOp {
    pub fn symbol(self) -> &'static str {
        match self {
            UnaryOp::Minus => "-",
            UnaryOp::Not => "!",
            UnaryOp::Ref => "&",
            UnaryOp::Deref => "*",
            UnaryOp::BitNot => "~",
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum BinaryOp {
    LogicOp(LogicOp),
    ArithOp(ArithOp),
    CmpOp(CmpOp),
}

impl BinaryOp {
    pub fn symbol(self) -> &'static str {
        match self {
            BinaryOp::LogicOp(op) => op.symbol(),
            BinaryOp::ArithOp(op) => op.symbol(),
            BinaryOp::CmpOp(op) => op.symbol(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum LogicOp {
    And,
    Or,
}

impl LogicOp {
    pub fn symbol(self) -> &'static str {
        match self {
            LogicOp::And => "&&",
            LogicOp::Or => "||",
        }
    }
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

impl ArithOp {
    pub fn symbol(self) -> &'static str {
        match self {
            ArithOp::Add => "+",
            ArithOp::Mul => "*",
            ArithOp::Sub => "-",
            ArithOp::Div => "/",
            ArithOp::Shl => "<<",
            ArithOp::Shr => ">>",
            ArithOp::BitXor => "^",
            ArithOp::BitOr => "|",
            ArithOp::BitAnd => "&",
            ArithOp::Modulo => "%",
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum CmpOp {
    Eq { negated: bool },
    Ord { ordering: Ordering, strict: bool },
}

impl CmpOp {
    pub fn symbol(self) -> &'static str {
        match self {
            CmpOp::Eq { negated: true } => "==",
            CmpOp::Eq { negated: false } => "!=",
            CmpOp::Ord {
                ordering: Ordering::Less,
                strict: false,
            } => "<=",
            CmpOp::Ord {
                ordering: Ordering::Less,
                strict: true,
            } => "<",
            CmpOp::Ord {
                ordering: Ordering::Greater,
                strict: false,
            } => ">=",
            CmpOp::Ord {
                ordering: Ordering::Greater,
                strict: true,
            } => ">",
        }
    }
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
