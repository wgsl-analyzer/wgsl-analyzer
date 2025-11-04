use std::mem;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[repr(u16)]
pub enum SyntaxKind {
    SourceFile,
    /// A name that can be referenced by a [`NameReference`]
    Name,
    /// a function
    FunctionDeclaration,
    /// the <a, b, c> of a template
    TemplateList,
    /// The parameters to a function call
    FunctionParameters,
    /// a function parameter
    Parameter,
    /// a function return type
    ReturnType,
    /// a group of statements contained in braces

    // Statements https://www.w3.org/TR/WGSL/#statements

    /// [10.1. Const Assert Statement](https://www.w3.org/TR/WGSL/#const-assert-statement)
    /// `const_assert 1 < 2;`
    AssertStatement,

    /// [9.1. Compound Statement](https://www.w3.org/TR/WGSL/#compound-statement-section)
    ///
    /// ```wgsl
    /// { }
    /// ```
    CompoundStatement,

    /// [9.2. Assignment Statement](https://www.w3.org/TR/WGSL/#assignment)
    ///
    /// ```wgsl
    /// a = b
    /// ```
    AssignmentStatement,

    /// ```wgsl
    /// _ = b
    /// ```
    PhonyAssignmentStatement,

    /// ```wgsl
    /// a += b
    /// ```
    CompoundAssignmentStatement,

    /// `break;`
    BreakStatement,
    /// `break if 4 < 5;`
    BreakIfStatement,

    /// `continue;`
    ContinueStatement,

    /// `discard;`
    DiscardStatement,

    /// A lonely `;`
    EmptyStatement,
    /// [9.5. Function Call Statement](https://www.w3.org/TR/WGSL/#function-call-statement)
    FunctionCallStatement,

    /// [9.4.3. Loop Statement](https://www.w3.org/TR/WGSL/#loop-statement)
    /// Structurally very similar to a compound statement
    /// ```wgsl
    /// loop { statements }
    /// ```
    LoopStatement,
    /// `while (expression) { statements }`
    WhileStatement,
    /// `if (expression) { statements }`
    IfStatement,
    /// `switch expression { case 1, 2: {} default: {} }`
    SwitchStatement,
    /// The body of a switch statement
    SwitchBody,
    /// `case 1, 2: {};`
    SwitchBodyCase,
    /// The `1, 2` in `case 1, 2: {}`
    SwitchCaseSelectors,
    /// `default` when it appears in a `case default`
    SwitchDefaultSelector,

    /// `i++`, `i--`
    IncrementDecrementStatement,
    IfClause,
    ElseIfClause,
    ElseClause,
    /// `for(init, cmp, update) {}`
    ForStatement,
    ForInitializer,
    ForCondition,
    ForContinuingPart,
    /// `a.b`
    FieldExpression,
    /// `pow(2, 3)`
    FunctionCall,
    /// Arguments in an attribute or in a function call
    Arguments,
    /// an identifier with an optional template `foo<bar>`
    /// can refer to a type
    IdentExpression,
    NameReference,
    /// `a\[0\]`
    IndexExpression,
    /// `return foo;`
    ReturnStatement,
    /// an expression of the form `left_side <operator> right_side`
    InfixExpression,
    /// an expression of the form `<operator> right_side`
    PrefixExpression,
    /// a literal expression
    Literal,
    /// an expression inside parenthesis
    ParenthesisExpression,
    /// a type with an optional template `foo<bar>`
    TypeSpecifier,
    /// `location(0, 1, 2)`
    Attribute,
    /// the definition of a struct
    StructDeclaration,
    /// the members of a struct definition inside of braces
    StructBody,
    /// one field of a struct declaration
    StructMember,
    /// `const global: u32 = 10u`
    ConstantDeclaration,
    /// `var<uniform> test: u32`
    VariableDeclaration,
    /// `let test: u32 = 3;`
    LetDeclaration,
    /// `override test: u32`
    OverrideDeclaration,

    /// `continuing { statements }`
    ContinuingStatement,
    /// Type alias declaration: `alias float4 = vec4<f32>`
    TypeAliasDeclaration,

    /// `enable f16`
    EnableDirective,
    EnableExtensionName,
    /// `requires unrestricted_pointer_parameters`
    RequiresDirective,
    LanguageExtensionName,

    // Tokens
    Blankspace,
    LineEndingComment,
    BlockComment,

    Identifier,
    FloatLiteral,
    IntLiteral,
    StringLiteral,
    Alias,
    Break,
    Case,
    /// <https://www.w3.org/TR/WGSL/#syntax_kw-const>
    Constant,
    ConstantAssert,
    Continue,
    Continuing,
    Default,
    Diagnostic,
    Discard,
    Else,
    Enable,
    False,
    Fn,
    For,
    If,
    Let,
    Loop,
    Override,
    Requires,
    Return,
    Struct,
    Switch,
    True,
    Var,
    While,
    // syntactic tokens
    And,
    AndAnd,
    Arrow,
    AttributeOperator,
    ForwardSlash,
    Bang,
    BracketLeft,
    BracketRight,
    BraceLeft,
    BraceRight,
    Colon,
    ColonColon,
    Comma,
    Equal,
    EqualEqual,
    NotEqual,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
    Modulo,
    Minus,
    MinusMinus,
    Period,
    Plus,
    PlusPlus,
    Or,
    OrOr,
    ParenthesisLeft,
    ParenthesisRight,
    Semicolon,
    Star,
    Tilde,
    Underscore,
    Xor,

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
    ShiftLeft,
    ShiftRight,
    TemplateStart,
    TemplateEnd,
    Error,
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind.as_u16())
    }
}

impl From<rowan::SyntaxKind> for SyntaxKind {
    fn from(kind: rowan::SyntaxKind) -> Self {
        let max_element = Self::Error.as_u16();
        assert!(kind.0 < max_element);
        Self::from_u16(kind.0)
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum WeslLanguage {}

impl rowan::Language for WeslLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= SyntaxKind::Error.as_u16());
        SyntaxKind::from_u16(raw.0)
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}

impl SyntaxKind {
    #[must_use]
    pub const fn is_whitespace(self) -> bool {
        matches!(self, Self::Blankspace)
    }

    #[must_use]
    pub const fn is_trivia(self) -> bool {
        matches!(
            self,
            Self::Blankspace | Self::LineEndingComment | Self::BlockComment
        )
    }

    #[must_use]
    #[expect(clippy::as_conversions, reason = "repr(u16)")]
    pub const fn as_u16(self) -> u16 {
        self as u16
    }

    #[must_use]
    pub const fn from_u16(value: u16) -> Self {
        // Safety: SyntaxKind is #[repr(u16)] and in range
        unsafe { mem::transmute::<u16, Self>(value) }
    }
}
