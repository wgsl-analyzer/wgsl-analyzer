mod event;
mod grammar;
mod lexer;
mod parser;
mod sink;
mod source;
mod syntax_kind;

use std::{fmt::Debug, marker::PhantomData};

use lexer::Lexer;
pub use parser::{ParseError, Parser, marker};
use rowan::{GreenNode, SyntaxNode as RowanSyntaxNode};
use sink::Sink;
use source::Source;

pub use edition::Edition;

pub fn parse<F: Fn(&mut Parser<'_, '_>), Language: rowan::Language<Kind = SyntaxKind>>(
    input: &str,
    f: F,
) -> Parse<Language> {
    let tokens: Vec<_> = Lexer::<SyntaxKind>::new(input).collect();
    let source = Source::new(&tokens);
    let parser = Parser::new(source);
    let events = parser.parse(f);
    let sink = Sink::new(&tokens, events);

    sink.finish()
}

pub struct Parse<Language: rowan::Language> {
    green_node: GreenNode,
    errors: Vec<ParseError>,
    _phantom: PhantomData<Language>,
}

impl<Language: rowan::Language> Debug for Parse<Language> {
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

impl<Language: rowan::Language> PartialEq for Parse<Language> {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        self.green_node == other.green_node
    }
}

impl<Language: rowan::Language> Eq for Parse<Language> {}

impl<Language: rowan::Language> Parse<Language> {
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

    pub fn syntax(&self) -> RowanSyntaxNode<Language> {
        RowanSyntaxNode::new_root(self.green_node.clone())
    }

    pub fn errors(&self) -> &[ParseError] {
        &self.errors
    }

    pub fn into_parts(self) -> (GreenNode, Vec<ParseError>) {
        (self.green_node, self.errors)
    }
}

pub use syntax_kind::SyntaxKind;

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub enum ParseEntryPoint {
    File,
    Expression,
    Statement,
    Type,
    AttributeList,
    FunctionParameterList,
}

pub fn parse_entrypoint<Language: rowan::Language<Kind = SyntaxKind>>(
    input: &str,
    entrypoint: ParseEntryPoint,
) -> Parse<Language> {
    match entrypoint {
        ParseEntryPoint::File => parse::<_, _>(input, grammar::file),
        ParseEntryPoint::Expression => parse::<_, _>(input, grammar::expression),
        ParseEntryPoint::Statement => parse::<_, _>(input, grammar::statement),
        ParseEntryPoint::Type => parse::<_, _>(input, |p| {
            grammar::type_declaration(p);
        }),
        ParseEntryPoint::AttributeList => parse::<_, _>(input, grammar::attribute_list),
        ParseEntryPoint::FunctionParameterList => {
            parse::<_, _>(input, grammar::inner_parameter_list)
        },
    }
}

pub fn parse_file<Language: rowan::Language<Kind = SyntaxKind>>(input: &str) -> Parse<Language> {
    parse_entrypoint(input, ParseEntryPoint::File)
}
