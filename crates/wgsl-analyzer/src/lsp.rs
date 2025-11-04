//! Custom LSP definitions and protocol conversions.

use core::fmt;

pub mod extensions;

pub(crate) mod capabilities;
pub(crate) mod from_proto;
pub(crate) mod to_proto;
pub(crate) mod utilities;

#[derive(Debug)]
pub(crate) struct LspError {
    pub(crate) code: i32,
    pub(crate) message: String,
}

impl LspError {
    pub(crate) const fn new(
        code: i32,
        message: String,
    ) -> Self {
        Self { code, message }
    }
}

impl fmt::Display for LspError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "Language Server request failed with {}. ({})",
            self.code, self.message
        )
    }
}

impl std::error::Error for LspError {}
