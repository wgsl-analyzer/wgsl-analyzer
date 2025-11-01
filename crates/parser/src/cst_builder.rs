use rowan::{GreenNode, GreenNodeBuilder};

use crate::{
    SyntaxKind,
    lexer::Token,
    parser::{Cst, CstIndex, Node, NodeRef, Rule},
};

pub struct CstBuilder<'source, 'cache> {
    pub builder: GreenNodeBuilder<'cache>,
    pub cst: Cst<'source>,
}
impl CstBuilder<'_, '_> {
    /// Turn a lelwel syntax tree into a rowan syntax tree.
    /// Empty nodes will be omitted.
    pub fn build(mut self) -> GreenNode {
        let mut rule_ends = vec![];
        for offset in 0..self.cst.nodes_count() {
            let node_ref = NodeRef(offset);
            let node = self.cst.get(node_ref);
            match node {
                Node::Rule(rule @ (Rule::Part | Rule::TranslationUnit), end_offset) => {
                    let end_offset = usize::from(end_offset);
                    // Unconditionally include the root
                    self.start_rule(rule);
                    rule_ends.push((node_ref, rule, offset + end_offset));
                },
                Node::Rule(rule, end_offset) => {
                    let end_offset = usize::from(end_offset);
                    // Omit nodes with a size of 0
                    if end_offset > 0 {
                        self.start_rule(rule);
                        rule_ends.push((node_ref, rule, offset + end_offset));
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
            Rule::CaseClause | Rule::DefaultAloneClause => {
                self.start_node(SyntaxKind::SwitchBodyCase)
            },
            Rule::CaseSelectors => self.start_node(SyntaxKind::SwitchCaseSelectors),
            Rule::CompoundAssignmentStatement => {
                self.start_node(SyntaxKind::CompoundAssignmentStatement);
            },
            Rule::CompoundStatement
            | Rule::ContinuingCompoundStatement
            | Rule::LoopCompoundStatement => self.start_node(SyntaxKind::CompoundStatement),
            Rule::ConstDeclaration => self.start_node(SyntaxKind::ConstantDeclaration),
            Rule::ContinueStatement => self.start_node(SyntaxKind::ContinueStatement),
            Rule::ContinuingStatement => self.start_node(SyntaxKind::ContinuingStatement),
            Rule::DecrementStatement | Rule::IncrementStatement => {
                self.start_node(SyntaxKind::IncrementDecrementStatement)
            },
            Rule::DefaultCaseSelector => self.start_node(SyntaxKind::SwitchDefaultSelector),
            Rule::DiagnosticAttr => todo!(),
            Rule::DiagnosticControl => todo!(),
            Rule::DiagnosticDirective => todo!(),
            Rule::DiagnosticRuleName => todo!(),
            Rule::DiscardStatement => self.start_node(SyntaxKind::DiscardStatement),
            Rule::ElseClause => self.start_node(SyntaxKind::ElseClause),
            Rule::ElseIfClause => self.start_node(SyntaxKind::ElseIfClause),
            Rule::EmptyStatement => self.start_node(SyntaxKind::EmptyStatement),
            Rule::EnableDirective => self.start_node(SyntaxKind::EnableDirective),
            Rule::EnableExtensionName => self.start_node(SyntaxKind::EnableExtensionName),
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
            Rule::IndexingExpression => self.start_node(SyntaxKind::IndexExpression),
            Rule::LetDeclaration => self.start_node(SyntaxKind::LetDeclaration),
            Rule::LoopStatement => self.start_node(SyntaxKind::LoopStatement),
            Rule::Name => self.start_node(SyntaxKind::Name),
            Rule::NameRef => self.start_node(SyntaxKind::NameReference),
            Rule::OverrideDeclaration => self.start_node(SyntaxKind::OverrideDeclaration),
            Rule::Parameter => self.start_node(SyntaxKind::Parameter),
            Rule::ParenExpression => self.start_node(SyntaxKind::ParenthesisExpression),
            Rule::PhonyAssignmentStatement => self.start_node(SyntaxKind::PhonyAssignmentStatement),
            Rule::RequiresDirective => self.start_node(SyntaxKind::RequiresDirective),
            Rule::LanguageExtensionName => self.start_node(SyntaxKind::LanguageExtensionName),
            Rule::ReturnStatement => self.start_node(SyntaxKind::ReturnStatement),
            Rule::ReturnType => self.start_node(SyntaxKind::ReturnType),
            Rule::SeverityControlName => todo!(),
            Rule::SimpleAssignmentStatement => self.start_node(SyntaxKind::AssignmentStatement),
            Rule::StructBody => self.start_node(SyntaxKind::StructBody),
            Rule::StructDeclaration => self.start_node(SyntaxKind::StructDeclaration),
            Rule::StructMember => self.start_node(SyntaxKind::StructMember),
            Rule::SwitchBody => self.start_node(SyntaxKind::SwitchBody),
            Rule::SwitchStatement => self.start_node(SyntaxKind::SwitchStatement),
            Rule::TemplateList => self.start_node(SyntaxKind::TemplateList),
            Rule::TranslationUnit | Rule::Part => self.start_node(SyntaxKind::SourceFile),
            Rule::TypeAliasDeclaration => self.start_node(SyntaxKind::TypeAliasDeclaration),
            Rule::TypeSpecifier => self.start_node(SyntaxKind::TypeSpecifier),
            Rule::UnaryExpression => self.start_node(SyntaxKind::PrefixExpression),
            Rule::VariableDeclaration => self.start_node(SyntaxKind::VariableDeclaration),
            Rule::WhileStatement => self.start_node(SyntaxKind::WhileStatement),
            Rule::ConstDeclarationSemi
            | Rule::CompoundAssignmentOperator
            | Rule::ExprTemplateList
            | Rule::GlobalItem
            | Rule::IdentOrFunction
            | Rule::LetDeclarationSemi
            | Rule::OverrideDeclarationSemi
            | Rule::SwitchClause
            | Rule::CaseSelector
            | Rule::TemplateArgs
            | Rule::TypedIdent
            | Rule::VariableDeclarationSemi => {
                panic!("{rule:?} is elided")
            },
            Rule::Expression
            | Rule::GlobalDeclaration
            | Rule::GlobalDirective
            | Rule::LhsExpression
            | Rule::Statement
            | Rule::VariableUpdating => {
                panic!("{rule:?} should always be a more specific node")
            },
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
        if matches!(
            token,
            Token::EOF
                | Token::EOFAttribute
                | Token::EOFExpression
                | Token::EOFStatement
                | Token::EOFTypeSpecifier
        ) {
            return; // Ignore
        }
        let text = &self.cst.get_text(index);
        let syntax_kind = SyntaxKind::try_from(token)
            .unwrap_or_else(|()| panic!("token {token:?} should be convertible to a SyntaxKind"));
        self.builder.token(syntax_kind.into(), text);
    }
}

impl TryFrom<Token> for SyntaxKind {
    type Error = ();

    fn try_from(value: Token) -> Result<Self, ()> {
        let output = match value {
            Token::EOF
            | Token::EOFAttribute
            | Token::EOFExpression
            | Token::EOFStatement
            | Token::EOFTypeSpecifier => return Err(()),
            Token::Enable => Self::Enable,
            Token::Requires => Self::Requires,
            Token::Fn => Self::Fn,
            Token::Alias => Self::Alias,
            Token::Struct => Self::Struct,
            Token::Var => Self::Var,
            Token::ConstAssert => Self::ConstantAssert,
            Token::If => Self::If,
            Token::For => Self::For,
            Token::Else => Self::Else,
            Token::Loop => Self::Loop,
            Token::Break => Self::Break,
            Token::While => Self::While,
            Token::Return => Self::Return,
            Token::Switch => Self::Switch,
            Token::Discard => Self::Discard,
            Token::Continuing => Self::Continuing,
            Token::Const => Self::Constant,
            Token::Case => Self::Case,
            Token::Default => Self::Default,
            Token::Override => Self::Override,
            Token::Continue => Self::Continue,
            Token::Let => Self::Let,
            Token::True => Self::True,
            Token::False => Self::False,
            Token::Diagnostic => Self::Diagnostic,
            Token::Semi => Self::Semicolon,
            Token::LPar => Self::ParenthesisLeft,
            Token::RPar => Self::ParenthesisRight,
            Token::Comma => Self::Comma,
            Token::Eq => Self::Equal,
            Token::Colon => Self::Colon,
            Token::LBrace => Self::BraceLeft,
            Token::RBrace => Self::BraceRight,
            Token::Arrow => Self::Arrow,
            Token::Lt => Self::LessThan,
            Token::Gt => Self::GreaterThan,
            Token::Dot => Self::Period,
            Token::At => Self::AttributeOperator,
            Token::LBrak => Self::BracketLeft,
            Token::RBrak => Self::BracketRight,
            Token::And => Self::And,
            Token::Excl => Self::Bang,
            Token::Star => Self::Star,
            Token::Minus => Self::Minus,
            Token::Tilde => Self::Tilde,
            Token::Plus => Self::Plus,
            Token::Eq2 => Self::EqualEqual,
            Token::Pipe => Self::Or,
            Token::And2 => Self::AndAnd,
            Token::Slash => Self::ForwardSlash,
            Token::Caret => Self::Xor,
            Token::Pipe2 => Self::OrOr,
            Token::ExclEq => Self::NotEqual,
            Token::Percent => Self::Modulo,
            Token::Underscore => Self::Underscore,
            Token::AndEq => Self::AndEqual,
            Token::StarEq => Self::TimesEqual,
            Token::PlusEq => Self::PlusEqual,
            Token::PipeEq => Self::OrEqual,
            Token::MinusEq => Self::MinusEqual,
            Token::SlashEq => Self::DivisionEqual,
            Token::CaretEq => Self::XorEqual,
            Token::PercentEq => Self::ModuloEqual,
            Token::Plus2 => Self::PlusPlus,
            Token::Minus2 => Self::MinusMinus,
            Token::Ident => Self::Identifier,
            Token::FloatLiteral => Self::FloatLiteral,
            Token::IntLiteral => Self::IntLiteral,
            Token::Blankspace => Self::Blankspace,
            Token::LineEndingComment => Self::LineEndingComment,
            Token::BlockComment => Self::BlockComment,
            Token::Error => Self::Error,
            Token::LtEq => Self::LessThanEqual,
            Token::ShiftLeft => Self::ShiftLeft,
            Token::ShiftLeftEq => Self::ShiftLeftEqual,
            Token::GtEq => Self::GreaterThanEqual,
            Token::ShiftRight => Self::ShiftRight,
            Token::ShiftRightEq => Self::ShiftRightEqual,
            Token::TemplateStart => Self::TemplateStart,
            Token::TemplateEnd => Self::TemplateEnd,
        };
        Ok(output)
    }
}
