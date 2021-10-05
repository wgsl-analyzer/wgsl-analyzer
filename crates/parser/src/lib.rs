#![allow(clippy::result_unit_err)]
//! The parser is mostly copied from <https://github.com/arzg/eldiro/tree/master/crates/parser> with some adaptions and extensions

mod event;
mod lexer;
mod parsing;
mod sink;
mod source;

pub use parsing::marker;
pub use parsing::{ParseError, Parser};

use std::{fmt::Debug, marker::PhantomData};

use rowan::{GreenNode, SyntaxNode};
use sink::Sink;
use source::Source;

use lexer::Lexer;

pub trait TokenKind: Copy + PartialEq + Debug {
    fn is_trivia(self) -> bool;
}

pub trait ParserDefinition {
    type Language: rowan::Language<Kind = Self::SyntaxKind>;
    type TokenKind: for<'a> logos::Logos<'a, Source = str, Extras = ()> + TokenKind + 'static;
    type SyntaxKind: Debug + PartialEq + std::convert::From<Self::TokenKind>;

    const DEFAULT_RECOVERY_SET: &'static [Self::TokenKind] = &[];
}

pub fn parse<P: ParserDefinition, F: Fn(&mut Parser<P>)>(input: &str, f: F) -> Parse<P> {
    let tokens: Vec<_> = Lexer::<P::TokenKind>::new(input).collect();
    let source = Source::new(&tokens);
    let parser = Parser::<P>::new(source);
    let events = parser.parse(f);
    let sink = Sink::new(&tokens, events);

    sink.finish()
}

pub struct Parse<P: ParserDefinition> {
    green_node: GreenNode,
    errors: Vec<ParseError<P>>,
    _marker: PhantomData<P::Language>,
}

impl<P: ParserDefinition> Debug for Parse<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Parse")
            .field("green_node", &self.green_node)
            .field("errors", &self.errors)
            .finish()
    }
}

impl<P: ParserDefinition> PartialEq for Parse<P> {
    fn eq(&self, other: &Self) -> bool {
        self.green_node == other.green_node
    }
}
impl<P: ParserDefinition> Eq for Parse<P> {}

impl<P: ParserDefinition> Parse<P> {
    pub fn debug_tree(&self) -> String {
        let mut s = String::new();

        let tree = format!("{:#?}", self.syntax());

        // We cut off the last byte because formatting the SyntaxNode adds on a newline at the end.
        s.push_str(&tree[0..tree.len() - 1]);

        if !self.errors.is_empty() {
            s.push('\n');
        }
        for error in self.errors.iter() {
            s.push_str(&format!("\n{}", error));
        }

        s
    }

    pub fn syntax(&self) -> SyntaxNode<P::Language> {
        SyntaxNode::new_root(self.green_node.clone())
    }

    pub fn errors(&self) -> &[ParseError<P>] {
        &self.errors
    }

    pub fn into_parts(self) -> (GreenNode, Vec<ParseError<P>>) {
        (self.green_node, self.errors)
    }
}
