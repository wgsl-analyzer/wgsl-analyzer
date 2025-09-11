use dprint_core::formatting::Signal;
use dprint_core_macros::sc;
use parser::{SyntaxKind, SyntaxToken};
use rowan::NodeOrToken;
use syntax::{AstNode as _, ast};

use crate::format::{
    ast_parse::parse_token,
    gen_comments::gen_comment,
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
        //TODO This clone is unnecessary if we had a cast that returned the passed in node
        // on a failure like std::any::Any (SyntaxNode -> Result<Item, Syntaxnode>)
        if let NodeOrToken::Node(child) = child
            && let Some(stmt) = ast::Statement::cast(child.clone())
        {
            gen_statement(&stmt)
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
            Err(FormatDocumentErrorKind::UnexpectedModuleNode.at(child.text_range(), err_src!()))
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
        ast::Statement::LetDeclaration(let_declaration) => todo_verbatim(let_declaration.syntax()),
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
        ast::Statement::Break(break_statement) => todo_verbatim(break_statement.syntax()),
        ast::Statement::Continue(continue_statement) => todo_verbatim(continue_statement.syntax()),
        ast::Statement::Discard(discard) => todo_verbatim(discard.syntax()),
        ast::Statement::ReturnStatement(return_statement) => {
            todo_verbatim(return_statement.syntax())
        },
        ast::Statement::ContinuingStatement(continuing_statement) => {
            todo_verbatim(continuing_statement.syntax())
        },
    }
}
