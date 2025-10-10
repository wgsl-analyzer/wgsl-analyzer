use crate::{HasName, ast, parse};

#[test]
fn smoke_test() {
    let ast = parse("fn foo(a: u32) -> f32 { let b = 1 + a; }").tree();

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
            ast::operators::ArithmeticOperation::Add
        ))
    );
}

#[test]
fn discard_statement() {
    let ast = parse("fn main() { discard; }").tree();

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
    let ast = parse("fn main() { foo(); }").tree();

    let ast::Item::FunctionDeclaration(function_declaration) = ast.items().next().unwrap() else {
        panic!()
    };
    let body = function_declaration.body().unwrap();
    let ast::Statement::FunctionCallStatement(function_call) = body.statements().next().unwrap()
    else {
        panic!()
    };
    let expression: ast::FunctionCall = function_call.expression().unwrap();
    assert_eq!(
        expression
            .ident_expression()
            .unwrap()
            .name_ref()
            .unwrap()
            .text()
            .as_str(),
        "foo"
    );
}
