//! This modules defines type to represent changes to the source code, that flow
//! from the server to the client.
//!
//! It can be viewed as a dual for `Change`.

use std::fmt;

/// An annotation ID associated with an indel, to describe changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChangeAnnotationId(u32);

impl fmt::Display for ChangeAnnotationId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        fmt::Display::fmt(&self.0, formatter)
    }
}

#[derive(Debug, Clone)]
pub struct ChangeAnnotation {
    pub label: String,
    pub needs_confirmation: bool,
    pub description: Option<String>,
}
