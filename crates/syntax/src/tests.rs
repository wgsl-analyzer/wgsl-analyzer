#![expect(clippy::wildcard_enum_match_arm, reason = "brevity in test data")]

use parser::Edition;

use crate::{
    AstNode, HasAttributes as _, HasName as _,
    ast::{self, Item, LiteralKind},
    parse,
};

#[test]
fn smoke_test() {
    let ast = parse("fn foo(a: u32) -> f32 { let b = 1 + a; }", Edition::LATEST).tree();

    let ast::Item::FunctionDeclaration(function_declaration) = ast.items().next().unwrap() else {
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
    let ast = parse("fn main() { discard; }", Edition::LATEST).tree();

    let ast::Item::FunctionDeclaration(function_declaration) = ast.items().next().unwrap() else {
        panic!()
    };
    let body = function_declaration.body().unwrap();
    let ast::Statement::DiscardStatement(_) = body.statements().next().unwrap() else {
        panic!()
    };
}

#[test]
fn function_call_statement() {
    let ast = parse("fn main() { foo(1,2,3); }", Edition::LATEST).tree();

    let ast::Item::FunctionDeclaration(function_declaration) = ast.items().next().unwrap() else {
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
    let ast = parse(
        "
fn main() {
    switch foo {
        case 1,2: {},
        case default, 2, default: {}
        default: {}
    }
}
    ",
        Edition::LATEST,
    )
    .tree();

    let ast::Item::FunctionDeclaration(function_declaration) = ast.items().next().unwrap() else {
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
    let ast = parse(
        "
fn main() {
    loop { let a = 3; }
}
    ",
        Edition::LATEST,
    )
    .tree();

    let ast::Item::FunctionDeclaration(function_declaration) = ast.items().next().unwrap() else {
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
    let parsed = parse(
        "
        @diagnostic(off, bla)
        fn main() {}
        ",
        Edition::LATEST,
    );

    assert!(parsed.errors().is_empty());

    match parsed.tree().items().next().unwrap() {
        Item::FunctionDeclaration(func) => match func.attributes().next().unwrap() {
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
    let parsed = parse(
        "
        @const
        fn foo() {}
        ",
        Edition::LATEST,
    );

    assert!(parsed.errors().is_empty());

    match parsed.tree().items().next().unwrap() {
        Item::FunctionDeclaration(func) => match func.attributes().next().unwrap() {
            ast::Attribute::ConstantAttribute(constant_attribute) => {
                assert_eq!(constant_attribute.const_token().unwrap().text(), "const");
            },
            _ => panic!("wrong attribute"),
        },
        _ => panic!("expected function"),
    }
}

#[test]
fn other_attribute() {
    let parsed = parse(
        "
        @nonexistent(wacky * 2)
        fn foo() {}
        ",
        Edition::LATEST,
    );

    assert!(parsed.errors().is_empty());

    match parsed.tree().items().next().unwrap() {
        Item::FunctionDeclaration(func) => match func.attributes().next().unwrap() {
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
