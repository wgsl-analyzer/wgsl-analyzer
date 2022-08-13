pub mod operators;

use crate::ast::operators::ArithOp;
use crate::ast::operators::CmpOp;
use crate::ast::operators::LogicOp;
use crate::support;
use crate::AstChildren;
use crate::AstNode;
use crate::AstToken;
use crate::HasAttrs;
use crate::HasGenerics;
use crate::HasName;
use crate::SyntaxToken;
use crate::TokenText;
use rowan::NodeOrToken;
use wgsl_parser::{SyntaxKind, SyntaxNode};

use self::operators::BinaryOp;
use self::operators::CompoundOp;
use self::operators::UnaryOp;

macro_rules! ast_node {
    ($kind:ident $($name:ident)? $(:
        $($descendant:ident: $amount_ty:ident < $return_ty:tt $($a:ident)? >;)+
    )?) => {
        ast_node! { @structdef $kind $($name)? }
        ast_node! { @impl $kind $($name)? }

        $(impl $kind {
            $(pub fn $descendant(&self) -> $amount_ty<$return_ty>  {
                ast_node!(@descendant self $amount_ty<$return_ty $($a)?>)
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
                    $($ty::$variant(it) => &it.syntax,)*
                }
            }
        }

        $(impl From<$variant> for $ty {
            fn from(val: $variant) -> $ty {
                $ty::$variant(val)
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
                    $($ty::$variant(it) => &it,)*
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
                    $($ty::$variant(it) => it.syntax(),)*
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
                    $($ty::$variant(it) => &it,)*
                }
            }
        }
    };
}

ast_node!(SourceFile:
    items: AstChildren<Item>;
);

ast_node!(Import:
    import_token: Option<SyntaxToken UnofficialPreprocessorImport>;
    import: Option<ImportKind>;
);

ast_node!(ImportPath:
    string_literal: Option<SyntaxToken StringLiteral>;
);
ast_node!(ImportCustom);
impl ImportCustom {
    pub fn segments(&self) -> impl Iterator<Item = ImportCustomSegment> {
        self.syntax
            .children_with_tokens()
            .filter_map(|token| ImportCustomSegment::cast(token.as_token()?.clone()))
    }

    pub fn key(&self) -> String {
        self.segments().fold(String::new(), |mut acc, segment| {
            match segment {
                ImportCustomSegment::Ident(ident) => acc.push_str(ident.text()),
                ImportCustomSegment::ColonColon(colon_colon) => acc.push_str(colon_colon.text()),
            };
            acc
        })
    }
}

ast_token_enum! {
    enum ImportCustomSegment {
        Ident,
        ColonColon,
    }
}

ast_enum! {
    enum ImportKind {
        ImportPath,
        ImportCustom,
    }
}

ast_node!(Function:
    fn_token: Option<SyntaxToken Fn>;
    param_list: Option<ParamList>;
    return_type: Option<ReturnType>;
    body: Option<CompoundStatement>;
);
impl HasName for Function {}
impl HasAttrs for Function {}
ast_node!(StructDecl:
    struct_token: Option<SyntaxToken Struct>;
    name: Option<Name>;
    body: Option<StructDeclBody>;
);
impl HasAttrs for StructDecl {}
ast_node!(StructDeclBody:
    fields: AstChildren<StructDeclField>;
);
ast_node!(StructDeclField:
    variable_ident_decl: Option<VariableIdentDecl>;
);
impl HasAttrs for StructDeclField {}
ast_node!(GlobalVariableDecl:
    var_token: Option<SyntaxToken Var>;
    binding: Option<Binding>;
    variable_qualifier: Option<VariableQualifier>;
    ty: Option<Type>;
    init: Option<Expr>;
);
impl HasAttrs for GlobalVariableDecl {}
ast_node!(GlobalConstantDecl:
    binding: Option<Binding>;
    variable_qualifier: Option<VariableQualifier>;
    ty: Option<Type>;
    init: Option<Expr>;
);

ast_node!(TypeAliasDecl:
    type_token: Option<SyntaxToken Type>;
    name: Option<Name>;
    equal_token: Option<SyntaxToken Equal>;
    type_decl: Option<Type>;
);

ast_enum! {
    enum Item {
        Function,
        StructDecl,
        GlobalVariableDecl,
        GlobalConstantDecl,
        Import,
        TypeAliasDecl,
    }
}

ast_node!(Name:
    ident_token: Option<SyntaxToken Ident>;
    text: TokenText<'_>;
);
ast_node!(Param:
    variable_ident_declaration: Option<VariableIdentDecl>;
    import: Option<Import>;
);
ast_node!(ParamList:
    left_paren_token: Option<SyntaxToken ParenLeft>;
    right_paren_token: Option<SyntaxToken ParenRight>;
    params: AstChildren<Param>;
);
ast_node!(Binding);
impl HasName for Binding {}
ast_node!(VariableIdentDecl:
    colon_token: Option<SyntaxToken Colon>;
    binding: Option<Binding>;
    ty: Option<Type>;
);
ast_node!(FunctionParamList:
    left_paren_token: Option<SyntaxToken ParenLeft>;
    right_paren_token: Option<SyntaxToken ParenRight>;
    args: AstChildren<Expr>;
);
ast_node!(ReturnType:
    arrow_token: Option<SyntaxToken Arrow>;
    ty: Option<Type>;
);
ast_node!(GenericArgList);

impl GenericArgList {
    #[rustfmt::skip]
    pub fn generics(&self) -> impl Iterator<Item = GenericArg> {
        self.syntax
            .children_with_tokens()
            .filter_map(|it| match it {
                rowan::NodeOrToken::Node(node) if Literal::can_cast(node.kind()) => Literal::cast(node).map(GenericArg::Literal),
                rowan::NodeOrToken::Node(node) if Type::can_cast(node.kind()) => Type::cast(node).map(GenericArg::Type),
                rowan::NodeOrToken::Token(token) if AccessMode::can_cast(token.clone()) => AccessMode::cast(token).map(GenericArg::AccessMode),
                rowan::NodeOrToken::Token(token) if StorageClass::can_cast(token.clone()) => StorageClass::cast(token).map(GenericArg::StorageClass),
                _ => None,
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
    enum StorageClass {
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
    StorageClass(StorageClass),
}
impl GenericArg {
    pub fn as_type(&self) -> Option<Type> {
        match self {
            GenericArg::Type(ty) => Some(ty.clone()),
            _ => None,
        }
    }
    pub fn as_literal(&self) -> Option<Literal> {
        match self {
            GenericArg::Literal(ty) => Some(ty.clone()),
            _ => None,
        }
    }
    pub fn as_access_mode(&self) -> Option<AccessMode> {
        match self {
            GenericArg::AccessMode(access) => Some(access.clone()),
            _ => None,
        }
    }
    pub fn as_storage_class(&self) -> Option<StorageClass> {
        match self {
            GenericArg::StorageClass(class) => Some(class.clone()),
            _ => None,
        }
    }
}

ast_node!(BinaryOperator);
ast_node!(TypeInitializer:
    ty: Option<Type>;
);

ast_node!(VariableQualifier);
impl VariableQualifier {
    pub fn access_mode(&self) -> Option<AccessMode> {
        support::child_token::<AccessMode>(self.syntax())
    }
    pub fn storage_class(self) -> Option<StorageClass> {
        support::child_token::<StorageClass>(self.syntax())
    }
}

ast_node!(InfixExpr);
ast_token_enum! {
    enum BinaryOpKind {
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
    }
}
ast_token_enum! {
    enum PrefixOpKind {
        Bang,
        Minus,
        Tilde,
        Star,
        And,
    }
}
ast_node!(PrefixExpr:
    expr: Option<Expr>;
);
ast_node!(Literal);
impl Literal {
    pub fn kind(&self) -> LiteralKind {
        support::child_token(self.syntax()).expect("invalid literal parsed")
    }
}
ast_token_enum! {
    enum LiteralKind {
        IntLiteral,
        UintLiteral,
        HexFloatLiteral,
        DecimalFloatLiteral,
        True,
        False,
    }
}
ast_node!(PathExpr:
    name_ref: Option<NameRef>;
);
ast_node!(NameRef:
    text: TokenText<'_>;
);
ast_node!(ParenExpr:
    inner: Option<Expr>;
);
ast_node!(FieldExpr:
    expr: Option<Expr>;
    name_ref: Option<NameRef>;
);
ast_node!(FunctionCall:
    expr: Option<Expr>;
    type_initializer: Option<TypeInitializer>;
    params: Option<FunctionParamList>;
);
ast_node!(IndexExpr);
impl IndexExpr {
    pub fn expr(&self) -> Option<Expr> {
        support::children(self.syntax()).next()
    }
    pub fn index(&self) -> Option<Expr> {
        support::children(self.syntax()).nth(1)
    }
}

ast_node!(AttributeList:
    attributes: AstChildren<Attribute>;
);
ast_node!(Attribute:
    ident_token: Option<SyntaxToken Ident>;
    params: Option<AttributeParameters>;
);
ast_node!(AttributeParameters:
    values: AstChildren<IdentOrLiteral>;
);

ast_node!(Ident:
    text: TokenText<'_>;
);
ast_enum! {
    enum IdentOrLiteral {
        Ident,
        Literal,
    }
}

ast_node!(CompoundStatement:
    left_brace_token: Option<SyntaxToken BraceLeft>;
    right_brace_token: Option<SyntaxToken BraceRight>;
    statements: AstChildren<Statement>;
);
ast_node!(AssignmentStmt:
    equal_token: Option<SyntaxToken Equal>;
);
impl AssignmentStmt {
    pub fn lhs(&self) -> Option<Expr> {
        crate::support::children(self.syntax()).next()
    }
    pub fn rhs(&self) -> Option<Expr> {
        crate::support::children(self.syntax()).nth(1)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IncrDecr {
    Increment,
    Decrement,
}

ast_node!(IncrDecrStatement);
impl IncrDecrStatement {
    pub fn expr(&self) -> Option<Expr> {
        crate::support::children(self.syntax()).next()
    }
    pub fn incr_decr(&self) -> Option<IncrDecr> {
        self.syntax()
            .children_with_tokens()
            .filter_map(|it| it.into_token())
            .find_map(|token| match token.kind() {
                SyntaxKind::PlusPlus => Some(IncrDecr::Increment),
                SyntaxKind::MinusMinus => Some(IncrDecr::Increment),
                _ => None,
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

ast_node!(CompoundAssignmentStmt);
impl CompoundAssignmentStmt {
    pub fn lhs(&self) -> Option<Expr> {
        crate::support::children(self.syntax()).next()
    }
    pub fn rhs(&self) -> Option<Expr> {
        crate::support::children(self.syntax()).nth(1)
    }
    pub fn op_token(&self) -> Option<SyntaxToken> {
        self.lhs()?.syntax().last_token()?.next_token()
    }
    pub fn op(&self) -> Option<CompoundOp> {
        let kind: CompoundAssignmentOperator = support::child_token(self.syntax())?;
        let op = match kind {
            CompoundAssignmentOperator::PlusEqual(_) => CompoundOp::Add,
            CompoundAssignmentOperator::MinusEqual(_) => CompoundOp::Sub,
            CompoundAssignmentOperator::TimesEqual(_) => CompoundOp::Mul,
            CompoundAssignmentOperator::DivisionEqual(_) => CompoundOp::Div,
            CompoundAssignmentOperator::ModuloEqual(_) => CompoundOp::Modulo,
            CompoundAssignmentOperator::AndEqual(_) => CompoundOp::BitAnd,
            CompoundAssignmentOperator::OrEqual(_) => CompoundOp::BitOr,
            CompoundAssignmentOperator::XorEqual(_) => CompoundOp::BitXor,
            CompoundAssignmentOperator::ShiftRightEqual(_) => CompoundOp::Shr,
            CompoundAssignmentOperator::ShiftLeftEqual(_) => CompoundOp::Shl,
        };
        Some(op)
    }
}
ast_node!(ElseIfBlock:
    else_token: Option<SyntaxToken Else>;
    if_token: Option<SyntaxToken If>;
    condition: Option<Expr>;
    block: Option<CompoundStatement>;
);
ast_node!(ElseBlock:
    else_token: Option<SyntaxToken Else>;
    block: Option<CompoundStatement>;
);
ast_node!(IfStatement:
    if_token: Option<SyntaxToken If>;
    condition: Option<Expr>;
    block: Option<CompoundStatement>;
    else_if_blocks: AstChildren<ElseIfBlock>;
    else_block: Option<ElseBlock>;
);

ast_node!(SwitchStatement:
    expr: Option<Expr>;
    block: Option<SwitchBlock>;
);
ast_node!(SwitchBlock:
    cases: AstChildren<SwitchBodyCase>;
    default: AstChildren<SwitchBodyDefault>;
);
ast_node!(SwitchBodyCase:
    selectors: Option<SwitchCaseSelectors>;
    block: Option<CompoundStatement>;
);
ast_node!(SwitchCaseSelectors:
    exprs: AstChildren<Expr>;
);
ast_node!(SwitchBodyDefault:
    block: Option<CompoundStatement>;
);

ast_node!(LoopStatement:
    block: Option<CompoundStatement>;
);
ast_node!(ReturnStmt:
    expr: Option<Expr>;
);
ast_node!(VariableStatement:
    variable_qualifier: Option<VariableQualifier>;
    binding: Option<Binding>;
    ty: Option<Type>;
    equal_token: Option<SyntaxToken Equal>;
    initializer: Option<Expr>;
);
impl VariableStatement {
    pub fn kind(&self) -> Option<VariableStatementKind> {
        self.syntax()
            .children_with_tokens()
            .filter_map(|it| it.into_token())
            .find_map(|token| match token.kind() {
                SyntaxKind::Let => Some(VariableStatementKind::Let),
                SyntaxKind::Var => Some(VariableStatementKind::Var),
                _ => None,
            })
    }
}
pub enum VariableStatementKind {
    Let,
    Var,
}

ast_node!(ForStatement:
    for_token: Option<SyntaxToken For>;
);
impl ForStatement {
    pub fn block(&self) -> Option<CompoundStatement> {
        support::child(self.syntax())
    }

    pub fn initializer(&self) -> Option<Statement> {
        support::child_syntax(self.syntax(), SyntaxKind::ForInitializer)
            .as_ref()
            .and_then(support::child::<Statement>)
    }
    pub fn condition(&self) -> Option<Expr> {
        support::child_syntax(self.syntax(), SyntaxKind::ForCondition)
            .as_ref()
            .and_then(support::child::<Expr>)
    }
    pub fn continuing_part(&self) -> Option<Statement> {
        support::child_syntax(self.syntax(), SyntaxKind::ForContinuingPart)
            .as_ref()
            .and_then(support::child::<Statement>)
    }
}

ast_node!(ExprStatement:
    expr: Option<Expr>;
);
ast_node!(Discard);
ast_node!(Break);
ast_node!(Continue);
ast_node!(ContinuingStatement:
    block: Option<CompoundStatement>;
);

ast_enum! {
    enum Statement {
        CompoundStatement,
        VariableStatement,
        ReturnStmt,
        AssignmentStmt,
        CompoundAssignmentStmt,
        IfStatement,
        ForStatement,
        SwitchStatement,
        LoopStatement,
        Discard,
        Break,
        Continue,
        ContinuingStatement,
        ExprStatement,
        IncrDecrStatement,
    }
}

ast_enum! {
    enum Expr {
        InfixExpr,
        PrefixExpr,
        Literal,
        ParenExpr,
        FieldExpr,
        FunctionCall,
        IndexExpr,
        PathExpr,
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
ast_node!(PathType:
    name: Option<NameRef>;
);
ast_node!(Atomic AtomicType);
ast_node!(Array ArrayType);
ast_node!(BindingArray BindingArrayType);
ast_node!(Pointer PtrType);

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
        PtrType,
    }
}

impl Type {
    pub fn as_name(&self) -> Option<NameRef> {
        match self {
            Type::PathType(path) => path.name(),
            _ => None,
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
impl HasGenerics for PtrType {}

impl InfixExpr {
    pub fn lhs(&self) -> Option<Expr> {
        crate::support::children(self.syntax()).next()
    }
    pub fn rhs(&self) -> Option<Expr> {
        crate::support::children(self.syntax()).nth(1)
    }
    pub fn op(&self) -> Option<NodeOrToken<SyntaxNode, SyntaxToken>> {
        if let Some(op) = self
            .syntax()
            .children()
            .find(|child| matches!(child.kind(), SyntaxKind::ShiftLeft | SyntaxKind::ShiftRight))
        {
            return Some(NodeOrToken::Node(op));
        }

        if let Some(token) = self.lhs()?.syntax().last_token()?.next_token() {
            return Some(NodeOrToken::Token(token));
        }

        None
    }

    pub fn op_kind(&self) -> Option<BinaryOp> {
        if let Some(kind) = support::child_token::<BinaryOpKind>(self.syntax()) {
            #[rustfmt::skip]
            let op = match kind {
                BinaryOpKind::EqualEqual(_) => BinaryOp::CmpOp(CmpOp::Eq { negated: false }),
                BinaryOpKind::NotEqual(_) => BinaryOp::CmpOp(CmpOp::Eq { negated: true }),
                BinaryOpKind::GreaterThan(_) => BinaryOp::CmpOp(CmpOp::Ord { ordering: operators::Ordering::Greater, strict: true }),
                BinaryOpKind::GreaterThanEqual(_) => BinaryOp::CmpOp(CmpOp::Ord { ordering: operators::Ordering::Greater, strict: false }),
                BinaryOpKind::LessThan(_) => BinaryOp::CmpOp(CmpOp::Ord { ordering: operators::Ordering::Less, strict: true }),
                BinaryOpKind::LessThanEqual(_) => BinaryOp::CmpOp(CmpOp::Ord { ordering: operators::Ordering::Less, strict: false }),
                BinaryOpKind::Minus(_) => BinaryOp::ArithOp(ArithOp::Sub),
                BinaryOpKind::Plus(_) => BinaryOp::ArithOp(ArithOp::Add),
                BinaryOpKind::Star(_) => BinaryOp::ArithOp(ArithOp::Mul),
                BinaryOpKind::ForwardSlash(_) => BinaryOp::ArithOp(ArithOp::Div),
                BinaryOpKind::Modulo(_) => BinaryOp::ArithOp(ArithOp::Modulo),
                BinaryOpKind::Or(_) => BinaryOp::ArithOp(ArithOp::BitOr),
                BinaryOpKind::And(_) => BinaryOp::ArithOp(ArithOp::BitAnd),
                BinaryOpKind::OrOr(_) => BinaryOp::LogicOp(LogicOp::Or),
                BinaryOpKind::AndAnd(_) => BinaryOp::LogicOp(LogicOp::And),
            };
            Some(op)
        } else {
            self.syntax()
                .children()
                .find_map(|child| match child.kind() {
                    SyntaxKind::ShiftLeft => Some(BinaryOp::ArithOp(ArithOp::Shl)),
                    SyntaxKind::ShiftRight => Some(BinaryOp::ArithOp(ArithOp::Shr)),
                    _ => None,
                })
        }
    }
}

impl PrefixExpr {
    pub fn op_kind(&self) -> Option<UnaryOp> {
        let kind: PrefixOpKind = support::child_token(self.syntax())?;
        let op = match kind {
            PrefixOpKind::Minus(_) => UnaryOp::Minus,
            PrefixOpKind::Bang(_) => UnaryOp::Not,
            PrefixOpKind::Tilde(_) => UnaryOp::BitNot,
            PrefixOpKind::Star(_) => UnaryOp::Deref,
            PrefixOpKind::And(_) => UnaryOp::Ref,
        };
        Some(op)
    }
}

ast_enum! {
    enum NameLike {
        NameRef,
        Name,
    }
}

impl NameLike {
    pub fn as_name_ref(&self) -> Option<&NameRef> {
        match self {
            NameLike::NameRef(name_ref) => Some(name_ref),
            NameLike::Name(_) => None,
        }
    }
}
