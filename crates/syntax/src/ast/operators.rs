#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum UnaryOperator {
    Minus,
    Not,
    Reference,
    Dereference,
    BitNot,
}

impl UnaryOperator {
    pub fn symbol(self) -> &'static str {
        match self {
            UnaryOperator::Minus => "-",
            UnaryOperator::Not => "!",
            UnaryOperator::Reference => "&",
            UnaryOperator::Dereference => "*",
            UnaryOperator::BitNot => "~",
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
    pub fn symbol(self) -> &'static str {
        match self {
            ArithmeticOperation::Add => "+",
            ArithmeticOperation::Multiply => "*",
            ArithmeticOperation::Subtract => "-",
            ArithmeticOperation::Divide => "/",
            ArithmeticOperation::ShiftLeft => "<<",
            ArithmeticOperation::ShiftRight => ">>",
            ArithmeticOperation::BitXor => "^",
            ArithmeticOperation::BitOr => "|",
            ArithmeticOperation::BitAnd => "&",
            ArithmeticOperation::Modulo => "%",
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ComparisonOperation {
    Equality { negated: bool },
    Ordering { ordering: Ordering, strict: bool },
}

impl ComparisonOperation {
    pub fn symbol(self) -> &'static str {
        match self {
            ComparisonOperation::Equality { negated: true } => "==",
            ComparisonOperation::Equality { negated: false } => "!=",
            ComparisonOperation::Ordering {
                ordering: Ordering::Less,
                strict: false,
            } => "<=",
            ComparisonOperation::Ordering {
                ordering: Ordering::Less,
                strict: true,
            } => "<",
            ComparisonOperation::Ordering {
                ordering: Ordering::Greater,
                strict: false,
            } => ">=",
            ComparisonOperation::Ordering {
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
            CompoundOperator::Add => BinaryOperation::Arithmetic(ArithmeticOperation::Add),
            CompoundOperator::Multiply => {
                BinaryOperation::Arithmetic(ArithmeticOperation::Multiply)
            },
            CompoundOperator::Subtract => {
                BinaryOperation::Arithmetic(ArithmeticOperation::Subtract)
            },
            CompoundOperator::Divide => BinaryOperation::Arithmetic(ArithmeticOperation::Divide),
            CompoundOperator::ShiftLeft => {
                BinaryOperation::Arithmetic(ArithmeticOperation::ShiftLeft)
            },
            CompoundOperator::ShiftRight => {
                BinaryOperation::Arithmetic(ArithmeticOperation::ShiftRight)
            },
            CompoundOperator::Modulo => BinaryOperation::Arithmetic(ArithmeticOperation::Modulo),
            CompoundOperator::BitAnd => BinaryOperation::Arithmetic(ArithmeticOperation::BitAnd),
            CompoundOperator::BitOr => BinaryOperation::Arithmetic(ArithmeticOperation::BitOr),
            CompoundOperator::BitXor => BinaryOperation::Arithmetic(ArithmeticOperation::BitXor),
        }
    }
}
