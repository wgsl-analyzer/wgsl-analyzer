//! Custom LSP definitions and protocol conversions.

use core::fmt;

pub mod extensions;

pub(crate) mod capabilities;
pub(crate) mod from_proto;
pub(crate) mod semantic_tokens;
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
    #[expect(clippy::min_ident_chars, reason = "trait impl")]
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            f,
            "Language Server request failed with {}. ({})",
            self.code, self.message
        )
    }
}

impl std::error::Error for LspError {}
