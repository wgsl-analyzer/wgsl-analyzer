use rowan::{GreenNode, GreenNodeBuilder};

use crate::{
    SyntaxKind,
    lexer::Token,
    parser::{Cst, CstIndex, Node, NodeRef, Rule},
};

pub struct CstBuilder<'a, 'cache> {
    pub builder: GreenNodeBuilder<'cache>,
    pub cst: Cst<'a>,
}
impl<'a, 'cache> CstBuilder<'a, 'cache> {
    /// Turn a lelwel syntax tree into a rowan syntax tree
    pub fn build(mut self) -> GreenNode {
        let mut rule_ends = vec![];
        for offset in 0..self.cst.nodes_count() {
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
            Rule::CaseSelectors => self.start_node(SyntaxKind::SwitchCaseSelectors),
            Rule::CompoundAssignmentStatement => {
                self.start_node(SyntaxKind::CompoundAssignmentStatement)
            },
            Rule::CompoundStatement => self.start_node(SyntaxKind::CompoundStatement),
            Rule::ConstDeclaration => self.start_node(SyntaxKind::ConstantDeclaration),
            Rule::ContinueStatement => self.start_node(SyntaxKind::ContinueStatement),
            Rule::ContinuingStatement => self.start_node(SyntaxKind::ContinuingStatement),
            Rule::ContinuingCompoundStatement => self.start_node(SyntaxKind::CompoundStatement),
            Rule::DecrementStatement => self.start_node(SyntaxKind::IncrementDecrementStatement),
            Rule::DefaultAloneClause => self.start_node(SyntaxKind::SwitchBodyCase),
            Rule::DefaultCaseSelector => self.start_node(SyntaxKind::SwitchDefaultSelector),
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
            Rule::LoopCompoundStatement => self.start_node(SyntaxKind::CompoundStatement),
            Rule::Name => self.start_node(SyntaxKind::Name),
            Rule::NameRef => self.start_node(SyntaxKind::NameReference),
            Rule::OverrideDeclaration => self.start_node(SyntaxKind::OverrideDeclaration),
            Rule::Parameter => self.start_node(SyntaxKind::Parameter),
            Rule::ParenExpression => self.start_node(SyntaxKind::ParenthesisExpression),
            Rule::PhonyAssignmentStatement => self.start_node(SyntaxKind::PhonyAssignmentStatement),
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
            | Rule::GlobalItem
            | Rule::IdentOrFunction
            | Rule::LetDeclarationSemi
            | Rule::OverrideDeclarationSemi
            | Rule::SwitchClause
            | Rule::CaseSelector
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
            Rule::Part => self.start_node(SyntaxKind::SourceFile),
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
            Token::Enable => SyntaxKind::Enable,
            Token::Requires => SyntaxKind::Requires,
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
            Token::Diagnostic => SyntaxKind::Diagnostic,
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
            Token::Underscore => SyntaxKind::Underscore,
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
            Token::LtEq => SyntaxKind::LessThanEqual,
            Token::ShiftLeft => SyntaxKind::ShiftLeft,
            Token::ShiftLeftEq => SyntaxKind::ShiftLeftEqual,
            Token::GtEq => SyntaxKind::GreaterThanEqual,
            Token::ShiftRight => SyntaxKind::ShiftRight,
            Token::ShiftRightEq => SyntaxKind::ShiftRightEqual,
            Token::TemplateStart => SyntaxKind::TemplateStart,
            Token::TemplateEnd => SyntaxKind::TemplateEnd,
        };
        Ok(output)
    }
}
