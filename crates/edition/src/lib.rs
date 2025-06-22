//! The edition of the shader language.
//! This should live here in a separate crate because we use it in both actual code and codegen.

use std::{error, fmt, str};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[non_exhaustive]
pub enum Edition {
    // The syntax context stuff needs the discriminants to start from 0 and be consecutive.
    Wgsl = 0,
    Wesl0_0_1,
}

impl Edition {
    pub const DEFAULT: Self = Self::Wgsl;
    pub const LATEST: Self = Self::Wgsl;
    pub const CURRENT: Self = Self::Wgsl;
    /// The current latest stable edition, note this is usually not the right choice in code.
    pub const CURRENT_FIXME: Self = Self::Wgsl;

    /// # Panics
    ///
    /// Panics if the value does not correspond to a variant of [`Edition`].
    #[must_use]
    pub fn from_u32(u32: u32) -> Self {
        match u32 {
            0 => Self::Wgsl,
            1 => Self::Wesl0_0_1,
            _ => panic!("invalid edition"),
        }
    }

    #[must_use]
    pub fn at_least_wesl_0_0_1(self) -> bool {
        self >= Self::Wesl0_0_1
    }

    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::Wgsl, Self::Wesl0_0_1].iter().copied()
    }
}

#[derive(Debug)]
pub struct ParseEditionError {
    invalid_input: String,
}

impl error::Error for ParseEditionError {}

impl fmt::Display for ParseEditionError {
    fn fmt(
        &self,
        #[expect(clippy::min_ident_chars, reason = "trait impl")] f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "invalid edition: {}", self.invalid_input)
    }
}

impl str::FromStr for Edition {
    type Err = ParseEditionError;

    fn from_str(
        #[expect(clippy::min_ident_chars, reason = "trait impl")] s: &str
    ) -> Result<Self, Self::Err> {
        match s {
            "WGSL" => Ok(Self::Wgsl),
            "WESL 0.0.1" => Ok(Self::Wesl0_0_1),
            _ => Err(ParseEditionError {
                invalid_input: s.to_owned(),
            }),
        }
    }
}

impl fmt::Display for Edition {
    fn fmt(
        &self,
        #[expect(clippy::min_ident_chars, reason = "trait impl")] f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.write_str(match self {
            Self::Wgsl => "WGSL",
            Self::Wesl0_0_1 => "WESL 0.0.1",
        })
    }
}
