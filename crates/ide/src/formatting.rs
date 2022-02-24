use base_db::FileId;
use base_db::SourceDatabase;
use base_db::TextRange;
use rowan::{GreenNode, GreenToken, NodeOrToken, WalkEvent};
use syntax::{ast, AstNode, SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken};

use crate::RootDatabase;

pub fn format(db: &RootDatabase, file_id: FileId, range: Option<TextRange>) -> Option<SyntaxNode> {
    let file: ast::SourceFile = db.parse_no_preprocessor(file_id).tree();

    let node = match range {
        None => file.syntax().clone_for_update(),
        Some(range) => match file.syntax().covering_element(range) {
            NodeOrToken::Node(node) => node.clone_for_update(),
            NodeOrToken::Token(_) => return None,
        },
    };

    format_recursive(node.clone());
    Some(node)
}

fn is_indent_kind(node: SyntaxNode) -> bool {
    if matches!(node.kind(), SyntaxKind::CompoundStatement) {
        return true;
    }

    let param_list_left_paren = ast::ParamList::cast(node.clone())
        .and_then(|l| l.left_paren_token())
        .or_else(|| ast::FunctionParamList::cast(node.clone()).and_then(|l| l.left_paren_token()));

    if param_list_left_paren
        .and_then(|token| token.next_token())
        .map_or(false, is_whitespace_with_newline)
    {
        return true;
    }

    false
}

fn format_recursive(syntax: SyntaxNode) {
    let preorder = syntax.preorder();

    let mut indentation: usize = 0;

    for event in preorder {
        match event {
            WalkEvent::Enter(node) => {
                if is_indent_kind(node.clone()) {
                    indentation += 1;
                }
                format_syntax_node(node, indentation);
            }
            WalkEvent::Leave(node) => {
                if is_indent_kind(node) {
                    indentation = indentation.saturating_sub(1);
                }
            }
        };
    }
}

fn format_syntax_node(syntax: SyntaxNode, indentation: usize) -> Option<()> {
    if syntax.parent().map_or(false, |parent| {
        parent.kind() == SyntaxKind::CompoundStatement
    }) {
        set_whitespace_before(
            syntax.first_token()?,
            create_whitespace(&format!("\n{}", "    ".repeat(indentation))),
        );
    }

    match syntax.kind() {
        // fn name ( param : type, param : type ) -> return_ty {}
        // fn name(
        //     param : type,
        //     param : type,
        // ) -> return_ty {}
        SyntaxKind::Function => {
            let function = ast::Function::cast(syntax)?;

            trim_whitespace_before_to_newline(function.fn_token()?);

            set_whitespace_single_after(function.fn_token()?);
            set_whitespace_single_before(function.body()?.left_brace_token()?);

            let param_list = function.param_list()?;

            remove_if_whitespace(param_list.left_paren_token()?.prev_token()?);

            let has_newline =
                is_whitespace_with_newline(param_list.left_paren_token()?.next_token()?);

            format_param_list(
                param_list.params(),
                param_list.params().count(),
                has_newline,
                1,
            );

            if has_newline {
                set_whitespace_before(param_list.right_paren_token()?, create_whitespace("\n"));
            } else {
                remove_if_whitespace(param_list.right_paren_token()?.prev_token()?);
            }
        }
        SyntaxKind::VariableIdentDecl => {
            let param_list = ast::VariableIdentDecl::cast(syntax)?;
            remove_if_whitespace(param_list.colon_token()?.prev_token()?);
            set_whitespace_single_after(param_list.colon_token()?);
        }
        SyntaxKind::ReturnType => {
            let return_type = ast::ReturnType::cast(syntax)?;
            whitespace_to_single_around(return_type.arrow_token()?);
        }
        SyntaxKind::StructDecl => {
            let strukt = ast::StructDecl::cast(syntax)?;

            trim_whitespace_before_to_newline(strukt.struct_token()?);

            let name = strukt.name()?.ident_token()?;
            whitespace_to_single_around(name);
        }
        SyntaxKind::IfStatement => {
            let if_statement = ast::IfStatement::cast(syntax)?;

            set_whitespace_single_after(if_statement.if_token()?);

            let condition = if_statement.condition()?.syntax().clone();
            remove_if_whitespace(condition.first_token()?.prev_token()?);
            remove_if_whitespace(condition.last_token()?);

            set_whitespace_single_before(if_statement.block()?.left_brace_token()?);

            if let Some(else_block) = if_statement.else_block() {
                whitespace_to_single_around(else_block.else_token()?);
            }

            for else_if_block in if_statement.else_if_blocks() {
                whitespace_to_single_around(else_if_block.else_token()?);
                whitespace_to_single_around(else_if_block.if_token()?);

                let condition = else_if_block.condition()?.syntax().clone();
                remove_if_whitespace(condition.first_token()?.prev_token()?);
                remove_if_whitespace(condition.last_token()?);

                set_whitespace_single_before(else_if_block.block()?.left_brace_token()?);
            }
        }
        SyntaxKind::ForStatement => {
            let for_statement = ast::ForStatement::cast(syntax)?;

            set_whitespace_single_after(for_statement.for_token()?);

            set_whitespace_single_before(for_statement.block()?.left_brace_token()?);

            remove_if_whitespace(
                for_statement
                    .initializer()?
                    .syntax()
                    .first_token()?
                    .prev_token()?,
            );
            set_whitespace_single_before(for_statement.condition()?.syntax().first_token()?);
            set_whitespace_single_before(for_statement.continuing_part()?.syntax().first_token()?);
            remove_if_whitespace(for_statement.continuing_part()?.syntax().last_token()?);
        }
        SyntaxKind::CompoundStatement => {
            let stmt = ast::CompoundStatement::cast(syntax)?;
            let has_newline = is_whitespace_with_newline(stmt.left_brace_token()?.next_token()?);

            if has_newline {
                set_whitespace_before(
                    stmt.right_brace_token()?,
                    create_whitespace(&format!(
                        "\n{}",
                        "    ".repeat(indentation.saturating_sub(1))
                    )),
                );
            }
        }
        SyntaxKind::FunctionCall => {
            let function_call = ast::FunctionCall::cast(syntax)?;

            if let Some(expr) = function_call.expr() {
                remove_if_whitespace(expr.syntax().last_token()?);
            }

            if let Some(type_initializer) = function_call.type_initializer() {
                remove_if_whitespace(type_initializer.syntax().first_token()?.next_token()?);
                remove_if_whitespace(type_initializer.syntax().last_token()?);
            }

            let param_list = function_call.params()?;

            let has_newline =
                is_whitespace_with_newline(param_list.left_paren_token()?.next_token()?);

            format_param_list(
                param_list.args(),
                param_list.args().count(),
                has_newline,
                indentation + 1,
            );

            if has_newline {
                set_whitespace_before(
                    param_list.right_paren_token()?,
                    create_whitespace(&format!("\n{}", "    ".repeat(indentation))),
                );
            } else {
                remove_if_whitespace(param_list.right_paren_token()?.prev_token()?);
            }
        }
        SyntaxKind::InfixExpr => {
            let expr = ast::InfixExpr::cast(syntax)?;
            whitespace_to_single_around(expr.op_token()?);
        }
        SyntaxKind::AssignmentStmt => {
            let stmt = ast::AssignmentStmt::cast(syntax)?;
            whitespace_to_single_around(stmt.equal_token()?);
        }
        SyntaxKind::CompoundAssignmentStmt => {
            let stmt = ast::CompoundAssignmentStmt::cast(syntax)?;
            whitespace_to_single_around(stmt.op_token()?);
        }
        SyntaxKind::VariableStatement => {
            let stmt = ast::VariableStatement::cast(syntax)?;
            whitespace_to_single_around(stmt.equal_token()?);
        }
        _ => {}
    }

    None
}

fn format_param_list<T: AstNode>(
    params: syntax::AstChildren<T>,
    count: usize,
    has_newline: bool,
    n_indentations: usize,
) -> Option<()> {
    let mut first = true;
    for (i, param) in params.enumerate() {
        let last = i == count - 1;

        let ws = match (first, has_newline) {
            (true, false) => create_whitespace(""),
            (_, true) => create_whitespace(&format!("\n{}", "    ".repeat(n_indentations))),
            (false, false) => create_whitespace(" "),
        };

        let first_token = param.syntax().first_token()?;
        set_whitespace_before(first_token, ws);

        let last_param_token = param.syntax().last_token()?;
        remove_if_whitespace(last_param_token);

        let token_after_param = match param.syntax().next_sibling_or_token()? {
            NodeOrToken::Node(node) => node.first_token()?,
            NodeOrToken::Token(token) => token,
        };
        match (last, token_after_param.kind() == SyntaxKind::Comma) {
            (true, false) if has_newline => {
                insert_after_syntax(param.syntax(), create_syntax_token(SyntaxKind::Comma, ","));
            }
            (true, false) => {}
            (true, true) if has_newline => {}
            (true, true) => token_after_param.detach(),
            (false, true) => {}
            (false, false) => {
                insert_after_syntax(param.syntax(), create_syntax_token(SyntaxKind::Comma, ","));
            }
        };

        first = false;
    }

    Some(())
}

// "\n  fn" -> "\nfn"
fn trim_whitespace_before_to_newline(before: SyntaxToken) -> Option<()> {
    let maybe_whitespace = before.prev_token()?;
    if maybe_whitespace.kind().is_whitespace() {
        let idx = maybe_whitespace.index();

        let text = maybe_whitespace.text().trim_end_matches(' ');

        maybe_whitespace.parent().unwrap().splice_children(
            idx..idx + 1,
            vec![SyntaxElement::Token(create_whitespace(text))],
        );
    }
    Some(())
}

fn is_whitespace_with_newline(maybe_whitespace: SyntaxToken) -> bool {
    maybe_whitespace.kind().is_whitespace() && maybe_whitespace.text().contains('\n')
}

fn remove_if_whitespace(maybe_whitespace: SyntaxToken) {
    if maybe_whitespace.kind().is_whitespace() {
        let idx = maybe_whitespace.index();
        maybe_whitespace
            .parent()
            .unwrap()
            .splice_children(idx..idx + 1, Vec::new());
    }
}

fn replace_token_with(token: SyntaxToken, replacement: SyntaxToken) {
    let idx = token.index();
    token
        .parent()
        .unwrap()
        .splice_children(idx..idx + 1, vec![SyntaxElement::Token(replacement)]);
}

fn insert_after(token: SyntaxToken, insert: SyntaxToken) {
    let idx = token.index();
    token
        .parent()
        .unwrap()
        .splice_children(idx + 1..idx + 1, vec![SyntaxElement::Token(insert)]);
}

fn insert_after_syntax(node: &SyntaxNode, insert: SyntaxToken) {
    let idx = node.index();
    node.parent()
        .unwrap()
        .splice_children(idx + 1..idx + 1, vec![SyntaxElement::Token(insert)]);
}

fn insert_before(token: SyntaxToken, insert: SyntaxToken) {
    let idx = token.index();
    token
        .parent()
        .unwrap()
        .splice_children(idx..idx, vec![SyntaxElement::Token(insert)]);
}

fn whitespace_to_single_around(around: SyntaxToken) -> Option<()> {
    set_whitespace_single_before(around.clone());
    set_whitespace_single_after(around);
    Some(())
}

fn set_whitespace_after(after: SyntaxToken, to: SyntaxToken) -> Option<()> {
    let maybe_whitespace = after.next_token()?;
    if maybe_whitespace.kind().is_whitespace() {
        replace_token_with(maybe_whitespace, to);
    } else {
        insert_after(after, to);
    }

    Some(())
}

fn set_whitespace_before(before: SyntaxToken, to: SyntaxToken) -> Option<()> {
    let maybe_whitespace = before.prev_token()?;
    if maybe_whitespace.kind().is_whitespace() {
        replace_token_with(maybe_whitespace, to);
    } else {
        insert_before(before, to);
    }

    Some(())
}

fn set_whitespace_single_after(after: SyntaxToken) -> Option<()> {
    set_whitespace_after(after, single_whitespace())
}
fn set_whitespace_single_before(before: SyntaxToken) -> Option<()> {
    set_whitespace_before(before, single_whitespace())
}

fn single_whitespace() -> SyntaxToken {
    create_whitespace(" ")
}
fn create_whitespace(text: &str) -> SyntaxToken {
    create_syntax_token(SyntaxKind::Whitespace, text)
}
fn create_syntax_token(kind: SyntaxKind, text: &str) -> SyntaxToken {
    let node = SyntaxNode::new_root(GreenNode::new(
        SyntaxKind::Error.into(),
        std::iter::once(NodeOrToken::Token(GreenToken::new(kind.into(), text))),
    ))
    .clone_for_update();
    node.first_token().unwrap()
}
#[cfg(test)]
mod tests {
    use expect_test::{expect, Expect};

    use super::format_recursive;

    fn check(before: &str, after: Expect) {
        let syntax = syntax::parse(before.trim_start())
            .syntax()
            .clone_for_update();
        format_recursive(syntax.clone());

        eprintln!("{:#?}", syntax);

        let new = syntax.to_string();
        after.assert_eq(&new);
    }

    #[test]
    fn format_empty() {
        check("", expect![[""]]);
    }

    #[test]
    fn format_fn_header() {
        check(
            "fn  main ( a :  b )  -> f32   {}",
            expect![[r#"fn main(a: b) -> f32 {}"#]],
        );
    }

    #[test]
    fn format_fn_header_2() {
        check(
            "fn  main ( a :  b,  c : d )  -> f32   {}",
            expect![[r#"fn main(a: b, c: d) -> f32 {}"#]],
        );
    }

    #[test]
    fn format_fn_header_comma_oneline() {
        check(
            "fn main(a: b , c: d ,)  -> f32   {}",
            expect![[r#"fn main(a: b, c: d) -> f32 {}"#]],
        );
    }
    #[test]
    fn format_fn_header_comma_multiline() {
        check(
            "fn main(
                a: b , c: d ,)  -> f32   {}",
            expect![[r#"
                fn main(
                    a: b,
                    c: d,
                ) -> f32 {}"#]],
        );
    }

    #[test]
    fn format_fn_header_missing_comma() {
        check(
            "fn main(a: b  c: d) {}",
            expect![[r#"fn main(a: b, c: d) {}"#]],
        );
    }

    #[test]
    fn format_fn_header_no_ws() {
        check(
            "fn main(a:b)->f32{}",
            expect![[r#"fn main(a: b) -> f32 {}"#]],
        );
    }

    #[test]
    fn format_fn_newline() {
        check(
            "fn main(
    a:b
)->f32{}",
            expect![[r#"
                fn main(
                    a: b,
                ) -> f32 {}"#]],
        );
    }

    #[test]
    fn format_fn_newline_2() {
        check(
            "fn main(
    a:b, c:d)->f32{}",
            expect![[r#"
                fn main(
                    a: b,
                    c: d,
                ) -> f32 {}"#]],
        );
    }

    #[test]
    fn format_fn_newline_3() {
        check(
            "fn main(
    a:b,
    c:d
)->f32{}",
            expect![[r#"
                fn main(
                    a: b,
                    c: d,
                ) -> f32 {}"#]],
        );
    }

    #[test]
    fn format_multiple_fns() {
        check(
            "
 fn  main( a:  b )  -> f32   {}
  fn  main( a:  b )  -> f32   {}
",
            expect![[r#"
                fn main(a: b) -> f32 {}
                fn main(a: b) -> f32 {}
            "#]],
        );
    }

    #[test]
    fn format_struct() {
        check(
            "
 struct  Test  {}
",
            expect![[r#"
                struct Test {}
            "#]],
        );
    }

    #[test]
    fn format_bevy_function() {
        check(
            "fn directional_light(light: DirectionalLight, roughness: f32, NdotV: f32, normal: vec3<f32>, view: vec3<f32>, R: vec3<f32>, F0: vec3<f32>, diffuseColor: vec3<f32>) -> vec3<f32> {}",
            expect![["fn directional_light(light: DirectionalLight, roughness: f32, NdotV: f32, normal: vec3<f32>, view: vec3<f32>, R: vec3<f32>, F0: vec3<f32>, diffuseColor: vec3<f32>) -> vec3<f32> {}"]],
        )
    }

    #[test]
    fn format_bevy_function_2() {
        check(
            "fn specular(f0: vec3<f32>, roughness: f32, h: vec3<f32>, NoV: f32, NoL: f32,
              NoH: f32, LoH: f32, specularIntensity: f32) -> vec3<f32> {",
            expect![["fn specular(f0: vec3<f32>, roughness: f32, h: vec3<f32>, NoV: f32, NoL: f32, NoH: f32, LoH: f32, specularIntensity: f32) -> vec3<f32> {"]],
        )
    }

    #[test]
    fn format_if() {
        check(
            "fn main() {
    if(x < 1){}
    if  (  x < 1   )  {}
}",
            expect![[r#"
                fn main() {
                    if (x < 1) {}
                    if (x < 1) {}
                }"#]],
        );
    }

    #[test]
    fn format_if_2() {
        check(
            "fn main() {
    if(x < 1){}
    else{
        let a = 3;
    }else     if(  x > 2 ){}
}",
            expect![[r#"
                fn main() {
                    if (x < 1) {} else {
                        let a = 3;
                    } else if (x > 2) {}
                }"#]],
        );
    }

    #[test]
    fn format_for() {
        check(
            "fn main() {
    for( var i = 0;i < 100;   i = i + 1  ){}
}",
            expect![[r#"
                fn main() {
                    for (var i = 0; i < 100; i = i + 1) {}
                }"#]],
        );
    }

    #[test]
    fn format_function_call() {
        check(
            "fn main() {
    min  (  x,y );
}",
            expect![[r#"
                fn main() {
                    min(x, y);
                }"#]],
        );
    }

    #[test]
    fn format_function_call_newline() {
        check(
            "fn main() {
    min  (  
        x,y );
}",
            expect![[r#"
                fn main() {
                    min(
                        x,
                        y,
                    );
                }"#]],
        );
    }

    #[test]
    fn format_function_call_newline_indent() {
        check(
            "fn main() {
    if (false) {
        min  (  
            x,y );
    }
}",
            expect![[r#"
                fn main() {
                    if (false) {
                        min(
                            x,
                            y,
                        );
                    }
                }"#]],
        );
    }

    #[test]
    fn format_function_call_newline_nested() {
        check(
            "fn main() {
    min(
        min(
            1,
            2,
        )
    )
}",
            expect![[r#"
                fn main() {
                    min(
                        min(
                            1,
                            2,
                        ),
                    )
                }"#]],
        );
    }

    #[test]
    fn format_function_call_2() {
        check(
            "fn main() {
    vec3  <f32>  (  x,y,z );
}",
            expect![[r#"
                fn main() {
                    vec3<f32>(x, y, z);
                }"#]],
        );
    }

    #[test]
    fn format_infix_expr() {
        check(
            "fn main() {
    x+y*z;
}",
            expect![[r#"
                fn main() {
                    x + y * z;
                }"#]],
        );
    }

    #[test]
    fn format_assignment() {
        check(
            "fn main() {
    x=0;
    y  +=  x + y;
}",
            expect![[r#"
                fn main() {
                    x = 0;
                    y += x + y;
                }"#]],
        );
    }

    #[test]
    fn format_variable() {
        check(
            "fn main() {
    var x=0;
}",
            expect![[r#"
                fn main() {
                    var x = 0;
                }"#]],
        );
    }

    #[test]
    fn format_statement_indent() {
        check(
            "fn main() {
var x=0;
}",
            expect![[r#"
                fn main() {
                    var x = 0;
                }"#]],
        );
    }

    #[test]
    fn format_statement_indent_nested() {
        check(
            "fn main() {
for() {
if() {
var x = 0;
}
}
}",
            expect![[r#"
                fn main() {
                    for () {
                        if () {
                            var x = 0;
                        }
                    }
                }"#]],
        );
    }
}
