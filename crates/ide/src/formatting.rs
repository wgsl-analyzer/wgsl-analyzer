use base_db::FileId;
use base_db::SourceDatabase;
use base_db::TextRange;
use rowan::{GreenNode, GreenToken, NodeOrToken, WalkEvent};
use syntax::{ast, AstNode, SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken};

use crate::RootDatabase;

pub fn format(db: &RootDatabase, file_id: FileId, range: Option<TextRange>) -> Option<SyntaxNode> {
    let file: ast::SourceFile = db.parse(file_id).tree();

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

fn format_recursive(syntax: SyntaxNode) {
    let preorder = syntax.preorder();
    for event in preorder {
        match event {
            WalkEvent::Enter(node) => drop(format_syntax_node(node)),
            WalkEvent::Leave(_) => {}
        }
    }
}

fn format_syntax_node(syntax: SyntaxNode) -> Option<()> {
    match syntax.kind() {
        // fn name ( param : type, param : type ) -> return_ty {}
        // fn name(
        //     param : type,
        //     param : type,
        // ) -> return_ty {}
        SyntaxKind::Function => {
            let function = ast::Function::cast(syntax.clone())?;

            trim_whitespace_before_to_newline(function.fn_token()?);

            set_whitespace_single_after(function.fn_token()?);
            set_whitespace_single_before(function.body()?.left_brace_token()?);

            let param_list = function.param_list()?;

            remove_if_whitespace(param_list.left_paren_token()?.prev_token()?);

            let has_newline =
                remove_whitespace_keep_newline(param_list.left_paren_token()?.next_token()?);

            let mut first = true;
            let n_params = param_list.params().count();
            for (i, param) in param_list.params().enumerate() {
                let last = i == n_params - 1;

                if !first {
                    let ws = create_whitespace(match has_newline {
                        true => "\n    ",
                        false => " ",
                    });

                    let first_token = param.syntax().first_token()?;
                    set_whitespace_before(first_token, ws);
                }

                let last_param_token = param.syntax().last_token()?;
                remove_if_whitespace(last_param_token);

                let token_after_param = match param.syntax().next_sibling_or_token()? {
                    NodeOrToken::Node(node) => node.first_token()?,
                    NodeOrToken::Token(token) => token,
                };
                match (last, token_after_param.kind() == SyntaxKind::Comma) {
                    (true, false) if has_newline => {
                        insert_after_syntax(
                            param.syntax(),
                            create_syntax_token(SyntaxKind::Comma, ","),
                        );
                    }
                    (true, false) => {}
                    (true, true) if has_newline => {}
                    (true, true) => token_after_param.detach(),
                    (false, true) => {}
                    (false, false) => {
                        insert_after_syntax(
                            param.syntax(),
                            create_syntax_token(SyntaxKind::Comma, ","),
                        );
                    }
                };

                if has_newline {
                    set_whitespace_before(param_list.right_paren_token()?, create_whitespace("\n"));
                } else {
                    remove_if_whitespace(param_list.right_paren_token()?.prev_token()?);
                }

                first = false;
            }
        }
        SyntaxKind::VariableIdentDecl => {
            let param_list = ast::VariableIdentDecl::cast(syntax.clone())?;
            remove_if_whitespace(param_list.colon_token()?.prev_token()?);
            set_whitespace_single_after(param_list.colon_token()?);
        }
        SyntaxKind::ReturnType => {
            let return_type = ast::ReturnType::cast(syntax.clone())?;
            whitespace_to_single_around(return_type.arrow_token()?);
        }
        SyntaxKind::StructDecl => {
            let strukt = ast::StructDecl::cast(syntax.clone())?;

            trim_whitespace_before_to_newline(strukt.struct_token()?);

            let name = strukt.name()?.ident_token()?;
            whitespace_to_single_around(name);
        }
        _ => {}
    }

    None
}

// "\n  fn" -> "\nfn"
fn trim_whitespace_before_to_newline(before: SyntaxToken) -> Option<()> {
    let maybe_whitespace = before.prev_token()?;
    if maybe_whitespace.kind().is_whitespace() {
        let idx = maybe_whitespace.index();

        let text = maybe_whitespace.text().trim_end_matches(" ");

        maybe_whitespace.parent().unwrap().splice_children(
            idx..idx + 1,
            vec![SyntaxElement::Token(create_whitespace(text))],
        );
    }
    Some(())
}

fn remove_whitespace_keep_newline(maybe_whitespace: SyntaxToken) -> bool {
    if maybe_whitespace.kind().is_whitespace() {
        if maybe_whitespace.text().contains('\n') {
            let idx = maybe_whitespace.index();
            maybe_whitespace.parent().unwrap().splice_children(
                idx..idx + 1,
                vec![SyntaxElement::Token(create_whitespace("\n    "))],
            );
            true
        } else {
            let idx = maybe_whitespace.index();
            maybe_whitespace
                .parent()
                .unwrap()
                .splice_children(idx..idx + 1, Vec::new());
            false
        }
    } else {
        false
    }
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
}
