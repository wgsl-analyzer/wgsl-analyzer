#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum UnaryOperator {
    /// `-`
    Negation,
    /// `!`
    LogicalNegation,
    /// `&`
    AddressOf,
    /// `*`
    Indirection,
    /// `~`
    BitwiseComplement,
}

impl UnaryOperator {
    #[must_use]
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::Negation => "-",
            Self::LogicalNegation => "!",
            Self::AddressOf => "&",
            Self::Indirection => "*",
            Self::BitwiseComplement => "~",
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
    ShortCircuitAnd,
    ShortCircuitOr,
}

impl LogicOperation {
    #[must_use]
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::ShortCircuitAnd => "&&",
            Self::ShortCircuitOr => "||",
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ArithmeticOperation {
    Addition,
    Multiplication,
    Subtraction,
    Division,
    ShiftLeft,
    ShiftRight,
    BitwiseXor,
    BitwiseOr,
    BitwiseAnd,
    Remainder,
}

impl ArithmeticOperation {
    #[must_use]
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::Addition => "+",
            Self::Multiplication => "*",
            Self::Subtraction => "-",
            Self::Division => "/",
            Self::ShiftLeft => "<<",
            Self::ShiftRight => ">>",
            Self::BitwiseXor => "^",
            Self::BitwiseOr => "|",
            Self::BitwiseAnd => "&",
            Self::Remainder => "%",
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
pub enum AssignmentOperator {
    PlusEqual,
    TimesEqual,
    MinusEqual,
    DivisionEqual,
    ShiftLeftAssign,
    ShiftRightAssign,
    ModuloEqual,
    AndEqual,
    OrEqual,
    XorEqual,
}

impl From<AssignmentOperator> for BinaryOperation {
    fn from(op: AssignmentOperator) -> Self {
        match op {
            AssignmentOperator::PlusEqual => Self::Arithmetic(ArithmeticOperation::Addition),
            AssignmentOperator::TimesEqual => Self::Arithmetic(ArithmeticOperation::Multiplication),
            AssignmentOperator::MinusEqual => Self::Arithmetic(ArithmeticOperation::Subtraction),
            AssignmentOperator::DivisionEqual => Self::Arithmetic(ArithmeticOperation::Division),
            AssignmentOperator::ShiftLeftAssign => Self::Arithmetic(ArithmeticOperation::ShiftLeft),
            AssignmentOperator::ShiftRightAssign => {
                Self::Arithmetic(ArithmeticOperation::ShiftRight)
            },
            AssignmentOperator::ModuloEqual => Self::Arithmetic(ArithmeticOperation::Remainder),
            AssignmentOperator::AndEqual => Self::Arithmetic(ArithmeticOperation::BitwiseAnd),
            AssignmentOperator::OrEqual => Self::Arithmetic(ArithmeticOperation::BitwiseOr),
            AssignmentOperator::XorEqual => Self::Arithmetic(ArithmeticOperation::BitwiseXor),
        }
    }
}
