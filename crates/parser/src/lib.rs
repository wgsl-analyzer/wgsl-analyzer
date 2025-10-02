//! The parser is mostly copied from <https://github.com/arzg/eldiro/tree/master/crates/parser> with some adaptions and extensions

mod cst_builder;
mod lexer;
mod parser;
mod syntax_kind;

use std::fmt::{self, Debug};

pub use edition::Edition;
pub use parser::{Diagnostic as ParseError, parse_entrypoint};
use rowan::{GreenNode, SyntaxNode as RowanSyntaxNode};
use std::fmt::Write as _;

pub struct Parse {
    green_node: GreenNode,
    errors: Vec<ParseError>,
}
impl fmt::Debug for Parse {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("Parse")
            .field("green_node", &self.green_node)
            .field("errors", &self.errors)
            .finish()
    }
}

impl PartialEq for Parse {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        self.green_node == other.green_node
    }
}

impl Eq for Parse {}

impl Parse {
    #[must_use]
    pub fn debug_tree(&self) -> String {
        let mut buffer = String::new();

        let tree = format!("{:#?}", self.syntax());

        // We cut off the last byte because formatting the SyntaxNode adds on a newline at the end.
        buffer.push_str(&tree[0..tree.len() - 1]);

        if !self.errors.is_empty() {
            buffer.push('\n');
        }
        for diagnostic in &self.errors {
            write!(buffer, "\n{diagnostic}");
        }
        buffer
    }

    #[must_use]
    pub fn syntax(&self) -> rowan::SyntaxNode<WeslLanguage> {
        rowan::SyntaxNode::new_root(self.green_node.clone())
    }

    #[must_use]
    pub fn errors(&self) -> &[ParseError] {
        &self.errors
    }

    #[must_use]
    pub fn into_parts(self) -> (GreenNode, Vec<ParseError>) {
        (self.green_node, self.errors)
    }
}

pub use syntax_kind::SyntaxKind;

pub type SyntaxNode = rowan::SyntaxNode<WeslLanguage>;
pub type SyntaxToken = rowan::SyntaxToken<WeslLanguage>;
pub type SyntaxElement = rowan::SyntaxElement<WeslLanguage>;
pub type SyntaxNodeChildren = rowan::SyntaxNodeChildren<WeslLanguage>;
pub type SyntaxElementChildren = rowan::SyntaxElementChildren<WeslLanguage>;
pub type PreorderWithTokens = rowan::api::PreorderWithTokens<WeslLanguage>;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum WeslLanguage {}

impl rowan::Language for WeslLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        debug_assert!(raw.0 <= SyntaxKind::Error.as_u16());
        SyntaxKind::from_u16(raw.0)
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}

#[derive(Copy, PartialEq, Eq, Clone, Hash, Debug)]
pub enum ParseEntryPoint {
    File,
    Expression,
    Statement,
    Type,
    Attribute,
}

#[must_use]
pub fn parse_file(input: &str) -> Parse {
    parse_entrypoint(input, ParseEntryPoint::File)
}

#[cfg(test)]
fn check_entrypoint(
    input: &str,
    entry_point: ParseEntryPoint,
    expected_tree: &expect_test::Expect,
) {
    let parse = crate::parser::parse_entrypoint(input, entry_point);
    expected_tree.assert_eq(&parse.debug_tree());
}

#[cfg(test)]
mod tests;
