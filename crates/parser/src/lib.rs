//! The parser is mostly copied from <https://github.com/arzg/eldiro/tree/master/crates/parser> with some adaptions and extensions

mod event;
mod grammar;
mod lexer;
mod parser;
mod sink;
mod source;
mod syntax_kind;

use std::fmt::Debug;

use lexer::Lexer;
pub use parser::{ParseError, Parser, marker};
use rowan::{GreenNode, SyntaxNode as RowanSyntaxNode};
use sink::Sink;
use source::Source;

pub use edition::Edition;

pub fn parse<F: Fn(&mut Parser<'_, '_>)>(
    input: &str,
    f: F,
) -> Parse {
    let tokens: Vec<_> = Lexer::<SyntaxKind>::new(input).collect();
    let source = Source::new(&tokens);
    let parser = Parser::new(source);
    let events = parser.parse(f);
    let sink = Sink::new(&tokens, events);

    sink.finish()
}

pub struct Parse {
    green_node: GreenNode,
    errors: Vec<ParseError>,
}

impl Debug for Parse {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        f.debug_struct("Parse")
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
        let mut s = String::new();

        let tree = format!("{:#?}", self.syntax());

        // We cut off the last byte because formatting the SyntaxNode adds on a newline at the end.
        s.push_str(&tree[0..tree.len() - 1]);

        if !self.errors.is_empty() {
            s.push('\n');
        }
        for error in &self.errors {
            s.push_str(&format!("\n{error}"));
        }

        s
    }

    #[must_use]
    pub fn syntax(&self) -> RowanSyntaxNode<WgslLanguage> {
        RowanSyntaxNode::new_root(self.green_node.clone())
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

pub type SyntaxNode = rowan::SyntaxNode<WgslLanguage>;
pub type SyntaxToken = rowan::SyntaxToken<WgslLanguage>;
pub type SyntaxElement = rowan::SyntaxElement<WgslLanguage>;
pub type SyntaxNodeChildren = rowan::SyntaxNodeChildren<WgslLanguage>;
pub type SyntaxElementChildren = rowan::SyntaxElementChildren<WgslLanguage>;
pub type PreorderWithTokens = rowan::api::PreorderWithTokens<WgslLanguage>;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum WgslLanguage {}

impl rowan::Language for WgslLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= SyntaxKind::Error as u16);
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub enum ParseEntryPoint {
    File,
    Expression,
    Statement,
    Type,
    AttributeList,
    FunctionParameterList,
}

pub fn parse_entrypoint(
    input: &str,
    entrypoint: ParseEntryPoint,
) -> Parse {
    match entrypoint {
        ParseEntryPoint::File => parse::<_>(input, grammar::file),
        ParseEntryPoint::Expression => parse::<_>(input, grammar::expression),
        ParseEntryPoint::Statement => parse::<_>(input, grammar::statement),
        ParseEntryPoint::Type => parse::<_>(input, |p| {
            grammar::type_declaration(p);
        }),
        ParseEntryPoint::AttributeList => parse::<_>(input, grammar::attribute_list),
        ParseEntryPoint::FunctionParameterList => parse::<_>(input, grammar::inner_parameter_list),
    }
}

#[must_use]
pub fn parse_file(input: &str) -> Parse {
    parse_entrypoint(input, ParseEntryPoint::File)
}

#[cfg(test)]
fn check_entrypoint(
    input: &str,
    entry_point: ParseEntryPoint,
    expected_tree: expect_test::Expect,
) {
    let parse = crate::parse_entrypoint(input, entry_point);
    expected_tree.assert_eq(&parse.debug_tree());
}

#[cfg(test)]
mod tests;
