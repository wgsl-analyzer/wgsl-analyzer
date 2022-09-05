#![allow(clippy::if_same_then_else, clippy::needless_return)]
mod expr;

pub use expr::expr;

use crate::SyntaxKind;

use self::expr::TOKENSET_LITERAL;

pub type Parser<'t, 'input> = parser::Parser<'t, 'input, crate::ParserDefinition>;
pub type CompletedMarker = parser::marker::CompletedMarker<crate::ParserDefinition>;
pub type Marker = parser::marker::Marker<crate::ParserDefinition>;

pub fn file(p: &mut Parser) {
    let m = p.start();

    while !p.at_end() {
        item(p);
    }

    m.complete(p, SyntaxKind::SourceFile);
}

const ITEM_RECOVERY_SET: &[SyntaxKind] =
    &[SyntaxKind::Fn, SyntaxKind::Struct, SyntaxKind::AttrLeft];

fn item(p: &mut Parser) {
    let m = p.start();
    attribute_list_opt(p);
    if p.at(SyntaxKind::UnofficialPreprocessorImport) {
        import(p, m);
    } else if p.at(SyntaxKind::Fn) {
        function(p, m);
    } else if p.at(SyntaxKind::Struct) {
        struct_(p, m);
    } else if p.at(SyntaxKind::Var) {
        global_variable_decl(p, m);
    } else if p.at(SyntaxKind::Let) {
        global_constant_decl(p, m);
    } else if p.at(SyntaxKind::Type) {
        type_alias_decl(p, m);
    } else {
        p.error_expected(&[
            SyntaxKind::Fn,
            SyntaxKind::Struct,
            SyntaxKind::Var,
            SyntaxKind::Let,
            SyntaxKind::Type,
        ]);
        m.complete(p, SyntaxKind::Error);
    }
}

fn import(p: &mut Parser, m: Marker) {
    p.expect(SyntaxKind::UnofficialPreprocessorImport);

    if p.at(SyntaxKind::StringLiteral) {
        let m = p.start();
        p.bump();
        m.complete(p, SyntaxKind::ImportPath);
    } else if p.at(SyntaxKind::Ident) {
        let m = p.start();
        while p.at(SyntaxKind::Ident) || p.at(SyntaxKind::ColonColon) {
            p.bump();
        }
        m.complete(p, SyntaxKind::ImportCustom);
    }

    m.complete(p, SyntaxKind::Import);
}

fn global_variable_decl(p: &mut Parser, m: Marker) {
    global_decl(p, m, SyntaxKind::Var, SyntaxKind::GlobalVariableDecl);
}
fn global_constant_decl(p: &mut Parser, m: Marker) {
    global_decl(p, m, SyntaxKind::Let, SyntaxKind::GlobalConstantDecl);
}
fn global_decl(p: &mut Parser, m: Marker, var_kind: SyntaxKind, kind: SyntaxKind) {
    p.expect(var_kind);
    if p.at(SyntaxKind::LessThan) {
        variable_qualifier(p);
    }

    if p.at_set(ITEM_RECOVERY_SET) {
        p.error_no_bump(&[SyntaxKind::Binding]);
        m.complete(p, SyntaxKind::GlobalVariableDecl);
        return;
    }

    binding(p);

    if p.at(SyntaxKind::Colon) {
        p.expect(SyntaxKind::Colon);
        type_decl(p);
    }

    if p.at(SyntaxKind::Equal) {
        p.expect(SyntaxKind::Equal);
        // const expr
        expr(p);
    }

    p.expect_no_bump(SyntaxKind::Semicolon);

    m.complete(p, kind);
}

fn type_alias_decl(p: &mut Parser, m: Marker) {
    p.expect(SyntaxKind::Type);

    name(p);

    p.expect(SyntaxKind::Equal);

    type_decl(p);

    p.expect_no_bump(SyntaxKind::Semicolon);

    m.complete(p, SyntaxKind::TypeAliasDecl);
}

fn struct_(p: &mut Parser, m: Marker) {
    p.expect(SyntaxKind::Struct);

    name_recover(p, ITEM_RECOVERY_SET);

    if p.at_set(ITEM_RECOVERY_SET) {
        p.error_no_bump(&[SyntaxKind::BraceLeft]);
        m.complete(p, SyntaxKind::Struct);
        return;
    }

    list_multisep(
        p,
        SyntaxKind::BraceLeft,
        SyntaxKind::BraceRight,
        &[SyntaxKind::Semicolon, SyntaxKind::Comma],
        SyntaxKind::StructDeclBody,
        struct_member,
    );

    if p.at(SyntaxKind::Semicolon) {
        p.bump();
    }

    m.complete(p, SyntaxKind::StructDecl);
}

fn struct_member(p: &mut Parser) {
    let m = p.start();
    attribute_list_opt(p);
    variable_ident_decl(p);

    if p.at(SyntaxKind::Semicolon) || p.at(SyntaxKind::Comma) {
        p.bump();
    }

    m.complete(p, SyntaxKind::StructDeclField);
}

fn function(p: &mut Parser, m: Marker) {
    p.expect(SyntaxKind::Fn);

    if p.at(SyntaxKind::Ident) {
        name(p);
    } else {
        m.complete(p, SyntaxKind::Function);
        return;
    }

    if p.at(SyntaxKind::ParenLeft) {
        param_list(p);
    } else {
        p.error_recovery(ITEM_RECOVERY_SET);
    }

    if p.at(SyntaxKind::Arrow) {
        let m = p.start();
        p.bump();

        attribute_list_opt(p);

        if p.at(SyntaxKind::BraceLeft) {
            p.error_no_bump(&[SyntaxKind::Type]);
            m.complete(p, SyntaxKind::ReturnType);
        } else {
            type_decl(p);
            m.complete(p, SyntaxKind::ReturnType);
        }
    }

    if p.at(SyntaxKind::BraceLeft) {
        compound_statement(p);
    } else {
        p.error_recovery(&[SyntaxKind::Fn]);
    }

    m.complete(p, SyntaxKind::Function);
}

fn name(p: &mut Parser) {
    let m = p.start();
    p.expect(SyntaxKind::Ident);
    m.complete(p, SyntaxKind::Name);
}
fn name_recover(p: &mut Parser, recovery_set: &[SyntaxKind]) {
    if p.at_set(recovery_set) {
        return;
    }
    name(p);
}

fn list(
    p: &mut Parser,
    begin: SyntaxKind,
    end: SyntaxKind,
    separator: SyntaxKind,
    kind: SyntaxKind,
    f: impl Fn(&mut Parser),
) {
    let m = p.start();
    p.expect(begin);
    while !p.at_or_end(end) {
        let location = p.location();
        f(p);
        if p.location() == location {
            p.error();
        }
        p.eat(separator);
    }
    p.expect(end);
    m.complete(p, kind);
}

fn list_multisep(
    p: &mut Parser,
    begin: SyntaxKind,
    end: SyntaxKind,
    separators: &[SyntaxKind],
    kind: SyntaxKind,
    f: impl Fn(&mut Parser),
) {
    let m = p.start();
    p.expect(begin);
    while !p.at_or_end(end) {
        p.peek();
        f(p);

        if p.at_set(separators) {
            p.bump();
        }
    }
    p.expect(end);
    m.complete(p, kind);
}

fn param_list(p: &mut Parser) {
    list(
        p,
        SyntaxKind::ParenLeft,
        SyntaxKind::ParenRight,
        SyntaxKind::Comma,
        SyntaxKind::ParamList,
        param,
    );
}

pub fn inner_param_list(p: &mut Parser) {
    let m = p.start();
    while !p.at_end() {
        let location = p.location();
        param(p);
        if p.location() == location {
            p.error();
        }
        p.eat(SyntaxKind::Comma);
    }
    m.complete(p, SyntaxKind::ParamList);
}

fn param(p: &mut Parser) {
    let m = p.start();

    if p.at(SyntaxKind::UnofficialPreprocessorImport) {
        let m_import = p.start();
        import(p, m_import);
        m.complete(p, SyntaxKind::Param);
        return;
    }

    attribute_list_opt(p);
    if p.at(SyntaxKind::ParenRight) {
        p.set_expected(vec![SyntaxKind::VariableIdentDecl]);
        p.error_recovery(&[SyntaxKind::ParenRight]);
        m.complete(p, SyntaxKind::Param);
        return;
    }
    variable_ident_decl(p);
    m.complete(p, SyntaxKind::Param);
}

fn variable_ident_decl(p: &mut Parser) {
    let m_var_ident_decl = p.start();
    binding(p);

    if p.at_set(&[SyntaxKind::ParenRight, SyntaxKind::BraceRight]) {
        p.error_no_bump(&[SyntaxKind::Colon]);
        m_var_ident_decl.complete(p, SyntaxKind::VariableIdentDecl);
        return;
    }

    p.expect(SyntaxKind::Colon);

    attribute_list_opt(p);

    if p.at_set(&[SyntaxKind::ParenRight, SyntaxKind::BraceRight]) {
        p.error_no_bump(&[SyntaxKind::Type]);
        m_var_ident_decl.complete(p, SyntaxKind::VariableIdentDecl);
        return;
    }

    type_decl(p);

    m_var_ident_decl.complete(p, SyntaxKind::VariableIdentDecl);
}

fn binding(p: &mut Parser) {
    let m_binding = p.start();
    name(p);
    m_binding.complete(p, SyntaxKind::Binding);
}

const TYPE_SET: &[SyntaxKind] = &[
    SyntaxKind::Array,
    SyntaxKind::Atomic,
    SyntaxKind::Bool,
    SyntaxKind::Float32,
    SyntaxKind::Int32,
    SyntaxKind::Mat2x2,
    SyntaxKind::Mat2x3,
    SyntaxKind::Mat2x4,
    SyntaxKind::Mat3x2,
    SyntaxKind::Mat3x3,
    SyntaxKind::Mat3x4,
    SyntaxKind::Mat4x2,
    SyntaxKind::Mat4x3,
    SyntaxKind::Mat4x4,
    SyntaxKind::Pointer,
    SyntaxKind::Sampler,
    SyntaxKind::SamplerComparison,
    SyntaxKind::Texture1d,
    SyntaxKind::Texture2d,
    SyntaxKind::Texture2dArray,
    SyntaxKind::Texture3d,
    SyntaxKind::TextureCube,
    SyntaxKind::TextureCubeArray,
    SyntaxKind::TextureMultisampled2d,
    SyntaxKind::TextureExternal,
    SyntaxKind::TextureStorage1d,
    SyntaxKind::TextureStorage2d,
    SyntaxKind::TextureStorage2dArray,
    SyntaxKind::TextureStorage3d,
    SyntaxKind::TextureDepth2d,
    SyntaxKind::TextureDepth2dArray,
    SyntaxKind::TextureDepthCube,
    SyntaxKind::TextureDepthCubeArray,
    SyntaxKind::TextureDepthMultisampled2d,
    SyntaxKind::Uint32,
    SyntaxKind::Vec2,
    SyntaxKind::Vec3,
    SyntaxKind::Vec4,
    SyntaxKind::BindingArray,
];
pub fn type_decl(p: &mut Parser) -> Option<CompletedMarker> {
    if p.at_set(TYPE_SET) {
        let m_ty = p.start();
        let ty = p.bump();
        if p.at(SyntaxKind::LessThan) {
            type_decl_generics(p);
        }
        Some(m_ty.complete(p, ty))
    } else if p.at(SyntaxKind::Ident) {
        let m_ty = p.start();
        let m_name_ref = p.start();
        p.bump();
        m_name_ref.complete(p, SyntaxKind::NameRef);
        Some(m_ty.complete(p, SyntaxKind::PathType))
    } else {
        p.error();
        None
    }
}
pub(crate) fn type_decl_generics(p: &mut Parser) {
    list(
        p,
        SyntaxKind::LessThan,
        SyntaxKind::GreaterThan,
        SyntaxKind::Comma,
        SyntaxKind::GenericArgList,
        |p| {
            let _ = if_at_set(p, ACCESS_MODE_SET) || if_at_set(p, STORAGE_CLASS_SET) || {
                if p.at_set(TOKENSET_LITERAL) {
                    expr::literal(p);
                } else {
                    type_decl(p);
                }
                true
            };
        },
    );
}

fn compound_statement(p: &mut Parser) {
    list(
        p,
        SyntaxKind::BraceLeft,
        SyntaxKind::BraceRight,
        SyntaxKind::Semicolon,
        SyntaxKind::CompoundStatement,
        statement,
    );
}

const STATEMENT_RECOVER_SET: &[SyntaxKind] = &[
    SyntaxKind::Let,
    SyntaxKind::Var,
    SyntaxKind::Return,
    SyntaxKind::If,
    SyntaxKind::Switch,
    SyntaxKind::Loop,
    SyntaxKind::For,
    SyntaxKind::Break,
    SyntaxKind::Continue,
    SyntaxKind::Fallthrough,
    SyntaxKind::Discard,
    SyntaxKind::BraceRight,
];

pub fn statement(p: &mut Parser) {
    /*
    | [x] return_statement SEMICOLON
    | [x] if_statement
    | [x] switch_statement
    | [x] loop_statement
    | [x] for_statement
    | [kinda] func_call_statement SEMICOLON
    | [x] variable_statement SEMICOLON
    | [x] break_statement SEMICOLON
    | [x] continue_statement SEMICOLON
    | [x] continuing_statement SEMICOLON
    | [x] DISCARD SEMICOLON
    | [x] assignment_statement SEMICOLON
    | [x] compound_statement
     */

    if p.at_set(&[SyntaxKind::Let, SyntaxKind::Var]) {
        variable_statement(p);
    } else if p.at(SyntaxKind::Return) {
        return_statement(p);
    } else if p.at(SyntaxKind::BraceLeft) {
        compound_statement(p);
    } else if p.at(SyntaxKind::If) {
        if_statement(p);
    } else if p.at(SyntaxKind::Switch) {
        switch_statement(p);
    } else if p.at(SyntaxKind::Loop) {
        loop_statement(p);
    } else if p.at(SyntaxKind::For) {
        for_statement(p);
    } else if p.at(SyntaxKind::Break) {
        p.bump();
    } else if p.at(SyntaxKind::Continue) {
        p.bump();
    } else if p.at(SyntaxKind::Discard) {
        p.bump();
    } else if p.at(SyntaxKind::Fallthrough) {
        p.bump();
    } else if p.at(SyntaxKind::Continuing) {
        continuing_statement(p);
    } else {
        let m = p.start();
        expr(p);

        if p.at(SyntaxKind::Equal) {
            p.expect(SyntaxKind::Equal);
            expr(p);
            m.complete(p, SyntaxKind::AssignmentStmt);
        } else if p.at_set(&[SyntaxKind::PlusPlus, SyntaxKind::MinusMinus]) {
            p.bump();
            m.complete(p, SyntaxKind::IncrDecrStatement);
        } else if p.at_set(COMPOUND_ASSIGNMENT_SET) {
            p.bump();
            expr(p);
            m.complete(p, SyntaxKind::CompoundAssignmentStmt);
        } else {
            // only function calls are actually allowed as statements in wgsl.
            m.complete(p, SyntaxKind::ExprStatement);
        }
    }
}

const COMPOUND_ASSIGNMENT_SET: &[SyntaxKind] = &[
    SyntaxKind::PlusEqual,
    SyntaxKind::MinusEqual,
    SyntaxKind::TimesEqual,
    SyntaxKind::DivisionEqual,
    SyntaxKind::ModuloEqual,
    SyntaxKind::AndEqual,
    SyntaxKind::OrEqual,
    SyntaxKind::XorEqual,
    SyntaxKind::ShiftRightEqual,
    SyntaxKind::ShiftLeftEqual,
];

fn loop_statement(p: &mut Parser) {
    let m = p.start();
    p.expect(SyntaxKind::Loop);
    compound_statement(p);
    m.complete(p, SyntaxKind::LoopStatement);
}

fn continuing_statement(p: &mut Parser) {
    let m = p.start();
    p.expect(SyntaxKind::Continuing);
    if !p.at(SyntaxKind::BraceLeft) {
        m.complete(p, SyntaxKind::ContinuingStatement);
        return;
    }
    compound_statement(p);
    m.complete(p, SyntaxKind::ContinuingStatement);
}

fn for_statement(p: &mut Parser) {
    let m = p.start();
    p.expect(SyntaxKind::For);

    surround(
        p,
        SyntaxKind::ParenLeft,
        SyntaxKind::ParenRight,
        &[SyntaxKind::BraceLeft],
        for_header,
    );

    if p.at_set(STATEMENT_RECOVER_SET) {
        m.complete(p, SyntaxKind::ForStatement);
        return;
    }
    compound_statement(p);

    m.complete(p, SyntaxKind::ForStatement);
}
const COMMA_SEMICOLON_SET: &[SyntaxKind] = &[SyntaxKind::Comma, SyntaxKind::Semicolon];
fn for_header(p: &mut Parser) {
    if p.at(SyntaxKind::Semicolon) {
        p.bump();
    } else if p.at(SyntaxKind::Comma) {
        p.error();
    } else {
        let m = p.start();
        statement(p);
        m.complete(p, SyntaxKind::ForInitializer);
        p.eat_set(COMMA_SEMICOLON_SET);
    }

    if p.at(SyntaxKind::Semicolon) {
        p.bump();
    } else if p.at(SyntaxKind::Comma) {
        p.error();
    } else {
        let m = p.start();
        expr(p);
        m.complete(p, SyntaxKind::ForCondition);
        p.eat_set(COMMA_SEMICOLON_SET);
    }

    if p.at_set(&[SyntaxKind::Semicolon, SyntaxKind::Comma]) {
        p.error();
    } else if p.at(SyntaxKind::ParenRight) {
        return;
    } else {
        let m = p.start();
        statement(p);
        m.complete(p, SyntaxKind::ForContinuingPart);
    }
}

fn if_statement(p: &mut Parser) {
    let m = p.start();
    p.expect(SyntaxKind::If);

    if p.at(SyntaxKind::ParenLeft) {
        surround(
            p,
            SyntaxKind::ParenLeft,
            SyntaxKind::ParenRight,
            &[SyntaxKind::BraceLeft],
            expr,
        );
    } else {
        expr(p);
    }

    compound_statement(p);

    while p.at(SyntaxKind::Else) {
        let m_else = p.start();
        p.bump();

        if p.at(SyntaxKind::If) {
            p.bump();

            if !p.at(SyntaxKind::BraceLeft) {
                surround(
                    p,
                    SyntaxKind::ParenLeft,
                    SyntaxKind::ParenRight,
                    &[SyntaxKind::BraceLeft],
                    expr,
                );
            }

            compound_statement(p);
            m_else.complete(p, SyntaxKind::ElseIfBlock);
        } else if p.at(SyntaxKind::BraceLeft) {
            compound_statement(p);
            m_else.complete(p, SyntaxKind::ElseBlock);
        } else {
            m_else.complete(p, SyntaxKind::Error);
            p.error_recovery(&[SyntaxKind::Else]);
        }
    }

    m.complete(p, SyntaxKind::IfStatement);
}

fn switch_statement(p: &mut Parser) {
    let m = p.start();
    p.expect(SyntaxKind::Switch);

    expr(p);

    list(
        p,
        SyntaxKind::BraceLeft,
        SyntaxKind::BraceRight,
        SyntaxKind::Semicolon,
        SyntaxKind::SwitchBlock,
        switch_body,
    );

    m.complete(p, SyntaxKind::SwitchStatement);
}

fn switch_body(p: &mut Parser) {
    let m = p.start();
    if p.at(SyntaxKind::Case) {
        p.expect(SyntaxKind::Case);

        let m_selectors = p.start();
        while !p.at_or_end(SyntaxKind::Colon) || p.at_end() {
            if p.at(SyntaxKind::BraceRight) {
                break;
            }
            expr(p); // actually only const_literals are allowed here, but we parse more liberally
            p.eat(SyntaxKind::Comma);
        }
        m_selectors.complete(p, SyntaxKind::SwitchCaseSelectors);

        if p.at(SyntaxKind::BraceRight) {
            m.complete(p, SyntaxKind::SwitchBodyCase);
            return;
        }

        p.expect(SyntaxKind::Colon);

        if p.at(SyntaxKind::BraceRight) {
            m.complete(p, SyntaxKind::SwitchBodyCase);
            return;
        }

        compound_statement(p);
        m.complete(p, SyntaxKind::SwitchBodyCase);
    } else if p.at(SyntaxKind::Default) {
        p.expect(SyntaxKind::Default);
        p.expect(SyntaxKind::Colon);
        compound_statement(p);
        m.complete(p, SyntaxKind::SwitchBodyDefault);
    } else {
        p.error();
        m.complete(p, SyntaxKind::SwitchBodyCase);
    }
}

fn surround(
    p: &mut Parser,
    before: SyntaxKind,
    after: SyntaxKind,
    recover: &[SyntaxKind],
    inner: impl Fn(&mut Parser),
) {
    if p.at_set(recover) {
        p.error_expected_no_bump(&[SyntaxKind::ParenLeft]);
        return;
    }

    p.expect(before);
    if p.eat(after) {
        return;
    }

    inner(p);

    p.expect(after);
}

fn return_statement(p: &mut Parser) {
    let m = p.start();
    p.expect(SyntaxKind::Return);
    if !p.at(SyntaxKind::Semicolon) {
        expr(p);
    }
    m.complete(p, SyntaxKind::ReturnStmt);
}

fn variable_statement(p: &mut Parser) {
    let m = p.start();

    if p.at(SyntaxKind::Let) {
        p.bump();
    } else if p.at(SyntaxKind::Var) {
        p.bump();
        if p.at(SyntaxKind::LessThan) {
            variable_qualifier(p);
        }
    } else {
        p.error_recovery(STATEMENT_RECOVER_SET);
        m.complete(p, SyntaxKind::VariableStatement);
        return;
    }

    if p.at_set(STATEMENT_RECOVER_SET) {
        p.error_no_bump(&[SyntaxKind::Binding]);
        m.complete(p, SyntaxKind::VariableStatement);
        return;
    }

    binding(p);

    if p.at_set(STATEMENT_RECOVER_SET) {
        p.error_no_bump(&[SyntaxKind::Binding]);
        m.complete(p, SyntaxKind::VariableStatement);
        return;
    }

    if p.at(SyntaxKind::Colon) {
        p.expect(SyntaxKind::Colon);
        type_decl(p);
    }

    match p.peek() {
        Some(SyntaxKind::Equal) => {
            p.expect(SyntaxKind::Equal);
            expr(p);

            m.complete(p, SyntaxKind::VariableStatement);
        }
        Some(SyntaxKind::Semicolon) => {
            m.complete(p, SyntaxKind::VariableStatement);
            return;
        }
        _ => {
            p.error();
            m.complete(p, SyntaxKind::VariableStatement);
        }
    }
}

fn variable_qualifier(p: &mut Parser) {
    let m = p.start();
    p.expect(SyntaxKind::LessThan);

    storage_class(p);
    if p.at(SyntaxKind::Comma) {
        p.bump();
        access_mode(p);
    }
    p.expect(SyntaxKind::GreaterThan);

    m.complete(p, SyntaxKind::VariableQualifier);
}

const STORAGE_CLASS_SET: &[SyntaxKind] = &[
    SyntaxKind::FunctionClass,
    SyntaxKind::Private,
    SyntaxKind::Workgroup,
    SyntaxKind::Uniform,
    SyntaxKind::Storage,
    SyntaxKind::PushConstant,
];

fn if_at_set(p: &mut Parser, set: &[SyntaxKind]) -> bool {
    if_at_set_inner(p, set, None)
}
fn if_at_set_or(p: &mut Parser, set: &[SyntaxKind], or: SyntaxKind) -> bool {
    if_at_set_inner(p, set, Some(or))
}
fn if_at_set_inner(p: &mut Parser, set: &[SyntaxKind], or: Option<SyntaxKind>) -> bool {
    if p.at_set(set) || or.map_or(false, |or| p.at(or)) {
        p.bump();
        true
    } else {
        false
    }
}

fn storage_class(p: &mut Parser) {
    if_at_set_or(p, STORAGE_CLASS_SET, SyntaxKind::Ident);
}
const ACCESS_MODE_SET: &[SyntaxKind] =
    &[SyntaxKind::Read, SyntaxKind::Write, SyntaxKind::ReadWrite];
fn access_mode(p: &mut Parser) {
    if_at_set_or(p, ACCESS_MODE_SET, SyntaxKind::Ident);
}

pub fn attribute_list_opt(p: &mut Parser) {
    if p.at(SyntaxKind::Attr) || p.at(SyntaxKind::AttrLeft) {
        attribute_list(p);
    }
}
pub fn attribute_list(p: &mut Parser) {
    if p.at(SyntaxKind::Attr) {
        attribute_list_modern(p);
    } else if p.at(SyntaxKind::AttrLeft) {
        attribute_list_legacy(p);
    }
}

fn attribute_list_modern(p: &mut Parser) {
    let m = p.start();
    while p.at(SyntaxKind::Attr) {
        p.bump();
        attribute(p);
    }
    m.complete(p, SyntaxKind::AttributeList);
}

fn attribute_list_legacy(p: &mut Parser) {
    list(
        p,
        SyntaxKind::AttrLeft,
        SyntaxKind::AttrRight,
        SyntaxKind::Comma,
        SyntaxKind::AttributeList,
        attribute,
    );
}

fn attribute(p: &mut Parser) {
    let m = p.start();
    if p.at(SyntaxKind::Ident) {
        p.bump();
    } else {
        p.error_no_bump(&[SyntaxKind::Ident])
    }

    if p.at(SyntaxKind::ParenLeft) {
        list(
            p,
            SyntaxKind::ParenLeft,
            SyntaxKind::ParenRight,
            SyntaxKind::Comma,
            SyntaxKind::AttributeParameters,
            |p| {
                if p.at(SyntaxKind::Ident) {
                    p.bump();
                } else if p.at_set(TOKENSET_LITERAL) {
                    expr::literal(p);
                } else {
                    p.error_recovery(&[SyntaxKind::ParenRight]);
                }
            },
        );
    }

    m.complete(p, SyntaxKind::Attribute);
}

fn name_ref(p: &mut Parser) {
    let m = p.start();
    p.expect(SyntaxKind::Ident);
    m.complete(p, SyntaxKind::NameRef);
}
