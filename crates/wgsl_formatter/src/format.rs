use rowan::NodeOrToken;
use syntax::{AstNode, HasName as _, HasTemplateParameters as _, SyntaxKind, SyntaxNode, ast};

use crate::FormattingOptions;
use crate::util::{
    create_syntax_token, create_whitespace, indent_after, indent_before, insert_after_syntax,
    is_whitespace_with_newline, n_newlines_in_whitespace, remove_if_whitespace, remove_token,
    set_whitespace_after, set_whitespace_before, set_whitespace_single_after,
    set_whitespace_single_before, trim_whitespace_before_to_newline, whitespace_to_single_around,
};

use crate::is_indent_kind;

#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "impractical to list all cases"
)]
#[expect(clippy::cognitive_complexity, clippy::too_many_lines, reason = "TODO")]
pub(crate) fn format_syntax_node(
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
            // If this node is itself an indent-kind (e.g. a nested CompoundStatement),
            // its first token (the opening `{`) should be at the parent's content level,
            // not the bumped level. Subtract 1 to compensate for the early increment.
            let indent = if is_indent_kind(&syntax) {
                indentation.saturating_sub(1)
            } else {
                indentation
            };
            set_whitespace_before(
                &syntax.first_token()?,
                create_whitespace(&format!(
                    "{}{}",
                    "\n".repeat(n_newlines),
                    options.indent_symbol.repeat(indent)
                )),
            );
        }
    }

    // Remove whitespace before semicolons in any statement node.
    if let Some(last) = syntax.last_token() {
        if last.kind() == SyntaxKind::Semicolon {
            remove_if_whitespace(&last.prev_token()?); // spellchecker:disable-line
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
            // Check if the struct has any real fields (the parser may emit
            // zero-width StructMember nodes for empty structs).
            let has_fields = body
                .fields()
                .any(|f| f.syntax().text_range().len() > 0.into());
            if !has_fields {
                // empty struct: `struct Foo {}`
                // The l_brace token is still valid. Set whitespace after it to empty.
                set_whitespace_after(&l_brace, create_whitespace(""));
            } else {
                // indent opening brace
                indent_after(&l_brace, indentation + 1, options)?;
                // indent each field line
                for field in body.fields() {
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
        SyntaxKind::SwitchBodyCase => {
            let case = ast::SwitchBodyCase::cast(syntax)?;
            // Collapse space after `case`/`default` keyword
            let first = case.syntax().first_token()?;
            if matches!(first.kind(), SyntaxKind::Case | SyntaxKind::Default) {
                set_whitespace_single_after(&first);
            }
            // Remove space before `:`
            for token in case.syntax().children_with_tokens() {
                if let Some(t) = token.as_token() {
                    if t.kind() == SyntaxKind::Colon {
                        remove_if_whitespace(&t.prev_token()?); // spellchecker:disable-line
                        break;
                    }
                }
            }
            // Space before the opening brace
            if let Some(block) = case.block() {
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
            // Remove whitespace before the semicolons separating for-header parts.
            for child in for_statement.syntax().children_with_tokens() {
                if let Some(t) = child.as_token() {
                    if t.kind() == SyntaxKind::Semicolon {
                        remove_if_whitespace(&t.prev_token()?); // spellchecker:disable-line
                    }
                }
            }
            set_whitespace_single_before(&for_statement.condition()?.syntax().first_token()?);
            set_whitespace_single_before(&for_statement.continuing_part()?.syntax().first_token()?);
            let cont_last = for_statement.continuing_part()?.syntax().last_token()?;
            if let Some(after) = cont_last.next_token() {
                remove_if_whitespace(&after);
            }
        },
        SyntaxKind::CompoundStatement => {
            let statement = ast::CompoundStatement::cast(syntax)?;
            let l_brace = statement.left_brace_token()?;
            let r_brace = statement.right_brace_token()?;
            let has_newline = is_whitespace_with_newline(&l_brace.next_token()?);

            if has_newline {
                set_whitespace_before(
                    &r_brace,
                    create_whitespace(&format!(
                        "\n{}",
                        options.indent_symbol.repeat(indentation.saturating_sub(1))
                    )),
                );
            } else {
                // Single-line block: ensure single space after `{` and before `}`
                if l_brace.next_token() != Some(r_brace.clone()) {
                    set_whitespace_single_after(&l_brace);
                    set_whitespace_single_before(&r_brace);
                }
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
            let left_last = statement.left_side()?.syntax().last_token()?;
            let mut tok = left_last.next_token()?;
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
        SyntaxKind::IncrementDecrementStatement => {
            for token in syntax.children_with_tokens() {
                if let Some(t) = token.as_token() {
                    if matches!(t.kind(), SyntaxKind::PlusPlus | SyntaxKind::MinusMinus) {
                        remove_if_whitespace(&t.prev_token()?); // spellchecker:disable-line
                        break;
                    }
                }
            }
        },
        SyntaxKind::VariableDeclaration => {
            let statement = ast::VariableDeclaration::cast(syntax)?;
            if let Some(tmpl) = statement.template_parameters() {
                if let Some(left_angle) = tmpl.left_angle_token() {
                    remove_if_whitespace(&left_angle.prev_token()?); // spellchecker:disable-line
                }
                if let Some(right_angle) = tmpl.t_angle_token() {
                    set_whitespace_single_after(&right_angle);
                }
            } else {
                set_whitespace_single_after(&statement.var_token()?);
            }
            if let Some(colon) = statement.colon() {
                remove_if_whitespace(&colon.prev_token()?); // spellchecker:disable-line
                set_whitespace_single_after(&colon);
            }
            whitespace_to_single_around(&statement.equal_token()?);
        },
        SyntaxKind::LetDeclaration => {
            let statement = ast::LetDeclaration::cast(syntax)?;
            set_whitespace_single_after(&statement.let_token()?);
            if let Some(colon) = statement.colon() {
                remove_if_whitespace(&colon.prev_token()?); // spellchecker:disable-line
                set_whitespace_single_after(&colon);
            }
            whitespace_to_single_around(&statement.equal_token()?);
        },
        SyntaxKind::ConstantDeclaration => {
            let statement = ast::ConstantDeclaration::cast(syntax)?;
            set_whitespace_single_after(&statement.constant_token()?);
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
            let first_token = syntax.first_token()?;
            if first_token.kind() == SyntaxKind::ConstantAssert {
                set_whitespace_single_after(&first_token);
            }
        },
        SyntaxKind::Attribute => {
            if let Some(last) = syntax.last_token() {
                set_whitespace_single_after(&last);
            }
        },
        SyntaxKind::ReturnStatement => {
            let first_token = syntax.first_token()?;
            if first_token.kind() == SyntaxKind::Return {
                let next = first_token.next_token()?;
                if next.kind() != SyntaxKind::Semicolon {
                    set_whitespace_single_after(&first_token);
                }
            }
        },
        SyntaxKind::PhonyAssignmentStatement => {
            let statement = ast::PhonyAssignmentStatement::cast(syntax)?;
            whitespace_to_single_around(&statement.equal_token()?);
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
    trailing_comma_policy: crate::Policy,
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

        let mut token_after_parameter = match parameter.syntax().next_sibling_or_token() {
            Some(NodeOrToken::Node(node)) => node.first_token(),
            Some(NodeOrToken::Token(token)) => Some(token),
            None => None,
        };
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
                token_after_parameter = tok.next_token();
            } else {
                break;
            }
        }

        if let Some(token_after_parameter) = token_after_parameter {
            let is_comma = token_after_parameter.kind() == SyntaxKind::Comma;
            match (last, is_comma) {
                (true, true) if !has_newline => token_after_parameter.detach(),
                (true, has_comma) => match (trailing_comma_policy, has_comma) {
                    (crate::Policy::Remove, true) => token_after_parameter.detach(),
                    (crate::Policy::Remove, false)
                    | (crate::Policy::Insert, true)
                    | (crate::Policy::Ignore, _) => {},
                    (crate::Policy::Insert, false) => {
                        insert_after_syntax(
                            parameter.syntax(),
                            create_syntax_token(SyntaxKind::Comma, ","),
                        );
                    },
                },
                (false, true) => {},
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
