#![allow(elided_lifetimes_in_paths)]
use logos::Logos;
use rowan::GreenNode;

use crate::ParseEntryPoint;

use super::lexer2::Token;
use std::collections::HashSet;

#[derive(Default)]
pub struct Context<'a> {
    template_start: HashSet<usize>,
    template_end: HashSet<usize>,
    marker: std::marker::PhantomData<&'a ()>,
}

pub struct Diagnostic {
    pub message: String,
    pub range: rowan::TextRange,
}

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

pub struct Parse2 {
    green_node: GreenNode,
    diags: Vec<Diagnostic>,
}

pub fn parse_entrypoint(
    input: &str,
    entrypoint: ParseEntryPoint,
) -> Parse2 {
    let mut diags = Vec::new();
    let parsed = match entrypoint {
        ParseEntryPoint::File => {
            Parser::parse_generic(input, &mut diags, Parser::rule_translation_unit)
        },
        ParseEntryPoint::Expression => {
            Parser::parse_generic(input, &mut diags, Parser::rule_expression)
        },
        ParseEntryPoint::Statement => {
            Parser::parse_generic(input, &mut diags, Parser::rule_statement)
        },
        ParseEntryPoint::Type => {
            Parser::parse_generic(input, &mut diags, Parser::rule_type_specifier)
        },
        ParseEntryPoint::AttributeList => {
            todo!(
                "Not supported. I think we can get rid of this, or replace it with a single attribute parser"
            )
        },
        ParseEntryPoint::FunctionParameterList => {
            todo!("Ask Benjamin what this is for")
        },
    };

    Parse2 {
        green_node: todo!(),
        diags,
    }
}

impl<'a> Parser<'a> {
    /// Returns the CST for a parse with the given `source` file and writes diagnostics to `diags`.
    ///
    /// The context can be explicitly defined for the parse.
    pub fn parse_with_context_generic<Function: Fn(&mut Parser<'a>, &mut Vec<Diagnostic>)>(
        source: &'a str,
        diags: &mut Vec<Diagnostic>,
        context: Context<'a>,
        start_rule: Function,
    ) -> Cst<'a> {
        let (tokens, spans) = Self::create_tokens(source, diags);
        let max_offset = source.len();
        let mut parser = Self {
            current: Token::EOF,
            cst: Cst::new(source, spans),
            tokens,
            pos: 0,
            last_error_span: Span::default(),
            max_offset,
            context,
            error_cooldown: false,
            in_ordered_choice: false,
        };
        start_rule(&mut parser, diags);
        parser.cst
    }
    /// Returns the CST for a parse with the given `source` file and writes diagnostics to `diags`.
    ///
    /// The context will be default initialized for the parse.
    pub fn parse_generic<Function: Fn(&mut Parser<'a>, &mut Vec<Diagnostic>)>(
        source: &'a str,
        diags: &mut Vec<Diagnostic>,
        start_rule: Function,
    ) -> Cst<'a> {
        Self::parse_with_context_generic(source, diags, Context::default(), start_rule)
    }

    fn is_swizzle_name(&self) -> bool {
        let name = self.cst.source[self.span()].as_bytes();
        matches!(
            name,
            [b'r' | b'g' | b'b' | b'a']
                | [b'r' | b'g' | b'b' | b'a', b'r' | b'g' | b'b' | b'a']
                | [
                    b'r' | b'g' | b'b' | b'a',
                    b'r' | b'g' | b'b' | b'a',
                    b'r' | b'g' | b'b' | b'a'
                ]
                | [
                    b'r' | b'g' | b'b' | b'a',
                    b'r' | b'g' | b'b' | b'a',
                    b'r' | b'g' | b'b' | b'a',
                    b'r' | b'g' | b'b' | b'a'
                ]
                | [b'x' | b'y' | b'z' | b'w']
                | [b'x' | b'y' | b'z' | b'w', b'x' | b'y' | b'z' | b'w']
                | [
                    b'x' | b'y' | b'z' | b'w',
                    b'x' | b'y' | b'z' | b'w',
                    b'x' | b'y' | b'z' | b'w'
                ]
                | [
                    b'x' | b'y' | b'z' | b'w',
                    b'x' | b'y' | b'z' | b'w',
                    b'x' | b'y' | b'z' | b'w',
                    b'x' | b'y' | b'z' | b'w'
                ]
        )
    }
    fn find_template_list(&mut self) {
        if self.context.template_start.contains(&self.pos) {
            return;
        }
        let mut nesting_depth = 0usize;
        let mut pending = vec![];
        let mut last_lt_offset = 0;
        for (offset, tok) in self.tokens[self.pos..].iter().enumerate() {
            match tok {
                Token::LPar | Token::LBrak => nesting_depth += 1,
                Token::RPar | Token::RBrak => {
                    while let Some((_, pending_nesting_depth)) = pending.last().copied() {
                        if pending_nesting_depth < nesting_depth {
                            break;
                        } else {
                            pending.pop();
                        }
                    }
                    nesting_depth = nesting_depth.saturating_sub(1);
                },
                Token::Semi | Token::LBrace | Token::Colon => break,
                Token::And2 | Token::Pipe2 => {
                    while let Some((_, pending_nesting_depth)) = pending.last().copied() {
                        if pending_nesting_depth < nesting_depth {
                            break;
                        } else {
                            pending.pop();
                        }
                    }
                },
                Token::Lt | Token::Eq if offset == last_lt_offset + 1 => {
                    pending.pop();
                },
                Token::Lt => {
                    last_lt_offset = offset;
                    pending.push((self.pos + offset, nesting_depth));
                },
                Token::Gt => {
                    if let Some((pending_pos, pending_nesting_depth)) = pending.last().copied() {
                        if pending_nesting_depth == nesting_depth {
                            pending.pop();
                            self.context.template_start.insert(pending_pos);
                            self.context.template_end.insert(self.pos + offset);
                        }
                    }
                },
                _ => {},
            }
            if pending.is_empty() {
                break;
            }
        }
    }
    fn is_func_call(&self) -> bool {
        matches!(self.peek(1), Token::LPar | Token::Lt) && self.peek(2) != Token::Lt
    }
    fn is_diagnostic(&self) -> bool {
        &self.cst.source[self.span()] == "diagnostic"
    }
}

impl<'a> ParserCallbacks for Parser<'a> {
    fn create_tokens(
        source: &str,
        _diags: &mut Vec<Diagnostic>,
    ) -> (Vec<Token>, Vec<Span>) {
        let tokens: Vec<_> = Token::lexer(source).collect();
        (tokens, vec![])
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
    fn predicate_parameters_1(&self) -> bool {
        self.peek(1) != Token::RPar
    }
    fn predicate_struct_body_1(&self) -> bool {
        self.peek(1) != Token::RBrace
    }
    fn predicate_attribute_1(&self) -> bool {
        self.peek(1) != Token::RPar
    }
    fn action_template_list_1(
        &mut self,
        _diags: &mut Vec<Diagnostic>,
    ) {
        self.find_template_list();
    }
    fn action_expr_template_list_1(
        &mut self,
        _diags: &mut Vec<Diagnostic>,
    ) {
        if self.current == Token::Lt {
            self.find_template_list();
        }
    }
    fn predicate_expr_template_list_1(&self) -> bool {
        self.context.template_start.contains(&self.pos)
    }
    fn predicate_template_args_1(&self) -> bool {
        self.peek(1) != Token::Gt
    }
    fn predicate_expression_1(&self) -> bool {
        self.is_swizzle_name()
    }
    fn predicate_expression_2(&self) -> bool {
        if self.current == Token::Gt && self.context.template_end.contains(&self.pos) {
            return false;
        }
        self.tokens[self.pos + 1] == self.current
    }
    fn predicate_expression_3(&self) -> bool {
        self.current != Token::Gt || !self.context.template_end.contains(&self.pos)
    }
    fn predicate_expression_4(&self) -> bool {
        self.tokens[self.pos + 1] != Token::Eq
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
    fn predicate_compound_assignment_operator_1(&self) -> bool {
        self.tokens[self.pos + 1] == self.current && self.tokens[self.pos + 2] == Token::Eq
    }
    fn predicate_lhs_expression_1(&self) -> bool {
        self.is_swizzle_name()
    }
}
