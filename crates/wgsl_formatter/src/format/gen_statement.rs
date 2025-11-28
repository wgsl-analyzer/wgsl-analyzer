use dprint_core::formatting::{ColumnNumber, LineNumber, LineNumberAnchor, PrintItems, Signal};
use dprint_core_macros::sc;
use itertools::put_back;
use parser::{SyntaxKind, SyntaxToken};
use rowan::NodeOrToken;
use syntax::{
    AstNode,
    ast::{
        self, CompoundStatement, ElseClause, ElseIfClause, Expression, FunctionCall,
        IdentExpression, IfClause, Literal, ParenthesisExpression, Statement,
    },
};

use crate::format::{
    self,
    ast_parse::{
        parse_end, parse_many_comments_and_blankspace, parse_node, parse_node_by_kind,
        parse_node_by_kind_optional, parse_node_optional, parse_token, parse_token_optional,
    },
    gen_comments::{gen_comment, gen_comments},
    gen_expression::{gen_expression, gen_parenthesis_expression},
    gen_function_call::gen_function_call,
    gen_if_statement::gen_if_statement,
    gen_switch_statement::gen_switch_statement,
    helpers::{create_is_multiple_lines_resolver, gen_spaced_lines, todo_verbatim},
    multiline_group::gen_multiline_group,
    print_item_buffer::{PrintItemBuffer, SeparationPolicy, SeparationRequest},
    reporting::{FormatDocumentError, FormatDocumentErrorKind, FormatDocumentResult},
};

#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "It will match future variants, and that's intentional"
)]
pub fn gen_compound_statement(
    syntax: &ast::CompoundStatement
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    //TODO I don't really like this, but its an easy way for now
    let body_empty = syntax.syntax().children_with_tokens().all(|child| {
        matches!(
            child.kind(),
            SyntaxKind::BraceLeft | SyntaxKind::BraceRight | SyntaxKind::Blankspace
        )
    });

    let lines = gen_spaced_lines(syntax.syntax(), |child| {
        //TODO This clone is unnecessary if we had a cast that returned the passed in node
        // on a failure like std::any::Any (SyntaxNode -> Result<Item, Syntaxnode>)
        if let NodeOrToken::Node(child) = child
            && let Some(statement) = ast::Statement::cast(child.clone())
        {
            let mut formatted = PrintItemBuffer::new();
            formatted.request_line_break(SeparationPolicy::Expected);
            formatted.extend(gen_statement(&statement)?);
            Ok(formatted)
        } else if let NodeOrToken::Token(child) = child
            && matches!(
                child.kind(),
                SyntaxKind::BlockComment | SyntaxKind::LineEndingComment
            )
        {
            Ok(gen_comment(child))
        } else if let NodeOrToken::Token(child) = child
            && matches!(child.kind(), SyntaxKind::BraceLeft | SyntaxKind::BraceRight)
        {
            //The braces are expected, we could pop them from the syntax before passing them to gen_spaced_lines,
            // but this way is just as fine
            Ok(PrintItemBuffer::new())
        } else {
            Err(FormatDocumentErrorKind::UnexpectedToken {
                received: child.clone(),
            }
            .at(child.text_range()))
        }
    })?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    formatted.push_sc(sc!("{"));

    if !body_empty {
        formatted.push_signal(Signal::StartIndent);
        formatted.request(SeparationRequest {
            empty_line: SeparationPolicy::Discouraged,
            line_break: SeparationPolicy::Expected,
            ..Default::default()
        });
        formatted.extend(lines);
        formatted.request(SeparationRequest {
            empty_line: SeparationPolicy::Discouraged,
            line_break: SeparationPolicy::Expected,
            ..Default::default()
        });
        formatted.push_signal(Signal::FinishIndent);
    }

    formatted.push_sc(sc!("}"));
    Ok(formatted)
}

fn gen_statement(item: &ast::Statement) -> Result<PrintItemBuffer, FormatDocumentError> {
    // Read comment in gen_statement_maybe_semicolon
    // As most of the calls to gen_statement will always have true as the include_semicolon
    // parameter, I want to have this alias function, so that once I properly clean up the
    // flag, i don't need to change code that doesn't semantically change.
    // Also, this way, places that need the flag are highlighted and easier to find.
    gen_statement_maybe_semicolon(item, true)
}

fn gen_statement_maybe_semicolon(
    item: &ast::Statement,
    // TODO Consider absorbing semicolon handling into PrintItemBuffer,
    // passing around random flags is bad, as it leads to spaghetti code and if
    // one gen_* function forgets that it would need the flag, that will lead to weird
    // corner cases bugs. Things like
    // PrintItemBuffer::request_semicolon() and PrintItemBuffer::forbid_semicolon()
    // would be much better. But there are many design questions if the PIB has to handle
    // more than just spaces, and i don't know about all the use cases until the formatter is
    // done.
    include_semicolon: bool,
) -> Result<PrintItemBuffer, FormatDocumentError> {
    match item {
        ast::Statement::IfStatement(if_statement) => gen_if_statement(if_statement),
        ast::Statement::SwitchStatement(switch_statement) => gen_switch_statement(switch_statement),
        ast::Statement::LoopStatement(loop_statement) => gen_loop_statement(loop_statement),
        ast::Statement::ForStatement(for_statement) => gen_for_statement(for_statement),
        ast::Statement::WhileStatement(while_statement) => gen_while_statement(while_statement),
        ast::Statement::CompoundStatement(compound_statement) => {
            gen_compound_statement(compound_statement)
        },
        ast::Statement::FunctionCallStatement(function_call_statement) => {
            gen_function_call_statement(function_call_statement, include_semicolon)
        },
        ast::Statement::VariableDeclaration(variable_declaration) => {
            gen_var_declaration_statement(variable_declaration, include_semicolon)
        },
        ast::Statement::LetDeclaration(let_declaration) => {
            gen_let_declaration_statement(let_declaration, include_semicolon)
        },
        ast::Statement::ConstantDeclaration(constant_declaration) => {
            gen_const_declaration_statement(constant_declaration, include_semicolon)
        },
        ast::Statement::AssignmentStatement(assignment_statement) => {
            gen_assignment_statement(assignment_statement, include_semicolon)
        },
        ast::Statement::PhonyAssignmentStatement(phony_assignment_statement) => {
            gen_phony_assignment_statement(phony_assignment_statement, include_semicolon)
        },
        ast::Statement::CompoundAssignmentStatement(compound_assignment_statement) => {
            todo_verbatim(compound_assignment_statement.syntax())
        },
        ast::Statement::IncrementDecrementStatement(increment_decrement_statement) => {
            todo_verbatim(increment_decrement_statement.syntax())
        },
        ast::Statement::ContinuingStatement(continuing_statement) => {
            gen_continuing_statement(continuing_statement)
        },
        ast::Statement::ReturnStatement(return_statement) => {
            gen_return_statement(return_statement, include_semicolon)
        },
        ast::Statement::BreakStatement(break_statement) => {
            // ==== Parse ====
            // We still parse through the break syntax even tho there is no information for
            // the formatter to get out of it. This exists to ensure we don't accidentally delete
            // user's code should future changes to wgsl allow more complex break statements.
            let mut syntax = put_back(break_statement.syntax().children_with_tokens());
            parse_token(&mut syntax, SyntaxKind::Break)?;
            parse_token_optional(&mut syntax, SyntaxKind::Semicolon);
            parse_end(&mut syntax);

            // ==== Format ====
            let mut formatted = PrintItemBuffer::new();
            formatted.push_sc(sc!("break"));
            if include_semicolon {
                formatted.push_sc(sc!(";"));
            }
            formatted.expect_line_break();
            Ok(formatted)
        },
        ast::Statement::ContinueStatement(continue_statement) => {
            // ==== Parse ====
            // We still parse through the discard syntax even tho there is no information for
            // the formatter to get out of it. This exists to ensure we don't accidentally delete
            // user's code should future changes to wgsl allow more complex continue statements.
            let mut syntax = put_back(continue_statement.syntax().children_with_tokens());
            parse_token(&mut syntax, SyntaxKind::Continue)?;
            let comments_after_continue = parse_many_comments_and_blankspace(&mut syntax)?;
            parse_token_optional(&mut syntax, SyntaxKind::Semicolon);
            parse_end(&mut syntax);

            // ==== Format ====
            let mut formatted = PrintItemBuffer::new();
            formatted.push_sc(sc!("continue"));
            if include_semicolon {
                formatted.push_sc(sc!(";"));
            }
            formatted.expect_line_break();
            formatted.extend(gen_comments(comments_after_continue));
            Ok(formatted)
        },
        ast::Statement::DiscardStatement(discard) => {
            // ==== Parse ====
            // We still parse through the discard syntax even tho there is no information for
            // the formatter to get out of it. This exists to ensure we don't accidentally delete
            // user's code should future changes to wgsl allow more complex discard statements.
            let mut syntax = put_back(discard.syntax().children_with_tokens());
            parse_token(&mut syntax, SyntaxKind::Discard)?;
            let comments_after_discard = parse_many_comments_and_blankspace(&mut syntax)?;
            parse_token_optional(&mut syntax, SyntaxKind::Semicolon);
            parse_end(&mut syntax);

            // ==== Format ====
            let mut formatted = PrintItemBuffer::new();
            formatted.push_sc(sc!("discard"));
            if include_semicolon {
                formatted.push_sc(sc!(";"));
            }
            formatted.expect_line_break();
            formatted.extend(gen_comments(comments_after_discard));
            Ok(formatted)
        },
        ast::Statement::AssertStatement(assert_statement) => {
            todo_verbatim(assert_statement.syntax())
        },
        ast::Statement::BreakIfStatement(break_if_statement) => {
            gen_break_if_statement(break_if_statement, include_semicolon)
        },
    }
}

fn gen_assignment_statement(
    assignment_statement: &ast::AssignmentStatement,
    include_semicolon: bool,
) -> Result<PrintItemBuffer, FormatDocumentError> {
    // NOTE!! - When changing this function, make sure to also update gen_phony_assignment_statement.
    // This is non-dry code, but when inevitably at some point there will be some differences between
    // the two, this should clearly communicate that they should be split up and not
    // continue to be one function with a whole lot of parameters and ifs.

    dbg!(assignment_statement.syntax());
    // ==== Parse ====
    let mut syntax = put_back(assignment_statement.syntax().children_with_tokens());
    let item_target = parse_node::<Expression>(&mut syntax)?;
    let item_comments_after_target = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, SyntaxKind::Equal)?;
    let item_comments_after_equal = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_value = parse_node::<Expression>(&mut syntax)?;
    let item_comments_after_value = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token_optional(&mut syntax, SyntaxKind::Semicolon);
    parse_end(&mut syntax);

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    formatted.extend(gen_expression(&item_target, true)?);
    formatted.extend(gen_comments(item_comments_after_target));
    formatted.expect_single_space();
    formatted.push_sc(sc!("="));
    formatted.expect_single_space();
    formatted.extend(gen_comments(item_comments_after_equal));
    formatted.extend(gen_expression(&item_value, true)?);
    formatted.extend(gen_comments(item_comments_after_value));
    if include_semicolon {
        formatted.request_space(SeparationPolicy::Discouraged);
        formatted.push_sc(sc!(";"));
    }
    Ok(formatted)
}

fn gen_phony_assignment_statement(
    phony_assignment_statement: &ast::PhonyAssignmentStatement,
    include_semicolon: bool,
) -> Result<PrintItemBuffer, FormatDocumentError> {
    // NOTE!! - When changing this function, make sure to also update gen_assignment_statement.
    // This is non-dry code, but when inevitably at some point there will be some differences between
    // the two, this should clearly communicate that they should be split up and not
    // continue to be one function with a whole lot of parameters and ifs.

    dbg!(phony_assignment_statement.syntax());
    // ==== Parse ====
    let mut syntax = put_back(phony_assignment_statement.syntax().children_with_tokens());
    parse_token(&mut syntax, SyntaxKind::Underscore)?;
    let item_comments_after_target = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, SyntaxKind::Equal)?;
    let item_comments_after_equal = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_value = parse_node::<Expression>(&mut syntax)?;
    let item_comments_after_value = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token_optional(&mut syntax, SyntaxKind::Semicolon);
    parse_end(&mut syntax);

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    formatted.push_sc(sc!("_"));
    formatted.extend(gen_comments(item_comments_after_target));
    formatted.expect_single_space();
    formatted.push_sc(sc!("="));
    formatted.expect_single_space();
    formatted.extend(gen_comments(item_comments_after_equal));
    formatted.extend(gen_expression(&item_value, true)?);
    formatted.extend(gen_comments(item_comments_after_value));
    if include_semicolon {
        formatted.request_space(SeparationPolicy::Discouraged);
        formatted.push_sc(sc!(";"));
    }
    Ok(formatted)
}

fn gen_function_call_statement(
    function_call_statement: &ast::FunctionCallStatement,
    include_semicolon: bool,
) -> Result<PrintItemBuffer, FormatDocumentError> {
    // ==== Parse ====
    let mut syntax = put_back(function_call_statement.syntax().children_with_tokens());
    let function_call = parse_node::<FunctionCall>(&mut syntax)?;
    let comments_after_function_call = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token_optional(&mut syntax, SyntaxKind::Semicolon);
    parse_end(&mut syntax);

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    formatted.extend(gen_function_call(&function_call)?);
    formatted.extend(gen_comments(comments_after_function_call));
    if include_semicolon {
        formatted.push_sc(sc!(";"));
    }
    Ok(formatted)
}

fn gen_for_statement(statement: &ast::ForStatement) -> FormatDocumentResult<PrintItemBuffer> {
    dbg!(statement.syntax());

    // ==== Parse ====
    let mut syntax = put_back(statement.syntax().children_with_tokens());
    parse_token(&mut syntax, SyntaxKind::For)?;
    let comments_after_for = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, SyntaxKind::ParenthesisLeft)?;
    let comments_after_open_paren = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_initializer = parse_node_by_kind_optional(&mut syntax, SyntaxKind::ForInitializer)
        .map(|item_initializer_container| {
            let mut sub_syntax =
                put_back(item_initializer_container.syntax().children_with_tokens());
            let item_initializer = parse_node::<Statement>(&mut sub_syntax)?;
            parse_end(&mut sub_syntax);
            Ok(item_initializer)
        })
        .transpose()?;
    let comments_after_initializer = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, SyntaxKind::Semicolon)?;
    let comments_after_initializer_semicolon = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_condition = parse_node_by_kind_optional(&mut syntax, SyntaxKind::ForCondition)
        .map(|item_condition_container| {
            let mut sub_syntax = put_back(item_condition_container.syntax().children_with_tokens());
            let item_condition = parse_node::<Expression>(&mut sub_syntax)?;
            parse_end(&mut sub_syntax);
            Ok(item_condition)
        })
        .transpose()?;
    let comments_after_condition = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, SyntaxKind::Semicolon)?;
    let comments_after_condition_semicolon = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_continuing = parse_node_by_kind_optional(&mut syntax, SyntaxKind::ForContinuingPart)
        .map(|item_continuing_container| {
            let mut sub_syntax =
                put_back(item_continuing_container.syntax().children_with_tokens());
            let item_continuing = parse_node::<Statement>(&mut sub_syntax)?;
            parse_end(&mut sub_syntax);
            Ok(item_continuing)
        })
        .transpose()?;
    let comments_after_continuing = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, SyntaxKind::ParenthesisRight)?;
    let comments_after_close_paren = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_body = parse_node::<CompoundStatement>(&mut syntax)?;
    parse_end(&mut syntax);

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    formatted.push_sc(sc!("for"));
    formatted.extend(gen_comments(comments_after_for));
    formatted.push_sc(sc!("("));

    formatted.extend(gen_multiline_group([
        gen_comments(comments_after_open_paren),
        {
            let mut formatted = PrintItemBuffer::new();
            if let Some(item_initializer) = item_initializer {
                formatted.extend(gen_statement_maybe_semicolon(&item_initializer, false)?);
            } else {
                formatted.request_space(SeparationPolicy::Discouraged);
            }
            formatted.extend(gen_comments(comments_after_initializer));
            formatted.request_space(SeparationPolicy::Discouraged);
            formatted.push_sc(sc!(";"));
            formatted.extend(gen_comments(comments_after_initializer_semicolon));
            formatted
        },
        {
            let mut formatted = PrintItemBuffer::new();
            if let Some(item_condition) = item_condition {
                formatted.extend(gen_expression(&item_condition, false)?);
            } else {
                formatted.request_space(SeparationPolicy::Discouraged);
            }
            formatted.extend(gen_comments(comments_after_condition));
            formatted.request_space(SeparationPolicy::Discouraged);
            formatted.push_sc(sc!(";"));
            formatted.extend(gen_comments(comments_after_condition_semicolon));
            formatted
        },
        {
            let mut formatted = PrintItemBuffer::new();
            if let Some(item_continuing) = item_continuing {
                formatted.extend(gen_statement_maybe_semicolon(&item_continuing, false)?);
            } else {
                formatted.request_space(SeparationPolicy::Discouraged);
            }
            formatted.extend(gen_comments(comments_after_continuing));
            formatted.request_space(SeparationPolicy::Discouraged);
            formatted
        },
    ]));

    formatted.push_sc(sc!(")"));
    formatted.request_space(SeparationPolicy::Expected);
    formatted.extend(gen_comments(comments_after_close_paren));
    formatted.extend(gen_compound_statement(&item_body)?);
    Ok(formatted)
}

fn gen_return_statement(
    statement: &ast::ReturnStatement,
    include_semicolon: bool,
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(statement.syntax().children_with_tokens());
    parse_token(&mut syntax, SyntaxKind::Return)?;
    let comments_after_return = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_expression = parse_node_optional::<Expression>(&mut syntax);
    let comments_after_expression = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token_optional(&mut syntax, SyntaxKind::Semicolon);
    parse_end(&mut syntax);

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    formatted.push_sc(sc!("return"));
    formatted.extend(gen_comments(comments_after_return));
    if let Some(item_expression) = item_expression {
        formatted.expect_single_space();
        formatted.extend(gen_expression(&item_expression, true)?);
    }
    formatted.extend(gen_comments(comments_after_expression));
    formatted.request_space(SeparationPolicy::Discouraged);

    if include_semicolon {
        formatted.push_sc(sc!(";"));
    }
    Ok(formatted)
}

fn gen_break_if_statement(
    statement: &ast::BreakIfStatement,
    include_semicolon: bool,
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(statement.syntax().children_with_tokens());
    parse_token(&mut syntax, SyntaxKind::Break)?;
    let comments_after_break = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, SyntaxKind::If)?;
    let comments_after_if = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_condition = parse_node::<Expression>(&mut syntax)?;
    let comments_after_condition = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token_optional(&mut syntax, SyntaxKind::Semicolon);
    parse_end(&mut syntax);

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    formatted.push_sc(sc!("break"));
    formatted.expect_single_space();
    formatted.extend(gen_comments(comments_after_break));
    formatted.push_sc(sc!("if"));
    formatted.push_signal(Signal::StartIndent);
    formatted.expect_single_space();
    formatted.extend(gen_comments(comments_after_if));
    formatted.extend(gen_expression(&item_condition, true)?);
    formatted.extend(gen_comments(comments_after_condition));
    formatted.request_space(SeparationPolicy::Discouraged);
    if include_semicolon {
        formatted.push_sc(sc!(";"));
    }
    formatted.push_signal(Signal::FinishIndent);

    Ok(formatted)
}

fn gen_loop_statement(statement: &ast::LoopStatement) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(statement.syntax().children_with_tokens());
    parse_token(&mut syntax, SyntaxKind::Loop)?;
    let comments_after_loop = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_body = parse_node::<CompoundStatement>(&mut syntax)?;
    parse_end(&mut syntax);

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    formatted.push_sc(sc!("loop"));
    formatted.extend(gen_comments(comments_after_loop));
    formatted.expect_single_space();
    formatted.extend(gen_compound_statement(&item_body)?);
    formatted.expect_line_break();

    Ok(formatted)
}

fn gen_continuing_statement(
    statement: &ast::ContinuingStatement
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(statement.syntax().children_with_tokens());
    parse_token(&mut syntax, SyntaxKind::Continuing)?;
    let comments_after_continuing = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_body = parse_node::<CompoundStatement>(&mut syntax)?;
    parse_end(&mut syntax);

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    formatted.push_sc(sc!("continuing"));
    formatted.extend(gen_comments(comments_after_continuing));
    formatted.expect_single_space();
    formatted.extend(gen_compound_statement(&item_body)?);
    formatted.expect_line_break();

    Ok(formatted)
}

fn gen_while_statement(statement: &ast::WhileStatement) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(statement.syntax().children_with_tokens());
    parse_token(&mut syntax, SyntaxKind::While)?;
    let comments_after_while = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_condition = parse_node::<Expression>(&mut syntax)?;
    let comments_after_condition = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_body = parse_node::<CompoundStatement>(&mut syntax)?;
    parse_end(&mut syntax);

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    formatted.push_sc(sc!("while"));
    formatted.extend(gen_comments(comments_after_while));
    formatted.expect_single_space(); // Request space, because we trim parentheses
    formatted.extend(gen_expression(&item_condition, true)?);
    formatted.expect_single_space();
    formatted.extend(gen_comments(comments_after_condition));
    formatted.extend(gen_compound_statement(&item_body)?);
    formatted.expect_line_break();

    Ok(formatted)
}

fn gen_const_declaration_statement(
    statement: &ast::ConstantDeclaration,
    include_semicolon: bool,
) -> FormatDocumentResult<PrintItemBuffer> {
    //
    // NOTE!! - When changing this function, make sure to also update gen_var_declaration_statement, gen_let_declaration_statemetn.
    // This is non-dry code, but when inevitably at some point there will be some differences between
    // let and var, this should clearly communicate that they should be split up and not
    // continue to be one function with a whole lot of parameters and ifs.
    //

    // ==== Parse ====
    let mut syntax = put_back(statement.syntax().children_with_tokens());
    parse_token(&mut syntax, SyntaxKind::Constant)?;
    let item_comments_after_let = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_name = parse_node::<ast::Name>(&mut syntax)?;
    let item_comments_after_name = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, SyntaxKind::Equal)?;
    let item_comments_after_equal = parse_many_comments_and_blankspace(&mut syntax)?;

    let value = parse_node::<ast::Expression>(&mut syntax)?;
    let item_comments_after_value = parse_many_comments_and_blankspace(&mut syntax)?;

    parse_token_optional(&mut syntax, SyntaxKind::Semicolon); //Not all var-statements have a semicolon (e.g for loop)
    parse_end(&mut syntax);

    // ==== Format ====
    let mut pi = PrintItems::new();
    pi.push_info(ColumnNumber::new("start_expr"));

    let mut formatted = PrintItemBuffer::new();
    formatted.push_sc(sc!("const"));
    formatted.push_signal(Signal::StartIndent);
    formatted.expect_single_space();
    formatted.extend(gen_comments(item_comments_after_let));
    formatted.push_string(item_name.text().to_string());
    formatted.extend(gen_comments(item_comments_after_name));
    formatted.expect_single_space();
    formatted.push_sc(sc!("="));
    formatted.expect_single_space();
    formatted.extend(gen_comments(item_comments_after_equal));
    formatted.extend(gen_expression(&value, false)?);
    formatted.extend(gen_comments(item_comments_after_value));
    formatted.request_space(SeparationPolicy::Discouraged);
    if include_semicolon {
        formatted.push_sc(sc!(";"));
    }
    formatted.push_signal(Signal::FinishIndent);

    Ok(formatted)
}

fn gen_let_declaration_statement(
    statement: &ast::LetDeclaration,
    include_semicolon: bool,
) -> FormatDocumentResult<PrintItemBuffer> {
    //
    // NOTE!! - When changing this function, make sure to also update gen_var_declaration_statement, gen_const_declaration_statement.
    // This is non-dry code, but when inevitably at some point there will be some differences between
    // let and var, this should clearly communicate that they should be split up and not
    // continue to be one function with a whole lot of parameters and ifs.
    //

    // ==== Parse ====
    let mut syntax = put_back(statement.syntax().children_with_tokens());
    parse_token(&mut syntax, SyntaxKind::Let)?;
    let item_comments_after_let = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_name = parse_node::<ast::Name>(&mut syntax)?;
    let item_comments_after_name = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, SyntaxKind::Equal)?;
    let item_comments_after_equal = parse_many_comments_and_blankspace(&mut syntax)?;

    let value = parse_node::<ast::Expression>(&mut syntax)?;
    let item_comments_after_value = parse_many_comments_and_blankspace(&mut syntax)?;

    parse_token_optional(&mut syntax, SyntaxKind::Semicolon); //Not all var-statements have a semicolon (e.g for loop)
    parse_end(&mut syntax);

    // ==== Format ====
    let mut pi = PrintItems::new();
    pi.push_info(ColumnNumber::new("start_expr"));

    let mut formatted = PrintItemBuffer::new();
    formatted.push_sc(sc!("let"));
    formatted.push_signal(Signal::StartIndent);
    formatted.expect_single_space();
    formatted.extend(gen_comments(item_comments_after_let));
    formatted.push_string(item_name.text().to_string());
    formatted.extend(gen_comments(item_comments_after_name));
    formatted.expect_single_space();
    formatted.push_sc(sc!("="));
    formatted.expect_single_space();
    formatted.extend(gen_comments(item_comments_after_equal));
    formatted.extend(gen_expression(&value, false)?);
    formatted.extend(gen_comments(item_comments_after_value));
    formatted.request_space(SeparationPolicy::Discouraged);
    if include_semicolon {
        formatted.push_sc(sc!(";"));
    }
    formatted.push_signal(Signal::FinishIndent);

    Ok(formatted)
}

fn gen_var_declaration_statement(
    statement: &ast::VariableDeclaration,
    include_semicolon: bool,
) -> FormatDocumentResult<PrintItemBuffer> {
    //
    // NOTE!! - When changing this function, make sure to also update gen_let_declaration_statement, gen_const_declaration_statement.
    // This is non-dry code, but when inevitably at some point there will be some differences between
    // let and var, this should clearly communicate that they should be split up and not
    // continue to be one function with a whole lot of parameters and ifs.
    //

    // ==== Parse ====
    let mut syntax = put_back(statement.syntax().children_with_tokens());
    parse_token(&mut syntax, SyntaxKind::Var)?;
    let item_comments_after_let = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_name = parse_node::<ast::Name>(&mut syntax)?;
    let item_comments_after_name = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, SyntaxKind::Equal)?;
    let item_comments_after_equal = parse_many_comments_and_blankspace(&mut syntax)?;

    let value = parse_node::<ast::Expression>(&mut syntax)?;
    let item_comments_after_value = parse_many_comments_and_blankspace(&mut syntax)?;

    parse_token_optional(&mut syntax, SyntaxKind::Semicolon); //Not all var-statements have a semicolon (e.g for loop)
    parse_end(&mut syntax);

    // ==== Format ====
    let mut pi = PrintItems::new();
    pi.push_info(ColumnNumber::new("start_expr"));

    let mut formatted = PrintItemBuffer::new();
    formatted.push_sc(sc!("var"));
    formatted.push_signal(Signal::StartIndent);
    formatted.expect_single_space();
    formatted.extend(gen_comments(item_comments_after_let));
    formatted.push_string(item_name.text().to_string());
    formatted.extend(gen_comments(item_comments_after_name));
    formatted.expect_single_space();
    formatted.push_sc(sc!("="));
    formatted.expect_single_space();
    formatted.extend(gen_comments(item_comments_after_equal));
    formatted.extend(gen_expression(&value, false)?);
    formatted.extend(gen_comments(item_comments_after_value));
    formatted.request_space(SeparationPolicy::Discouraged);
    if include_semicolon {
        formatted.push_sc(sc!(";"));
    }
    formatted.push_signal(Signal::FinishIndent);

    Ok(formatted)
}
