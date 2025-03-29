#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum UnaryOp {
    Minus,
    Not,
    Reference,
    Dereference,
    BitNot,
}

impl UnaryOp {
    pub fn symbol(self) -> &'static str {
        match self {
            UnaryOp::Minus => "-",
            UnaryOp::Not => "!",
            UnaryOp::Reference => "&",
            UnaryOp::Dereference => "*",
            UnaryOp::BitNot => "~",
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum BinaryOperation {
    Logical(LogicOperation),
    Arithmetic(ArithmeticOperation),
    Comparison(ComparisonOperation),
}

impl BinaryOperation {
    pub fn symbol(self) -> &'static str {
        match self {
            BinaryOperation::Logical(op) => op.symbol(),
            BinaryOperation::Arithmetic(op) => op.symbol(),
            BinaryOperation::Comparison(op) => op.symbol(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum LogicOperation {
    And,
    Or,
}

impl LogicOperation {
    pub fn symbol(self) -> &'static str {
        match self {
            LogicOperation::And => "&&",
            LogicOperation::Or => "||",
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ArithmeticOperation {
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

impl ArithmeticOperation {
    pub fn symbol(self) -> &'static str {
        match self {
            ArithmeticOperation::Add => "+",
            ArithmeticOperation::Mul => "*",
            ArithmeticOperation::Sub => "-",
            ArithmeticOperation::Div => "/",
            ArithmeticOperation::Shl => "<<",
            ArithmeticOperation::Shr => ">>",
            ArithmeticOperation::BitXor => "^",
            ArithmeticOperation::BitOr => "|",
            ArithmeticOperation::BitAnd => "&",
            ArithmeticOperation::Modulo => "%",
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ComparisonOperation {
    Eq { negated: bool },
    Ord { ordering: Ordering, strict: bool },
}

impl ComparisonOperation {
    pub fn symbol(self) -> &'static str {
        match self {
            ComparisonOperation::Eq { negated: true } => "==",
            ComparisonOperation::Eq { negated: false } => "!=",
            ComparisonOperation::Ord {
                ordering: Ordering::Less,
                strict: false,
            } => "<=",
            ComparisonOperation::Ord {
                ordering: Ordering::Less,
                strict: true,
            } => "<",
            ComparisonOperation::Ord {
                ordering: Ordering::Greater,
                strict: false,
            } => ">=",
            ComparisonOperation::Ord {
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

impl From<CompoundOp> for BinaryOperation {
    fn from(op: CompoundOp) -> Self {
        match op {
            CompoundOp::Add => BinaryOperation::Arithmetic(ArithmeticOperation::Add),
            CompoundOp::Mul => BinaryOperation::Arithmetic(ArithmeticOperation::Mul),
            CompoundOp::Sub => BinaryOperation::Arithmetic(ArithmeticOperation::Sub),
            CompoundOp::Div => BinaryOperation::Arithmetic(ArithmeticOperation::Div),
            CompoundOp::Shl => BinaryOperation::Arithmetic(ArithmeticOperation::Shl),
            CompoundOp::Shr => BinaryOperation::Arithmetic(ArithmeticOperation::Shr),
            CompoundOp::Modulo => BinaryOperation::Arithmetic(ArithmeticOperation::Modulo),
            CompoundOp::BitAnd => BinaryOperation::Arithmetic(ArithmeticOperation::BitAnd),
            CompoundOp::BitOr => BinaryOperation::Arithmetic(ArithmeticOperation::BitOr),
            CompoundOp::BitXor => BinaryOperation::Arithmetic(ArithmeticOperation::BitXor),
        }
    }
}
