#![expect(
    clippy::enum_variant_names,
    reason = "The variant names must be clear when self-standing. See the parser snapshot tests."
)]

pub mod operators;

use parser::{SyntaxKind, SyntaxNode};
use rowan::NodeOrToken;

use self::operators::{BinaryOperation, CompoundOperator, UnaryOperator};
use crate::{
    AstChildren, AstNode, AstToken, HasAttributes, HasGenerics, HasName, SyntaxToken, TokenText,
    ast::operators::{ArithmeticOperation, ComparisonOperation, LogicOperation},
    support,
};

macro_rules! ast_node {
    ($kind:ident $($name:ident)? $(:
        $($descendant:ident: $amount_ty:ident < $return_ty:tt $($a:ident)? >;)+
    )?) => {
        ast_node! { @structdef $kind $($name)? }
        ast_node! { @impl $kind $($name)? }

        $(impl $kind {
            $(#[must_use] pub fn $descendant(&self) -> $amount_ty<$return_ty>  {
                ast_node! { @descendant self $amount_ty<$return_ty $($a)?> }
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
    (enum $ty:ident { $($variant:ident,)* }) => {
        #[derive(Debug)]
        pub enum $ty {
            $($variant($variant),)*
        }

        impl AstNode for $ty {
            fn can_cast(kind: SyntaxKind) -> bool {
                match kind {
                    $(SyntaxKind::$variant)|* => true,
                    _ => false,
                }
            }

            fn cast(syntax: SyntaxNode) -> Option<Self> {
                match syntax.kind() {
                    $(SyntaxKind::$variant => Some($ty::$variant($variant { syntax })),)*
                    _ => None,
                }
            }

            fn syntax(&self) -> &SyntaxNode {
                match self {
                    $($ty::$variant(item) => &item.syntax,)*
                }
            }
        }

        $(impl From<$variant> for $ty {
            fn from(value: $variant) -> $ty {
                $ty::$variant(value)
            }
        })*
    };
}

macro_rules! ast_enum_raw {
    (enum $ty:ident { $($variant:ident,)* }) => {
        #[derive(Clone, Debug)]
        pub enum $ty {
            $($variant(SyntaxNode),)*
        }

        impl AstNode for $ty {
            fn can_cast(kind: SyntaxKind) -> bool {
                match kind {
                    $(SyntaxKind::$variant)|* => true,
                    _ => false,
                }
            }

            fn cast(syntax: SyntaxNode) -> Option<Self> {
                match syntax.kind() {
                    $(SyntaxKind::$variant => Some($ty::$variant(syntax)),)*
                    _ => None,
                }
            }

            fn syntax(&self) -> &SyntaxNode {
                match self {
                    $($ty::$variant(item) => &item,)*
                }
            }
        }
    };
}

macro_rules! ast_enum_compound {
    (enum $ty:ident { $($variant:ident,)* }) => {
        #[derive(Clone, Debug)]
        pub enum $ty {
            $($variant($variant),)*
        }

        impl AstNode for $ty {
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
                    $($ty::$variant(item) => item.syntax(),)*
                }
            }
        }
    };
}

macro_rules! ast_token_enum {
    (enum $ty:ident { $($variant:ident,)* }) => {
        #[derive(Clone)]
        pub enum $ty {
            $($variant(SyntaxToken),)*
        }

        impl AstToken for $ty {
            fn can_cast(token: SyntaxToken) -> bool {
                match token.kind() {
                    $(SyntaxKind::$variant)|* => true,
                    _ => false,
                }
            }

            fn cast(syntax: SyntaxToken) -> Option<Self> {
                match syntax.kind() {
                    $(SyntaxKind::$variant => Some($ty::$variant(syntax)),)*
                    _ => None,
                }
            }

            fn syntax(&self) -> &SyntaxToken {
                match self {
                    $($ty::$variant(item) => &item,)*
                }
            }
        }
    };
}

ast_node! {
    SourceFile:
    items: AstChildren<Item>;
}

ast_node! {
    Import:
    import_token: Option<SyntaxToken UnofficialPreprocessorImport>;
    import: Option<ImportKind>;
}

ast_node! {
    ImportPath:
    string_literal: Option<SyntaxToken StringLiteral>;
}

ast_node! {
    ImportCustom
}

impl ImportCustom {
    pub fn segments(&self) -> impl Iterator<Item = ImportCustomSegment> {
        self.syntax
            .children_with_tokens()
            .filter_map(|token| ImportCustomSegment::cast(token.as_token()?.clone()))
    }

    #[must_use]
    pub fn key(&self) -> String {
        self.segments()
            .fold(String::new(), |mut accumulator, segment| {
                match segment {
                    ImportCustomSegment::Identifier(identifier) => {
                        accumulator.push_str(identifier.text());
                    },
                    ImportCustomSegment::ColonColon(colon_colon) => {
                        accumulator.push_str(colon_colon.text());
                    },
                }
                accumulator
            })
    }
}

ast_token_enum! {
    enum ImportCustomSegment {
        Identifier,
        ColonColon,
    }
}

ast_enum! {
    enum ImportKind {
        ImportPath,
        ImportCustom,
    }
}

ast_node! {
    Function:
    fn_token: Option<SyntaxToken Fn>;
    parameter_list: Option<ParameterList>;
    return_type: Option<ReturnType>;
    body: Option<CompoundStatement>;
}

impl HasName for Function {}

impl HasAttributes for Function {}

ast_node! {
    StructDeclaration:
    struct_token: Option<SyntaxToken Struct>;
    name: Option<Name>;
    body: Option<StructDeclBody>;
}

impl HasAttributes for StructDeclaration {}

ast_node! {
    StructDeclBody:
    left_brace_token: Option<SyntaxToken BraceLeft>;
    right_brace_token: Option<SyntaxToken BraceRight>;
    fields: AstChildren<StructDeclarationField>;
}

ast_node! {
    StructDeclarationField:
    variable_ident_declaration: Option<VariableIdentDeclaration>;
}

impl HasAttributes for StructDeclarationField {}

ast_node! {
    GlobalVariableDeclaration:
    var_token: Option<SyntaxToken Var>;
    binding: Option<Binding>;
    variable_qualifier: Option<VariableQualifier>;
    ty: Option<Type>;
    init: Option<Expression>;
}

impl HasAttributes for GlobalVariableDeclaration {}

ast_node! {
    GlobalConstantDeclaration:
    binding: Option<Binding>;
    variable_qualifier: Option<VariableQualifier>;
    ty: Option<Type>;
    init: Option<Expression>;
}

impl HasAttributes for OverrideDeclaration {}

ast_node! {
    OverrideDeclaration:
    binding: Option<Binding>;
    variable_qualifier: Option<VariableQualifier>;
    ty: Option<Type>;
    init: Option<Expression>;
}

ast_node! {
    TypeAliasDeclaration:
    alias_token: Option<SyntaxToken Alias>;
    name: Option<Name>;
    equal_token: Option<SyntaxToken Equal>;
    type_declaration: Option<Type>;
}

ast_enum! {
    enum Item {
        Function,
        StructDeclaration,
        GlobalVariableDeclaration,
        GlobalConstantDeclaration,
        OverrideDeclaration,
        Import,
        TypeAliasDeclaration,
    }
}

ast_node! {
    Name:
    ident_token: Option<SyntaxToken Identifier>;
    text: TokenText<'_>;
}

ast_node! {
    Parameter:
    variable_ident_declaration: Option<VariableIdentDeclaration>;
    import: Option<Import>;
}

ast_node! {
    ParameterList:
    left_parenthesis_token: Option<SyntaxToken ParenthesisLeft>;
    right_parenthesis_token: Option<SyntaxToken ParenthesisRight>;
    parameters: AstChildren<Parameter>;
}

ast_node!(Binding);

impl HasName for Binding {}

ast_node! {
    VariableIdentDeclaration:
    colon_token: Option<SyntaxToken Colon>;
    binding: Option<Binding>;
    ty: Option<Type>;
}

ast_node! {
    FunctionParameterList:
    left_parenthesis_token: Option<SyntaxToken ParenthesisLeft>;
    right_parenthesis_token: Option<SyntaxToken ParenthesisRight>;
    arguments: AstChildren<Expression>;
}

ast_node! {
    ReturnType:
    arrow_token: Option<SyntaxToken Arrow>;
    ty: Option<Type>;
}

ast_node! {
    GenericArgumentList:
    left_angle_token: Option<SyntaxToken LessThan>;
    t_angle_token: Option<SyntaxToken GreaterThan>;
}

impl GenericArgumentList {
    #[rustfmt::skip]
    pub fn generics(&self) -> impl Iterator<Item = GenericArg> + use<> {
        self.syntax
            .children_with_tokens()
            .filter_map(|node_or_token| match node_or_token {
                rowan::NodeOrToken::Node(node) if Literal::can_cast(node.kind()) => Literal::cast(node).map(GenericArg::Literal),
                rowan::NodeOrToken::Node(node) if Type::can_cast(node.kind()) => Type::cast(node).map(GenericArg::Type),
                rowan::NodeOrToken::Token(token) if AccessMode::can_cast(token.clone()) => AccessMode::cast(token).map(GenericArg::AccessMode),
                rowan::NodeOrToken::Token(token) if AddressSpace::can_cast(token.clone()) => AddressSpace::cast(token).map(GenericArg::AddressSpace),
                rowan::NodeOrToken::Node(_) | rowan::NodeOrToken::Token(_) => None,
            })
    }
}

ast_token_enum! {
    enum AccessMode {
        Read,
        Write,
        ReadWrite,
    }
}

ast_token_enum! {
    enum AddressSpace {
        FunctionClass,
        Private,
        Workgroup,
        Uniform,
        Storage,
        PushConstant,
    }
}

pub enum GenericArg {
    Type(Type),
    Literal(Literal),
    AccessMode(AccessMode),
    AddressSpace(AddressSpace),
}

impl GenericArg {
    #[must_use]
    pub fn as_type(&self) -> Option<Type> {
        match self {
            Self::Type(r#type) => Some(r#type.clone()),
            Self::Literal(_) | Self::AccessMode(_) | Self::AddressSpace(_) => None,
        }
    }

    #[must_use]
    pub fn as_literal(&self) -> Option<Literal> {
        match self {
            Self::Literal(r#type) => Some(r#type.clone()),
            Self::Type(_) | Self::AccessMode(_) | Self::AddressSpace(_) => None,
        }
    }

    #[must_use]
    pub fn as_access_mode(&self) -> Option<AccessMode> {
        match self {
            Self::AccessMode(access) => Some(access.clone()),
            Self::Type(_) | Self::Literal(_) | Self::AddressSpace(_) => None,
        }
    }

    #[must_use]
    pub fn as_address_space(&self) -> Option<AddressSpace> {
        match self {
            Self::AddressSpace(class) => Some(class.clone()),
            Self::Type(_) | Self::Literal(_) | Self::AccessMode(_) => None,
        }
    }
}

ast_node! {
    BinaryOperator
}

ast_node! {
    TypeInitializer:
    ty: Option<Type>;
    arguments: Option<FunctionParameterList>;
}

ast_node!(VariableQualifier);
impl VariableQualifier {
    #[must_use]
    pub fn access_mode(&self) -> Option<AccessMode> {
        support::child_token::<AccessMode>(self.syntax())
    }

    #[must_use]
    pub fn address_space(self) -> Option<AddressSpace> {
        support::child_token::<AddressSpace>(self.syntax())
    }
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

ast_node! {
    PathExpression:
    name_ref: Option<NameReference>;
}

ast_node! {
    NameReference:
    text: TokenText<'_>;
}

ast_node! {
    ParenthesisExpression:
    left_parenthesis_token: Option<SyntaxToken ParenthesisLeft>;
    right_parenthesis_token: Option<SyntaxToken ParenthesisRight>;
    inner: Option<Expression>;
}

ast_node! {
    BitcastExpression:
    bitcast_token: Option<SyntaxToken Bitcast>;
    left_angle_token: Option<SyntaxToken LessThan>;
    right_angle_token: Option<SyntaxToken GreaterThan>;
    ty: Option<Type>;
    inner: Option<ParenthesisExpression>;
}

ast_node! {
    FieldExpression:
    expression: Option<Expression>;
    name_ref: Option<NameReference>;
}

ast_node! {
    FunctionCall:
    name_ref: Option<NameReference>;
    parameters: Option<FunctionParameterList>;
}

ast_node! {
    InvalidFunctionCall:
    expression: Option<Expression>;
    parameters: Option<FunctionParameterList>;
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

ast_node! {AttributeList:
    attributes: AstChildren<Attribute>;
}

ast_node! {Attribute:
    ident_token: Option<SyntaxToken Identifier>;
    parameters: Option<AttributeParameters>;
}

ast_node! {AttributeParameters:
    values: AstChildren<IdentOrLiteral>;
}

ast_node! {Identifier:
    text: TokenText<'_>;
}

ast_enum! {
    enum IdentOrLiteral {
        Identifier,
        Literal,
    }
}

ast_node! {
    CompoundStatement:
    left_brace_token: Option<SyntaxToken BraceLeft>;
    right_brace_token: Option<SyntaxToken BraceRight>;
    statements: AstChildren<Statement>;
}

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
    pub fn operator(&self) -> Option<CompoundOperator> {
        let kind: CompoundAssignmentOperator = support::child_token(self.syntax())?;
        let operator = match kind {
            CompoundAssignmentOperator::PlusEqual(_) => CompoundOperator::Add,
            CompoundAssignmentOperator::MinusEqual(_) => CompoundOperator::Subtract,
            CompoundAssignmentOperator::TimesEqual(_) => CompoundOperator::Multiply,
            CompoundAssignmentOperator::DivisionEqual(_) => CompoundOperator::Divide,
            CompoundAssignmentOperator::ModuloEqual(_) => CompoundOperator::Modulo,
            CompoundAssignmentOperator::AndEqual(_) => CompoundOperator::BitAnd,
            CompoundAssignmentOperator::OrEqual(_) => CompoundOperator::BitOr,
            CompoundAssignmentOperator::XorEqual(_) => CompoundOperator::BitXor,
            CompoundAssignmentOperator::ShiftRightEqual(_) => CompoundOperator::ShiftRight,
            CompoundAssignmentOperator::ShiftLeftEqual(_) => CompoundOperator::ShiftLeft,
        };
        Some(operator)
    }
}

ast_node! {
    ElseIfBlock:
    else_token: Option<SyntaxToken Else>;
    if_token: Option<SyntaxToken If>;
    condition: Option<Expression>;
    block: Option<CompoundStatement>;
}

ast_node! {
    ElseBlock:
    else_token: Option<SyntaxToken Else>;
    block: Option<CompoundStatement>;
}

ast_node! {
    IfStatement:
    if_token: Option<SyntaxToken If>;
    condition: Option<Expression>;
    block: Option<CompoundStatement>;
    else_if_blocks: AstChildren<ElseIfBlock>;
    else_block: Option<ElseBlock>;
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
    block: Option<SwitchBlock>;
}

ast_node! {
    SwitchBlock:
    cases: AstChildren<SwitchBodyCase>;
    default: AstChildren<SwitchBodyDefault>;
}

ast_node! {
    SwitchBodyCase:
    selectors: Option<SwitchCaseSelectors>;
    block: Option<CompoundStatement>;
}

ast_node! {
    SwitchCaseSelectors:
    exprs: AstChildren<Expression>;
}

ast_node! {
    SwitchBodyDefault:
    block: Option<CompoundStatement>;
}

ast_node! {
    LoopStatement:
    block: Option<CompoundStatement>;
}

ast_node! {
    ReturnStatement:
    expression: Option<Expression>;
}

ast_node! {
    VariableStatement:
    variable_qualifier: Option<VariableQualifier>;
    binding: Option<Binding>;
    colon: Option<SyntaxToken Colon>;
    ty: Option<Type>;
    equal_token: Option<SyntaxToken Equal>;
    initializer: Option<Expression>;
}

impl VariableStatement {
    #[must_use]
    pub fn kind(&self) -> Option<VariableStatementKind> {
        #[expect(clippy::wildcard_enum_match_arm, reason = "not readable")]
        self.syntax()
            .children_with_tokens()
            .filter_map(rowan::NodeOrToken::into_token)
            .find_map(|token| match token.kind() {
                SyntaxKind::Constant => Some(VariableStatementKind::Constant),
                SyntaxKind::Let => Some(VariableStatementKind::Let),
                SyntaxKind::Var => Some(VariableStatementKind::Var),
                _ => None,
            })
    }
}

pub enum VariableStatementKind {
    Constant,
    Let,
    Var,
}

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
    FunctionCallStatement:
    expression: Option<Expression>;
}

ast_node! {
    Discard
}

ast_node! {
    Break
}

ast_node! {
    Continue
}

ast_node! {
    ContinuingStatement:
    block: Option<CompoundStatement>;
}

ast_enum! {
    enum Statement {
        CompoundStatement,
        VariableStatement,
        ReturnStatement,
        AssignmentStatement,
        CompoundAssignmentStatement,
        IfStatement,
        ForStatement,
        SwitchStatement,
        LoopStatement,
        WhileStatement,
        Discard,
        Break,
        Continue,
        ContinuingStatement,
        FunctionCallStatement,
        IncrementDecrementStatement,
    }
}

ast_enum! {
    enum Expression {
        InfixExpression,
        PrefixExpression,
        Literal,
        ParenthesisExpression,
        FieldExpression,
        FunctionCall,
        TypeInitializer,
        IndexExpression,
        PathExpression,
        BitcastExpression,
        InvalidFunctionCall,
    }
}

ast_enum_raw! {
    enum MatrixType {
        Mat2x2,
        Mat2x3,
        Mat2x4,
        Mat3x2,
        Mat3x3,
        Mat3x4,
        Mat4x2,
        Mat4x3,
        Mat4x4,
    }
}

ast_enum_raw! {
    enum VecType {
        Vec2,
        Vec3,
        Vec4,
    }
}

ast_enum_raw! {
    enum ScalarType {
        Bool,
        Float32,
        Int32,
        Uint32,
    }
}

ast_enum_raw! {
    enum TextureType {
        Texture1d,
        Texture2d,
        Texture2dArray,
        Texture3d,
        TextureCube,
        TextureCubeArray,
        TextureMultisampled2d,
        TextureExternal,
        TextureStorage1d,
        TextureStorage2d,
        TextureStorage2dArray,
        TextureStorage3d,
        TextureDepth2d,
        TextureDepth2dArray,
        TextureDepthCube,
        TextureDepthCubeArray,
        TextureDepthMultisampled2d,
    }
}

ast_enum_raw! {
    enum SamplerType {
        Sampler,
        SamplerComparison,
    }
}

ast_node! {
    TypeExpression:
    name: Option<NameReference>;
}

ast_node!(Atomic AtomicType);
ast_node!(Array ArrayType);
ast_node!(BindingArray BindingArrayType);
ast_node!(Pointer PointerType);

ast_enum_compound! {
    enum Type {
        PathType,
        ScalarType,
        VecType,
        MatrixType,
        TextureType,
        SamplerType,
        AtomicType,
        ArrayType,
        BindingArrayType,
        PointerType,
    }
}

impl Type {
    #[must_use]
    pub fn as_name(&self) -> Option<NameReference> {
        match self {
            Self::PathType(path) => path.name(),
            Self::ScalarType(_)
            | Self::VecType(_)
            | Self::MatrixType(_)
            | Self::TextureType(_)
            | Self::SamplerType(_)
            | Self::AtomicType(_)
            | Self::ArrayType(_)
            | Self::BindingArrayType(_)
            | Self::PointerType(_) => None,
        }
    }
}

impl HasGenerics for Type {}

impl HasGenerics for VecType {}

impl HasGenerics for MatrixType {}

impl HasGenerics for TextureType {}

impl HasGenerics for ScalarType {}

impl HasGenerics for AtomicType {}

impl HasGenerics for ArrayType {}

impl HasGenerics for BindingArrayType {}

impl HasGenerics for PointerType {}

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
    pub fn op(&self) -> Option<NodeOrToken<SyntaxNode, SyntaxToken>> {
        if let Some(op) = self
            .syntax()
            .children()
            .find(|child| matches!(child.kind(), SyntaxKind::ShiftLeft | SyntaxKind::ShiftRight))
        {
            return Some(NodeOrToken::Node(op));
        }

        if let Some(token) = self.left_side()?.syntax().last_token()?.next_token() {
            return Some(NodeOrToken::Token(token));
        }

        None
    }

    #[must_use]
    pub fn op_kind(&self) -> Option<BinaryOperation> {
        if let Some(kind) = support::child_token::<BinaryOperatorKind>(self.syntax()) {
            #[rustfmt::skip]
            let op = match kind {
                BinaryOperatorKind::EqualEqual(_) => BinaryOperation::Comparison(ComparisonOperation::Equality { negated: false }),
                BinaryOperatorKind::NotEqual(_) => BinaryOperation::Comparison(ComparisonOperation::Equality { negated: true }),
                BinaryOperatorKind::GreaterThan(_) => BinaryOperation::Comparison(ComparisonOperation::Ordering { ordering: operators::Ordering::Greater, strict: true }),
                BinaryOperatorKind::GreaterThanEqual(_) => BinaryOperation::Comparison(ComparisonOperation::Ordering { ordering: operators::Ordering::Greater, strict: false }),
                BinaryOperatorKind::LessThan(_) => BinaryOperation::Comparison(ComparisonOperation::Ordering { ordering: operators::Ordering::Less, strict: true }),
                BinaryOperatorKind::LessThanEqual(_) => BinaryOperation::Comparison(ComparisonOperation::Ordering { ordering: operators::Ordering::Less, strict: false }),
                BinaryOperatorKind::Minus(_) => BinaryOperation::Arithmetic(ArithmeticOperation::Subtract),
                BinaryOperatorKind::Plus(_) => BinaryOperation::Arithmetic(ArithmeticOperation::Add),
                BinaryOperatorKind::Star(_) => BinaryOperation::Arithmetic(ArithmeticOperation::Multiply),
                BinaryOperatorKind::ForwardSlash(_) => BinaryOperation::Arithmetic(ArithmeticOperation::Divide),
                BinaryOperatorKind::Modulo(_) => BinaryOperation::Arithmetic(ArithmeticOperation::Modulo),
                BinaryOperatorKind::Or(_) => BinaryOperation::Arithmetic(ArithmeticOperation::BitOr),
                BinaryOperatorKind::And(_) => BinaryOperation::Arithmetic(ArithmeticOperation::BitAnd),
                BinaryOperatorKind::Xor(_)=>BinaryOperation::Arithmetic(ArithmeticOperation::BitXor),
                BinaryOperatorKind::OrOr(_) => BinaryOperation::Logical(LogicOperation::Or),
                BinaryOperatorKind::AndAnd(_) => BinaryOperation::Logical(LogicOperation::And),
            };
            Some(op)
        } else {
            #[expect(clippy::wildcard_enum_match_arm, reason = "not readable")]
            self.syntax()
                .children()
                .find_map(|child| match child.kind() {
                    SyntaxKind::ShiftLeft => {
                        Some(BinaryOperation::Arithmetic(ArithmeticOperation::ShiftLeft))
                    },
                    SyntaxKind::ShiftRight => {
                        Some(BinaryOperation::Arithmetic(ArithmeticOperation::ShiftRight))
                    },
                    _ => None,
                })
        }
    }
}

impl PrefixExpression {
    #[must_use]
    pub fn op_kind(&self) -> Option<UnaryOperator> {
        let kind: PrefixOperatorKind = support::child_token(self.syntax())?;
        let op = match kind {
            PrefixOperatorKind::Minus(_) => UnaryOperator::Minus,
            PrefixOperatorKind::Bang(_) => UnaryOperator::Not,
            PrefixOperatorKind::Tilde(_) => UnaryOperator::BitNot,
            PrefixOperatorKind::Star(_) => UnaryOperator::Dereference,
            PrefixOperatorKind::And(_) => UnaryOperator::Reference,
        };
        Some(op)
    }
}

ast_enum! {
    enum NameLike {
        NameReference,
        Name,
    }
}

impl NameLike {
    #[must_use]
    pub const fn as_name_ref(&self) -> Option<&NameReference> {
        match self {
            Self::NameReference(name_ref) => Some(name_ref),
            Self::Name(_) => None,
        }
    }
}
