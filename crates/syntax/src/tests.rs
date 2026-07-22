#![expect(clippy::wildcard_enum_match_arm, reason = "brevity in test data")]

use expect_test::{Expect, expect};
use parser::Edition;

use crate::{
    AstNode, HasAttributes as _, HasName as _, Parse,
    ast::{
        self, Directive, EnableDirective, EnableExtension, EnableExtensionName, Item, LiteralKind,
        UnknownExtension,
    },
    parse,
};

fn check_errors(
    wa_fixture: &str,
    expect: Expect,
) -> Parse {
    let parse = parse(wa_fixture, Edition::LATEST);
    let errors = parse.errors();
    let actual = errors
        .iter()
        .map(|error| error.to_string())
        .collect::<Vec<String>>()
        .join("\n");
    expect.assert_eq(&actual);
    parse
}

#[test]
fn smoke_test() {
    let parsed = check_errors(
        "
        fn foo(a: u32) -> f32 { let b = 1 + a; }
        ",
        expect![""],
    );

    let ast::Item::FunctionDeclaration(function_declaration) =
        parsed.tree().items().next().unwrap()
    else {
        panic!()
    };
    let mut function_parameters = function_declaration.parameter_list().unwrap().parameters();
    let a_parameter = function_parameters.next().unwrap();
    assert_eq!(a_parameter.name().unwrap().text().as_str(), "a");
    let body = function_declaration.body().unwrap();
    let ast::Statement::LetDeclaration(let_statement) = body.statements().next().unwrap() else {
        panic!()
    };
    assert_eq!(let_statement.name().unwrap().text().as_str(), "b");
    let ast::Expression::InfixExpression(addition) = let_statement.init().unwrap() else {
        panic!();
    };
    assert_eq!(
        addition.op_kind(),
        Some(ast::operators::BinaryOperation::Arithmetic(
            ast::operators::ArithmeticOperation::Addition
        ))
    );
}

#[test]
fn discard_statement() {
    let parsed = check_errors(
        "
        fn main() { discard; }
        ",
        expect![""],
    );

    let ast::Item::FunctionDeclaration(function_declaration) =
        parsed.tree().items().next().unwrap()
    else {
        panic!()
    };
    let body = function_declaration.body().unwrap();
    let ast::Statement::DiscardStatement(_) = body.statements().next().unwrap() else {
        panic!()
    };
}

#[test]
fn function_call_statement() {
    let parsed = check_errors(
        "
        fn main() { foo(1,2,3); }
        ",
        expect![""],
    );

    let ast::Item::FunctionDeclaration(function_declaration) =
        parsed.tree().items().next().unwrap()
    else {
        panic!()
    };
    let body = function_declaration.body().unwrap();
    let ast::Statement::FunctionCallStatement(function_call) = body.statements().next().unwrap()
    else {
        panic!()
    };
    let expression: ast::FunctionCall = function_call.expression().unwrap();
    let path = expression.ident_expression().unwrap().path().unwrap();
    assert_eq!(path.segments().count(), 1);
    assert_eq!(path.segments().next().unwrap().text(), "foo");
}

#[test]
fn switch_with_case_default() {
    let parsed = check_errors(
        "
        fn main() {
            switch foo {
                case 1,2: {}
                case default, 2, default: {}
                default: {}
            }
        }
        ",
        expect![""],
    );
    let ast::Item::FunctionDeclaration(function_declaration) =
        parsed.tree().items().next().unwrap()
    else {
        panic!()
    };
    let body = function_declaration.body().unwrap();
    let ast::Statement::SwitchStatement(switch_statement) = body.statements().next().unwrap()
    else {
        panic!()
    };
    let cases = switch_statement
        .block()
        .unwrap()
        .cases()
        .collect::<Vec<_>>();
    assert_eq!(cases[0].selectors().unwrap().exprs().count(), 2);
    assert_eq!(cases[1].selectors().unwrap().exprs().count(), 3);
    assert!(matches!(
        cases[1].selectors().unwrap().exprs().next(),
        Some(ast::SwitchCaseSelector::SwitchDefaultSelector(_))
    ));
    assert!(cases[2].selectors().is_none());
    assert!(matches!(
        cases[2].case_token().unwrap(),
        ast::CaseToken::Default(_)
    ));
}

#[test]
fn loop_with_block() {
    let parsed = check_errors(
        "
        fn main() {
            loop { let a = 3; }
        }
        ",
        expect![""],
    );
    let ast::Item::FunctionDeclaration(function_declaration) =
        parsed.tree().items().next().unwrap()
    else {
        panic!()
    };
    let body = function_declaration.body().unwrap();
    let ast::Statement::LoopStatement(loop_statement) = body.statements().next().unwrap() else {
        panic!()
    };
    assert!(loop_statement.block().is_some());
}

#[test]
fn diagnostic_attribute() {
    let parsed = check_errors(
        "
        @diagnostic(off, bla)
        fn main() {}
        ",
        expect![""],
    );
    match parsed.tree().items().next().unwrap() {
        Item::FunctionDeclaration(func) => match func.attributes().unwrap().next().unwrap() {
            ast::Attribute::DiagnosticAttribute(diagnostic_attribute) => {
                assert_eq!(
                    diagnostic_attribute
                        .parameters()
                        .unwrap()
                        .severity_control_name()
                        .unwrap()
                        .ident_token()
                        .unwrap()
                        .text(),
                    "off"
                );
                assert_eq!(
                    diagnostic_attribute
                        .parameters()
                        .unwrap()
                        .diagnostic_rule_name()
                        .unwrap()
                        .ident_token()
                        .unwrap()
                        .text(),
                    "bla"
                );
            },
            _ => panic!("wrong attribute"),
        },
        _ => panic!("expected function"),
    }
}

#[test]
fn const_attribute() {
    let parsed = check_errors(
        "
        @const
        fn foo() {}
        ",
        expect![""],
    );
    match parsed.tree().items().next().unwrap() {
        Item::FunctionDeclaration(func) => match func.attributes().unwrap().next().unwrap() {
            ast::Attribute::ConstantAttribute(constant_attribute) => {
                assert_eq!(constant_attribute.name().unwrap().text(), "const");
            },
            _ => panic!("wrong attribute"),
        },
        _ => panic!("expected function"),
    }
}

#[test]
fn other_attribute() {
    let parsed = check_errors(
        "
        @nonexistent(wacky * 2)
        fn foo() {}
        ",
        expect![""],
    );
    match parsed.tree().items().next().unwrap() {
        Item::FunctionDeclaration(func) => match func.attributes().unwrap().next().unwrap() {
            ast::Attribute::OtherAttribute(other_attribute) => {
                assert_eq!(other_attribute.name().unwrap().text(), "nonexistent");
                match other_attribute
                    .parameters()
                    .unwrap()
                    .arguments()
                    .next()
                    .unwrap()
                {
                    ast::Expression::InfixExpression(infix_expression) => {
                        match infix_expression.left_side().unwrap() {
                            ast::Expression::IdentExpression(ident_expression) => {
                                assert_eq!(
                                    ident_expression
                                        .path()
                                        .unwrap()
                                        .segments()
                                        .next()
                                        .unwrap()
                                        .text(),
                                    "wacky"
                                );
                            },
                            _ => panic!("wrong expression"),
                        }
                        match infix_expression.right_side().unwrap() {
                            ast::Expression::Literal(literal) => match literal.kind() {
                                LiteralKind::IntLiteral(syntax_token) => {
                                    assert_eq!(syntax_token.text(), "2");
                                },
                                _ => panic!("wrong literal"),
                            },
                            _ => panic!("wrong expression"),
                        }
                        assert_eq!(infix_expression.op_kind().unwrap().symbol(), "*");
                        assert_eq!(infix_expression.operator().unwrap().text(), "*");
                    },
                    _ => panic!("wrong argument"),
                }
            },
            _ => panic!("wrong attribute"),
        },
        _ => panic!("expected function"),
    }
}

#[test]
fn struct_translate_attribute() {
    let parsed = check_errors(
        "
        @if(true)
        struct Foo { bar: bool }
        ",
        expect![""],
    );
    match parsed.tree().items().next().unwrap() {
        Item::StructDeclaration(r#struct) => match r#struct.attributes().unwrap().next().unwrap() {
            ast::Attribute::IfAttribute(if_attribute) => {
                assert_eq!(if_attribute.name().unwrap().text(), "if");
            },
            _ => panic!("wrong attribute"),
        },
        _ => panic!("expected function"),
    }
}

#[test]
fn assert_translate_attribute() {
    let parsed = check_errors(
        "
        @if(true)
        const_assert 2 > 1;
        ",
        expect![""],
    );
    match parsed.tree().items().next().unwrap() {
        Item::AssertStatement(assert) => match assert.attributes().unwrap().next().unwrap() {
            ast::Attribute::IfAttribute(if_attribute) => {
                assert_eq!(if_attribute.name().unwrap().text(), "if");
            },
            _ => panic!("wrong attribute"),
        },
        _ => panic!("expected function"),
    }
}

#[test]
fn diagnostic_translate_attribute() {
    let parsed = check_errors(
        "
        @if(true)
        diagnostic(off, bla);
        ",
        expect![""],
    );
    match parsed.tree().directives().next().unwrap() {
        Directive::DiagnosticDirective(diagnostic) => {
            match diagnostic.attributes().unwrap().next().unwrap() {
                ast::Attribute::IfAttribute(if_attribute) => {
                    assert_eq!(if_attribute.name().unwrap().text(), "if");
                },
                _ => panic!("wrong attribute"),
            }
        },
        _ => panic!("expected function"),
    }
}

#[test]
fn enable_translate_attribute() {
    let parsed = check_errors(
        "
        @if(true)
        enable f16;
        ",
        expect![""],
    );
    match parsed.tree().directives().next().unwrap() {
        Directive::EnableDirective(enable) => match enable.attributes().unwrap().next().unwrap() {
            ast::Attribute::IfAttribute(if_attribute) => {
                assert_eq!(if_attribute.name().unwrap().text(), "if");
            },
            _ => panic!("wrong attribute"),
        },
        _ => panic!("expected function"),
    }
}

#[test]
fn requires_translate_attribute() {
    let parsed = check_errors(
        "
        @if(true)
        requires packed_4x8_integer_dot_product;
        ",
        expect![""],
    );
    match parsed.tree().directives().next().unwrap() {
        Directive::RequiresDirective(enable) => {
            match enable.attributes().unwrap().next().unwrap() {
                ast::Attribute::IfAttribute(if_attribute) => {
                    assert_eq!(if_attribute.name().unwrap().text(), "if");
                },
                _ => panic!("wrong attribute"),
            }
        },
        _ => panic!("expected function"),
    }
}

#[test]
fn type_translate_attribute() {
    let parsed = check_errors(
        "
        @if(true)
        alias Foo = bool;
        ",
        expect![""],
    );
    match parsed.tree().items().next().unwrap() {
        Item::TypeAliasDeclaration(enable) => match enable.attributes().unwrap().next().unwrap() {
            ast::Attribute::IfAttribute(if_attribute) => {
                assert_eq!(if_attribute.name().unwrap().text(), "if");
            },
            _ => panic!("wrong attribute"),
        },
        _ => panic!("expected function"),
    }
}

#[test]
fn no_attributes() {
    let parsed = check_errors(
        "
        struct Foo { bar: bool }
        fn foo() {}
        ",
        expect![""],
    );
    let Some(Item::FunctionDeclaration(func)) = parsed.tree().items().nth(1) else {
        panic!("expected function");
    };
    assert!(func.attributes().is_none());
}

#[test]
fn enable_extension_names() {
    let parsed = check_errors(
        "
        enable f16, clip_distances, dual_source_blending, subgroups, primitive_index, unknown_nonsense;
        ",
        expect!["error at 87..103: unknown extension unknown_nonsense"],
    );
    let items = vec![
        Ok(EnableExtension::F16),
        Ok(EnableExtension::ClipDistances),
        Ok(EnableExtension::DualSourceBlending),
        Ok(EnableExtension::Subgroups),
        Ok(EnableExtension::PrimitiveIndex),
        Err(UnknownExtension),
    ];
    let map = parsed
        .tree()
        .directives()
        .flat_map(|directive| {
            match directive {
                ast::Directive::EnableDirective(enable_directive) => {
                    enable_directive.enable_extensions()
                },
                _ => panic!("wrong directive kind"),
            }
            .map(|x| x.extension())
        })
        .collect::<Vec<_>>();
    assert_eq!(map, items);
}
