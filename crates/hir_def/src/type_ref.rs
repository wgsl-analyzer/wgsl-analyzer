use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum VecDimensionality {
    Two,
    Three,
    Four,
}

impl fmt::Display for VecDimensionality {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Two => formatter.write_str("2"),
            Self::Three => formatter.write_str("3"),
            Self::Four => formatter.write_str("4"),
        }
    }
}
