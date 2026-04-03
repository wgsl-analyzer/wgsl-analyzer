use rowan::{GreenNode, GreenNodeBuilder};

use crate::{
    SyntaxKind,
    lexer::Token,
    parser::{Cst, CstIndex, Node, NodeRef, Rule},
};

pub struct CstBuilder<'source, 'cache> {
    pub builder: GreenNodeBuilder<'cache>,
    pub token_start_index: usize,
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

    #[expect(
        clippy::too_many_lines,
        reason = "Exhaustively covering all SyntaxKind variants. There is no obvious way of splitting this."
    )]
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
                self.start_node(SyntaxKind::SwitchBodyCase);
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
                self.start_node(SyntaxKind::IncrementDecrementStatement);
            },
            Rule::DefaultCaseSelector => self.start_node(SyntaxKind::SwitchDefaultSelector),
            Rule::DiagnosticControl => self.start_node(SyntaxKind::DiagnosticControl),
            Rule::DiagnosticDirective => self.start_node(SyntaxKind::DiagnosticDirective),
            Rule::DiagnosticRuleName => self.start_node(SyntaxKind::DiagnosticRuleName),
            Rule::DiscardStatement => self.start_node(SyntaxKind::DiscardStatement),
            Rule::ElseClause => self.start_node(SyntaxKind::ElseClause),
            Rule::ElseIfClause => self.start_node(SyntaxKind::ElseIfClause),
            Rule::EmptyStatement => self.start_node(SyntaxKind::EmptyStatement),
            Rule::EnableDirective => self.start_node(SyntaxKind::EnableDirective),
            Rule::EnableExtensionName => self.start_node(SyntaxKind::EnableExtensionName),
            #[expect(clippy::match_same_arms, reason = "Reasons might be different")]
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
            // This node exists purely for better parser error messages.
            Rule::GlobalLetDeclaration => self.start_node(SyntaxKind::Error),
            Rule::IdentExpression => self.start_node(SyntaxKind::IdentExpression),
            Rule::IfClause => self.start_node(SyntaxKind::IfClause),
            Rule::IfStatement => self.start_node(SyntaxKind::IfStatement),
            Rule::ImportCollection => self.start_node(SyntaxKind::ImportCollection),
            Rule::ImportItem => self.start_node(SyntaxKind::ImportItem),
            Rule::ImportPath => self.start_node(SyntaxKind::ImportPath),
            Rule::ImportPackageRelative => self.start_node(SyntaxKind::ImportPackageRelative),
            Rule::ImportSuperRelative => self.start_node(SyntaxKind::ImportSuperRelative),
            Rule::ImportStatement => self.start_node(SyntaxKind::ImportStatement),
            Rule::IndexingExpression => self.start_node(SyntaxKind::IndexExpression),
            Rule::LetDeclaration => self.start_node(SyntaxKind::LetDeclaration),
            Rule::LoopStatement => self.start_node(SyntaxKind::LoopStatement),
            Rule::Name => self.start_node(SyntaxKind::Name),
            Rule::OverrideDeclaration => self.start_node(SyntaxKind::OverrideDeclaration),
            Rule::Parameter => self.start_node(SyntaxKind::Parameter),
            Rule::ParenExpression => self.start_node(SyntaxKind::ParenthesisExpression),
            Rule::Path => self.start_node(SyntaxKind::Path),
            Rule::PhonyAssignmentStatement => self.start_node(SyntaxKind::PhonyAssignmentStatement),
            Rule::RequiresDirective => self.start_node(SyntaxKind::RequiresDirective),
            Rule::LanguageExtensionName => self.start_node(SyntaxKind::LanguageExtensionName),
            Rule::ReturnStatement => self.start_node(SyntaxKind::ReturnStatement),
            Rule::ReturnType => self.start_node(SyntaxKind::ReturnType),
            Rule::SeverityControlName => self.start_node(SyntaxKind::SeverityControlName),
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
            | Rule::VariableUpdating => {
                panic!("{rule:?} should always be a more specific node")
            },
            // This is reachable when an attribute is parsed, but no statement variant applies
            #[expect(clippy::match_same_arms, reason = "Reasons might be different")]
            Rule::Statement => self.start_node(SyntaxKind::Error),

            // Attributes
            // Note: The commented out attribute variants are parsed as OtherAttribute because they do not use
            // keywords and it confuses the lexer. These variants can be separated in higher layers.
            Rule::AlignAttr => self.start_node(SyntaxKind::AlignAttribute),
            Rule::BindingAttr => self.start_node(SyntaxKind::BindingAttribute),
            Rule::BlendSrcAttr => self.start_node(SyntaxKind::BlendSrcAttribute),
            Rule::BuiltinAttr => self.start_node(SyntaxKind::BuiltinAttribute),
            Rule::ComputeAttr => self.start_node(SyntaxKind::ComputeAttribute),
            Rule::ConstAttr => self.start_node(SyntaxKind::ConstantAttribute),
            Rule::DiagnosticAttr => self.start_node(SyntaxKind::DiagnosticAttribute),
            Rule::FragmentAttr => self.start_node(SyntaxKind::FragmentAttribute),
            Rule::GroupAttr => self.start_node(SyntaxKind::GroupAttribute),
            Rule::IdAttr => self.start_node(SyntaxKind::IdAttribute),
            Rule::InterpolateAttr => self.start_node(SyntaxKind::InterpolateAttribute),
            Rule::InvariantAttr => self.start_node(SyntaxKind::InvariantAttribute),
            Rule::LocationAttr => self.start_node(SyntaxKind::LocationAttribute),
            Rule::MustUseAttr => self.start_node(SyntaxKind::MustUseAttribute),
            Rule::SizeAttr => self.start_node(SyntaxKind::SizeAttribute),
            Rule::VertexAttr => self.start_node(SyntaxKind::VertexAttribute),
            Rule::WorkgroupSizeAttr => self.start_node(SyntaxKind::WorkgroupSizeAttribute),
            Rule::BuiltinValueName => self.start_node(SyntaxKind::BuiltinValueName),
            Rule::InterpolateSamplingName => self.start_node(SyntaxKind::InterpolateSamplingName),
            Rule::InterpolateTypeName => self.start_node(SyntaxKind::InterpolateTypeName),
            Rule::OtherAttr => self.start_node(SyntaxKind::OtherAttribute),
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
        let token_span = self.cst.get_span(index);
        assert_eq!(
            token_span.start, self.token_start_index,
            "Parser must produce contiguous tokens"
        );
        self.token_start_index = token_span.end;

        let text = &self.cst.get_text(index);
        self.builder.token(token.into(), text);
    }
}
