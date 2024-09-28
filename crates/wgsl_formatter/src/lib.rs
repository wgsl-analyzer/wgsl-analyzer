#[cfg(test)]
mod tests;

use rowan::{GreenNode, GreenToken, NodeOrToken, WalkEvent};
use syntax::{ast, AstNode, HasGenerics, SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken};

pub fn format_str(input: &str, options: &FormattingOptions) -> String {
    let parse = wgsl_parser::parse_file(input);
    let node = parse.syntax().clone_for_update();
    format_recursive(node.clone(), options);
    node.to_string()
}

#[derive(Debug)]
pub struct FormattingOptions {
    pub trailing_commas: Policy,
    pub indent_symbol: String,
}

impl Default for FormattingOptions {
    fn default() -> Self {
        Self {
            trailing_commas: Policy::Ignore,
            indent_symbol: "    ".to_string(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Policy {
    Ignore,
    Remove,
    Insert,
}

pub fn format_recursive(syntax: SyntaxNode, options: &FormattingOptions) {
    let preorder = syntax.preorder();

    let mut indentation: usize = 0;

    for event in preorder {
        match event {
            WalkEvent::Enter(node) => {
                if is_indent_kind(node.clone()) {
                    indentation += 1;
                }
                format_syntax_node(node, indentation, options);
            }
            WalkEvent::Leave(node) => {
                if is_indent_kind(node) {
                    indentation = indentation.saturating_sub(1);
                }
            }
        };
    }
}

fn is_indent_kind(node: SyntaxNode) -> bool {
    if matches!(
        node.kind(),
        SyntaxKind::CompoundStatement | SyntaxKind::SwitchBlock
    ) {
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

fn format_syntax_node(
    syntax: SyntaxNode,
    indentation: usize,
    options: &FormattingOptions,
) -> Option<()> {
    if syntax.parent().map_or(false, |parent| {
        parent.kind() == SyntaxKind::CompoundStatement
    }) {
        let start = syntax.first_token()?;

        let n_newlines = n_newlines_in_whitespace(start.prev_token()?).unwrap_or(0);

        if n_newlines > 0 {
            set_whitespace_before(
                syntax.first_token()?,
                create_whitespace(&format!(
                    "{}{}",
                    "\n".repeat(n_newlines),
                    options.indent_symbol.repeat(indentation)
                )),
            );
        }
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
                options.trailing_commas,
                &options.indent_symbol,
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

            set_whitespace_single_before(if_statement.block()?.left_brace_token()?);

            if let Some(else_block) = if_statement.else_block() {
                whitespace_to_single_around(else_block.else_token()?);
            }

            for else_if_block in if_statement.else_if_blocks() {
                whitespace_to_single_around(else_if_block.else_token()?);
                whitespace_to_single_around(else_if_block.if_token()?);

                set_whitespace_single_before(else_if_block.block()?.left_brace_token()?);
            }
        }
        SyntaxKind::WhileStatement => {
            let while_statement = ast::WhileStatement::cast(syntax)?;

            set_whitespace_single_after(while_statement.while_token()?);

            set_whitespace_single_before(while_statement.block()?.left_brace_token()?);
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
                        options.indent_symbol.repeat(indentation.saturating_sub(1))
                    )),
                );
            }
        }
        SyntaxKind::TypeInitializer => {
            let type_initialiser = ast::TypeInitializer::cast(syntax)?;

            if let Some(expr) = type_initialiser.ty() {
                remove_if_whitespace(expr.syntax().last_token()?);
            }

            format_params(type_initialiser.args()?, indentation, options)?;
        }
        SyntaxKind::FunctionCall => {
            let function_call = ast::FunctionCall::cast(syntax)?;

            if let Some(name_ref) = function_call.name_ref() {
                remove_if_whitespace(name_ref.syntax().last_token()?);
            }

            let param_list = function_call.params()?;

            format_params(param_list, indentation, options)?;
        }
        SyntaxKind::InfixExpr => {
            let expr = ast::InfixExpr::cast(syntax)?;

            match expr.op()? {
                NodeOrToken::Node(node) => {
                    set_whitespace_single_before(node.first_token()?);
                    set_whitespace_single_before(node.last_token()?.next_token()?);
                }
                NodeOrToken::Token(token) => {
                    whitespace_to_single_around(token);
                }
            }
        }
        SyntaxKind::ParenExpr => {
            let paren_expr = ast::ParenExpr::cast(syntax)?;
            remove_if_whitespace(paren_expr.left_paren_token()?.next_token()?);
            remove_if_whitespace(paren_expr.right_paren_token()?.prev_token()?);

            if paren_expr.syntax().parent().map_or(false, |parent| {
                matches!(
                    parent.kind(),
                    |SyntaxKind::WhileStatement| SyntaxKind::IfStatement | SyntaxKind::ElseIfBlock
                )
            }) {
                remove_token(paren_expr.right_paren_token()?);
                remove_token(paren_expr.left_paren_token()?);
            }
        }
        SyntaxKind::BitcastExpr => {
            let bitcast_expr = ast::BitcastExpr::cast(syntax)?;
            remove_if_whitespace(bitcast_expr.bitcast_token()?.next_token()?);

            remove_if_whitespace(bitcast_expr.l_angle_token()?.next_token()?);
            remove_if_whitespace(bitcast_expr.r_angle_token()?.prev_token()?);

            remove_if_whitespace(bitcast_expr.r_angle_token()?.next_token()?);
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
            if let Some(colon) = stmt.colon() {
                remove_if_whitespace(colon.prev_token()?);
                set_whitespace_single_after(colon);
            }
            whitespace_to_single_around(stmt.equal_token()?);
        }
        _ => {
            if let Some(ty) = ast::Type::cast(syntax) {
                let generics = ty.generic_arg_list()?;
                let l_angle = generics.l_angle_token()?;
                remove_if_whitespace(l_angle.prev_token()?);
                remove_if_whitespace(l_angle.next_token()?);
                let r_angle = generics.l_angle_token()?;
                remove_if_whitespace(r_angle.prev_token()?);
            }
        }
    }

    None
}

fn format_params(
    param_list: ast::FunctionParamList,
    indentation: usize,
    options: &FormattingOptions,
) -> Option<()> {
    let has_newline = is_whitespace_with_newline(param_list.left_paren_token()?.next_token()?);
    format_param_list(
        param_list.args(),
        param_list.args().count(),
        has_newline,
        indentation + 1,
        options.trailing_commas,
        &options.indent_symbol,
    );
    if has_newline {
        set_whitespace_before(
            param_list.right_paren_token()?,
            create_whitespace(&format!("\n{}", options.indent_symbol.repeat(indentation))),
        );
    } else {
        remove_if_whitespace(param_list.right_paren_token()?.prev_token()?);
    };
    Some(())
}

fn format_param_list<T: AstNode>(
    params: syntax::AstChildren<T>,
    count: usize,
    has_newline: bool,
    n_indentations: usize,
    trailing_comma_policy: Policy,
    indent_symbol: &str,
) -> Option<()> {
    let mut first = true;
    for (i, param) in params.enumerate() {
        let last = i == count - 1;

        let first_token = param.syntax().first_token()?;
        let previous_had_newline = first_token.prev_token().map_or(false, |token| {
            token.kind().is_whitespace() && token.text().contains('\n')
        });

        let ws = match (first, previous_had_newline) {
            (true, false) => create_whitespace(""),
            (_, true) => create_whitespace(&format!("\n{}", indent_symbol.repeat(n_indentations))),
            (false, false) => create_whitespace(" "),
        };

        set_whitespace_before(first_token, ws);

        let last_param_token = param.syntax().last_token()?;
        remove_if_whitespace(last_param_token);

        let token_after_param = match param.syntax().next_sibling_or_token()? {
            NodeOrToken::Node(node) => node.first_token()?,
            NodeOrToken::Token(token) => token,
        };
        match (last, token_after_param.kind() == SyntaxKind::Comma) {
            (true, true) if !has_newline => token_after_param.detach(),
            (true, has_comma) => match (trailing_comma_policy, has_comma) {
                (Policy::Ignore, _) => {}
                (Policy::Remove, true) => token_after_param.detach(),
                (Policy::Remove, false) => {}
                (Policy::Insert, true) => {}
                (Policy::Insert, false) => {
                    insert_after_syntax(
                        param.syntax(),
                        create_syntax_token(SyntaxKind::Comma, ","),
                    );
                }
            },
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

fn n_newlines_in_whitespace(maybe_whitespace: SyntaxToken) -> Option<usize> {
    maybe_whitespace
        .kind()
        .is_whitespace()
        .then(|| maybe_whitespace.text().matches('\n').count())
}

fn remove_if_whitespace(maybe_whitespace: SyntaxToken) {
    if maybe_whitespace.kind().is_whitespace() {
        remove_token(maybe_whitespace);
    }
}

fn remove_token(token: SyntaxToken) {
    let idx = token.index();
    token
        .parent()
        .unwrap()
        .splice_children(idx..idx + 1, Vec::new())
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
