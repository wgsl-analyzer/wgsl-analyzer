//! Markdown formatting.
//!
//! Sometimes, we want to display a "rich text" in the UI. At the moment, we use
//! markdown for this purpose. It doesn't feel like a right option, but that's
//! what is used by LSP, so let's keep it simple.

use std::fmt;

#[derive(Clone, Default, Debug, Hash, PartialEq, Eq)]
pub struct Markup {
    text: String,
}

impl From<Markup> for String {
    #[inline]
    fn from(markup: Markup) -> Self {
        markup.text
    }
}

impl From<String> for Markup {
    fn from(text: String) -> Self {
        Self { text }
    }
}

impl fmt::Display for Markup {
    fn fmt(
        &self,
        #[expect(clippy::min_ident_chars, reason = "trait impl")] f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        fmt::Display::fmt(&self.text, f)
    }
}

impl Markup {
    pub fn as_str(&self) -> &str {
        self.text.as_str()
    }
    pub fn fenced_block<Displayable: fmt::Display>(contents: Displayable) -> Self {
        format!("```rust\n{contents}\n```").into()
    }
    pub fn fenced_block_text<Displayable: fmt::Display>(contents: Displayable) -> Self {
        format!("```text\n{contents}\n```").into()
    }
}
