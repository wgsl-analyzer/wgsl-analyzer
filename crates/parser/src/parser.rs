// For some reason allow(clippy::all) gets ignored
#![allow(
    clippy::wildcard_enum_match_arm,
    clippy::min_ident_chars,
    clippy::use_self,
    clippy::equatable_if_let,
    clippy::needless_pass_by_ref_mut,
    clippy::cognitive_complexity,
    clippy::too_many_lines,
    clippy::redundant_closure_for_method_calls,
    clippy::use_debug,
    clippy::doc_markdown,
    clippy::inconsistent_struct_constructor,
    clippy::missing_const_for_fn,
    clippy::unused_self,
    clippy::disallowed_names,
    clippy::uninlined_format_args,
    clippy::range_plus_one,
    clippy::needless_pass_by_value,
    clippy::little_endian_bytes,
    clippy::single_char_lifetime_names,
    clippy::allow_attributes,
    clippy::allow_attributes_without_reason,
    clippy::nonstandard_macro_braces,
    clippy::needless_continue,
    reason = "Lelwel generated code"
)]
use super::lexer::Token;
use crate::{
    Parse, ParseEntryPoint, SyntaxKind, cst_builder::CstBuilder, lexer::lex_with_templates,
};
use logos::Logos as _;
use rowan::{GreenNode, GreenNodeBuilder};
use std::fmt::{self, Write as _};

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[derive(Default)]
pub struct Context<'a> {
    marker: std::marker::PhantomData<&'a ()>,
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

#[must_use]
pub fn parse_entrypoint(
    input: &str,
    entrypoint: ParseEntryPoint,
) -> Parse {
    let mut diags = Vec::new();
    let parsed = match entrypoint {
        ParseEntryPoint::File => Parser::new(input, &mut diags).parse(&mut diags),
        ParseEntryPoint::Expression => Parser::new(input, &mut diags).parse_expression(&mut diags),
        ParseEntryPoint::Statement => Parser::new(input, &mut diags).parse_statement(&mut diags),
        ParseEntryPoint::Type => Parser::new(input, &mut diags).parse_type_specifier(&mut diags),
        ParseEntryPoint::Attribute => Parser::new(input, &mut diags).parse_attribute(&mut diags),
    };
    let green_node = CstBuilder {
        builder: GreenNodeBuilder::new(),
        token_start_index: 0,
        cst: parsed,
    }
    .build();
    Parse {
        green_node,
        errors: diags,
    }
}

impl Cst<'_> {
    pub const fn nodes_count(&self) -> usize {
        self.data.nodes.len()
    }
    pub fn get_text(
        &self,
        index: CstIndex,
    ) -> &str {
        &self.source[self.get_span(index)]
    }
    pub fn get_span(
        &self,
        index: CstIndex,
    ) -> std::ops::Range<usize> {
        self.data.spans[usize::from(index)].clone()
    }
}

impl Parser<'_> {
    fn is_func_call(&self) -> bool {
        matches!(self.peek(1), Token::LPar | Token::Lt) && self.peek(2) != Token::Lt
    }
}

impl<'source> ParserCallbacks<'source> for Parser<'source> {
    type Diagnostic = Diagnostic;
    type Context = ();
    fn create_tokens(
        _context: &mut Self::Context,
        source: &'source str,
        _diags: &mut Vec<Self::Diagnostic>,
    ) -> (Vec<Token>, Vec<Span>) {
        lex_with_templates(Token::lexer(source))
    }

    fn create_diagnostic(
        &self,
        span: Span,
        message: String,
    ) -> Self::Diagnostic {
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
    fn predicate_continuing_compound_statement_1(&self) -> bool {
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
    fn assertion_struct_body_1(&self) -> Option<Self::Diagnostic> {
        Some(self.create_diagnostic(self.span(), "invalid syntax, expected ','".to_owned()))
    }
}
