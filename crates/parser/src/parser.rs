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
use crate::{Parse, ParseEntryPoint, cst_builder::CstBuilder, lexer::lex};

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
        let mut i = 1;
        // Skip past path segments: (:: Ident)*
        // This handles paths like `foo::bar::baz()` where we need to look
        // past the `::` separators to find the `(` or `<` that indicates a call.
        while self.peek(i) == Token::DoubleColon {
            i += 1;
            if matches!(self.peek(i), Token::Ident | Token::Super) {
                i += 1;
            } else {
                break;
            }
        }
        matches!(self.peek(i), Token::LPar | Token::Lt | Token::TemplateStart)
            && self.peek(i + 1) != Token::Lt
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
        self.peek(1) != Token::RBrace
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
        self.peek(1) != Token::TemplateEnd
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

    fn create_node_translation_unit(
        &mut self,
        node_ref: NodeRef,
        diags: &mut Vec<Self::Diagnostic>,
    ) {
        let mut seen_declaration = false;
        for child in self.cst.children(node_ref) {
            match self.cst.get(child) {
                Node::Rule(rule, end_offset) if usize::from(end_offset) > 0 => match rule {
                    // Declarations and asserts
                    Rule::FunctionDeclaration
                    | Rule::VariableDeclaration
                    | Rule::ConstDeclaration
                    | Rule::OverrideDeclaration
                    | Rule::TypeAliasDeclaration
                    | Rule::StructDeclaration
                    | Rule::GlobalLetDeclaration
                    | Rule::AssertStatement => {
                        seen_declaration = true;
                    },
                    // Directives
                    Rule::DiagnosticDirective | Rule::EnableDirective | Rule::RequiresDirective => {
                        if seen_declaration {
                            diags.push(self.create_diagnostic(
                                self.cst.span(child),
                                "directives must come before any declarations".to_owned(),
                            ));
                        }
                    },
                    _ => {},
                },
                _ => {},
            }
        }
    }

    fn create_node_let_declaration(
        &mut self,
        node_ref: NodeRef,
        diags: &mut Vec<Self::Diagnostic>,
    ) {
        // Only emit this diagnostic when the parser successfully parsed a name
        // but did not find an '=' token (i.e. the initializer is missing).
        // Empty Name nodes (end_offset == 0) are created by error recovery and should be ignored.
        let has_name = self.cst.children(node_ref).any(|child| {
            matches!(
                self.cst.get(child),
                Node::Rule(Rule::Name, end_offset) if usize::from(end_offset) > 0
            )
        });
        let has_eq = self
            .cst
            .children(node_ref)
            .any(|child| self.cst.match_token(child, Token::Eq).is_some());
        if has_name && !has_eq {
            diags.push(self.create_diagnostic(
                self.cst.span(node_ref),
                "let declaration requires an initializer expression".to_owned(),
            ));
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

    /// Validates WGSL operator precedence rules.
    ///
    /// WGSL requires parentheses for certain operator combinations:
    /// - Comparison operators (`<`, `>`, `<=`, `>=`, `==`, `!=`) cannot be chained:
    ///   `a < b < c` is invalid, `(a < b) < c` is valid.
    /// - Shift operators (`<<`, `>>`) cannot have binary expression operands.
    /// - Bitwise operators (`&`, `|`, `^`) cannot be mixed:
    ///   `a & b | c` is invalid, `a & b & c` is valid.
    /// - Logical operators (`&&`, `||`) cannot be mixed:
    ///   `a && b || c` is invalid, `a && b && c` is valid.
    ///
    /// See: <https://github.com/gpuweb/gpuweb/issues/1146#issuecomment-714721825>
    /// See: <https://github.com/wgsl-analyzer/wgsl-analyzer/issues/616>
    fn create_node_binary_expression(
        &mut self,
        node_ref: NodeRef,
        diags: &mut Vec<Self::Diagnostic>,
    ) {
        let Some(op) = self.binary_expression_operator(node_ref) else {
            return;
        };

        for child in self.cst.children(node_ref) {
            // Skip tokens and parenthesized expressions — only check bare binary expressions.
            if !self.cst.match_rule(child, Rule::BinaryExpression) {
                continue;
            }

            let Some(child_op) = self.binary_expression_operator(child) else {
                continue;
            };

            match Self::classify_op(op) {
                // Comparison operators cannot be chained at all.
                OpClass::Comparison => {
                    if matches!(Self::classify_op(child_op), OpClass::Comparison) {
                        diags.push(self.create_diagnostic(
                            self.cst.span(child),
                            format!(
                                "comparison expressions must be parenthesized when used as an operand of another comparison"
                            ),
                        ));
                    }
                },
                // Shift operators cannot have any binary expression operands.
                OpClass::Shift => {
                    diags.push(self.create_diagnostic(
                        self.cst.span(child),
                        format!("shift expressions require parenthesized operands"),
                    ));
                },
                // Bitwise operators can be sequenced with the same operator, but not mixed.
                OpClass::Bitwise => {
                    if Self::classify_op(child_op) == OpClass::Bitwise && child_op != op {
                        diags.push(self.create_diagnostic(
                            self.cst.span(child),
                            format!("bitwise expressions of different types must be parenthesized"),
                        ));
                    }
                },
                // Logical operators can be sequenced with the same operator, but not mixed.
                OpClass::Logical => {
                    if Self::classify_op(child_op) == OpClass::Logical && child_op != op {
                        diags.push(self.create_diagnostic(
                            self.cst.span(child),
                            format!("logical expressions of different types must be parenthesized"),
                        ));
                    }
                },
                OpClass::Other => {},
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OpClass {
    Comparison,
    Shift,
    Bitwise,
    Logical,
    Other,
}

impl Parser<'_> {
    /// Returns the operator token of a binary expression node, if any.
    fn binary_expression_operator(
        &self,
        node_ref: NodeRef,
    ) -> Option<Token> {
        for child in self.cst.children(node_ref) {
            let node = self.cst.get(child);
            if let Node::Token(token, _) = node {
                // Skip trivia tokens (whitespace, comments, errors).
                if matches!(token, Token::Blankspace | Token::LineEndingComment | Token::BlockComment | Token::Error) {
                    continue;
                }
                return Some(token);
            }
        }
        None
    }

    fn classify_op(token: Token) -> OpClass {
        match token {
            Token::Lt | Token::Gt | Token::LtEq | Token::GtEq | Token::Eq2 | Token::ExclEq => {
                OpClass::Comparison
            },
            Token::ShiftLeft | Token::ShiftRight => OpClass::Shift,
            Token::And | Token::Pipe | Token::Caret => OpClass::Bitwise,
            Token::And2 | Token::Pipe2 => OpClass::Logical,
            _ => OpClass::Other,
        }
    }
}
