#[derive(logos::Logos, Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[repr(u16)]
pub enum SyntaxKind {
    SourceFile,
    /// Emergent nodes
    Name,
    /// a function
    Function,
    /// ident: type
    VariableIdentDeclaration,
    /// the <a, b, c> of a generic
    GenericArgumentList,
    /// a function parameter
    Parameter,
    /// a function parameter or name of a variable statement
    Binding,
    /// a list of function arguments
    ParameterList,
    /// a function return type
    ReturnType,
    /// a group of statements contained in braces
    CompoundStatement,
    /// a `let` or `var` statement
    VariableStatement,
    /// an expression in statement position. Only function calls are allowed there in WGSL, but we parse it nonetheless
    ExpressionStatement,
    /// `loop { statements }`
    LoopStatement,
    /// `while (expression) { statements }`
    WhileStatement,
    /// `if (expression) { statements }`
    IfStatement,
    /// `switch expression { case 1, 2: {} default: {} }`
    SwitchStatement,
    /// The block of a switch statement
    SwitchBlock,
    /// case 1, 2: {};
    SwitchBodyCase,
    /// the `1, 2` in `case 1, 2: {}`
    SwitchCaseSelectors,
    /// default: {}
    SwitchBodyDefault,

    /// `i++`, `i--`
    IncrementDecrementStatement,
    ElseIfBlock,
    ElseBlock,
    /// `for(init, cmp, update) {}`
    ForStatement,
    ForInitializer,
    ForCondition,
    ForContinuingPart,
    /// the brackets in `var<uniform>`
    VariableQualifier,
    /// a binary operator
    BinaryOperator,
    /// The parameters to a function call
    FunctionParameterList,
    /// `a.b`
    FieldExpression,
    /// `pow(2, 3)`
    FunctionCall,
    /// `(pow)(2, 3)`
    InvalidFunctionCall,
    /// `a\[0\]`
    IndexExpression,
    /// `vec3<f32>(1.0)`
    TypeInitializer,
    /// `vec3(1.0)`
    InferredInitializer,
    /// `return foo`
    ReturnStatement,
    /// an expression of the form `left_side <op> right_side`
    InfixExpression,
    /// an expression of the form `<op> right_side`
    PrefixExpression,
    /// a literal expression
    Literal,
    /// an expression resolving to a definition
    PathExpression,
    /// a reference to a definition
    NameReference,
    /// an expression inside parenthesis
    ParenthesisExpression,
    /// an expression of the form `bitcast< <type> >(expression)`
    BitcastExpression,
    /// a non-builtin type
    PathType,
    /// `a = b`
    AssignmentStatement,
    /// `a += b`
    CompoundAssignmentStatement,
    /// `[[location(0), interpolate(flat)]]`
    AttributeList,
    /// `location(0, 1, 2)`
    Attribute,
    /// `(0, 1, ident)`
    AttributeParameters,
    /// the definition of a struct
    StructDeclaration,
    /// the members of a struct definition inside of braces
    StructDeclBody,
    /// one field of a struct declaration
    StructDeclarationField,
    /// `var<uniform> test: u32`
    GlobalVariableDeclaration,
    /// `let global: u32 = 10u`
    GlobalConstantDeclaration,
    /// `override gain: f32;`
    OverrideDeclaration,
    /// `continuing { statements }`
    ContinuingStatement,
    /// Type alias declaration: `type float4 = vec4<f32>`
    TypeAliasDeclaration,

    /// `#import foo` or `#import "file.wgsl"`
    Import,
    ImportPath,
    ImportCustom,

    #[regex("[ \n\r\t]+")]
    Whitespace,
    #[regex("#ifdef.*")]
    UnofficialPreprocessorIfDef,
    #[regex("#endif.*")]
    UnofficialPreprocessorEndif,
    #[regex("#else.*")]
    UnofficialPreprocessorElse,
    #[regex("#import")]
    UnofficialPreprocessorImport,
    #[regex("#define_import_path.*")]
    UnofficialPreprocessorDefineImportPath,
    #[regex("#if.*")]
    UnofficialPreprocessIf,

    /// https://gpuweb.github.io/gpuweb/wgsl/#line-ending-comment
    #[regex("//.*")]
    LineEndingComment,

    /// https://gpuweb.github.io/gpuweb/wgsl/#block-comment
    #[regex(r"/\*", lex_block_comment)]
    BlockComment,

    #[regex(r#"([_\p{XID_Start}]\p{XID_Continue}*)|(\p{XID_Start})"#)]
    Identifier,

    // literals
    // These regexes are taken from the spec, with `-?` added to allow negative floats too
    // This is a hack to avoid implementing all the rules around floats and const evaluation
    #[regex(r"-?0[fh]")]
    #[regex(r"-?[1-9][0-9]*[fh]")]
    // We need priorities so that we avoid the fact that e.g. 1.2 would match both otherwise
    #[regex(r"-?[0-9]*\.[0-9]+([eE][+-]?[0-9]+)?[fh]?", priority = 1)]
    #[regex(r"-?[0-9]+\.[0-9]*([eE][+-]?[0-9]+)?[fh]?")]
    #[regex(r"-?[0-9]+[eE][+-]?[0-9]+[fh]?")]
    DecimalFloatLiteral,
    // Because above we need priorities here
    #[regex(
        r"-?0[xX][0-9a-fA-F]*\.[0-9a-fA-F]+([pP][+-]?[0-9]+[fh]?)?",
        priority = 1
    )]
    #[regex(r"-?0[xX][0-9a-fA-F]+\.[0-9a-fA-F]*([pP][+-]?[0-9]+[fh]?)?")]
    #[regex(r"-?0[xX][0-9a-fA-F]+[pP][+-]?[0-9]+[fh]?")]
    HexFloatLiteral,
    #[regex(r"-?0[xX][0-9a-fA-F]+[iu]?")]
    HexIntLiteral,
    // This represents potentially signed ints
    // This is a hack to avoid implementing const evaluation
    // TODO: We really should implement const evaluation
    #[regex(r"-?0i?")]
    #[regex(r"-?[1-9][0-9]*i?")]
    DecimalIntLiteral,
    // This is definitely unsigned ints
    #[regex(r"-?0u")]
    #[regex(r"-?[1-9][0-9]*u")]
    #[regex(r"0[xX][0-9a-fA-F]+u")]
    UnsignedIntLiteral,

    #[regex("\"[^\"]*\"")]
    StringLiteral,

    // type keywords
    #[token("array")]
    Array,
    #[token("atomic")]
    Atomic,
    #[token("bool")]
    Bool,
    #[token("f32")]
    Float32,
    #[token("i32")]
    Int32,
    #[token("mat2x2")]
    Mat2x2,
    #[token("mat2x3")]
    Mat2x3,
    #[token("mat2x4")]
    Mat2x4,
    #[token("mat3x2")]
    Mat3x2,
    #[token("mat3x3")]
    Mat3x3,
    #[token("mat3x4")]
    Mat3x4,
    #[token("mat4x2")]
    Mat4x2,
    #[token("mat4x3")]
    Mat4x3,
    #[token("mat4x4")]
    Mat4x4,
    #[token("ptr")]
    Pointer,
    #[token("sampler")]
    Sampler,
    #[token("sampler_comparison")]
    SamplerComparison,
    #[token("struct")]
    Struct,
    #[token("texture_1d")]
    Texture1d,
    #[token("texture_2d")]
    Texture2d,
    #[token("texture_2d_array")]
    Texture2dArray,
    #[token("texture_3d")]
    Texture3d,
    #[token("texture_cube")]
    TextureCube,
    #[token("texture_cube_array")]
    TextureCubeArray,
    #[token("texture_multisampled_2d")]
    TextureMultisampled2d,
    #[token("texture_external")]
    TextureExternal,
    #[token("texture_storage_1d")]
    TextureStorage1d,
    #[token("texture_storage_2d")]
    TextureStorage2d,
    #[token("texture_storage_2d_array")]
    TextureStorage2dArray,
    #[token("texture_storage_3d")]
    TextureStorage3d,
    #[token("texture_depth_2d")]
    TextureDepth2d,
    #[token("texture_depth_2d_array")]
    TextureDepth2dArray,
    #[token("texture_depth_cube")]
    TextureDepthCube,
    #[token("texture_depth_cube_array")]
    TextureDepthCubeArray,
    #[token("texture_depth_multisampled_2d")]
    TextureDepthMultisampled2d,
    #[token("u32")]
    Uint32,
    #[token("vec2")]
    Vec2,
    #[token("vec3")]
    Vec3,
    #[token("vec4")]
    Vec4,
    #[token("binding_array")]
    BindingArray,

    // other keywords
    #[token("alias")]
    Alias,
    #[token("bitcast")]
    Bitcast,
    // #[token("block")]
    // Block,
    #[token("break")]
    Break,
    #[token("case")]
    Case,
    #[token("continue")]
    Continue,
    #[token("continuing")]
    Continuing,
    #[token("const")]
    Constant,
    #[token("default")]
    Default,
    #[token("discard")]
    Discard,
    #[token("else")]
    Else,
    #[token("enable")]
    Enable,
    #[token("fallthrough")]
    Fallthrough,
    #[token("false")]
    False,
    #[token("fn")]
    Fn,
    #[token("for")]
    For,
    #[token("function")]
    FunctionClass,
    #[token("if")]
    If,
    #[token("let")]
    Let,
    #[token("loop")]
    Loop,
    #[token("override")]
    Override,
    #[token("private")]
    Private,
    #[token("read")]
    Read,
    #[token("read_write")]
    ReadWrite,
    #[token("return")]
    Return,
    #[token("storage")]
    Storage,
    #[token("push_constant")]
    PushConstant,
    #[token("switch")]
    Switch,
    #[token("true")]
    True,
    #[token("type")]
    Type,
    #[token("uniform")]
    Uniform,
    #[token("var")]
    Var,
    #[token("while")]
    While,
    #[token("workgroup")]
    Workgroup,
    #[token("write")]
    Write,

    // syntactic tokens
    #[token("&")]
    And,
    #[token("&&")]
    AndAnd,
    #[token("->")]
    Arrow,
    #[token("@")]
    AttributeOperator,
    #[token("/")]
    ForwardSlash,
    #[token("!")]
    Bang,
    #[token("[")]
    BracketLeft,
    #[token("]")]
    BracketRight,
    #[token("{")]
    BraceLeft,
    #[token("}")]
    BraceRight,
    #[token(":")]
    Colon,
    #[token("::")]
    ColonColon,
    #[token(",")]
    Comma,
    #[token("=")]
    Equal,
    #[token("==")]
    EqualEqual,
    #[token("!=")]
    NotEqual,
    #[token(">")]
    GreaterThan,
    #[token(">=")]
    GreaterThanEqual,
    #[token("<")]
    LessThan,
    #[token("<=")]
    LessThanEqual,
    #[token("%")]
    Modulo,
    #[token("-")]
    Minus,
    #[token("--")]
    MinusMinus,
    #[token(".")]
    Period,
    #[token("+")]
    Plus,
    #[token("++")]
    PlusPlus,
    #[token("|")]
    Or,
    #[token("||")]
    OrOr,
    #[token("(")]
    ParenthesisLeft,
    #[token(")")]
    ParenthesisRight,
    #[token(";")]
    Semicolon,
    #[token("*")]
    Star,
    #[token("~")]
    Tilde,
    #[token("^")]
    Xor,

    #[token("+=")]
    PlusEqual,
    #[token("-=")]
    MinusEqual,
    #[token("*=")]
    TimesEqual,
    #[token("/=")]
    DivisionEqual,
    #[token("%=")]
    ModuloEqual,
    #[token("&=")]
    AndEqual,
    #[token("|=")]
    OrEqual,
    #[token("^=")]
    XorEqual,
    #[token(">>=")]
    ShiftRightEqual,
    #[token("<<=")]
    ShiftLeftEqual,

    // compound tokens
    ShiftLeft,
    ShiftRight,

    #[error]
    Error,
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

impl From<rowan::SyntaxKind> for SyntaxKind {
    fn from(kind: rowan::SyntaxKind) -> Self {
        let max_element = SyntaxKind::Error as u16;
        assert!(kind.0 < max_element);

        // Safety: SyntaxKind is #[repr(u16)] and in range
        unsafe { std::mem::transmute(kind.0) }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum WgslLanguage {}

impl rowan::Language for WgslLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= SyntaxKind::Error as u16);
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}

impl SyntaxKind {
    pub fn is_whitespace(self) -> bool {
        matches!(self, SyntaxKind::Whitespace)
    }

    pub fn is_trivia(self) -> bool {
        matches!(
            self,
            SyntaxKind::Whitespace
                | SyntaxKind::LineEndingComment
                | SyntaxKind::BlockComment
                | SyntaxKind::UnofficialPreprocessorEndif
                | SyntaxKind::UnofficialPreprocessorIfDef
                | SyntaxKind::UnofficialPreprocessorElse
                | SyntaxKind::UnofficialPreprocessorDefineImportPath
                | SyntaxKind::UnofficialPreprocessIf
        )
    }
}

fn lex_block_comment(lex: &mut logos::Lexer<SyntaxKind>) -> Option<()> {
    let mut depth = 1;
    let slice = lex.remainder();
    let mut i = 0;
    let bytes = slice.as_bytes();
    while i + 1 < bytes.len() {
        if bytes[i] == b'/' && bytes[i + 1] == b'*' {
            depth += 1;
            i += 2;
        } else if bytes[i] == b'*' && bytes[i + 1] == b'/' {
            depth -= 1;
            i += 2;
            if depth == 0 {
                lex.bump(i);
                return Some(());
            }
        } else {
            i += 1;
        }
    }
    // If we reach here, the comment was unterminated; consume the rest.
    lex.bump(i);
    Some(())
}

#[cfg(test)]
mod tests {
    use expect_test::expect;
    use logos::Logos;

    use super::SyntaxKind;

    fn check_lex(
        source: &str,
        expect: expect_test::Expect,
    ) {
        let tokens: Vec<_> = SyntaxKind::lexer(source).collect();
        expect.assert_eq(&format!("{tokens:?}"));
    }

    #[test]
    fn lex_decimal_float() {
        check_lex("10.0", expect![[r#"[DecimalFloatLiteral]"#]]);
        check_lex("-10.0", expect![[r#"[DecimalFloatLiteral]"#]]);
        check_lex("1e9f", expect![[r#"[DecimalFloatLiteral]"#]]);
        check_lex("-0.0e7", expect![[r#"[DecimalFloatLiteral]"#]]);
        check_lex(".1", expect![[r#"[DecimalFloatLiteral]"#]]);
        check_lex("1.", expect![[r#"[DecimalFloatLiteral]"#]]);
    }

    #[test]
    fn lex_hex_float() {
        check_lex("0x0.0", expect![[r#"[HexFloatLiteral]"#]]);
        check_lex("0X1p9", expect![[r#"[HexFloatLiteral]"#]]);
        check_lex("-0x0.0", expect![[r#"[HexFloatLiteral]"#]]);
        check_lex("0xff.13p13", expect![[r#"[HexFloatLiteral]"#]]);
    }

    #[test]
    fn lex_comment() {
        check_lex(
            "// test asdf\nnot_comment",
            expect![[r#"[Comment, Whitespace, Identifier]"#]],
        );
    }

    #[test]
    fn lex_nested_brackets() {
        // Expect: Identifier (a), [, Identifier (a), [, DecimalIntLiteral (0), ], ]
        check_lex(
            "a[a[0]]",
            expect![[
                r#"[Identifier, BracketLeft, Identifier, BracketLeft, DecimalIntLiteral, BracketRight, BracketRight]"#
            ]],
        );
    }
}
