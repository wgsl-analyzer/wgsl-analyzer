#[cfg(test)]
mod tests;

use rowan::{GreenNode, GreenToken, NodeOrToken, WalkEvent};
use syntax::{
    AstNode, HasName as _, HasTemplateParameters as _, SyntaxElement, SyntaxKind, SyntaxNode,
    SyntaxToken, ast,
};

#[must_use]
pub fn format_str(
    input: &str,
    options: &FormattingOptions,
) -> String {
    let parse = parser::parse_file(input);
    let node = parse.syntax().clone_for_update();
    format_recursive(&node, options);
    node.to_string()
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FormattingOptions {
    #[cfg_attr(feature = "serde", serde(alias = "trailingCommas"))]
    pub trailing_commas: Policy,
    #[cfg_attr(feature = "serde", serde(alias = "indentSymbol"))]
    pub indent_symbol: String,
}

impl Default for FormattingOptions {
    fn default() -> Self {
        Self {
            trailing_commas: Policy::Ignore,
            indent_symbol: "    ".to_owned(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum Policy {
    Ignore,
    Remove,
    Insert,
}

impl std::str::FromStr for Policy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ignore" => Ok(Self::Ignore),
            "insert" => Ok(Self::Insert),
            "remove" => Ok(Self::Remove),
            _ => Err(format!("invalid policy: {s}")),
        }
    }
}

pub fn format_recursive(
    syntax: &SyntaxNode,
    options: &FormattingOptions,
) {
    let preorder = syntax.preorder();

    let mut indentation: usize = 0;

    for event in preorder {
        match event {
            WalkEvent::Enter(node) => {
                if is_indent_kind(&node) {
                    indentation += 1;
                }
                format_syntax_node(node, indentation, options);
            },
            WalkEvent::Leave(node) => {
                if is_indent_kind(&node) {
                    indentation = indentation.saturating_sub(1);
                }
            },
        }
    }
}

fn is_indent_kind(node: &SyntaxNode) -> bool {
    // NOTE: LoopStatement is intentionally excluded here. Its body is a
    // CompoundStatement which already increments indentation; including
    // LoopStatement would double-indent the loop contents.
    if matches!(
        node.kind(),
        SyntaxKind::CompoundStatement | SyntaxKind::SwitchBody
    ) {
        return true;
    }

    let param_list_left_paren = ast::FunctionParameters::cast(node.clone())
        .and_then(|list| list.left_parenthesis_token())
        .or_else(|| {
            let list = ast::Arguments::cast(node.clone())?;
            list.left_parenthesis_token()
        });

    if param_list_left_paren
        .and_then(|token| token.next_token())
        .as_ref()
        .is_some_and(is_whitespace_with_newline)
    {
        return true;
    }

    false
}

#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "impractical to list all cases"
)]
#[expect(clippy::cognitive_complexity, clippy::too_many_lines, reason = "TODO")]
fn format_syntax_node(
    syntax: SyntaxNode,
    indentation: usize,
    options: &FormattingOptions,
) -> Option<()> {
    if syntax.parent().is_some_and(|parent| {
        matches!(
            parent.kind(),
            SyntaxKind::CompoundStatement | SyntaxKind::SwitchBody
        )
    }) {
        let start = syntax.first_token()?;

        let n_newlines = n_newlines_in_whitespace(&start.prev_token()?).unwrap_or(0); // spellchecker:disable-line

        if n_newlines > 0 {
            set_whitespace_before(
                &syntax.first_token()?,
                create_whitespace(&format!(
                    "{}{}",
                    "\n".repeat(n_newlines),
                    options.indent_symbol.repeat(indentation)
                )),
            );
        }
    }

    match syntax.kind() {
        SyntaxKind::FunctionDeclaration => {
            let function = ast::FunctionDeclaration::cast(syntax)?;

            trim_whitespace_before_to_newline(&function.fn_token()?);

            set_whitespace_single_after(&function.fn_token()?);
            set_whitespace_single_before(&function.body()?.left_brace_token()?);

            let param_list = function.parameter_list()?;

            remove_if_whitespace(&param_list.left_parenthesis_token()?.prev_token()?); // spellchecker:disable-line

            let has_newline =
                is_whitespace_with_newline(&param_list.left_parenthesis_token()?.next_token()?);

            format_param_list(
                param_list.parameters(),
                param_list.parameters().count(),
                has_newline,
                1,
                options.trailing_commas,
                &options.indent_symbol,
            );

            if has_newline {
                set_whitespace_before(
                    &param_list.right_parenthesis_token()?,
                    create_whitespace("\n"),
                );
            } else {
                remove_if_whitespace(&param_list.right_parenthesis_token()?.prev_token()?); // spellchecker:disable-line
            }
        },
        SyntaxKind::Parameter => {
            let item = ast::Parameter::cast(syntax)?;
            remove_if_whitespace(&item.colon_token()?.prev_token()?); // spellchecker:disable-line
            set_whitespace_single_after(&item.colon_token()?);
        },
        SyntaxKind::ReturnType => {
            let return_type = ast::ReturnType::cast(syntax)?;
            whitespace_to_single_around(&return_type.arrow_token()?);
        },
        SyntaxKind::StructDeclaration => {
            let r#struct = ast::StructDeclaration::cast(syntax)?;

            trim_whitespace_before_to_newline(&r#struct.struct_token()?);

            let name = r#struct.name()?;
            whitespace_to_single_around(&name.ident_token()?);

            let body = r#struct.body()?;
            let l_brace = body.left_brace_token()?;
            let r_brace = body.right_brace_token()?;
            let mut fields = body.fields();
            // indent opening brace
            indent_after(&l_brace, indentation + 1, options)?;
            if fields.next().is_none() {
                // empty struct: no inner indentation
                set_whitespace_before(&r_brace, create_whitespace(""));
            } else {
                // indent each field line
                for field in fields {
                    let first = field.syntax().first_token()?;
                    indent_before(&first, indentation + 1, options)?;
                }
                // closing brace on its own line
                indent_before(&r_brace, indentation, options)?;
            }
        },
        SyntaxKind::StructMember => {
            let item = ast::StructMember::cast(syntax)?;
            remove_if_whitespace(&item.colon_token()?.prev_token()?); // spellchecker:disable-line
            set_whitespace_single_after(&item.colon_token()?);
            // Remove whitespace between the last token of the member and a following comma.
            // The comma lives in StructBody (parent), so walk from the member's last token.
            if let Some(last) = item.syntax().last_token() {
                let mut tok = last.next_token()?;
                while tok.kind().is_whitespace() {
                    let next = tok.next_token()?;
                    remove_token(&tok);
                    tok = next;
                }
                // tok is now the comma (or something else); no action needed on it.
            }
        },
        SyntaxKind::IfStatement => {
            let if_statement = ast::IfStatement::cast(syntax)?;

            if let Some(if_block) = if_statement.if_block() {
                set_whitespace_single_after(&if_block.if_token()?);
                set_whitespace_single_before(&if_block.block()?.left_brace_token()?);
            }

            if let Some(else_block) = if_statement.else_block() {
                whitespace_to_single_around(&else_block.else_token()?);
            }

            for else_if_block in if_statement.else_if_blocks() {
                whitespace_to_single_around(&else_if_block.else_token()?);
                whitespace_to_single_around(&else_if_block.if_token()?);

                set_whitespace_single_before(&else_if_block.block()?.left_brace_token()?);
            }
        },
        SyntaxKind::WhileStatement => {
            let while_statement = ast::WhileStatement::cast(syntax)?;

            set_whitespace_single_after(&while_statement.while_token()?);

            set_whitespace_single_before(&while_statement.block()?.left_brace_token()?);
        },
        SyntaxKind::SwitchStatement => {
            let switch_statement = ast::SwitchStatement::cast(syntax)?;
            // Space after `switch` keyword
            let first_token = switch_statement.syntax().first_token()?;
            if first_token.kind() == SyntaxKind::Switch {
                set_whitespace_single_after(&first_token);
            }
            // Space before the opening brace of SwitchBody
            if let Some(body) = switch_statement.block() {
                let l_brace = body.syntax().first_token()?;
                set_whitespace_single_before(&l_brace);
            }
        },
        SyntaxKind::LoopStatement => {
            let loop_statement = ast::LoopStatement::cast(syntax)?;
            // Space before the opening brace of the loop body
            if let Some(block) = loop_statement.block() {
                set_whitespace_single_before(&block.left_brace_token()?);
            }
        },
        SyntaxKind::ContinuingStatement => {
            let continuing = ast::ContinuingStatement::cast(syntax)?;
            // Space before the opening brace of the continuing body
            if let Some(block) = continuing.block() {
                set_whitespace_single_before(&block.left_brace_token()?);
            }
        },
        SyntaxKind::ForStatement => {
            let for_statement = ast::ForStatement::cast(syntax)?;

            set_whitespace_single_after(&for_statement.for_token()?);

            set_whitespace_single_before(&for_statement.block()?.left_brace_token()?);

            remove_if_whitespace(
                &for_statement
                    .initializer()?
                    .syntax()
                    .first_token()?
                    .prev_token()?, // spellchecker:disable-line
            );
            set_whitespace_single_before(&for_statement.condition()?.syntax().first_token()?);
            set_whitespace_single_before(&for_statement.continuing_part()?.syntax().first_token()?);
            // The whitespace before ')' is a sibling of ForContinuingPart in
            // ForStatement, not inside it. Remove it by looking at the token
            // *after* the continuing part's last token.
            let cont_last = for_statement.continuing_part()?.syntax().last_token()?;
            if let Some(after) = cont_last.next_token() {
                remove_if_whitespace(&after);
            }
        },
        SyntaxKind::CompoundStatement => {
            let statement = ast::CompoundStatement::cast(syntax)?;
            let has_newline =
                is_whitespace_with_newline(&statement.left_brace_token()?.next_token()?);

            if has_newline {
                set_whitespace_before(
                    &statement.right_brace_token()?,
                    create_whitespace(&format!(
                        "\n{}",
                        options.indent_symbol.repeat(indentation.saturating_sub(1))
                    )),
                );
            }
        },
        SyntaxKind::IdentExpression => {
            let ident_expression = ast::IdentExpression::cast(syntax)?;

            let template_parameters = ident_expression.template_parameters()?;
            let left_angle = template_parameters.left_angle_token()?;
            remove_if_whitespace(&left_angle.prev_token()?); // spellchecker:disable-line
            remove_if_whitespace(&left_angle.next_token()?);
            let right_angle = template_parameters.t_angle_token()?;
            remove_if_whitespace(&right_angle.prev_token()?); // spellchecker:disable-line
        },
        SyntaxKind::FunctionCall => {
            let function_call = ast::FunctionCall::cast(syntax)?;

            if let Some(name_ref) = function_call.ident_expression()
                && let Some(NodeOrToken::Token(token)) = name_ref.syntax().next_sibling_or_token()
            {
                remove_if_whitespace(&token);
            }

            let param_list = function_call.parameters()?;
            format_parameters(&param_list, indentation, options)?;
        },
        SyntaxKind::InfixExpression => {
            let expression = ast::InfixExpression::cast(syntax)?;

            whitespace_to_single_around(&expression.operator()?);
        },
        SyntaxKind::ParenthesisExpression => {
            let parenthesis_expression = ast::ParenthesisExpression::cast(syntax)?;
            remove_if_whitespace(
                &parenthesis_expression
                    .left_parenthesis_token()?
                    .next_token()?,
            );
            remove_if_whitespace(
                &parenthesis_expression
                    .right_parenthesis_token()?
                    .prev_token()?, // spellchecker:disable-line
            );

            if parenthesis_expression
                .syntax()
                .parent()
                .is_some_and(|parent| {
                    matches!(
                        parent.kind(),
                        |SyntaxKind::WhileStatement| SyntaxKind::IfClause
                            | SyntaxKind::ElseIfClause
                    )
                })
            {
                remove_token(&parenthesis_expression.right_parenthesis_token()?);
                remove_token(&parenthesis_expression.left_parenthesis_token()?);
            }
        },
        SyntaxKind::AssignmentStatement => {
            let statement = ast::AssignmentStatement::cast(syntax)?;
            whitespace_to_single_around(&statement.equal_token()?);
        },
        SyntaxKind::CompoundAssignmentStatement => {
            let statement = ast::CompoundAssignmentStatement::cast(syntax)?;
            // operator_token() walks from left_side().last_token().next_token(),
            // which may return a whitespace token if multiple whitespace nodes
            // exist between the identifier and operator. Walk forward, removing
            // extra whitespace, to find the actual operator token.
            let left_last = statement.left_side()?.syntax().last_token()?;
            let mut tok = left_last.next_token()?;
            // Advance past trivia (whitespace and comments) to find the
            // actual operator token. Only remove whitespace; comments are
            // preserved with a single space around them.
            loop {
                let kind = tok.kind();
                let is_comment = matches!(
                    kind,
                    SyntaxKind::BlockComment | SyntaxKind::LineEndingComment
                );
                if !kind.is_whitespace() && !is_comment {
                    break;
                }
                let next = tok.next_token()?;
                if kind.is_whitespace() {
                    remove_token(&tok);
                }
                if is_comment {
                    whitespace_to_single_around(&tok);
                }
                tok = next;
            }
            whitespace_to_single_around(&tok);
        },
        SyntaxKind::VariableDeclaration => {
            let statement = ast::VariableDeclaration::cast(syntax)?;
            // Ensure a space after the template closing `>` (e.g. `var<uniform> camera`)
            if let Some(tmpl) = statement.template_parameters()
                && let Some(right_angle) = tmpl.t_angle_token()
            {
                set_whitespace_single_after(&right_angle);
            }
            if let Some(colon) = statement.colon() {
                remove_if_whitespace(&colon.prev_token()?); // spellchecker:disable-line
                set_whitespace_single_after(&colon);
            }
            whitespace_to_single_around(&statement.equal_token()?);
        },
        SyntaxKind::LetDeclaration => {
            let statement = ast::LetDeclaration::cast(syntax)?;
            if let Some(colon) = statement.colon() {
                remove_if_whitespace(&colon.prev_token()?); // spellchecker:disable-line
                set_whitespace_single_after(&colon);
            }
            whitespace_to_single_around(&statement.equal_token()?);
        },
        SyntaxKind::ConstantDeclaration => {
            let statement = ast::ConstantDeclaration::cast(syntax)?;
            if let Some(colon) = statement.colon() {
                remove_if_whitespace(&colon.prev_token()?); // spellchecker:disable-line
                set_whitespace_single_after(&colon);
            }
            whitespace_to_single_around(&statement.equal_token()?);
        },
        SyntaxKind::OverrideDeclaration => {
            let statement = ast::OverrideDeclaration::cast(syntax)?;
            set_whitespace_single_after(&statement.override_token()?);
            if let Some(colon) = statement.colon() {
                remove_if_whitespace(&colon.prev_token()?); // spellchecker:disable-line
                set_whitespace_single_after(&colon);
            }
            whitespace_to_single_around(&statement.equal_token()?);
        },
        SyntaxKind::TypeAliasDeclaration => {
            let statement = ast::TypeAliasDeclaration::cast(syntax)?;
            set_whitespace_single_after(&statement.alias_token()?);
            whitespace_to_single_around(&statement.equal_token()?);
        },
        SyntaxKind::AssertStatement => {
            // Collapse extra spaces after `const_assert` keyword.
            let first_token = syntax.first_token()?;
            if first_token.kind() == SyntaxKind::ConstantAssert {
                set_whitespace_single_after(&first_token);
            }
        },
        SyntaxKind::ReturnStatement => {
            // Collapse multiple spaces after `return` to a single space.
            let first_token = syntax.first_token()?;
            if first_token.kind() == SyntaxKind::Return {
                set_whitespace_single_after(&first_token);
            }
        },
        _ => {
            if let Some(r#type) = ast::TypeSpecifier::cast(syntax) {
                let template_parameters = r#type.template_parameters()?;
                let left_angle = template_parameters.left_angle_token()?;
                remove_if_whitespace(&left_angle.prev_token()?); // spellchecker:disable-line
                remove_if_whitespace(&left_angle.next_token()?);
                let right_angle = template_parameters.t_angle_token()?;
                remove_if_whitespace(&right_angle.prev_token()?); // spellchecker:disable-line
            }
        },
    }

    None
}

fn format_parameters(
    param_list: &ast::Arguments,
    indentation: usize,
    options: &FormattingOptions,
) -> Option<()> {
    let has_newline =
        is_whitespace_with_newline(&param_list.left_parenthesis_token()?.next_token()?);
    format_param_list(
        param_list.arguments(),
        param_list.arguments().count(),
        has_newline,
        indentation + 1,
        options.trailing_commas,
        &options.indent_symbol,
    );
    if has_newline {
        set_whitespace_before(
            &param_list.right_parenthesis_token()?,
            create_whitespace(&format!("\n{}", options.indent_symbol.repeat(indentation))),
        );
    } else {
        remove_if_whitespace(&param_list.right_parenthesis_token()?.prev_token()?); // spellchecker:disable-line
    }
    Some(())
}

fn format_param_list<T: AstNode>(
    parameters: syntax::AstChildren<T>,
    count: usize,
    has_newline: bool,
    n_indentations: usize,
    trailing_comma_policy: Policy,
    indent_symbol: &str,
) -> Option<()> {
    let mut first = true;
    for (index, parameter) in parameters.enumerate() {
        let last = index == count - 1;

        let first_token = parameter.syntax().first_token()?;
        let previous_had_newline = first_token
            .prev_token() // spellchecker:disable-line
            .is_some_and(|token| token.kind().is_whitespace() && token.text().contains('\n'));

        let whitespace = match (first, previous_had_newline) {
            (true, false) => create_whitespace(""),
            (_, true) => create_whitespace(&format!("\n{}", indent_symbol.repeat(n_indentations))),
            (false, false) => create_whitespace(" "),
        };

        set_whitespace_before(&first_token, whitespace);

        let last_param_token = parameter.syntax().last_token()?;
        remove_if_whitespace(&last_param_token);

        // Find the token after the parameter, skipping (and removing) any
        // whitespace between the parameter and the comma.
        let mut token_after_parameter = match parameter.syntax().next_sibling_or_token() {
            Some(NodeOrToken::Node(node)) => node.first_token(),
            Some(NodeOrToken::Token(token)) => Some(token),
            None => None,
        };
        // Remove whitespace between the parameter and its comma, scanning
        // forward through all trivia tokens (whitespace and comments).
        // Only whitespace is removed; comments are preserved.
        while let Some(tok) = &token_after_parameter {
            let kind = tok.kind();
            if kind.is_whitespace() {
                let next = tok.next_token();
                remove_token(tok);
                token_after_parameter = next;
            } else if matches!(
                kind,
                SyntaxKind::BlockComment | SyntaxKind::LineEndingComment
            ) {
                // Skip past the comment but keep it
                token_after_parameter = tok.next_token();
            } else {
                break;
            }
        }

        if let Some(token_after_parameter) = token_after_parameter {
            let is_comma = token_after_parameter.kind() == SyntaxKind::Comma;
            match (last, is_comma) {
                // Single-line trailing comma: remove it
                (true, true) if !has_newline => token_after_parameter.detach(),
                // Last param: apply trailing comma policy
                (true, has_comma) => match (trailing_comma_policy, has_comma) {
                    (Policy::Remove, true) => token_after_parameter.detach(),
                    (Policy::Remove, false) | (Policy::Insert, true) | (Policy::Ignore, _) => {},
                    (Policy::Insert, false) => {
                        insert_after_syntax(
                            parameter.syntax(),
                            create_syntax_token(SyntaxKind::Comma, ","),
                        );
                    },
                },
                // Not last, comma already present: nothing to do
                (false, true) => {},
                // Not last, no comma: insert one
                (false, false) => {
                    insert_after_syntax(
                        parameter.syntax(),
                        create_syntax_token(SyntaxKind::Comma, ","),
                    );
                },
            }
        }

        first = false;
    }

    Some(())
}

// "\n  fn" -> "\nfn"
fn trim_whitespace_before_to_newline(before: &SyntaxToken) -> Option<()> {
    let maybe_whitespace = before.prev_token()?; // spellchecker:disable-line
    if maybe_whitespace.kind().is_whitespace() {
        let index = maybe_whitespace.index();

        let text = maybe_whitespace.text().trim_end_matches(' ');

        maybe_whitespace.parent().unwrap().splice_children(
            index..index + 1,
            vec![SyntaxElement::Token(create_whitespace(text))],
        );
    }
    Some(())
}

fn is_whitespace_with_newline(maybe_whitespace: &SyntaxToken) -> bool {
    maybe_whitespace.kind().is_whitespace() && maybe_whitespace.text().contains('\n')
}

fn n_newlines_in_whitespace(maybe_whitespace: &SyntaxToken) -> Option<usize> {
    maybe_whitespace
        .kind()
        .is_whitespace()
        .then(|| maybe_whitespace.text().matches('\n').count())
}

fn remove_if_whitespace(maybe_whitespace: &SyntaxToken) {
    if maybe_whitespace.kind().is_whitespace() {
        remove_token(maybe_whitespace);
    }
}

fn remove_token(token: &SyntaxToken) {
    let index = token.index();
    token
        .parent()
        .unwrap()
        .splice_children(index..index + 1, Vec::new());
}

fn replace_token_with(
    token: &SyntaxToken,
    replacement: SyntaxToken,
) {
    let index = token.index();
    token
        .parent()
        .unwrap()
        .splice_children(index..index + 1, vec![SyntaxElement::Token(replacement)]);
}

fn insert_after(
    token: &SyntaxToken,
    insert: SyntaxToken,
) {
    let index = token.index();
    token
        .parent()
        .unwrap()
        .splice_children((index + 1)..index + 1, vec![SyntaxElement::Token(insert)]);
}

fn insert_after_syntax(
    node: &SyntaxNode,
    insert: SyntaxToken,
) {
    let index = node.index();
    node.parent()
        .unwrap()
        .splice_children((index + 1)..index + 1, vec![SyntaxElement::Token(insert)]);
}

fn insert_before(
    token: &SyntaxToken,
    insert: SyntaxToken,
) {
    let index = token.index();
    token
        .parent()
        .unwrap()
        .splice_children(index..index, vec![SyntaxElement::Token(insert)]);
}

fn whitespace_to_single_around(around: &SyntaxToken) {
    set_whitespace_single_before(around);
    set_whitespace_single_after(around);
}

fn set_whitespace_after(
    after: &SyntaxToken,
    to: SyntaxToken,
) -> Option<()> {
    let maybe_whitespace = after.next_token()?;
    if maybe_whitespace.kind().is_whitespace() {
        replace_token_with(&maybe_whitespace, to);
    } else {
        insert_after(after, to);
    }

    Some(())
}

fn set_whitespace_before(
    before: &SyntaxToken,
    to: SyntaxToken,
) -> Option<()> {
    let maybe_whitespace = before.prev_token()?; // spellchecker:disable-line
    if maybe_whitespace.kind().is_whitespace() {
        replace_token_with(&maybe_whitespace, to);
    } else {
        insert_before(before, to);
    }

    Some(())
}

fn set_whitespace_single_after(after: &SyntaxToken) -> Option<()> {
    set_whitespace_after(after, single_whitespace())
}

fn set_whitespace_single_before(before: &SyntaxToken) -> Option<()> {
    set_whitespace_before(before, single_whitespace())
}

fn single_whitespace() -> SyntaxToken {
    create_whitespace(" ")
}

fn create_whitespace(text: &str) -> SyntaxToken {
    create_syntax_token(SyntaxKind::Blankspace, text)
}

fn create_syntax_token(
    kind: SyntaxKind,
    text: &str,
) -> SyntaxToken {
    let node = SyntaxNode::new_root(GreenNode::new(
        SyntaxKind::Error.into(),
        std::iter::once(NodeOrToken::Token(GreenToken::new(kind.into(), text))),
    ))
    .clone_for_update();
    node.first_token().unwrap()
}

fn indent_after(
    token: &SyntaxToken,
    indent_level: usize,
    options: &FormattingOptions,
) -> Option<()> {
    let whitespace =
        create_whitespace(&format!("\n{}", options.indent_symbol.repeat(indent_level)));
    set_whitespace_after(token, whitespace)
}

fn indent_before(
    token: &SyntaxToken,
    indent_level: usize,
    options: &FormattingOptions,
) -> Option<()> {
    let whitespace =
        create_whitespace(&format!("\n{}", options.indent_symbol.repeat(indent_level)));
    set_whitespace_before(token, whitespace)
}

#[cfg(test)]
mod policy_tests {
    use super::*;
    use std::str::FromStr as _;
    #[test]
    fn policy_from_str_valid_values() {
        assert!(matches!(Policy::from_str("ignore"), Ok(Policy::Ignore)));
        assert!(matches!(Policy::from_str("insert"), Ok(Policy::Insert)));
        assert!(matches!(Policy::from_str("remove"), Ok(Policy::Remove)));
    }
    #[test]
    fn policy_from_str_invalid_value() {
        let result = Policy::from_str("invalid_value");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "invalid policy: invalid_value");
    }
}
