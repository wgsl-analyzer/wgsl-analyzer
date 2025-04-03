#![allow(clippy::if_same_then_else, clippy::needless_return)]
mod expression;

pub(crate) use expression::expression;

use self::expression::TOKENSET_LITERAL;
use crate::{
    Parser, SyntaxKind,
    marker::{CompletedMarker, Marker},
};

pub(crate) fn file(parser: &mut Parser) {
    let marker = parser.start();

    while !parser.at_end() {
        item(parser);
    }

    marker.complete(parser, SyntaxKind::SourceFile);
}

const ITEM_RECOVERY_SET: &[SyntaxKind] = &[
    SyntaxKind::Fn,
    SyntaxKind::Struct,
    SyntaxKind::AttributeLeft,
    SyntaxKind::Override,
];

fn item(parser: &mut Parser) {
    let marker = parser.start();
    attribute_list_opt(parser);
    if parser.at(SyntaxKind::UnofficialPreprocessorImport) {
        import(parser, marker);
    } else if parser.at(SyntaxKind::Fn) {
        function(parser, marker);
    } else if parser.at(SyntaxKind::Struct) {
        struct_(parser, marker);
    } else if parser.at(SyntaxKind::Var) {
        global_variable_declaration(parser, marker);
    } else if parser.at(SyntaxKind::Let) {
        global_constant_declaration(parser, marker, SyntaxKind::Let);
    } else if parser.at(SyntaxKind::Constant) {
        global_constant_declaration(parser, marker, SyntaxKind::Constant);
    } else if parser.at(SyntaxKind::Alias) || parser.at(SyntaxKind::Type) {
        type_alias_declaration(parser, marker);
    } else if parser.at(SyntaxKind::Override) {
        override_declaration(parser, marker);
    } else {
        parser.error_expected(&[
            SyntaxKind::Fn,
            SyntaxKind::Struct,
            SyntaxKind::Var,
            SyntaxKind::Let,
            SyntaxKind::Constant,
            SyntaxKind::Alias,
            SyntaxKind::Override,
        ]);
        marker.complete(parser, SyntaxKind::Error);
    }
}

fn import(
    parser: &mut Parser,
    marker: Marker,
) {
    parser.expect(SyntaxKind::UnofficialPreprocessorImport);

    if parser.at(SyntaxKind::StringLiteral) {
        let marker = parser.start();
        parser.bump();
        marker.complete(parser, SyntaxKind::ImportPath);
    } else if parser.at(SyntaxKind::Identifier) {
        let marker = parser.start();
        while parser.at(SyntaxKind::Identifier) || parser.at(SyntaxKind::ColonColon) {
            parser.bump();
        }
        marker.complete(parser, SyntaxKind::ImportCustom);
    }

    marker.complete(parser, SyntaxKind::Import);
}

fn override_declaration(
    parser: &mut Parser,
    marker: Marker,
) {
    global_declaration(
        parser,
        marker,
        SyntaxKind::Override,
        SyntaxKind::OverrideDeclaration,
    );
}

fn global_variable_declaration(
    parser: &mut Parser,
    marker: Marker,
) {
    global_declaration(
        parser,
        marker,
        SyntaxKind::Var,
        SyntaxKind::GlobalVariableDeclaration,
    );
}

fn global_constant_declaration(
    parser: &mut Parser,
    marker: Marker,
    kind: SyntaxKind,
) {
    global_declaration(parser, marker, kind, SyntaxKind::GlobalConstantDeclaration);
}

fn global_declaration(
    parser: &mut Parser,
    marker: Marker,
    var_kind: SyntaxKind,
    kind: SyntaxKind,
) {
    parser.expect(var_kind);
    if parser.at(SyntaxKind::LessThan) {
        variable_qualifier(parser);
    }

    if parser.at_set(ITEM_RECOVERY_SET) {
        parser.error_no_bump(&[SyntaxKind::Binding]);
        marker.complete(parser, SyntaxKind::GlobalVariableDeclaration);
        return;
    }

    binding(parser);

    if parser.at(SyntaxKind::Colon) {
        parser.expect(SyntaxKind::Colon);
        type_declaration(parser);
    }

    if parser.at(SyntaxKind::Equal) {
        parser.expect(SyntaxKind::Equal);

        if parser.at_set(ITEM_RECOVERY_SET) {
            marker.complete(parser, kind);
            return;
        }

        // const expression
        expression(parser);
    }

    parser.expect_no_bump(SyntaxKind::Semicolon);

    marker.complete(parser, kind);
}

fn type_alias_declaration(
    parser: &mut Parser,
    marker: Marker,
) {
    if parser.at(SyntaxKind::Alias) || parser.at(SyntaxKind::Type) {
        parser.bump();
    } else {
        parser.error();
    }

    name(parser);

    parser.expect(SyntaxKind::Equal);

    type_declaration(parser);

    parser.expect_no_bump(SyntaxKind::Semicolon);

    marker.complete(parser, SyntaxKind::TypeAliasDeclaration);
}

fn struct_(
    parser: &mut Parser,
    marker: Marker,
) {
    parser.expect(SyntaxKind::Struct);

    name_recover(parser, ITEM_RECOVERY_SET);

    if parser.at_set(ITEM_RECOVERY_SET) {
        parser.error_no_bump(&[SyntaxKind::BraceLeft]);
        marker.complete(parser, SyntaxKind::Struct);
        return;
    }

    list_multisep(
        parser,
        SyntaxKind::BraceLeft,
        SyntaxKind::BraceRight,
        &[SyntaxKind::Semicolon, SyntaxKind::Comma],
        SyntaxKind::StructDeclBody,
        struct_member,
    );

    if parser.at(SyntaxKind::Semicolon) {
        parser.bump();
    }

    marker.complete(parser, SyntaxKind::StructDeclaration);
}

fn struct_member(parser: &mut Parser) {
    let marker = parser.start();
    attribute_list_opt(parser);
    variable_ident_declaration(parser);

    if parser.at(SyntaxKind::Semicolon) || parser.at(SyntaxKind::Comma) {
        parser.bump();
    }

    marker.complete(parser, SyntaxKind::StructDeclarationField);
}

fn function(
    parser: &mut Parser,
    marker: Marker,
) {
    parser.expect(SyntaxKind::Fn);

    if parser.at(SyntaxKind::Identifier) {
        name(parser);
    } else {
        marker.complete(parser, SyntaxKind::Function);
        return;
    }

    if parser.at(SyntaxKind::ParenthesisLeft) {
        paramarker_list(parser);
    } else {
        parser.error_recovery(ITEM_RECOVERY_SET);
    }

    if parser.at(SyntaxKind::Arrow) {
        let marker = parser.start();
        parser.bump();

        attribute_list_opt(parser);

        if parser.at(SyntaxKind::BraceLeft) {
            parser.error_no_bump(&[SyntaxKind::Type]);
            marker.complete(parser, SyntaxKind::ReturnType);
        } else {
            type_declaration(parser);
            marker.complete(parser, SyntaxKind::ReturnType);
        }
    }

    if parser.at(SyntaxKind::BraceLeft) {
        compound_statement(parser);
    } else {
        parser.error_recovery(&[SyntaxKind::Fn]);
    }

    marker.complete(parser, SyntaxKind::Function);
}

fn name(parser: &mut Parser) {
    let marker = parser.start();
    parser.expect(SyntaxKind::Identifier);
    marker.complete(parser, SyntaxKind::Name);
}

fn name_recover(
    parser: &mut Parser,
    recovery_set: &[SyntaxKind],
) {
    if parser.at_set(recovery_set) {
        return;
    }
    name(parser);
}

fn list(
    parser: &mut Parser,
    begin: SyntaxKind,
    end: SyntaxKind,
    separator: SyntaxKind,
    kind: SyntaxKind,
    f: impl Fn(&mut Parser),
) {
    let marker = parser.start();
    parser.expect(begin);
    while !parser.at_or_end(end) {
        let location = parser.location();
        f(parser);
        if parser.location() == location {
            parser.error();
        }
        parser.eat(separator);
    }
    parser.expect(end);
    marker.complete(parser, kind);
}

fn list_multisep(
    parser: &mut Parser,
    begin: SyntaxKind,
    end: SyntaxKind,
    separators: &[SyntaxKind],
    kind: SyntaxKind,
    f: impl Fn(&mut Parser),
) {
    let marker = parser.start();
    parser.expect(begin);
    while !parser.at_or_end(end) {
        parser.peek();
        f(parser);

        if parser.at_set(separators) {
            parser.bump();
        }
    }
    parser.expect(end);
    marker.complete(parser, kind);
}

fn paramarker_list(parser: &mut Parser) {
    list(
        parser,
        SyntaxKind::ParenthesisLeft,
        SyntaxKind::ParenthesisRight,
        SyntaxKind::Comma,
        SyntaxKind::ParameterList,
        parameter,
    );
}

pub(crate) fn inner_parameter_list(parser: &mut Parser) {
    let marker = parser.start();
    while !parser.at_end() {
        let location = parser.location();
        parameter(parser);
        if parser.location() == location {
            parser.error();
        }
        parser.eat(SyntaxKind::Comma);
    }
    marker.complete(parser, SyntaxKind::ParameterList);
}

fn parameter(parser: &mut Parser) {
    let marker = parser.start();

    if parser.at(SyntaxKind::UnofficialPreprocessorImport) {
        let marker_import = parser.start();
        import(parser, marker_import);
        marker.complete(parser, SyntaxKind::Parameter);
        return;
    }

    attribute_list_opt(parser);
    if parser.at(SyntaxKind::ParenthesisRight) {
        parser.set_expected(vec![SyntaxKind::VariableIdentDeclaration]);
        parser.error_recovery(&[SyntaxKind::ParenthesisRight]);
        marker.complete(parser, SyntaxKind::Parameter);
        return;
    }
    variable_ident_declaration(parser);
    marker.complete(parser, SyntaxKind::Parameter);
}

fn variable_ident_declaration(parser: &mut Parser) {
    let marker_var_ident_declaration = parser.start();
    binding(parser);

    if parser.at_set(&[SyntaxKind::ParenthesisRight, SyntaxKind::BraceRight]) {
        parser.error_no_bump(&[SyntaxKind::Colon]);
        marker_var_ident_declaration.complete(parser, SyntaxKind::VariableIdentDeclaration);
        return;
    }

    parser.expect(SyntaxKind::Colon);

    attribute_list_opt(parser);

    if parser.at_set(&[SyntaxKind::ParenthesisRight, SyntaxKind::BraceRight]) {
        parser.error_no_bump(&[SyntaxKind::Type]);
        marker_var_ident_declaration.complete(parser, SyntaxKind::VariableIdentDeclaration);
        return;
    }

    type_declaration(parser);

    marker_var_ident_declaration.complete(parser, SyntaxKind::VariableIdentDeclaration);
}

fn binding(parser: &mut Parser) {
    let marker_binding = parser.start();
    name(parser);
    marker_binding.complete(parser, SyntaxKind::Binding);
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

pub(crate) fn type_declaration(parser: &mut Parser) -> Option<CompletedMarker> {
    if parser.at_set(TYPE_SET) {
        let marker_ty = parser.start();
        let ty = parser.bump();
        // We do not validate which types should have generics and which should not here,
        // because `expression` relies on that (specifically for vec3(1.0) etc., where the
        // type is inferred)
        if parser.at(SyntaxKind::LessThan) {
            type_decl_generics(parser);
        }
        Some(marker_ty.complete(parser, ty))
    } else if parser.at(SyntaxKind::Identifier) {
        let marker_ty = parser.start();
        let marker_name_reference = parser.start();
        parser.bump();
        marker_name_reference.complete(parser, SyntaxKind::NameReference);
        Some(marker_ty.complete(parser, SyntaxKind::PathType))
    } else {
        parser.error();
        None
    }
}

pub(crate) fn type_decl_generics(parser: &mut Parser) {
    list(
        parser,
        SyntaxKind::LessThan,
        SyntaxKind::GreaterThan,
        SyntaxKind::Comma,
        SyntaxKind::GenericArgumentList,
        |parser| {
            let _ = if_at_set(parser, ACCESS_MODE_SET) || if_at_set(parser, STORAGE_CLASS_SET) || {
                if parser.at_set(TOKENSET_LITERAL) {
                    expression::literal(parser);
                } else {
                    type_declaration(parser);
                }
                true
            };
        },
    );
}

fn compound_statement(parser: &mut Parser) {
    list(
        parser,
        SyntaxKind::BraceLeft,
        SyntaxKind::BraceRight,
        SyntaxKind::Semicolon,
        SyntaxKind::CompoundStatement,
        statement,
    );
}

const STATEMENT_RECOVER_SET: &[SyntaxKind] = &[
    SyntaxKind::Constant,
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

pub(crate) fn statement(parser: &mut Parser) {
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

    if parser.at_set(&[SyntaxKind::Constant, SyntaxKind::Let, SyntaxKind::Var]) {
        variable_statement(parser);
    } else if parser.at(SyntaxKind::Return) {
        return_statement(parser);
    } else if parser.at(SyntaxKind::BraceLeft) {
        compound_statement(parser);
    } else if parser.at(SyntaxKind::If) {
        if_statement(parser);
    } else if parser.at(SyntaxKind::Switch) {
        switch_statement(parser);
    } else if parser.at(SyntaxKind::Loop) {
        loop_statement(parser);
    } else if parser.at(SyntaxKind::While) {
        while_statement(parser);
    } else if parser.at(SyntaxKind::For) {
        for_statement(parser);
    } else if parser.at(SyntaxKind::Break) {
        parser.bump();
    } else if parser.at(SyntaxKind::Continue) {
        parser.bump();
    } else if parser.at(SyntaxKind::Discard) {
        parser.bump();
    } else if parser.at(SyntaxKind::Fallthrough) {
        parser.bump();
    } else if parser.at(SyntaxKind::Continuing) {
        continuing_statement(parser);
    } else {
        let marker = parser.start();
        expression(parser);

        if parser.at(SyntaxKind::Equal) {
            parser.expect(SyntaxKind::Equal);
            expression(parser);
            marker.complete(parser, SyntaxKind::AssignmentStatement);
        } else if parser.at_set(&[SyntaxKind::PlusPlus, SyntaxKind::MinusMinus]) {
            parser.bump();
            marker.complete(parser, SyntaxKind::IncrementDecrementStatement);
        } else if parser.at_set(COMPOUND_ASSIGNMENT_SET) {
            parser.bump();
            expression(parser);
            marker.complete(parser, SyntaxKind::CompoundAssignmentStatement);
        } else {
            // only function calls are actually allowed as statements in wgsl.
            marker.complete(parser, SyntaxKind::ExpressionStatement);
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

fn loop_statement(parser: &mut Parser) {
    let marker = parser.start();
    parser.expect(SyntaxKind::Loop);
    compound_statement(parser);
    marker.complete(parser, SyntaxKind::LoopStatement);
}

fn continuing_statement(parser: &mut Parser) {
    let marker = parser.start();
    parser.expect(SyntaxKind::Continuing);
    if !parser.at(SyntaxKind::BraceLeft) {
        marker.complete(parser, SyntaxKind::ContinuingStatement);
        return;
    }
    compound_statement(parser);
    marker.complete(parser, SyntaxKind::ContinuingStatement);
}

fn while_statement(parser: &mut Parser) {
    let marker = parser.start();
    parser.expect(SyntaxKind::While);
    if parser.at_set(&[SyntaxKind::BraceLeft]) {
        // TODO: Better error here
        parser.error_expected_no_bump(&[SyntaxKind::Bool]);
        marker.complete(parser, SyntaxKind::WhileStatement);
        return;
    }

    expression(parser);

    compound_statement(parser);
    marker.complete(parser, SyntaxKind::WhileStatement);
}

fn for_statement(parser: &mut Parser) {
    let marker = parser.start();
    parser.expect(SyntaxKind::For);

    surround(
        parser,
        SyntaxKind::ParenthesisLeft,
        SyntaxKind::ParenthesisRight,
        &[SyntaxKind::BraceLeft],
        for_header,
    );

    if parser.at_set(STATEMENT_RECOVER_SET) {
        marker.complete(parser, SyntaxKind::ForStatement);
        return;
    }
    compound_statement(parser);

    marker.complete(parser, SyntaxKind::ForStatement);
}

const COMMA_SEMICOLON_SET: &[SyntaxKind] = &[SyntaxKind::Comma, SyntaxKind::Semicolon];

fn for_header(parser: &mut Parser) {
    if parser.at(SyntaxKind::Semicolon) {
        parser.bump();
    } else if parser.at(SyntaxKind::Comma) {
        parser.error();
    } else {
        let marker = parser.start();
        statement(parser);
        marker.complete(parser, SyntaxKind::ForInitializer);
        parser.eat_set(COMMA_SEMICOLON_SET);
    }

    if parser.at(SyntaxKind::Semicolon) {
        parser.bump();
    } else if parser.at(SyntaxKind::Comma) {
        parser.error();
    } else {
        let marker = parser.start();
        expression(parser);
        marker.complete(parser, SyntaxKind::ForCondition);
        parser.eat_set(COMMA_SEMICOLON_SET);
    }

    if parser.at_set(&[SyntaxKind::Semicolon, SyntaxKind::Comma]) {
        parser.error();
    } else if parser.at(SyntaxKind::ParenthesisRight) {
        return;
    } else {
        let marker = parser.start();
        statement(parser);
        marker.complete(parser, SyntaxKind::ForContinuingPart);
    }
}

fn if_statement(parser: &mut Parser) {
    let marker = parser.start();
    parser.expect(SyntaxKind::If);

    if parser.at_set(&[SyntaxKind::BraceLeft]) {
        // TODO: Better error here
        parser.error_expected_no_bump(&[SyntaxKind::Bool]);
    } else {
        expression(parser);
    }

    compound_statement(parser);

    while parser.at(SyntaxKind::Else) {
        let marker_else = parser.start();
        parser.bump();

        if parser.at(SyntaxKind::If) {
            parser.bump();

            if parser.at_set(&[SyntaxKind::BraceLeft]) {
                // TODO: Better error here
                parser.error_expected_no_bump(&[SyntaxKind::Bool]);
            } else {
                expression(parser);
            }

            compound_statement(parser);
            marker_else.complete(parser, SyntaxKind::ElseIfBlock);
        } else if parser.at(SyntaxKind::BraceLeft) {
            compound_statement(parser);
            marker_else.complete(parser, SyntaxKind::ElseBlock);
        } else {
            marker_else.complete(parser, SyntaxKind::Error);
            parser.error_recovery(&[SyntaxKind::Else]);
        }
    }

    marker.complete(parser, SyntaxKind::IfStatement);
}

fn switch_statement(parser: &mut Parser) {
    let marker = parser.start();
    parser.expect(SyntaxKind::Switch);

    expression(parser);

    list(
        parser,
        SyntaxKind::BraceLeft,
        SyntaxKind::BraceRight,
        SyntaxKind::Semicolon,
        SyntaxKind::SwitchBlock,
        switch_body,
    );

    marker.complete(parser, SyntaxKind::SwitchStatement);
}

fn switch_body(parser: &mut Parser) {
    let marker = parser.start();
    if parser.at(SyntaxKind::Case) {
        parser.expect(SyntaxKind::Case);

        let marker_selectors = parser.start();
        while !parser.at_set(&[
            SyntaxKind::Colon,
            SyntaxKind::BraceLeft,
            SyntaxKind::BraceRight,
        ]) || parser.at_end()
        {
            if parser.at(SyntaxKind::BraceRight) {
                break;
            }
            expression(parser); // actually only const_literals are allowed here, but we parse more liberally
            parser.eat(SyntaxKind::Comma);
        }
        marker_selectors.complete(parser, SyntaxKind::SwitchCaseSelectors);

        parser.eat(SyntaxKind::Colon);

        if parser.at(SyntaxKind::BraceRight) {
            marker.complete(parser, SyntaxKind::SwitchBodyCase);
            return;
        }

        compound_statement(parser);
        marker.complete(parser, SyntaxKind::SwitchBodyCase);
    } else if parser.at(SyntaxKind::Default) {
        parser.expect(SyntaxKind::Default);
        if parser.at(SyntaxKind::Colon) {
            parser.bump();
        }
        compound_statement(parser);
        marker.complete(parser, SyntaxKind::SwitchBodyDefault);
    } else {
        parser.error();
        marker.complete(parser, SyntaxKind::SwitchBodyCase);
    }
}

fn surround(
    parser: &mut Parser,
    before: SyntaxKind,
    after: SyntaxKind,
    recover: &[SyntaxKind],
    inner: impl Fn(&mut Parser),
) {
    if parser.at_set(recover) {
        parser.error_expected_no_bump(&[SyntaxKind::ParenthesisLeft]);
        return;
    }

    parser.expect(before);
    if parser.eat(after) {
        return;
    }

    inner(parser);

    parser.expect(after);
}

fn return_statement(parser: &mut Parser) {
    let marker = parser.start();
    parser.expect(SyntaxKind::Return);
    if !parser.at(SyntaxKind::Semicolon) {
        expression(parser);
    }
    marker.complete(parser, SyntaxKind::ReturnStatement);
}

fn variable_statement(parser: &mut Parser) {
    let marker = parser.start();

    if parser.at(SyntaxKind::Let) {
        parser.bump();
    } else if parser.at(SyntaxKind::Constant) {
        parser.bump();
    } else if parser.at(SyntaxKind::Var) {
        parser.bump();
        if parser.at(SyntaxKind::LessThan) {
            variable_qualifier(parser);
        }
    } else {
        parser.error_recovery(STATEMENT_RECOVER_SET);
        marker.complete(parser, SyntaxKind::VariableStatement);
        return;
    }

    if parser.at_set(STATEMENT_RECOVER_SET) {
        parser.error_no_bump(&[SyntaxKind::Binding]);
        marker.complete(parser, SyntaxKind::VariableStatement);
        return;
    }

    binding(parser);

    if parser.at_set(STATEMENT_RECOVER_SET) {
        parser.error_no_bump(&[SyntaxKind::Binding]);
        marker.complete(parser, SyntaxKind::VariableStatement);
        return;
    }

    if parser.at(SyntaxKind::Colon) {
        parser.expect(SyntaxKind::Colon);
        type_declaration(parser);
    }

    match parser.peek() {
        Some(SyntaxKind::Equal) => {
            parser.expect(SyntaxKind::Equal);
            expression(parser);

            marker.complete(parser, SyntaxKind::VariableStatement);
        },
        Some(SyntaxKind::Semicolon) => {
            marker.complete(parser, SyntaxKind::VariableStatement);
            return;
        },
        _ => {
            parser.error();
            marker.complete(parser, SyntaxKind::VariableStatement);
        },
    }
}

fn variable_qualifier(parser: &mut Parser) {
    let marker = parser.start();
    parser.expect(SyntaxKind::LessThan);

    storage_class(parser);
    if parser.at(SyntaxKind::Comma) {
        parser.bump();
        access_mode(parser);
    }
    parser.expect(SyntaxKind::GreaterThan);

    marker.complete(parser, SyntaxKind::VariableQualifier);
}

const STORAGE_CLASS_SET: &[SyntaxKind] = &[
    SyntaxKind::FunctionClass,
    SyntaxKind::Private,
    SyntaxKind::Workgroup,
    SyntaxKind::Uniform,
    SyntaxKind::Storage,
    SyntaxKind::PushConstant,
];

fn if_at_set(
    parser: &mut Parser,
    set: &[SyntaxKind],
) -> bool {
    if_at_set_inner(parser, set, None)
}

fn if_at_set_or(
    parser: &mut Parser,
    set: &[SyntaxKind],
    or: SyntaxKind,
) -> bool {
    if_at_set_inner(parser, set, Some(or))
}

fn if_at_set_inner(
    parser: &mut Parser,
    set: &[SyntaxKind],
    or: Option<SyntaxKind>,
) -> bool {
    if parser.at_set(set) || or.is_some_and(|or| parser.at(or)) {
        parser.bump();
        true
    } else {
        false
    }
}

fn storage_class(parser: &mut Parser) {
    if_at_set_or(parser, STORAGE_CLASS_SET, SyntaxKind::Identifier);
}

const ACCESS_MODE_SET: &[SyntaxKind] =
    &[SyntaxKind::Read, SyntaxKind::Write, SyntaxKind::ReadWrite];
fn access_mode(parser: &mut Parser) {
    if_at_set_or(parser, ACCESS_MODE_SET, SyntaxKind::Identifier);
}

pub(crate) fn attribute_list_opt(parser: &mut Parser) {
    if parser.at(SyntaxKind::AttributeOperator) || parser.at(SyntaxKind::AttributeLeft) {
        attribute_list(parser);
    }
}

pub(crate) fn attribute_list(parser: &mut Parser) {
    if parser.at(SyntaxKind::AttributeOperator) {
        attribute_list_modern(parser);
    } else if parser.at(SyntaxKind::AttributeLeft) {
        attribute_list_legacy(parser);
    }
}

fn attribute_list_modern(parser: &mut Parser) {
    let marker = parser.start();
    while parser.at(SyntaxKind::AttributeOperator) {
        parser.bump();
        attribute(parser);
    }
    marker.complete(parser, SyntaxKind::AttributeList);
}

fn attribute_list_legacy(parser: &mut Parser) {
    list(
        parser,
        SyntaxKind::AttributeLeft,
        SyntaxKind::AttributeRight,
        SyntaxKind::Comma,
        SyntaxKind::AttributeList,
        attribute,
    );
}

fn attribute(parser: &mut Parser) {
    let marker = parser.start();
    if parser.at(SyntaxKind::Identifier) {
        parser.bump();
    } else {
        parser.error_no_bump(&[SyntaxKind::Identifier])
    }

    if parser.at(SyntaxKind::ParenthesisLeft) {
        list(
            parser,
            SyntaxKind::ParenthesisLeft,
            SyntaxKind::ParenthesisRight,
            SyntaxKind::Comma,
            SyntaxKind::AttributeParameters,
            |parser| {
                if parser.at(SyntaxKind::Identifier) {
                    parser.bump();
                } else if parser.at_set(TOKENSET_LITERAL) {
                    expression::literal(parser);
                } else {
                    parser.error_recovery(&[SyntaxKind::ParenthesisRight]);
                }
            },
        );
    }

    marker.complete(parser, SyntaxKind::Attribute);
}

fn name_ref(parser: &mut Parser) {
    let marker = parser.start();
    parser.expect(SyntaxKind::Identifier);
    marker.complete(parser, SyntaxKind::NameReference);
}
