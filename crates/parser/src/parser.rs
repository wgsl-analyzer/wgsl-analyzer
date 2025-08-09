#![allow(elided_lifetimes_in_paths)]
use logos::Logos;
use rowan::{GreenNode, GreenNodeBuilder};
use std::fmt::{self, Write as _};

use crate::{Parse, ParseEntryPoint, SyntaxKind};

use super::lexer::Token;
use std::collections::HashSet;

pub struct Context<'a> {
    template_start: HashSet<usize>,
    template_end: HashSet<usize>,
    marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> Default for Context<'a> {
    fn default() -> Self {
        Self {
            template_start: Default::default(),
            template_end: Default::default(),
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
        ParseEntryPoint::File => Parser::parse(input, &mut diags),
        ParseEntryPoint::Expression => Parser::parse_start_expression(input, &mut diags),
        ParseEntryPoint::Statement => Parser::parse_start_statement(input, &mut diags),
        ParseEntryPoint::Type => Parser::parse_start_type_specifier(input, &mut diags),
        ParseEntryPoint::Attribute => Parser::parse_start_attribute(input, &mut diags),
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

struct CstBuilder<'a, 'cache> {
    builder: GreenNodeBuilder<'cache>,
    cst: Cst<'a>,
}
impl<'a, 'cache> CstBuilder<'a, 'cache> {
    /// Turn a lelwel syntax tree into a rowan syntax tree
    fn build(mut self) -> GreenNode {
        println!("{}", self.cst);
        let mut rule_ends = vec![];
        for offset in 0..self.cst.nodes.len() {
            let node_ref = NodeRef(offset);
            let node = self.cst.get(node_ref);
            match node {
                Node::Rule(rule, end_offset) => {
                    self.start_rule(rule);
                    let end_offset = usize::from(end_offset);
                    if end_offset > 0 {
                        rule_ends.push((node_ref, rule, offset + end_offset));
                    } else {
                        self.end_rule(node_ref, rule);
                    }
                },
                Node::Token(token, index) => self.token(token, index),
            }

            while let Some((node_ref, rule, _)) = rule_ends.pop_if(|(_, _, end)| offset >= *end) {
                self.end_rule(node_ref, rule);
            }
        }
        assert_eq!(rule_ends.len(), 0, "All rules should have been consumed");
        self.builder.finish()
    }

    fn start_rule(
        &mut self,
        rule: Rule,
    ) {
        match rule {
            Rule::Arguments => self.start_node(SyntaxKind::Arguments),
            Rule::ArgumentExpressionList => panic!("should be arguments instead"),
            Rule::ArgumentExpressionListExpr => {
                panic!("should be arguments instead")
            },
            Rule::AssertStatement => self.start_node(SyntaxKind::AssertStatement),
            Rule::Attribute => self.start_node(SyntaxKind::Attribute),
            Rule::BinaryExpression => self.start_node(SyntaxKind::InfixExpression),
            Rule::Literal => self.start_node(SyntaxKind::Literal),
            Rule::BreakIfStatement => self.start_node(SyntaxKind::BreakIfStatement),
            Rule::BreakStatement => self.start_node(SyntaxKind::BreakStatement),
            Rule::CaseClause => self.start_node(SyntaxKind::SwitchBodyCase),
            Rule::CaseSelector => self.start_node(SyntaxKind::SwitchCaseSelector),
            Rule::CaseSelectors => self.start_node(SyntaxKind::SwitchCaseSelectors),
            Rule::CompoundAssignmentStatement => {
                self.start_node(SyntaxKind::CompoundAssignmentStatement)
            },
            Rule::CompoundStatement => self.start_node(SyntaxKind::CompoundStatement),
            Rule::ConstDeclaration => self.start_node(SyntaxKind::ConstantDeclaration),
            Rule::ContinueStatement => self.start_node(SyntaxKind::ContinueStatement),
            Rule::ContinuingStatement => self.start_node(SyntaxKind::ContinuingStatement),
            Rule::DecrementStatement => self.start_node(SyntaxKind::IncrementDecrementStatement),
            Rule::DefaultAloneClause => self.start_node(SyntaxKind::SwitchBodyDefault),
            Rule::DiagnosticAttr => todo!(),
            Rule::DiagnosticControl => todo!(),
            Rule::DiagnosticDirective => todo!(),
            Rule::DiagnosticRuleName => todo!(),
            Rule::DiscardStatement => self.start_node(SyntaxKind::DiscardStatement),
            Rule::ElseClause => self.start_node(SyntaxKind::ElseClause),
            Rule::ElseIfClause => self.start_node(SyntaxKind::ElseIfClause),
            Rule::EmptyStatement => self.start_node(SyntaxKind::EmptyStatement),
            Rule::EnableDirective => todo!(),
            Rule::Error => self.start_node(SyntaxKind::Error),
            Rule::FieldExpression => self.start_node(SyntaxKind::FieldExpression),
            Rule::ForCondition => self.start_node(SyntaxKind::ForCondition),
            Rule::ForInit => self.start_node(SyntaxKind::ForInitializer),
            Rule::ForStatement => self.start_node(SyntaxKind::ForStatement),
            Rule::ForUpdate => self.start_node(SyntaxKind::ForContinuingPart),
            Rule::FunctionCall => self.start_node(SyntaxKind::FunctionCall),
            Rule::FunctionCallStatement => self.start_node(SyntaxKind::FunctionCallStatement),
            Rule::FunctionDeclaration => self.start_node(SyntaxKind::FunctionDeclaration),
            Rule::FunctionParameters => self.start_node(SyntaxKind::FunctionParameters),
            Rule::GlobalAssert => panic!("should be assert_statement instead"),
            Rule::IdentExpression => self.start_node(SyntaxKind::IdentExpression),
            Rule::IfClause => self.start_node(SyntaxKind::IfClause),
            Rule::IfStatement => self.start_node(SyntaxKind::IfStatement),
            Rule::IncrementStatement => self.start_node(SyntaxKind::IncrementDecrementStatement),
            Rule::IndexingExpression => self.start_node(SyntaxKind::IndexExpression),
            Rule::LetDeclaration => self.start_node(SyntaxKind::LetDeclaration),
            Rule::LoopStatement => self.start_node(SyntaxKind::LoopStatement),
            Rule::OverrideDeclaration => self.start_node(SyntaxKind::OverrideDeclaration),
            Rule::Parameter => self.start_node(SyntaxKind::Parameter),
            Rule::ParenExpression => self.start_node(SyntaxKind::ParenthesisExpression),
            Rule::PhonyAssignmentStatement => todo!(),
            Rule::RequiresDirective => todo!(),
            Rule::ReturnStatement => self.start_node(SyntaxKind::ReturnStatement),
            Rule::ReturnType => self.start_node(SyntaxKind::ReturnType),
            Rule::SeverityControlName => todo!(),
            Rule::SimpleAssignmentStatement => self.start_node(SyntaxKind::AssignmentStatement),
            Rule::StructBody => self.start_node(SyntaxKind::StructBody),
            Rule::StructDeclaration => self.start_node(SyntaxKind::StructDeclaration),
            Rule::StructMember => self.start_node(SyntaxKind::StructMember),
            Rule::SwitchBody => self.start_node(SyntaxKind::SwitchBody),
            Rule::SwitchStatement => self.start_node(SyntaxKind::SwitchStatement),
            Rule::TemplateList => self.start_node(SyntaxKind::GenericArgumentList),
            Rule::TranslationUnit => self.start_node(SyntaxKind::SourceFile),
            Rule::TypeAliasDeclaration => self.start_node(SyntaxKind::TypeAliasDeclaration),
            Rule::TypeSpecifier => self.start_node(SyntaxKind::TypeSpecifier),
            Rule::UnaryExpression => self.start_node(SyntaxKind::PrefixExpression),
            Rule::VariableDeclaration => self.start_node(SyntaxKind::VariableDeclaration),
            Rule::WhileStatement => self.start_node(SyntaxKind::WhileStatement),
            Rule::ConstDeclarationSemi
            | Rule::CompoundAssignmentOperator
            | Rule::ExprTemplateList
            | Rule::FullIdent
            | Rule::GlobalItem
            | Rule::IdentOrFunction
            | Rule::LetDeclarationSemi
            | Rule::OverrideDeclarationSemi
            | Rule::SwitchClause
            | Rule::TemplateArgs
            | Rule::TypedIdent
            | Rule::VariableDeclarationSemi => {
                panic!("{:?} is elided", rule)
            },
            Rule::Expression
            | Rule::GlobalDeclaration
            | Rule::GlobalDirective
            | Rule::LhsExpression
            | Rule::Statement
            | Rule::VariableUpdating => {
                panic!("{:?} should always be a more specific node", rule)
            },
            // Custom parsing entrypoints. The extra "SourceFile" exists so that parsing errors are definitely caught in the root node
            Rule::StartAttribute => self.start_node(SyntaxKind::SourceFile),
            Rule::StartExpression => self.start_node(SyntaxKind::SourceFile),
            Rule::StartStatement => self.start_node(SyntaxKind::SourceFile),
            Rule::StartTypeSpecifier => self.start_node(SyntaxKind::SourceFile),
        }
    }

    fn start_node(
        &mut self,
        node: SyntaxKind,
    ) {
        self.builder.start_node(rowan::SyntaxKind::from(node));
    }

    fn end_rule(
        &mut self,
        _node_ref: NodeRef,
        _rule: Rule,
    ) {
        self.builder.finish_node();
    }

    fn token(
        &mut self,
        token: Token,
        index: CstIndex,
    ) {
        if token == Token::EOF {
            return; // Ignore
        }
        let text = &self.cst.source[self.cst.spans[usize::from(index)].clone()];
        let syntax_kind = SyntaxKind::try_from(token)
            .unwrap_or_else(|()| panic!("token {token:?} should be convertible to a SyntaxKind"));
        self.builder.token(syntax_kind.into(), text);
    }
}

impl TryFrom<Token> for SyntaxKind {
    type Error = ();

    fn try_from(value: Token) -> Result<Self, ()> {
        let output = match value {
            Token::EOF => return Err(()),
            Token::Enable => SyntaxKind::Enable,
            Token::Requires => todo!(),
            Token::Fn => SyntaxKind::Fn,
            Token::Alias => SyntaxKind::Alias,
            Token::Struct => SyntaxKind::Struct,
            Token::Var => SyntaxKind::Var,
            Token::ConstAssert => SyntaxKind::ConstantAssert,
            Token::If => SyntaxKind::If,
            Token::For => SyntaxKind::For,
            Token::Else => SyntaxKind::Else,
            Token::Loop => SyntaxKind::Loop,
            Token::Break => SyntaxKind::Break,
            Token::While => SyntaxKind::While,
            Token::Return => SyntaxKind::Return,
            Token::Switch => SyntaxKind::Switch,
            Token::Discard => SyntaxKind::Discard,
            Token::Continuing => SyntaxKind::Continuing,
            Token::Const => SyntaxKind::Constant,
            Token::Case => SyntaxKind::Case,
            Token::Default => SyntaxKind::Default,
            Token::Override => SyntaxKind::Override,
            Token::Continue => SyntaxKind::Continue,
            Token::Let => SyntaxKind::Let,
            Token::True => SyntaxKind::True,
            Token::False => SyntaxKind::False,
            Token::Diagnostic => todo!(),
            Token::Semi => SyntaxKind::Semicolon,
            Token::LPar => SyntaxKind::ParenthesisLeft,
            Token::RPar => SyntaxKind::ParenthesisRight,
            Token::Comma => SyntaxKind::Comma,
            Token::Eq => SyntaxKind::Equal,
            Token::Colon => SyntaxKind::Colon,
            Token::LBrace => SyntaxKind::BraceLeft,
            Token::RBrace => SyntaxKind::BraceRight,
            Token::Arrow => SyntaxKind::Arrow,
            Token::Lt => SyntaxKind::LessThan,
            Token::Gt => SyntaxKind::GreaterThan,
            Token::Dot => SyntaxKind::Period,
            Token::At => SyntaxKind::AttributeOperator,
            Token::LBrak => SyntaxKind::BracketLeft,
            Token::RBrak => SyntaxKind::BracketRight,
            Token::And => SyntaxKind::And,
            Token::Excl => SyntaxKind::Bang,
            Token::Star => SyntaxKind::Star,
            Token::Minus => SyntaxKind::Minus,
            Token::Tilde => SyntaxKind::Tilde,
            Token::Plus => SyntaxKind::Plus,
            Token::Eq2 => SyntaxKind::EqualEqual,
            Token::Pipe => SyntaxKind::Or,
            Token::And2 => SyntaxKind::AndAnd,
            Token::Slash => SyntaxKind::ForwardSlash,
            Token::Caret => SyntaxKind::Xor,
            Token::Pipe2 => SyntaxKind::OrOr,
            Token::ExclEq => SyntaxKind::NotEqual,
            Token::Percent => SyntaxKind::Modulo,
            Token::Underscore => todo!(),
            Token::AndEq => SyntaxKind::AndEqual,
            Token::StarEq => SyntaxKind::TimesEqual,
            Token::PlusEq => SyntaxKind::PlusEqual,
            Token::PipeEq => SyntaxKind::OrEqual,
            Token::MinusEq => SyntaxKind::MinusEqual,
            Token::SlashEq => SyntaxKind::DivisionEqual,
            Token::CaretEq => SyntaxKind::XorEqual,
            Token::PercentEq => SyntaxKind::ModuloEqual,
            Token::Plus2 => SyntaxKind::PlusPlus,
            Token::Minus2 => SyntaxKind::MinusMinus,
            Token::Ident => SyntaxKind::Identifier,
            Token::FloatLiteral => SyntaxKind::FloatLiteral,
            Token::IntLiteral => SyntaxKind::IntLiteral,
            Token::Blankspace => SyntaxKind::Blankspace,
            Token::LineEndingComment => SyntaxKind::LineEndingComment,
            Token::BlockComment => SyntaxKind::BlockComment,
            Token::Error => SyntaxKind::Error,
        };
        Ok(output)
    }
}

impl<'a> Parser<'a> {
    /// Implements the template disambiguation algorithm and remembers the results in a hash set
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
}

impl<'a> ParserCallbacks for Parser<'a> {
    fn create_tokens(
        source: &str,
        _diags: &mut Vec<Diagnostic>,
    ) -> (Vec<Token>, Vec<Span>) {
        let (tokens, spans): (Vec<_>, Vec<_>) = Token::lexer(source).spanned().collect();
        (tokens, spans)
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

    fn assertion_struct_body_1(&self) -> Option<Diagnostic> {
        Some(self.create_diagnostic(self.span(), "invalid syntax, expected ','".to_string()))
    }
}
