use crate::{
    Parse, ParseEntryPoint, SyntaxKind, cst_builder::CstBuilder, lexer::lex_with_templates,
};
use logos::Logos;
use rowan::{GreenNode, GreenNodeBuilder};
use std::fmt::{self, Write as _};

use super::lexer::Token;
use std::collections::HashSet;

pub struct Context<'a> {
    marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> Default for Context<'a> {
    fn default() -> Self {
        Self {
            marker: Default::default(),
        }
    }
}

pub struct Diagnostic {
    pub message: String,
    pub range: rowan::TextRange,
}

impl fmt::Debug for Diagnostic {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.debug_struct("Diagnostic")
            .field("message", &self.message)
            .field("range", &self.range)
            .finish()
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            f,
            "error at {}..{}: {}",
            u32::from(self.range.start()),
            u32::from(self.range.end()),
            self.message
        )
    }
}

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

pub fn parse_entrypoint(
    input: &str,
    entrypoint: ParseEntryPoint,
) -> Parse {
    let mut diags = Vec::new();
    let parsed = match entrypoint {
        ParseEntryPoint::File => Parser::new(&input, &mut diags).parse(&mut diags),
        ParseEntryPoint::Expression => Parser::new(&input, &mut diags).parse_expression(&mut diags),
        ParseEntryPoint::Statement => Parser::new(&input, &mut diags).parse_statement(&mut diags),
        ParseEntryPoint::Type => Parser::new(&input, &mut diags).parse_type_specifier(&mut diags),
        ParseEntryPoint::Attribute => Parser::new(&input, &mut diags).parse_attribute(&mut diags),
        ParseEntryPoint::FunctionParameterList => {
            todo!("Remove this")
        },
    };
    let green_node = CstBuilder {
        builder: GreenNodeBuilder::new(),
        cst: parsed,
    }
    .build();
    Parse {
        green_node,
        errors: diags,
    }
}

impl<'a> Cst<'a> {
    pub fn nodes_count(&self) -> usize {
        self.nodes.len()
    }
    pub fn get_text(
        &self,
        index: CstIndex,
    ) -> &str {
        &self.source[self.spans[usize::from(index)].clone()]
    }
}

impl<'a> Parser<'a> {
    fn is_func_call(&self) -> bool {
        matches!(self.peek(1), Token::LPar | Token::Lt) && self.peek(2) != Token::Lt
    }
}

impl<'a> ParserCallbacks for Parser<'a> {
    fn create_tokens(
        source: &str,
        _diags: &mut Vec<Diagnostic>,
    ) -> (Vec<Token>, Vec<Span>) {
        lex_with_templates(Token::lexer(source))
    }
    fn create_diagnostic(
        &self,
        span: Span,
        message: String,
    ) -> Diagnostic {
        let range = {
            let std::ops::Range { start, end } = span;
            let start = rowan::TextSize::try_from(start).unwrap();
            let end = rowan::TextSize::try_from(end).unwrap();

            rowan::TextRange::new(start, end)
        };

        Diagnostic { message, range }
    }
    fn predicate_global_directive_1(&self) -> bool {
        self.peek(1) != Token::Semi
    }
    fn predicate_function_parameters_1(&self) -> bool {
        self.peek(1) != Token::RPar
    }
    fn predicate_struct_body_1(&self) -> bool {
        self.peek(1) != Token::RBrace
    }
    fn predicate_template_args_1(&self) -> bool {
        self.peek(1) != Token::Gt
    }
    fn predicate_argument_expression_list_1(&self) -> bool {
        self.peek(1) != Token::RPar
    }
    fn predicate_argument_expression_list_expr_1(&self) -> bool {
        self.peek(1) != Token::RPar
    }
    fn predicate_statement_1(&self) -> bool {
        self.peek(1) == Token::If
    }
    fn predicate_statement_2(&self) -> bool {
        self.is_func_call()
    }
    fn predicate_continuing_statement_1(&self) -> bool {
        self.peek(1) != Token::If
    }
    fn predicate_for_init_1(&self) -> bool {
        self.is_func_call()
    }
    fn predicate_for_update_1(&self) -> bool {
        self.is_func_call()
    }
    fn predicate_case_selectors_1(&self) -> bool {
        !matches!(self.peek(1), Token::At | Token::Colon | Token::LBrace)
    }
    fn assertion_struct_body_1(&self) -> Option<Diagnostic> {
        Some(self.create_diagnostic(self.span(), "invalid syntax, expected ','".to_string()))
    }
}
