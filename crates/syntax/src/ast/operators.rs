#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum UnaryOperator {
    /// `-`
    Minus,
    /// `!`
    Not,
    /// `&`
    Reference,
    /// `*`
    Dereference,
    /// `~`
    BitNot,
}

impl UnaryOperator {
    #[must_use]
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::Minus => "-",
            Self::Not => "!",
            Self::Reference => "&",
            Self::Dereference => "*",
            Self::BitNot => "~",
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
    #[must_use]
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::Logical(op) => op.symbol(),
            Self::Arithmetic(op) => op.symbol(),
            Self::Comparison(op) => op.symbol(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum LogicOperation {
    And,
    Or,
}

impl LogicOperation {
    #[must_use]
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::And => "&&",
            Self::Or => "||",
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ArithmeticOperation {
    Add,
    Multiply,
    Subtract,
    Divide,
    ShiftLeft,
    ShiftRight,
    BitXor,
    BitOr,
    BitAnd,
    Modulo,
}

impl ArithmeticOperation {
    #[must_use]
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::Add => "+",
            Self::Multiply => "*",
            Self::Subtract => "-",
            Self::Divide => "/",
            Self::ShiftLeft => "<<",
            Self::ShiftRight => ">>",
            Self::BitXor => "^",
            Self::BitOr => "|",
            Self::BitAnd => "&",
            Self::Modulo => "%",
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ComparisonOperation {
    Equality { negated: bool },
    Ordering { ordering: Ordering, strict: bool },
}

impl ComparisonOperation {
    #[must_use]
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::Equality { negated: true } => "==",
            Self::Equality { negated: false } => "!=",
            Self::Ordering {
                ordering: Ordering::Less,
                strict: false,
            } => "<=",
            Self::Ordering {
                ordering: Ordering::Less,
                strict: true,
            } => "<",
            Self::Ordering {
                ordering: Ordering::Greater,
                strict: false,
            } => ">=",
            Self::Ordering {
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
pub enum CompoundOperator {
    Add,
    Multiply,
    Subtract,
    Divide,
    ShiftLeft,
    ShiftRight,
    Modulo,
    BitAnd,
    BitOr,
    BitXor,
}

impl From<CompoundOperator> for BinaryOperation {
    fn from(op: CompoundOperator) -> Self {
        match op {
            CompoundOperator::Add => Self::Arithmetic(ArithmeticOperation::Add),
            CompoundOperator::Multiply => Self::Arithmetic(ArithmeticOperation::Multiply),
            CompoundOperator::Subtract => Self::Arithmetic(ArithmeticOperation::Subtract),
            CompoundOperator::Divide => Self::Arithmetic(ArithmeticOperation::Divide),
            CompoundOperator::ShiftLeft => Self::Arithmetic(ArithmeticOperation::ShiftLeft),
            CompoundOperator::ShiftRight => Self::Arithmetic(ArithmeticOperation::ShiftRight),
            CompoundOperator::Modulo => Self::Arithmetic(ArithmeticOperation::Modulo),
            CompoundOperator::BitAnd => Self::Arithmetic(ArithmeticOperation::BitAnd),
            CompoundOperator::BitOr => Self::Arithmetic(ArithmeticOperation::BitOr),
            CompoundOperator::BitXor => Self::Arithmetic(ArithmeticOperation::BitXor),
        }
    }
}
