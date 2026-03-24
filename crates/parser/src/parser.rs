#![allow(
    clippy::all,
    clippy::pedantic,
    clippy::restriction,
    clippy::style,
    clippy::nursery,
    reason = "Lelwel generated code"
)]
use std::fmt;

use edition::Edition;
use logos::Logos as _;
use rowan::GreenNodeBuilder;

use super::lexer::Token;
use crate::{cst_builder::CstBuilder, lexer::lex, Parse, ParseEntryPoint};

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

pub struct ParserContext {
    edition: Edition,
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

pub(crate) fn to_range(span: Span) -> rowan::TextRange {
    let start = rowan::TextSize::try_from(span.start).unwrap();
    let end = rowan::TextSize::try_from(span.end).unwrap();
    rowan::TextRange::new(start, end)
}

#[must_use]
pub fn parse_entrypoint(
    input: &str,
    entrypoint: ParseEntryPoint,
    edition: Edition,
) -> Parse {
    let mut diagnostics = Vec::new();
    let parser = Parser::new_with_context(input, &mut diagnostics, ParserContext { edition });
    let parsed = match entrypoint {
        ParseEntryPoint::File => parser.parse(&mut diagnostics),
        ParseEntryPoint::Expression => parser.parse_expression(&mut diagnostics),
        ParseEntryPoint::Statement => parser.parse_statement(&mut diagnostics),
        ParseEntryPoint::Type => parser.parse_type_specifier(&mut diagnostics),
        ParseEntryPoint::Attribute => parser.parse_attribute(&mut diagnostics),
    };
    let green_node = CstBuilder {
        builder: GreenNodeBuilder::new(),
        token_start_index: 0,
        cst: parsed,
    }
    .build();
    Parse {
        green_node,
        errors: diagnostics,
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
        matches!(
            self.peek(1),
            Token::ParenthesisLeft | Token::LessThan | Token::TemplateStart
        ) && self.peek(2) != Token::LessThan
    }
}

impl<'source> ParserCallbacks<'source> for Parser<'source> {
    type Context = ParserContext;
    type Diagnostic = Diagnostic;

    fn create_tokens(
        _context: &mut Self::Context,
        source: &'source str,
        diags: &mut Vec<Self::Diagnostic>,
    ) -> (Vec<Token>, Vec<Span>) {
        lex(source, diags)
    }

    fn create_diagnostic(
        &self,
        span: Span,
        message: String,
    ) -> Self::Diagnostic {
        Diagnostic {
            message,
            range: to_range(span),
        }
    }

    fn create_node_import_statement(
        &mut self,
        node_ref: NodeRef,
        diags: &mut Vec<Self::Diagnostic>,
    ) {
        if !self.context.edition.at_least_wesl_0_0_1() {
            diags.push(self.create_diagnostic(
                self.cst.span(node_ref),
                "import statements are not allowed in WGSL mode".to_owned(),
            ));
        }
    }

    fn predicate_import_collection_1(&self) -> bool {
        self.peek(1) != Token::BraceRight
    }

    fn predicate_global_directive_1(&self) -> bool {
        self.peek(1) != Token::Semicolon
    }

    fn predicate_function_parameters_1(&self) -> bool {
        self.peek(1) != Token::ParenthesisRight
    }

    fn predicate_struct_body_1(&self) -> bool {
        self.peek(1) != Token::BraceRight
    }

    fn predicate_template_args_1(&self) -> bool {
        self.peek(1) != Token::TemplateEnd
    }

    fn predicate_argument_expression_list_1(&self) -> bool {
        self.peek(1) != Token::ParenthesisRight
    }

    fn predicate_argument_expression_list_expr_1(&self) -> bool {
        self.peek(1) != Token::ParenthesisRight
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
        !matches!(
            self.peek(1),
            Token::AttributeOperator | Token::Colon | Token::BraceLeft
        )
    }

    fn assertion_function_parameters_1(&self) -> Option<Self::Diagnostic> {
        Some(self.create_diagnostic(self.span(), "expected ',' between parameters".to_owned()))
    }

    fn assertion_struct_body_1(&self) -> Option<Self::Diagnostic> {
        Some(self.create_diagnostic(self.span(), "invalid syntax, expected ','".to_owned()))
    }

    fn create_node_if_statement(
        &mut self,
        node_ref: NodeRef,
        diags: &mut Vec<Self::Diagnostic>,
    ) {
        let mut seen_else = false;
        for child in self.cst.children(node_ref) {
            if self.cst.match_rule(child, Rule::ElseClause) {
                if seen_else {
                    diags.push(self.create_diagnostic(
                        self.cst.span(child),
                        "multiple 'else' clauses are not allowed".to_owned(),
                    ));
                }
                seen_else = true;
            } else if self.cst.match_rule(child, Rule::ElseIfClause) && seen_else {
                diags.push(self.create_diagnostic(
                    self.cst.span(child),
                    "'else if' after 'else' is not allowed".to_owned(),
                ));
            }
        }
    }

    /// This node exists for better error messages. It also improves the lelwel error recovery quality.
    fn create_node_global_let_declaration(
        &mut self,
        node_ref: NodeRef,
        diags: &mut Vec<Self::Diagnostic>,
    ) {
        diags.push(self.create_diagnostic(
            self.cst.span(node_ref),
            "global let declarations are not allowed".to_owned(),
        ));
    }
}
