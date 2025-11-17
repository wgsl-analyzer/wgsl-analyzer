#![expect(
    clippy::enum_variant_names,
    reason = "The variant names must be clear when self-standing. See the parser snapshot tests."
)]

pub mod operators;
pub mod trivia;

use parser::{SyntaxKind, SyntaxNode};
pub use trivia::{Comment, Whitespace};

use self::operators::{AssignmentOperator, BinaryOperation, UnaryOperator};
use crate::{
    AstChildren, AstNode, AstToken, HasAttributes, HasName, HasTemplateParameters, SyntaxToken,
    TokenText,
    ast::operators::{ArithmeticOperation, ComparisonOperation, LogicOperation},
    support,
};

macro_rules! ast_node {
    ($kind:ident $($name:ident)? $(:
        $($descendant:ident: $amount_type:ident < $return_type:tt $($a:ident)? >;)+
    )?) => {
        ast_node! { @structdef $kind $($name)? }
        ast_node! { @impl $kind $($name)? }

        $(impl $kind {
            $(#[must_use] pub fn $descendant(&self) -> $amount_type<$return_type>  {
                ast_node! { @descendant self $amount_type<$return_type $($a)?> }
            })*
        })?
    };

    (@structdef $kind:ident $name:ident) => {
        #[derive(Clone, Debug)]
        pub struct $name {
            syntax: SyntaxNode,
        }
    };
    (@structdef $kind:ident) => {
        #[derive(Clone, Debug)]
        pub struct $kind {
            syntax: SyntaxNode,
        }
    };

    (@impl $kind:ident) => {
        ast_node! { @impl_inner $kind $kind }
    };
    (@impl $kind:ident $name:ident) => {
        ast_node! { @impl_inner $name $kind }
    };
    (@impl_inner $name:ident $kind:ident) => {
        impl AstNode for $name {
            fn can_cast(kind: SyntaxKind) -> bool {
                kind == SyntaxKind::$kind
            }

            fn cast(syntax: SyntaxNode) -> Option<Self> {
                Self::can_cast(syntax.kind()).then(|| Self { syntax })
            }

            fn syntax(&self) -> &SyntaxNode {
                &self.syntax
            }
        }
    };

    (@descendant $self:ident TokenText<'_>) => {
        crate::support::text_of_first_token(&$self.syntax)
    };
    (@descendant $self:ident Option<SyntaxToken $syntax:ident>) => {
        crate::support::token(&$self.syntax, SyntaxKind::$syntax)
    };
    (@descendant $self:ident Option<$syntax:ident>) => {
        crate::support::child(&$self.syntax)
    };
    (@descendant $self:ident AstChildren<$syntax:ident>) => {
        crate::support::children(&$self.syntax)
    };
}

macro_rules! ast_enum {
    (enum $enum_type:ident { $($variant:ident,)* }) => {
        #[derive(Debug)]
        pub enum $enum_type {
            $($variant($variant),)*
        }

        impl AstNode for $enum_type {
            fn can_cast(kind: SyntaxKind) -> bool {
                match kind {
                    $(SyntaxKind::$variant)|* => true,
                    _ => false,
                }
            }

            fn cast(syntax: SyntaxNode) -> Option<Self> {
                match syntax.kind() {
                    $(SyntaxKind::$variant => Some($enum_type::$variant($variant { syntax })),)*
                    _ => None,
                }
            }

            fn syntax(&self) -> &SyntaxNode {
                match self {
                    $($enum_type::$variant(item) => &item.syntax,)*
                }
            }
        }

        $(impl From<$variant> for $enum_type {
            fn from(value: $variant) -> $enum_type {
                $enum_type::$variant(value)
            }
        })*
    };
}

macro_rules! ast_enum_raw {
    (enum $enum_type:ident { $($variant:ident,)* }) => {
        #[derive(Clone, Debug)]
        pub enum $enum_type {
            $($variant(SyntaxNode),)*
        }

        impl AstNode for $enum_type {
            fn can_cast(kind: SyntaxKind) -> bool {
                match kind {
                    $(SyntaxKind::$variant)|* => true,
                    _ => false,
                }
            }

            fn cast(syntax: SyntaxNode) -> Option<Self> {
                match syntax.kind() {
                    $(SyntaxKind::$variant => Some($enum_type::$variant(syntax)),)*
                    _ => None,
                }
            }

            fn syntax(&self) -> &SyntaxNode {
                match self {
                    $($enum_type::$variant(item) => &item,)*
                }
            }
        }
    };
}

macro_rules! ast_enum_compound {
    (enum $enum_type:ident { $($variant:ident,)* }) => {
        #[derive(Clone, Debug)]
        pub enum $enum_type {
            $($variant($variant),)*
        }

        impl AstNode for $enum_type {
            fn can_cast(kind: SyntaxKind) -> bool {
                $($variant::can_cast(kind))||*
            }

            fn cast(syntax: SyntaxNode) -> Option<Self> {
                $(if $variant::can_cast(syntax.kind()) {
                    return Some(Self::$variant($variant::cast(syntax).unwrap()));
                })*

                None
            }

            fn syntax(&self) -> &SyntaxNode {
                match self {
                    $($enum_type::$variant(item) => item.syntax(),)*
                }
            }
        }
    };
}

macro_rules! ast_token_enum {
    (enum $enum_type:ident { $($variant:ident,)* }) => {
        #[derive(Clone)]
        pub enum $enum_type {
            $($variant(SyntaxToken),)*
        }

        impl AstToken for $enum_type {
            fn can_cast(kind: SyntaxKind) -> bool {
                match kind {
                    $(SyntaxKind::$variant)|* => true,
                    _ => false,
                }
            }

            fn cast(syntax: SyntaxToken) -> Option<Self> {
                match syntax.kind() {
                    $(SyntaxKind::$variant => Some($enum_type::$variant(syntax)),)*
                    _ => None,
                }
            }

            fn syntax(&self) -> &SyntaxToken {
                match self {
                    $($enum_type::$variant(item) => &item,)*
                }
            }
        }
    };
}

ast_node! {
    SourceFile:
    directives: AstChildren<Directive>;
    items: AstChildren<Item>;
}

ast_node! {
    FunctionDeclaration:
    fn_token: Option<SyntaxToken Fn>;
    parameter_list: Option<FunctionParameters>;
    return_type: Option<ReturnType>;
    body: Option<CompoundStatement>;
}

impl HasName for FunctionDeclaration {}
impl HasAttributes for FunctionDeclaration {}

ast_node! {
    StructDeclaration:
    struct_token: Option<SyntaxToken Struct>;
    body: Option<StructBody>;
}

impl HasName for StructDeclaration {}
impl HasAttributes for StructDeclaration {}

ast_node! {
    StructBody:
    left_brace_token: Option<SyntaxToken BraceLeft>;
    right_brace_token: Option<SyntaxToken BraceRight>;
    fields: AstChildren<StructMember>;
}

ast_node! {
    StructMember:
    colon_token: Option<SyntaxToken Colon>;
    r#type: Option<TypeSpecifier>;
}

impl HasName for StructMember {}
impl HasAttributes for StructMember {}

ast_node! {
    VariableDeclaration:
    var_token: Option<SyntaxToken Var>;
    colon: Option<SyntaxToken Colon>;
    r#type: Option<TypeSpecifier>;
    equal_token: Option<SyntaxToken Equal>;
    init: Option<Expression>;
}
impl HasTemplateParameters for VariableDeclaration {}
impl HasName for VariableDeclaration {}
impl HasAttributes for VariableDeclaration {}

ast_node! {
    LetDeclaration:
    let_token: Option<SyntaxToken Let>;
    colon: Option<SyntaxToken Colon>;
    r#type: Option<TypeSpecifier>;
    equal_token: Option<SyntaxToken Equal>;
    init: Option<Expression>;
}
impl HasName for LetDeclaration {}
impl HasAttributes for LetDeclaration {}

ast_node! {
    ConstantDeclaration:
    constant_token: Option<SyntaxToken Constant>;
    colon: Option<SyntaxToken Colon>;
    r#type: Option<TypeSpecifier>;
    equal_token: Option<SyntaxToken Equal>;
    init: Option<Expression>;
}

impl HasName for ConstantDeclaration {}
impl HasAttributes for ConstantDeclaration {}

ast_node! {
    OverrideDeclaration:
    override_token: Option<SyntaxToken Override>;
    colon: Option<SyntaxToken Colon>;
    r#type: Option<TypeSpecifier>;
    equal_token: Option<SyntaxToken Equal>;
    init: Option<Expression>;
}
impl HasName for OverrideDeclaration {}
impl HasAttributes for OverrideDeclaration {}

ast_node! {
    TypeAliasDeclaration:
    alias_token: Option<SyntaxToken Alias>;
    equal_token: Option<SyntaxToken Equal>;
    type_declaration: Option<TypeSpecifier>;
}
impl HasName for TypeAliasDeclaration {}
impl HasAttributes for TypeAliasDeclaration {}

ast_enum! {
    enum Item {
        FunctionDeclaration,
        VariableDeclaration,
        ConstantDeclaration,
        OverrideDeclaration,
        TypeAliasDeclaration,
        StructDeclaration,
    }
}

ast_node! {
    EnableDirective:
    enable_extensions: AstChildren<EnableExtensionName>;
}

ast_node! {
    EnableExtensionName:
    ident_token: Option<SyntaxToken Identifier>;
    text: TokenText<'_>;
}

pub struct UnknownExtension;

impl EnableExtensionName {
    pub fn extension(&self) -> Result<EnableExtension, UnknownExtension> {
        match self.text().as_str() {
            "f16" => Ok(EnableExtension::F16),
            "clip_distances" => Ok(EnableExtension::ClipDistances),
            "dual_source_blending" => Ok(EnableExtension::DualSourceBlending),
            "subgroups" => Ok(EnableExtension::Subgroups),
            "primitive_index" => Ok(EnableExtension::PrimitiveIndex),
            _ => Err(UnknownExtension),
        }
    }
}

/// Names that can be `enable`d <https://www.w3.org/TR/WGSL/#syntax-enable_extension_name>
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum EnableExtension {
    F16,
    ClipDistances,
    DualSourceBlending,
    Subgroups,
    PrimitiveIndex,
}

ast_node! {
    RequiresDirective:
    enable_extensions: AstChildren<LanguageExtensionName>;
}

ast_node! {
    LanguageExtensionName:
    ident_token: Option<SyntaxToken Identifier>;
    text: TokenText<'_>;
}

impl LanguageExtensionName {
    pub fn extension(&self) -> Result<LanguageExtension, UnknownExtension> {
        match self.text().as_str() {
            "readonly_and_readwrite_storage_textures" => {
                Ok(LanguageExtension::ReadonlyAndReadwriteStorageTextures)
            },
            "packed_4x8_integer_dot_product" => Ok(LanguageExtension::Packed4x8IntegerDotProduct),
            "unrestricted_pointer_parameters" => {
                Ok(LanguageExtension::UnrestrictedPointerParameters)
            },
            "pointer_composite_access" => Ok(LanguageExtension::PointerCompositeAccess),
            _ => Err(UnknownExtension),
        }
    }
}

/// Language extensions that can be `require`d <https://www.w3.org/TR/WGSL/#syntax-enable_extension_name>
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum LanguageExtension {
    ReadonlyAndReadwriteStorageTextures,
    Packed4x8IntegerDotProduct,
    UnrestrictedPointerParameters,
    PointerCompositeAccess,
}

ast_enum! {
    enum Directive {
        // Diagnostic directive goes here
        EnableDirective,
        RequiresDirective,
    }
}

ast_node! {
    Name:
    ident_token: Option<SyntaxToken Identifier>;
    text: TokenText<'_>;
}

ast_node! {
    NameReference:
    ident_token: Option<SyntaxToken Identifier>;
    text: TokenText<'_>;
}

ast_node! {
    Parameter:
    colon_token: Option<SyntaxToken Colon>;
    r#type: Option<TypeSpecifier>;
}
impl HasName for Parameter {}
impl HasAttributes for Parameter {}

ast_node! {
    FunctionParameters:
    left_parenthesis_token: Option<SyntaxToken ParenthesisLeft>;
    right_parenthesis_token: Option<SyntaxToken ParenthesisRight>;
    parameters: AstChildren<Parameter>;
}

ast_node! {
    ReturnType:
    arrow_token: Option<SyntaxToken Arrow>;
    r#type: Option<TypeSpecifier>;
}

ast_node! {
    TemplateList:
    left_angle_token: Option<SyntaxToken TemplateStart>;
    parameters: AstChildren<Expression>;
    t_angle_token: Option<SyntaxToken TemplateEnd>;
}

ast_node!(InfixExpression);
ast_token_enum! {
    enum BinaryOperatorKind {
        EqualEqual,
        NotEqual,
        GreaterThan,
        GreaterThanEqual,
        LessThan,
        LessThanEqual,
        Modulo,
        Minus,
        Plus,
        Or,
        OrOr,
        And,
        AndAnd,
        Star,
        ForwardSlash,
        Xor,
        ShiftRight,
        ShiftLeft,
    }
}

ast_token_enum! {
    enum PrefixOperatorKind {
        Bang,
        Minus,
        Tilde,
        Star,
        And,
    }
}

ast_node! {
    PrefixExpression:
    expression: Option<Expression>;
}

ast_node! {
    Literal
}

impl Literal {
    /// Returns the kind of this [`Literal`].
    ///
    /// # Panics
    ///
    /// Panics if the literal is invalid.
    #[must_use]
    pub fn kind(&self) -> LiteralKind {
        support::child_token(self.syntax()).expect("invalid literal parsed")
    }
}

ast_token_enum! {
    enum LiteralKind {
        IntLiteral,
        FloatLiteral,
        True,
        False,
    }
}

/// Can be an identifier or a type
ast_node! {
    IdentExpression:
    name_ref: Option<NameReference>;
}
impl HasTemplateParameters for IdentExpression {}

ast_node! {
    ParenthesisExpression:
    left_parenthesis_token: Option<SyntaxToken ParenthesisLeft>;
    right_parenthesis_token: Option<SyntaxToken ParenthesisRight>;
    inner: Option<Expression>;
}

ast_node! {
    FieldExpression:
    expression: Option<Expression>;
    field: Option<SyntaxToken Identifier>;
}

ast_node! {
    FunctionCall:
    ident_expression: Option<IdentExpression>;
    parameters: Option<Arguments>;
}

ast_node! {
    Arguments:
    left_parenthesis_token: Option<SyntaxToken ParenthesisLeft>;
    right_parenthesis_token: Option<SyntaxToken ParenthesisRight>;
    arguments: AstChildren<Expression>;
}

ast_node! {
    IndexExpression
}

impl IndexExpression {
    #[must_use]
    pub fn expression(&self) -> Option<Expression> {
        support::children(self.syntax()).next()
    }

    #[must_use]
    pub fn index(&self) -> Option<Expression> {
        support::children(self.syntax()).nth(1)
    }
}

ast_node! {
    Attribute:
    ident_token: Option<SyntaxToken Identifier>;
    parameters: Option<Arguments>;
}

ast_node! {
    CompoundStatement:
    left_brace_token: Option<SyntaxToken BraceLeft>;
    right_brace_token: Option<SyntaxToken BraceRight>;
    statements: AstChildren<Statement>;
}
impl HasAttributes for CompoundStatement {}

ast_node! {
    AssignmentStatement:
    equal_token: Option<SyntaxToken Equal>;
}

impl AssignmentStatement {
    #[must_use]
    pub fn left_side(&self) -> Option<Expression> {
        crate::support::children(self.syntax()).next()
    }

    #[must_use]
    pub fn right_side(&self) -> Option<Expression> {
        crate::support::children(self.syntax()).nth(1)
    }
}

ast_node! {
    PhonyAssignmentStatement:
    equal_token: Option<SyntaxToken Equal>;
}

impl PhonyAssignmentStatement {
    #[must_use]
    pub fn right_side(&self) -> Option<Expression> {
        crate::support::children(self.syntax()).next()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IncrementDecrement {
    Increment,
    Decrement,
}

ast_node!(IncrementDecrementStatement);
impl IncrementDecrementStatement {
    #[must_use]
    pub fn expression(&self) -> Option<Expression> {
        crate::support::children(self.syntax()).next()
    }

    #[must_use]
    pub fn increment_decrement(&self) -> Option<IncrementDecrement> {
        self.syntax()
            .children_with_tokens()
            .filter_map(rowan::NodeOrToken::into_token)
            .find_map(|token| {
                if let SyntaxKind::MinusMinus | SyntaxKind::PlusPlus = token.kind() {
                    Some(IncrementDecrement::Increment)
                } else {
                    None
                }
            })
    }
}

ast_node! {
    AssertStatement:
        expression: Option<Expression>;
}

ast_token_enum! {
    enum CompoundAssignmentOperator {
        PlusEqual,
        MinusEqual,
        TimesEqual,
        DivisionEqual,
        ModuloEqual,
        AndEqual,
        OrEqual,
        XorEqual,
        ShiftRightEqual,
        ShiftLeftEqual,
    }
}

ast_node!(CompoundAssignmentStatement);

impl CompoundAssignmentStatement {
    #[must_use]
    pub fn left_side(&self) -> Option<Expression> {
        crate::support::children(self.syntax()).next()
    }

    #[must_use]
    pub fn right_side(&self) -> Option<Expression> {
        crate::support::children(self.syntax()).nth(1)
    }

    #[must_use]
    pub fn operator_token(&self) -> Option<SyntaxToken> {
        self.left_side()?.syntax().last_token()?.next_token()
    }

    #[must_use]
    pub fn operator(&self) -> Option<AssignmentOperator> {
        let kind: CompoundAssignmentOperator = support::child_token(self.syntax())?;
        let operator = match kind {
            CompoundAssignmentOperator::PlusEqual(_) => AssignmentOperator::PlusEqual,
            CompoundAssignmentOperator::MinusEqual(_) => AssignmentOperator::MinusEqual,
            CompoundAssignmentOperator::TimesEqual(_) => AssignmentOperator::TimesEqual,
            CompoundAssignmentOperator::DivisionEqual(_) => AssignmentOperator::DivisionEqual,
            CompoundAssignmentOperator::ModuloEqual(_) => AssignmentOperator::ModuloEqual,
            CompoundAssignmentOperator::AndEqual(_) => AssignmentOperator::AndEqual,
            CompoundAssignmentOperator::OrEqual(_) => AssignmentOperator::OrEqual,
            CompoundAssignmentOperator::XorEqual(_) => AssignmentOperator::XorEqual,
            CompoundAssignmentOperator::ShiftRightEqual(_) => AssignmentOperator::ShiftRightAssign,
            CompoundAssignmentOperator::ShiftLeftEqual(_) => AssignmentOperator::ShiftLeftAssign,
        };
        Some(operator)
    }
}

ast_node! {
    IfClause:
    if_token: Option<SyntaxToken If>;
    condition: Option<Expression>;
    block: Option<CompoundStatement>;
}

ast_node! {
    ElseIfClause:
    else_token: Option<SyntaxToken Else>;
    if_token: Option<SyntaxToken If>;
    condition: Option<Expression>;
    block: Option<CompoundStatement>;
}

ast_node! {
    ElseClause:
    else_token: Option<SyntaxToken Else>;
    block: Option<CompoundStatement>;
}

ast_node! {
    IfStatement:
    if_block: Option<IfClause>;
    else_if_blocks: AstChildren<ElseIfClause>;
    else_block: Option<ElseClause>;
}

ast_node! {
    WhileStatement:
    while_token: Option<SyntaxToken While>;
    condition: Option<Expression>;
    block: Option<CompoundStatement>;
}

ast_node! {
    SwitchStatement:
    expression: Option<Expression>;
    block: Option<SwitchBody>;
}

ast_node! {
    SwitchBody:
    cases: AstChildren<SwitchBodyCase>;
}

ast_node! {
    SwitchBodyCase:
    selectors: Option<SwitchCaseSelectors>;
    block: Option<CompoundStatement>;
}

ast_token_enum! {
    enum CaseToken {
        Case,
        Default,
    }
}

impl SwitchBodyCase {
    #[must_use]
    pub fn case_token(&self) -> Option<CaseToken> {
        support::child_token(self.syntax())
    }
}

ast_node! {
    SwitchCaseSelectors:
    exprs: AstChildren<SwitchCaseSelector>;
}

#[derive(Debug)]
pub enum SwitchCaseSelector {
    Expression(Expression),
    SwitchDefaultSelector(SwitchDefaultSelector),
}

impl AstNode for SwitchCaseSelector {
    fn can_cast(kind: SyntaxKind) -> bool {
        if kind == SyntaxKind::SwitchDefaultSelector {
            true
        } else {
            Expression::can_cast(kind)
        }
    }

    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if syntax.kind() == SyntaxKind::SwitchDefaultSelector {
            Some(Self::SwitchDefaultSelector(SwitchDefaultSelector {
                syntax,
            }))
        } else {
            Expression::cast(syntax).map(SwitchCaseSelector::Expression)
        }
    }

    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::SwitchDefaultSelector(item) => &item.syntax,
            Self::Expression(item) => item.syntax(),
        }
    }
}

ast_node! {
    SwitchDefaultSelector:
    default_token: Option<SyntaxToken Default>;
}

ast_node! {
    LoopStatement:
    block: Option<CompoundStatement>;
}

ast_node! {
    ReturnStatement:
    expression: Option<Expression>;
}

ast_node!(BreakStatement);
ast_node!(ContinueStatement);
ast_node!(DiscardStatement);

ast_node! {
    ForStatement:
    for_token: Option<SyntaxToken For>;
}

impl ForStatement {
    #[must_use]
    pub fn block(&self) -> Option<CompoundStatement> {
        support::child(self.syntax())
    }

    pub fn initializer(&self) -> Option<Statement> {
        support::child_syntax(self.syntax(), SyntaxKind::ForInitializer)
            .as_ref()
            .and_then(support::child::<Statement>)
    }

    pub fn condition(&self) -> Option<Expression> {
        support::child_syntax(self.syntax(), SyntaxKind::ForCondition)
            .as_ref()
            .and_then(support::child::<Expression>)
    }

    pub fn continuing_part(&self) -> Option<Statement> {
        support::child_syntax(self.syntax(), SyntaxKind::ForContinuingPart)
            .as_ref()
            .and_then(support::child::<Statement>)
    }
}

ast_node! {
    FunctionCallStatement
}

impl FunctionCallStatement {
    #[must_use]
    pub fn expression(&self) -> Option<FunctionCall> {
        if let crate::ast::Expression::FunctionCall(function_call) =
            crate::support::child::<Expression>(&self.syntax)?
        {
            Some(function_call)
        } else {
            None
        }
    }
}

ast_node! {
    ContinuingStatement:
    block: Option<CompoundStatement>;
}

ast_node! {
    BreakIfStatement:
    condition: Option<Expression>;
}

ast_enum! {
    enum Statement {
        IfStatement,
        SwitchStatement,
        LoopStatement,
        ForStatement,
        WhileStatement,
        CompoundStatement,
        FunctionCallStatement,

        VariableDeclaration,
        LetDeclaration,
        ConstantDeclaration,

        AssignmentStatement,
        CompoundAssignmentStatement,
        PhonyAssignmentStatement,
        IncrementDecrementStatement,

        AssertStatement,
        BreakStatement,
        ContinueStatement,
        // Empty statements are ignored
        DiscardStatement,
        ReturnStatement,
        ContinuingStatement,
        BreakIfStatement,
    }
}

ast_enum! {
    enum Expression {
        IndexExpression,
        FieldExpression,
        PrefixExpression,
        InfixExpression,
        IdentExpression,
        FunctionCall,
        ParenthesisExpression,
        Literal,
    }
}

ast_node! {
    TypeSpecifier:
    name_ref: Option<NameReference>;
}
impl HasTemplateParameters for TypeSpecifier {}

impl InfixExpression {
    #[must_use]
    pub fn left_side(&self) -> Option<Expression> {
        crate::support::children(self.syntax()).next()
    }

    #[must_use]
    pub fn right_side(&self) -> Option<Expression> {
        crate::support::children(self.syntax()).nth(1)
    }

    #[must_use]
    pub fn operator(&self) -> Option<SyntaxToken> {
        self.syntax()
            .children_with_tokens()
            .filter_map(rowan::NodeOrToken::into_token)
            .find(|operator| BinaryOperatorKind::can_cast(operator.kind()))
    }

    #[must_use]
    pub fn op_kind(&self) -> Option<BinaryOperation> {
        if let Some(kind) = support::child_token::<BinaryOperatorKind>(self.syntax()) {
            #[rustfmt::skip]
            let operation = match kind {
                BinaryOperatorKind::EqualEqual(_) => BinaryOperation::Comparison(ComparisonOperation::Equality),
                BinaryOperatorKind::NotEqual(_) => BinaryOperation::Comparison(ComparisonOperation::Inequality),
                BinaryOperatorKind::GreaterThan(_) => BinaryOperation::Comparison(ComparisonOperation::GreaterThan),
                BinaryOperatorKind::GreaterThanEqual(_) => BinaryOperation::Comparison(ComparisonOperation::GreaterThanEqual),
                BinaryOperatorKind::LessThan(_) => BinaryOperation::Comparison(ComparisonOperation::LessThan),
                BinaryOperatorKind::LessThanEqual(_) => BinaryOperation::Comparison(ComparisonOperation::LessThanEqual),
                BinaryOperatorKind::Minus(_) => BinaryOperation::Arithmetic(ArithmeticOperation::Subtraction),
                BinaryOperatorKind::Plus(_) => BinaryOperation::Arithmetic(ArithmeticOperation::Addition),
                BinaryOperatorKind::Star(_) => BinaryOperation::Arithmetic(ArithmeticOperation::Multiplication),
                BinaryOperatorKind::ForwardSlash(_) => BinaryOperation::Arithmetic(ArithmeticOperation::Division),
                BinaryOperatorKind::Modulo(_) => BinaryOperation::Arithmetic(ArithmeticOperation::Remainder),
                BinaryOperatorKind::Or(_) => BinaryOperation::Arithmetic(ArithmeticOperation::BitwiseOr),
                BinaryOperatorKind::And(_) => BinaryOperation::Arithmetic(ArithmeticOperation::BitwiseAnd),
                BinaryOperatorKind::Xor(_)=>BinaryOperation::Arithmetic(ArithmeticOperation::BitwiseXor),
                BinaryOperatorKind::OrOr(_) => BinaryOperation::Logical(LogicOperation::ShortCircuitOr),
                BinaryOperatorKind::AndAnd(_) => BinaryOperation::Logical(LogicOperation::ShortCircuitAnd),
                BinaryOperatorKind::ShiftRight(_) => BinaryOperation::Arithmetic(ArithmeticOperation::ShiftRight),
                BinaryOperatorKind::ShiftLeft(_) => BinaryOperation::Arithmetic(ArithmeticOperation::ShiftLeft),
            };
            Some(operation)
        } else {
            None
        }
    }
}

impl PrefixExpression {
    #[must_use]
    pub fn operator_kind(&self) -> Option<UnaryOperator> {
        let kind: PrefixOperatorKind = support::child_token(self.syntax())?;
        let operator = match kind {
            PrefixOperatorKind::Minus(_) => UnaryOperator::Negation,
            PrefixOperatorKind::Bang(_) => UnaryOperator::LogicalNegation,
            PrefixOperatorKind::Tilde(_) => UnaryOperator::BitwiseComplement,
            PrefixOperatorKind::Star(_) => UnaryOperator::Indirection,
            PrefixOperatorKind::And(_) => UnaryOperator::AddressOf,
        };
        Some(operator)
    }
}
