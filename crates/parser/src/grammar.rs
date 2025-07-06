mod expression;

pub(crate) use expression::expression;

use self::expression::TOKENSET_LITERAL;
use crate::{
    Parser, SyntaxKind,
    marker::{CompletedMarker, Marker},
};

pub(crate) fn file(parser: &mut Parser<'_, '_>) {
    let marker = parser.start();

    while !parser.at_end() {
        item(parser);
    }

    marker.complete(parser, SyntaxKind::SourceFile);
}

const ITEM_RECOVERY_SET: &[SyntaxKind] =
    &[SyntaxKind::Fn, SyntaxKind::Struct, SyntaxKind::Override];

fn item(parser: &mut Parser<'_, '_>) {
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
    parser: &mut Parser<'_, '_>,
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
    parser: &mut Parser<'_, '_>,
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
    parser: &mut Parser<'_, '_>,
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
    parser: &mut Parser<'_, '_>,
    marker: Marker,
    kind: SyntaxKind,
) {
    global_declaration(parser, marker, kind, SyntaxKind::GlobalConstantDeclaration);
}

fn global_declaration(
    parser: &mut Parser<'_, '_>,
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
    parser: &mut Parser<'_, '_>,
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
    parser: &mut Parser<'_, '_>,
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

fn struct_member(parser: &mut Parser<'_, '_>) {
    let marker = parser.start();
    attribute_list_opt(parser);
    variable_ident_declaration(parser);

    if parser.at(SyntaxKind::Semicolon) || parser.at(SyntaxKind::Comma) {
        parser.bump();
    }

    marker.complete(parser, SyntaxKind::StructDeclarationField);
}

fn function(
    parser: &mut Parser<'_, '_>,
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
        } else {
            type_declaration(parser);
        }
        marker.complete(parser, SyntaxKind::ReturnType);
    }

    if parser.at(SyntaxKind::BraceLeft) {
        compound_statement(parser);
    } else {
        parser.error_recovery(&[SyntaxKind::Fn]);
    }

    marker.complete(parser, SyntaxKind::Function);
}

fn name(parser: &mut Parser<'_, '_>) {
    let marker = parser.start();
    parser.expect(SyntaxKind::Identifier);
    marker.complete(parser, SyntaxKind::Name);
}

fn name_recover(
    parser: &mut Parser<'_, '_>,
    recovery_set: &[SyntaxKind],
) {
    if parser.at_set(recovery_set) {
        return;
    }
    name(parser);
}

fn list(
    parser: &mut Parser<'_, '_>,
    begin: SyntaxKind,
    end: SyntaxKind,
    separator: SyntaxKind,
    kind: SyntaxKind,
    parser_implementation: impl Fn(&mut Parser<'_, '_>),
) {
    let marker = parser.start();
    parser.expect(begin);
    while !parser.at_or_end(end) {
        let location = parser.location();
        parser_implementation(parser);
        if parser.location() == location {
            parser.error();
        }
        parser.eat(separator);
    }
    parser.expect(end);
    marker.complete(parser, kind);
}

fn list_multisep(
    parser: &mut Parser<'_, '_>,
    begin: SyntaxKind,
    end: SyntaxKind,
    separators: &[SyntaxKind],
    kind: SyntaxKind,
    parser_implementation: impl Fn(&mut Parser<'_, '_>),
) {
    let marker = parser.start();
    parser.expect(begin);
    while !parser.at_or_end(end) {
        parser.peek();
        parser_implementation(parser);

        if parser.at_set(separators) {
            parser.bump();
        }
    }
    parser.expect(end);
    marker.complete(parser, kind);
}

fn paramarker_list(parser: &mut Parser<'_, '_>) {
    list(
        parser,
        SyntaxKind::ParenthesisLeft,
        SyntaxKind::ParenthesisRight,
        SyntaxKind::Comma,
        SyntaxKind::ParameterList,
        parameter,
    );
}

pub(crate) fn inner_parameter_list(parser: &mut Parser<'_, '_>) {
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

fn parameter(parser: &mut Parser<'_, '_>) {
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

fn variable_ident_declaration(parser: &mut Parser<'_, '_>) {
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

fn binding(parser: &mut Parser<'_, '_>) {
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

pub(crate) fn type_declaration(parser: &mut Parser<'_, '_>) -> Option<CompletedMarker> {
    if parser.at_set(TYPE_SET) {
        let marker_ty = parser.start();
        let r#type = parser.bump();
        // We do not validate which types should have generics and which should not here,
        // because `expression` relies on that (specifically for vec3(1.0) etc., where the
        // type is inferred)
        if parser.at(SyntaxKind::LessThan) {
            type_decl_generics(parser);
        }
        Some(marker_ty.complete(parser, r#type))
    } else if parser.at(SyntaxKind::Identifier) {
        let marker_ty = parser.start();
        let marker_name_reference = parser.start();
        parser.bump();

        // Support for path expressions like `dim.x`
        _ = marker_name_reference.complete(parser, SyntaxKind::NameReference);
        while parser.at(SyntaxKind::Period) {
            let marker = parser.start();
            parser.bump();
            parser.expect(SyntaxKind::Identifier);
            _ = marker.complete(parser, SyntaxKind::FieldExpression);
        }

        Some(marker_ty.complete(parser, SyntaxKind::PathType))
    } else {
        // TODO remove this branch
        parser.error();
        None
    }
}

pub(crate) fn type_decl_generics(parser: &mut Parser<'_, '_>) {
    list(
        parser,
        SyntaxKind::LessThan,
        SyntaxKind::GreaterThan,
        SyntaxKind::Comma,
        SyntaxKind::GenericArgumentList,
        |parser| {
            _ = if_at_set(parser, ACCESS_MODE_SET) || if_at_set(parser, ADDRESS_SPACE_SET) || {
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

const STATEMENT_RECOVER_SET: &[SyntaxKind] = &[
    // SyntaxKind::ReturnStatement,
    SyntaxKind::Return,
    // SyntaxKind::IfStatement,
    SyntaxKind::If,
    // SyntaxKind::SwitchStatement,
    SyntaxKind::Switch,
    // SyntaxKind::LoopStatement,
    SyntaxKind::Loop,
    // SyntaxKind::ForStatement,
    SyntaxKind::For,
    // TODO: Why is this not included?
    // SyntaxKind::WhileStatement,
    // SyntaxKind::While,
    // TODO: Why is this not included?
    // SyntaxKind::FunctionCallStatement,
    // SyntaxKind::FunctionCall,
    // SyntaxKind::VariableOrValueStatement,
    SyntaxKind::Var,
    SyntaxKind::Constant,
    SyntaxKind::Let,
    // TODO: Why does SyntaxKind::BreakStatement not exist?
    SyntaxKind::Break,
    // TODO: Why does SyntaxKind::ContinueStatement not exist?
    SyntaxKind::Continue,
    SyntaxKind::Discard,
    SyntaxKind::BraceRight,
];

/// [9. Statements](https://www.w3.org/TR/WGSL/#statements)
///
/// [Grammar](https://www.w3.org/TR/WGSL/#syntax-statement)
pub(crate) fn statement(parser: &mut Parser<'_, '_>) {
    if parser.at_set(&[SyntaxKind::Constant, SyntaxKind::Let, SyntaxKind::Var]) {
        variable_or_value_statement(parser);
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
        break_statement(parser);
    } else if parser.at(SyntaxKind::Continue) {
        continue_statement(parser);
    } else if parser.at(SyntaxKind::Discard) {
        discard_statement(parser);
    } else if parser.at(SyntaxKind::Continuing) {
        continuing_statement(parser);
    } else {
        let marker = parser.start();
        expression(parser);

        // https://www.w3.org/TR/WGSL/#assignment

        // TODO: phony assignments are not clearly handled
        // https://www.w3.org/TR/WGSL/#phony-assignment-section
        if parser.at(SyntaxKind::Equal) {
            simple_assignment_statement(parser, marker);
        } else if parser.at_set(&[SyntaxKind::PlusPlus, SyntaxKind::MinusMinus]) {
            increment_decrement_statement(parser, marker);
        } else if parser.at_set(COMPOUND_ASSIGNMENT_SET) {
            compound_assignment_statement(parser, marker);
        } else {
            // only function calls are actually allowed as statements.
            marker.complete(parser, SyntaxKind::FunctionCallStatement);
        }
    }
}

/// [9.1. Compound Statement](https://www.w3.org/TR/WGSL/#compound-statement-section)
fn compound_statement(parser: &mut Parser<'_, '_>) {
    list(
        parser,
        SyntaxKind::BraceLeft,
        SyntaxKind::BraceRight,
        SyntaxKind::Semicolon,
        SyntaxKind::CompoundStatement,
        statement,
    );
}

// 9.2. Assignment Statement

/// [9.2.1. Simple Assignment](https://www.w3.org/TR/WGSL/#simple-assignment-section)
fn simple_assignment_statement(
    parser: &mut Parser<'_, '_>,
    marker: Marker,
) {
    parser.expect(SyntaxKind::Equal);
    expression(parser);
    marker.complete(parser, SyntaxKind::AssignmentStatement);
}

/// [9.2.2. Phony Assignment](https://www.w3.org/TR/WGSL/#phony-assignment-section)
fn phony_assignment_statement(
    parser: &mut Parser<'_, '_>,
    marker: Marker,
) {
    parser.bump();
    expression(parser);
    marker.complete(parser, SyntaxKind::CompoundAssignmentStatement);
}

/// [9.2.3. Compound Assignment](https://www.w3.org/TR/WGSL/#compound-assignment-sec)
fn compound_assignment_statement(
    parser: &mut Parser<'_, '_>,
    marker: Marker,
) {
    parser.bump();
    expression(parser);
    marker.complete(parser, SyntaxKind::CompoundAssignmentStatement);
}

/// [9.3. Increment and Decrement Statements](https://www.w3.org/TR/WGSL/#increment-decrement)
fn increment_decrement_statement(
    parser: &mut Parser<'_, '_>,
    marker: Marker,
) {
    parser.bump();
    marker.complete(parser, SyntaxKind::IncrementDecrementStatement);
}

// https://www.w3.org/TR/WGSL/#control-flow

/// [9.4.1. If Statement](https://www.w3.org/TR/WGSL/#if-statement)
fn if_statement(parser: &mut Parser<'_, '_>) {
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

/// [9.4.2. Switch Statement](https://www.w3.org/TR/WGSL/#switch-statement)
fn switch_statement(parser: &mut Parser<'_, '_>) {
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

/// [9.4.3. Loop Statement](https://www.w3.org/TR/WGSL/#loop-statement)
fn loop_statement(parser: &mut Parser<'_, '_>) {
    let marker = parser.start();
    parser.expect(SyntaxKind::Loop);
    compound_statement(parser);
    marker.complete(parser, SyntaxKind::LoopStatement);
}

/// [9.4.4. For Statement](https://www.w3.org/TR/WGSL/#for-statement)
fn for_statement(parser: &mut Parser<'_, '_>) {
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

/// [9.4.5. While Statement](https://www.w3.org/TR/WGSL/#while-statement)
fn while_statement(parser: &mut Parser<'_, '_>) {
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

/// [9.4.6. Break Statement](https://www.w3.org/TR/WGSL/#break-statement)
fn break_statement(parser: &mut Parser<'_, '_>) {
    parser.bump();
}

// /// [9.4.7. Break-If Statement](https://www.w3.org/TR/WGSL/#break-if-statement)

/// [9.4.8. Continue Statement](https://www.w3.org/TR/WGSL/#continue-statement)
fn continue_statement(parser: &mut Parser<'_, '_>) {
    parser.bump();
}

/// [9.4.9. Continuing Statement](https://www.w3.org/TR/WGSL/#continuing-statement)
fn continuing_statement(parser: &mut Parser<'_, '_>) {
    let marker = parser.start();
    parser.expect(SyntaxKind::Continuing);
    if !parser.at(SyntaxKind::BraceLeft) {
        marker.complete(parser, SyntaxKind::ContinuingStatement);
        return;
    }
    compound_statement(parser);
    marker.complete(parser, SyntaxKind::ContinuingStatement);
}

/// [9.4.10. Return Statement](https://www.w3.org/TR/WGSL/#return-statement)
fn return_statement(parser: &mut Parser<'_, '_>) {
    let marker = parser.start();
    parser.expect(SyntaxKind::Return);
    if !parser.at(SyntaxKind::Semicolon) {
        expression(parser);
    }
    marker.complete(parser, SyntaxKind::ReturnStatement);
}

/// [9.4.11. Discard Statement](https://www.w3.org/TR/WGSL/#discard-statement)
fn discard_statement(parser: &mut Parser<'_, '_>) {
    parser.bump();
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

const COMMA_SEMICOLON_SET: &[SyntaxKind] = &[SyntaxKind::Comma, SyntaxKind::Semicolon];

fn for_header(parser: &mut Parser<'_, '_>) {
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
    } else {
        let marker = parser.start();
        statement(parser);
        marker.complete(parser, SyntaxKind::ForContinuingPart);
    }
}

fn switch_body(parser: &mut Parser<'_, '_>) {
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
    parser: &mut Parser<'_, '_>,
    before: SyntaxKind,
    after: SyntaxKind,
    recover: &[SyntaxKind],
    inner: impl Fn(&mut Parser<'_, '_>),
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

// https://www.w3.org/TR/WGSL/#syntax-variable_or_value_statement
fn variable_or_value_statement(parser: &mut Parser<'_, '_>) {
    let marker = parser.start();

    if parser.at(SyntaxKind::Let) || parser.at(SyntaxKind::Constant) {
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
        },
        _ => {
            parser.error();
            marker.complete(parser, SyntaxKind::VariableStatement);
        },
    }
}

fn variable_qualifier(parser: &mut Parser<'_, '_>) {
    let marker = parser.start();
    parser.expect(SyntaxKind::LessThan);

    address_space(parser);
    if parser.at(SyntaxKind::Comma) {
        parser.bump();
        access_mode(parser);
    }
    parser.expect(SyntaxKind::GreaterThan);

    marker.complete(parser, SyntaxKind::VariableQualifier);
}

const ADDRESS_SPACE_SET: &[SyntaxKind] = &[
    SyntaxKind::FunctionClass,
    SyntaxKind::Private,
    SyntaxKind::Workgroup,
    SyntaxKind::Uniform,
    SyntaxKind::Storage,
    SyntaxKind::PushConstant,
];

fn if_at_set(
    parser: &mut Parser<'_, '_>,
    set: &[SyntaxKind],
) -> bool {
    if_at_set_inner(parser, set, None)
}

fn if_at_set_or(
    parser: &mut Parser<'_, '_>,
    set: &[SyntaxKind],
    or: SyntaxKind,
) -> bool {
    if_at_set_inner(parser, set, Some(or))
}

fn if_at_set_inner(
    parser: &mut Parser<'_, '_>,
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

fn address_space(parser: &mut Parser<'_, '_>) {
    if_at_set_or(parser, ADDRESS_SPACE_SET, SyntaxKind::Identifier);
}

const ACCESS_MODE_SET: &[SyntaxKind] =
    &[SyntaxKind::Read, SyntaxKind::Write, SyntaxKind::ReadWrite];
fn access_mode(parser: &mut Parser<'_, '_>) {
    if_at_set_or(parser, ACCESS_MODE_SET, SyntaxKind::Identifier);
}

pub(crate) fn attribute_list_opt(parser: &mut Parser<'_, '_>) {
    if parser.at(SyntaxKind::AttributeOperator) {
        attribute_list(parser);
    }
}

pub(crate) fn attribute_list(parser: &mut Parser<'_, '_>) {
    if parser.at(SyntaxKind::AttributeOperator) {
        attribute_list_modern(parser);
    }
}

fn attribute_list_modern(parser: &mut Parser<'_, '_>) {
    let marker = parser.start();
    while parser.at(SyntaxKind::AttributeOperator) {
        parser.bump();
        attribute(parser);
    }
    marker.complete(parser, SyntaxKind::AttributeList);
}

fn attribute(parser: &mut Parser<'_, '_>) {
    let marker = parser.start();
    if parser.at(SyntaxKind::Identifier) {
        parser.bump();
    } else {
        parser.error_no_bump(&[SyntaxKind::Identifier]);
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

fn name_ref(parser: &mut Parser<'_, '_>) {
    let marker = parser.start();
    parser.expect(SyntaxKind::Identifier);
    marker.complete(parser, SyntaxKind::NameReference);
}
