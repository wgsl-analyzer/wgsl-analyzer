use dprint_core::formatting::{ColumnNumber, PrintItems, Signal};
use dprint_core_macros::sc;
use itertools::put_back;
use parser::{SyntaxKind, SyntaxToken};
use rowan::NodeOrToken;
use syntax::{
    AstNode,
    ast::{self, Literal},
};

use crate::format::{
    ast_parse::{parse_end, parse_many_comments_and_blankspace, parse_node, parse_token},
    gen_comments::{gen_comment, gen_comments},
    gen_expression::gen_expression,
    helpers::{gen_spaced_lines, todo_verbatim},
    print_item_buffer::{PrintItemBuffer, SeparationPolicy, SeparationRequest},
    reporting::{FormatDocumentError, FormatDocumentErrorKind, FormatDocumentResult, err_src},
};

#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "It will match future variants, and that's intentional"
)]
pub fn gen_compound_statement(
    syntax: &ast::CompoundStatement
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    //let mut syntax = put_back(node.syntax().children_with_tokens());

    //TODO I don't really like this, but its an easy way for now
    let body_empty = syntax.syntax().children_with_tokens().all(|child| {
        matches!(
            child.kind(),
            SyntaxKind::BraceLeft | SyntaxKind::BraceRight | SyntaxKind::Blankspace
        )
    });

    let lines = gen_spaced_lines(syntax.syntax(), |child| {
        if let NodeOrToken::Node(child) = child {
            dbg!(&child);
            dbg!(ast::Discard::cast(child.clone()));
            dbg!(ast::Statement::cast(child.clone()));
        }
        //TODO This clone is unnecessary if we had a cast that returned the passed in node
        // on a failure like std::any::Any (SyntaxNode -> Result<Item, Syntaxnode>)
        if let NodeOrToken::Node(child) = child
            && let Some(statement) = ast::Statement::cast(child.clone())
        {
            gen_statement(&statement)
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
            todo!();
            Err(FormatDocumentErrorKind::UnexpectedToken {
                received: child.clone(),
            }
            .at(child.text_range(), err_src!()))
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
    match item {
        ast::Statement::IfStatement(if_statement) => todo_verbatim(if_statement.syntax()),
        ast::Statement::SwitchStatement(switch_statement) => {
            todo_verbatim(switch_statement.syntax())
        },
        ast::Statement::LoopStatement(loop_statement) => todo_verbatim(loop_statement.syntax()),
        ast::Statement::ForStatement(for_statement) => todo_verbatim(for_statement.syntax()),
        ast::Statement::WhileStatement(while_statement) => todo_verbatim(while_statement.syntax()),
        ast::Statement::CompoundStatement(compound_statement) => {
            todo_verbatim(compound_statement.syntax())
        },
        ast::Statement::FunctionCallStatement(function_call_statement) => {
            todo_verbatim(function_call_statement.syntax())
        },
        ast::Statement::VariableDeclaration(variable_declaration) => {
            todo_verbatim(variable_declaration.syntax())
        },
        ast::Statement::LetDeclaration(let_declaration) => {
            gen_let_declaration_statement(let_declaration)
        },
        ast::Statement::ConstantDeclaration(constant_declaration) => {
            todo_verbatim(constant_declaration.syntax())
        },
        ast::Statement::AssignmentStatement(assignment_statement) => {
            todo_verbatim(assignment_statement.syntax())
        },
        ast::Statement::CompoundAssignmentStatement(compound_assignment_statement) => {
            todo_verbatim(compound_assignment_statement.syntax())
        },
        ast::Statement::IncrementDecrementStatement(increment_decrement_statement) => {
            todo_verbatim(increment_decrement_statement.syntax())
        },
        ast::Statement::ReturnStatement(return_statement) => {
            todo_verbatim(return_statement.syntax())
        },
        ast::Statement::ContinuingStatement(continuing_statement) => {
            todo_verbatim(continuing_statement.syntax())
        },

        // == Keywords ==
        ast::Statement::BreakStatement(break_statement) => {
            // ==== Parse ====
            // We still parse through the discard syntax even tho there is no information for
            // the formatter to get out of it. This exists to ensure we don't accidentally delete
            // user's code should future changes to wgsl allow more complex break statements.
            let mut syntax = put_back(break_statement.syntax().children_with_tokens());
            parse_token(&mut syntax, SyntaxKind::Break)?;
            parse_end(&mut syntax);

            // ==== Format ====
            let mut formatted = PrintItemBuffer::new();
            formatted.push_sc(sc!("break;"));
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
            parse_end(&mut syntax);

            // ==== Format ====
            let mut formatted = PrintItemBuffer::new();
            formatted.push_sc(sc!("continue;"));
            formatted.expect_line_break();
            Ok(formatted)
        },
        ast::Statement::DiscardStatement(discard) => {
            // ==== Parse ====
            // We still parse through the discard syntax even tho there is no information for
            // the formatter to get out of it. This exists to ensure we don't accidentally delete
            // user's code should future changes to wgsl allow more complex discard statements.
            let mut syntax = put_back(discard.syntax().children_with_tokens());
            parse_token(&mut syntax, SyntaxKind::Discard)?;
            parse_end(&mut syntax);

            // ==== Format ====
            let mut formatted = PrintItemBuffer::new();
            formatted.push_sc(sc!("discard;"));
            formatted.expect_line_break();
            Ok(formatted)
        },
    }
}

fn gen_let_declaration_statement(
    statement: &ast::LetDeclaration
) -> FormatDocumentResult<PrintItemBuffer> {
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

    parse_token(&mut syntax, SyntaxKind::Semicolon)?;
    parse_end(&mut syntax);

    let mut pi = PrintItems::new();
    pi.push_info(ColumnNumber::new("start_expr"));

    let mut formatted = PrintItemBuffer::new();
    // There are no circumstances where a let statement would not be the first item on a line.
    formatted.expect_line_break();
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
    formatted.extend(gen_expression(&value)?);
    formatted.extend(gen_comments(item_comments_after_value));
    formatted.push_sc(sc!(";"));
    formatted.push_signal(Signal::FinishIndent);

    Ok(formatted)
}
