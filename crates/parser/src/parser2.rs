#![allow(elided_lifetimes_in_paths)]
use logos::Logos;
use rowan::{GreenNode, GreenNodeBuilder};
use std::fmt::{self, Write as _};

use crate::{ParseEntryPoint, SyntaxKind};

use super::lexer2::Token;
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

pub struct Parse2 {
    green_node: GreenNode,
    errors: Vec<Diagnostic>,
}
impl fmt::Debug for Parse2 {
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

impl PartialEq for Parse2 {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        self.green_node == other.green_node
    }
}

impl Eq for Parse2 {}

impl Parse2 {
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
    pub fn syntax(&self) -> rowan::SyntaxNode<crate::syntax_kind::WeslLanguage> {
        rowan::SyntaxNode::new_root(self.green_node.clone())
    }

    #[must_use]
    pub fn errors(&self) -> &[Diagnostic] {
        &self.errors
    }

    #[must_use]
    pub fn into_parts(self) -> (GreenNode, Vec<Diagnostic>) {
        (self.green_node, self.errors)
    }
}

pub fn parse_entrypoint(
    input: &str,
    entrypoint: ParseEntryPoint,
) -> Parse2 {
    let mut diags = Vec::new();
    let parsed = match entrypoint {
        ParseEntryPoint::File => Parser::parse(input, &mut diags),
        ParseEntryPoint::Expression => {
            Parser::parse_generic(input, &mut diags, Parser::rule_expression)
        },
        ParseEntryPoint::Statement => {
            Parser::parse_generic(input, &mut diags, Parser::rule_statement)
        },
        ParseEntryPoint::Type => {
            Parser::parse_generic(input, &mut diags, Parser::rule_type_expression)
        },
        ParseEntryPoint::Attribute => {
            Parser::parse_generic(input, &mut diags, Parser::rule_attribute)
        },
        ParseEntryPoint::FunctionParameterList => {
            todo!("Remove this")
        },
    };
    let green_node = CstBuilder {
        builder: GreenNodeBuilder::new(),
        cst: parsed,
    }
    .build();
    Parse2 {
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
            Rule::ArgumentExpressionList => panic!("should be a Arguments instead"),
            Rule::ArgumentExpressionListExpr => {
                panic!("should be a Arguments instead")
            },
            Rule::AssertStatement => todo!(),
            Rule::Attribute => self.start_node(SyntaxKind::Attribute),
            Rule::BinaryExpression => todo!(),
            Rule::Literal => self.start_node(SyntaxKind::Literal),
            Rule::BreakIfStatement => todo!(),
            Rule::BreakStatement => todo!(),
            Rule::CaseClause => todo!(),
            Rule::CaseSelector => todo!(),
            Rule::CaseSelectors => todo!(),
            Rule::CompoundAssignmentOperator => todo!(),
            Rule::CompoundAssignmentStatement => todo!(),
            Rule::CompoundStatement => todo!(),
            Rule::ConstDeclaration => todo!(),
            Rule::ContinueStatement => todo!(),
            Rule::ContinuingStatement => todo!(),
            Rule::DecrementStatement => todo!(),
            Rule::DefaultAloneClause => todo!(),
            Rule::DiagnosticAttr => todo!(),
            Rule::DiagnosticControl => todo!(),
            Rule::DiagnosticDirective => todo!(),
            Rule::DiagnosticRuleName => todo!(),
            Rule::DiscardStatement => todo!(),
            Rule::ElseClause => todo!(),
            Rule::ElseIfClause => todo!(),
            Rule::EmptyStatement => todo!(),
            Rule::EnableDirective => todo!(),
            Rule::Error => todo!(),
            Rule::ExprTemplateList => todo!(),
            Rule::Expression => panic!("expressions should always be a more specific node"),
            Rule::FieldExpression => todo!(),
            Rule::ForInit => todo!(),
            Rule::ForStatement => todo!(),
            Rule::ForUpdate => todo!(),
            Rule::FullIdent => panic!("full idents should be flattened"),
            Rule::FunctionCall => self.start_node(SyntaxKind::FunctionCall),
            Rule::FunctionCallStatement => self.start_node(SyntaxKind::FunctionCallStatement),
            Rule::FunctionDeclaration => todo!(),
            Rule::FunctionHeader => todo!(),
            Rule::GlobalAssert => todo!(),
            Rule::GlobalDeclaration => todo!(),
            Rule::GlobalDirective => todo!(),
            Rule::GlobalItem => todo!(),
            Rule::GlobalValueDeclaration => todo!(),
            Rule::GlobalVariableDeclaration => todo!(),
            Rule::IdentExpression => todo!(),
            Rule::IfClause => todo!(),
            Rule::IfStatement => todo!(),
            Rule::IncrementStatement => todo!(),
            Rule::IndexingExpression => todo!(),
            Rule::LetDeclaration => todo!(),
            Rule::LhsExpression => todo!(),
            Rule::LoopStatement => todo!(),
            Rule::MemberIdent => todo!(),
            Rule::OverrideDeclaration => todo!(),
            Rule::Parameter => self.start_node(SyntaxKind::Parameter),
            Rule::Parameters => todo!(),
            Rule::ParenExpression => todo!(),
            Rule::PhonyAssignmentStatement => todo!(),
            Rule::RequiresDirective => todo!(),
            Rule::ReturnStatement => todo!(),
            Rule::ReturnType => todo!(),
            Rule::SeverityControlName => todo!(),
            Rule::SimpleAssignmentStatement => todo!(),
            Rule::Statement => todo!(),
            Rule::StructBody => self.start_node(SyntaxKind::StructDeclBody),
            Rule::StructDeclaration => self.start_node(SyntaxKind::StructDeclaration),
            Rule::StructMember => self.start_node(SyntaxKind::StructDeclarationField),
            Rule::SwitchClause => todo!(),
            Rule::SwitchStatement => todo!(),
            Rule::TemplateArgs => todo!(),
            Rule::TemplateList => self.start_node(SyntaxKind::GenericArgumentList),
            Rule::TranslationUnit => todo!(),
            Rule::TypeAliasDeclaration => self.start_node(SyntaxKind::TypeAliasDeclaration),
            Rule::TypeExpression => self.start_node(SyntaxKind::TypeExpression),
            Rule::TypedIdent => todo!(),
            Rule::UnaryExpression => todo!(),
            Rule::VariableDeclaration => todo!(),
            Rule::VariableOrValue => todo!(),
            Rule::VariableOrValueStatement => todo!(),
            Rule::VariableUpdating => todo!(),
            Rule::WhileStatement => todo!(),
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
            Token::ConstAssert => todo!(),
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
            Token::FloatLiteral => SyntaxKind::DecimalFloatLiteral,
            Token::HexFloatLiteral => SyntaxKind::HexFloatLiteral,
            Token::IntLiteral => SyntaxKind::DecimalIntLiteral,
            Token::HexIntLiteral => SyntaxKind::HexIntLiteral,
            Token::Whitespace => SyntaxKind::Blankspace,
            Token::LineEndingComment => SyntaxKind::LineEndingComment,
            Token::BlockComment => SyntaxKind::BlockComment,
            Token::Error => SyntaxKind::Error,
        };
        Ok(output)
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
        parser.init_skip();
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
    fn is_diagnostic(&self) -> bool {
        &self.cst.source[self.span()] == "diagnostic"
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
    fn predicate_parameters_1(&self) -> bool {
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
}
