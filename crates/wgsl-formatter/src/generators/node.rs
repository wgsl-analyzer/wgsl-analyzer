//! Formatting the trivia of a [`NodeWithTrivia`] and dispatching to the correct generator for its content.
//!
//! The entry point to formatting [`NodeWithTrivia`]s is [`gen_node_with_trivia`].
use dprint_core::formatting::Arena;
use dprint_core_macros::sc;
use rowan::NodeOrToken;
use syntax::{AstNode as _, SyntaxKind, SyntaxNode, SyntaxToken, ast};

use crate::{
    blankspace::{gen_blankspace, read_blankspace},
    generators::{
        attributes::{gen_attribute, gen_attribute_arguments, gen_attribute_list},
        comments::{gen_comment, read_comment},
        diagnostic_directive::{
            gen_diagnostic_control, gen_diagnostic_rule_name, gen_severity_control_name,
        },
        directives::{
            gen_diagnostic_directive, gen_enable_directive, gen_enable_extension_name,
            gen_language_extension_name, gen_requires_directive,
        },
        expressions::{
            field_expression::gen_field_expression, ident_expression::gen_ident_expression,
            index_expression::gen_index_expression, infix_expression::gen_infix_expression,
            literal_expression::gen_literal_expression,
            parenthesis_expression::gen_parenthesis_expression,
            prefix_expression::gen_prefix_expression,
        },
        function_declaration::{
            gen_fn_parameter, gen_fn_parameters, gen_fn_return_type, gen_function_declaration,
        },
        global_compound_declaration::gen_global_compound_declaration,
        name::gen_name,
        path::gen_path,
        source_file::gen_source_file,
        statements::{
            assignment_statement::{
                gen_assignment_statement, gen_compound_assignment_statement,
                gen_phony_assignment_statement,
            },
            break_if_statement::gen_break_if_statement,
            break_statement::gen_break_statement,
            compound_statement::gen_compound_statement,
            const_assert_statement::gen_const_assert_statement,
            continue_statement::gen_continue_statement,
            continuing_statement::gen_continuing_statement,
            discard_statement::gen_discard_statement,
            for_statement::{
                gen_for_statement, gen_for_statement_condition, gen_for_statement_continuing_part,
                gen_for_statement_initializer,
            },
            function_call_statement::{
                gen_function_call, gen_function_call_arguments, gen_function_call_statement,
            },
            if_statement::{
                gen_if_statement, gen_if_statement_else_clause, gen_if_statement_else_if_clause,
                gen_if_statement_if_clause,
            },
            import_statement::{
                gen_import_collection, gen_import_item, gen_import_package_relative,
                gen_import_path, gen_import_statement, gen_import_super_relative,
            },
            increment_decrement_statement::gen_increment_decrement_statement,
            loop_statement::gen_loop_statement,
            return_statement::gen_return_statement,
            switch_statement::{
                gen_switch_body, gen_switch_body_case, gen_switch_case_default_selector,
                gen_switch_case_selectors, gen_switch_statement,
            },
            var_let_const_override_statement::{
                gen_const_declaration_statement, gen_let_declaration_statement,
                gen_override_declaration_statement, gen_var_declaration_statement,
            },
            while_statement::gen_while_statement,
        },
        struct_declaration::{gen_struct_body, gen_struct_declaration, gen_struct_member},
        type_alias_declaration::gen_type_alias_declaration,
        types::{gen_template_list, gen_type_specifier},
        verbatim::gen_node_syntax_verbatim,
    },
    ignore::gen_ignore_pragma,
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem},
    },
    reporting::{FormatDocumentError, FormatDocumentResult},
    trivia::{NodeTriviaItem, NodeWithTrivia, NodeWithTriviaContent},
};

macro_rules! with_sc {
    ($arena:ident, $text:expr) => {{
        let mut buffer = PrintItemBuffer::new($arena);
        buffer.push_sc($arena, sc!($text));
        Ok(buffer)
    }};
}

macro_rules! with_cast {
    ($generator:ident, $ast_type:ty, $arena:ident, $node:ident) => {
        $generator(
            $arena,
            &$node
                .clone()
                .into_node()
                .and_then(<$ast_type>::cast)
                .expect("We just matched on the SyntaxKind"),
        )
    };
}

macro_rules! with_node {
    ($generator:ident, $arena:ident, $node:ident) => {
        $generator(
            $arena,
            &($node
                .as_node()
                .expect("We just matched on the SyntaxKind")
                .clone()),
        )
    };
}

#[expect(
    clippy::too_many_lines,
    reason = "It does not make sense to split this up"
)]
#[rustfmt::skip]
fn gen_node<'buffer>(
    arena: &'buffer Arena,
    with_trivia: &NodeWithTrivia,
    node: &NodeOrToken<SyntaxNode, SyntaxToken>
) -> FormatDocumentResult<PrintItemBuffer<'buffer>> {
    match node.kind() {
        SyntaxKind::SourceFile => with_cast!(gen_source_file, ast::SourceFile, arena, node),
        SyntaxKind::FunctionDeclaration => with_cast!(gen_function_declaration, ast::FunctionDeclaration, arena, node),
        SyntaxKind::TemplateList => with_cast!(gen_template_list, ast::TemplateList, arena, node),
        SyntaxKind::FunctionParameters => with_cast!(gen_fn_parameters, ast::FunctionParameters, arena, node),
        SyntaxKind::Parameter => with_cast!(gen_fn_parameter, ast::Parameter, arena, node),
        SyntaxKind::ReturnType => with_cast!(gen_fn_return_type, ast::ReturnType, arena, node),
        SyntaxKind::AssertStatement => with_cast!(gen_const_assert_statement, ast::AssertStatement, arena, node),
        SyntaxKind::CompoundStatement => gen_compound_statement(arena,
            with_trivia,
            &node.clone().into_node().and_then(<ast::CompoundStatement>::cast).expect("We just matched on the SyntaxKind"),
        ),
        SyntaxKind::GlobalCompoundDeclaration => gen_global_compound_declaration(arena,
            with_trivia,
            &node.clone().into_node().and_then(<ast::GlobalCompoundDeclaration>::cast).expect("We just matched on the SyntaxKind"),
        ),
        SyntaxKind::AssignmentStatement => with_cast!(gen_assignment_statement, ast::AssignmentStatement, arena, node),
        SyntaxKind::PhonyAssignmentStatement => with_cast!(gen_phony_assignment_statement, ast::PhonyAssignmentStatement, arena, node),
        SyntaxKind::CompoundAssignmentStatement => with_cast!(gen_compound_assignment_statement, ast::CompoundAssignmentStatement, arena, node),
        SyntaxKind::FunctionCallStatement => with_cast!(gen_function_call_statement, ast::FunctionCallStatement, arena, node),
        SyntaxKind::BreakIfStatement => with_cast!(gen_break_if_statement, ast::BreakIfStatement, arena, node),
        SyntaxKind::LoopStatement => with_cast!(gen_loop_statement, ast::LoopStatement, arena, node),
        SyntaxKind::WhileStatement => with_cast!(gen_while_statement, ast::WhileStatement, arena, node),
        SyntaxKind::IfStatement => with_cast!(gen_if_statement, ast::IfStatement, arena, node),
        SyntaxKind::SwitchStatement => with_cast!(gen_switch_statement, ast::SwitchStatement, arena, node),
        SyntaxKind::SwitchBody => with_cast!(gen_switch_body, ast::SwitchBody, arena, node),
        SyntaxKind::SwitchBodyCase => with_cast!(gen_switch_body_case, ast::SwitchBodyCase, arena, node),
        SyntaxKind::SwitchCaseSelectors => with_cast!(gen_switch_case_selectors, ast::SwitchCaseSelectors, arena, node),
        SyntaxKind::SwitchDefaultSelector => with_cast!(gen_switch_case_default_selector, ast::SwitchDefaultSelector, arena, node),
        SyntaxKind::IncrementDecrementStatement => with_cast!(gen_increment_decrement_statement, ast::IncrementDecrementStatement, arena, node),
        SyntaxKind::IfClause => with_cast!(gen_if_statement_if_clause, ast::IfClause, arena, node),
        SyntaxKind::ElseIfClause => with_cast!(gen_if_statement_else_if_clause, ast::ElseIfClause, arena, node),
        SyntaxKind::ElseClause => with_cast!(gen_if_statement_else_clause, ast::ElseClause, arena, node),
        SyntaxKind::ForStatement => with_cast!(gen_for_statement, ast::ForStatement, arena, node),
        SyntaxKind::FieldExpression => with_cast!(gen_field_expression, ast::FieldExpression, arena, node),
        SyntaxKind::FunctionCall => with_cast!(gen_function_call, ast::FunctionCall, arena, node),
        SyntaxKind::Arguments => with_cast!(gen_function_call_arguments, ast::Arguments, arena, node),
        SyntaxKind::IdentExpression => with_cast!(gen_ident_expression, ast::IdentExpression, arena, node),
        SyntaxKind::Path => with_cast!(gen_path, ast::Path, arena, node),
        SyntaxKind::IndexExpression => with_cast!(gen_index_expression, ast::IndexExpression, arena, node),
        SyntaxKind::ReturnStatement => with_cast!(gen_return_statement, ast::ReturnStatement, arena, node),
        SyntaxKind::InfixExpression => with_cast!(gen_infix_expression, ast::InfixExpression, arena, node),
        SyntaxKind::PrefixExpression => with_cast!(gen_prefix_expression, ast::PrefixExpression, arena, node),
        SyntaxKind::Literal => with_cast!(gen_literal_expression, ast::Literal, arena, node),
        SyntaxKind::ParenthesisExpression => with_cast!(gen_parenthesis_expression, ast::ParenthesisExpression, arena, node),
        SyntaxKind::TypeSpecifier => with_cast!(gen_type_specifier, ast::TypeSpecifier, arena, node),
        SyntaxKind::Attribute => with_cast!(gen_attribute, ast::Attribute, arena, node),
        SyntaxKind::AttributeArguments => with_cast!(gen_attribute_arguments, ast::AttributeArguments, arena, node),
        SyntaxKind::StructDeclaration => with_cast!(gen_struct_declaration, ast::StructDeclaration, arena, node),
        SyntaxKind::StructBody => with_cast!(gen_struct_body, ast::StructBody, arena, node),
        SyntaxKind::StructMember => with_cast!(gen_struct_member, ast::StructMember, arena, node),
        SyntaxKind::ConstantDeclaration => with_cast!(gen_const_declaration_statement, ast::ConstantDeclaration, arena, node),
        SyntaxKind::VariableDeclaration => with_cast!(gen_var_declaration_statement, ast::VariableDeclaration, arena, node),
        SyntaxKind::LetDeclaration => with_cast!(gen_let_declaration_statement, ast::LetDeclaration, arena, node),
        SyntaxKind::OverrideDeclaration => with_cast!(gen_override_declaration_statement, ast::OverrideDeclaration, arena, node),
        SyntaxKind::ContinuingStatement => with_cast!(gen_continuing_statement, ast::ContinuingStatement, arena, node),
        SyntaxKind::TypeAliasDeclaration => with_cast!(gen_type_alias_declaration, ast::TypeAliasDeclaration, arena, node),
        SyntaxKind::EnableDirective => with_cast!(gen_enable_directive, ast::EnableDirective, arena, node),
        SyntaxKind::EnableExtensionName => with_cast!(gen_enable_extension_name, ast::EnableExtensionName, arena, node),
        SyntaxKind::RequiresDirective => with_cast!(gen_requires_directive, ast::RequiresDirective, arena, node),
        SyntaxKind::LanguageExtensionName => with_cast!(gen_language_extension_name, ast::LanguageExtensionName, arena, node),
        SyntaxKind::ImportStatement => with_cast!(gen_import_statement, ast::ImportStatement, arena, node),
        SyntaxKind::DiagnosticControl => with_cast!(gen_diagnostic_control, ast::DiagnosticControl, arena, node),
        SyntaxKind::DiagnosticDirective => with_cast!(gen_diagnostic_directive, ast::DiagnosticDirective, arena, node),
        SyntaxKind::DiagnosticRuleName => with_cast!(gen_diagnostic_rule_name, ast::DiagnosticRuleName, arena, node),
        SyntaxKind::SeverityControlName => with_cast!(gen_severity_control_name, ast::SeverityControlName, arena, node),
        SyntaxKind::ImportPackageRelative => with_cast!(gen_import_package_relative, ast::ImportPackageRelative, arena, node),
        SyntaxKind::ImportSuperRelative => with_cast!(gen_import_super_relative, ast::ImportSuperRelative, arena, node),
        SyntaxKind::ImportItem => with_cast!(gen_import_item, ast::ImportItem, arena, node),
        SyntaxKind::ImportPath => with_cast!(gen_import_path, ast::ImportPath, arena, node),
        SyntaxKind::ImportCollection => with_cast!(gen_import_collection, ast::ImportCollection, arena, node),
        SyntaxKind::Name => with_cast!(gen_name, ast::Name, arena, node),
        SyntaxKind::AttributeList => with_cast!(gen_attribute_list, ast::AttributeList, arena, node),
        SyntaxKind::BreakStatement => with_cast!(gen_break_statement, ast::BreakStatement, arena, node),
        SyntaxKind::ContinueStatement => with_cast!(gen_continue_statement, ast::ContinueStatement, arena, node),
        SyntaxKind::DiscardStatement => with_cast!(gen_discard_statement, ast::DiscardStatement, arena, node),
        SyntaxKind::ForInitializer => with_node!(gen_for_statement_initializer, arena, node),
        SyntaxKind::ForCondition => with_node!(gen_for_statement_condition, arena, node),
        SyntaxKind::ForContinuingPart => with_node!(gen_for_statement_continuing_part, arena, node),

        SyntaxKind::EmptyStatement => {
            // We remove lonely semicolons
            Ok(PrintItemBuffer::new(arena))
        },

        SyntaxKind::Alias => with_sc!(arena, "alias"),
        SyntaxKind::Break => with_sc!(arena, "break"),
        SyntaxKind::Case => with_sc!(arena, "case"),
        SyntaxKind::Const => with_sc!(arena, "const"),
        SyntaxKind::ConstantAssert => with_sc!(arena, "assert"),
        SyntaxKind::Continue => with_sc!(arena, "continue"),
        SyntaxKind::Continuing => with_sc!(arena, "continuing"),
        SyntaxKind::Default => with_sc!(arena, "default"),
        SyntaxKind::Diagnostic => with_sc!(arena, "diagnostic"),
        SyntaxKind::Discard => with_sc!(arena, "discard"),
        SyntaxKind::Else => with_sc!(arena, "else"),
        SyntaxKind::Enable => with_sc!(arena, "enable"),
        SyntaxKind::False => with_sc!(arena, "false"),
        SyntaxKind::Fn => with_sc!(arena, "fn"),
        SyntaxKind::For => with_sc!(arena, "for"),
        SyntaxKind::If => with_sc!(arena, "if"),
        SyntaxKind::Let => with_sc!(arena, "let"),
        SyntaxKind::Loop => with_sc!(arena, "loop"),
        SyntaxKind::Override => with_sc!(arena, "override"),
        SyntaxKind::Requires => with_sc!(arena, "requires"),
        SyntaxKind::Return => with_sc!(arena, "return"),
        SyntaxKind::Struct => with_sc!(arena, "struct"),
        SyntaxKind::Switch => with_sc!(arena, "switch"),
        SyntaxKind::True => with_sc!(arena, "true"),
        SyntaxKind::Var => with_sc!(arena, "var"),
        SyntaxKind::While => with_sc!(arena, "while"),
        SyntaxKind::And => with_sc!(arena, "&"),
        SyntaxKind::AndAnd => with_sc!(arena, "&&"),
        SyntaxKind::Arrow => with_sc!(arena, "=>"),
        SyntaxKind::AttributeOperator => with_sc!(arena, "@"),
        SyntaxKind::ForwardSlash => with_sc!(arena, "/"),
        SyntaxKind::Bang => with_sc!(arena, "!"),
        SyntaxKind::BracketLeft => with_sc!(arena, "["),
        SyntaxKind::BracketRight => with_sc!(arena, "]"),
        SyntaxKind::BraceLeft => with_sc!(arena, "{"),
        SyntaxKind::BraceRight => with_sc!(arena, "}"),
        SyntaxKind::Colon => with_sc!(arena, ":"),
        SyntaxKind::ColonColon => with_sc!(arena, "::"),
        SyntaxKind::Comma => with_sc!(arena, ","),
        SyntaxKind::Equal => with_sc!(arena, "="),
        SyntaxKind::EqualEqual => with_sc!(arena, "=="),
        SyntaxKind::NotEqual => with_sc!(arena, "!="),
        SyntaxKind::GreaterThan |
        SyntaxKind::TemplateEnd => with_sc!(arena, ">"),
        SyntaxKind::GreaterThanEqual => with_sc!(arena, ">="),
        SyntaxKind::LessThanEqual => with_sc!(arena, "<="),
        SyntaxKind::LessThan |
        SyntaxKind::TemplateStart => with_sc!(arena, "<"),
        SyntaxKind::Modulo => with_sc!(arena, "%"),
        SyntaxKind::Minus => with_sc!(arena, "-"),
        SyntaxKind::MinusMinus => with_sc!(arena, "--"),
        SyntaxKind::Period => with_sc!(arena, "."),
        SyntaxKind::Plus => with_sc!(arena, "+"),
        SyntaxKind::PlusPlus => with_sc!(arena, "++"),
        SyntaxKind::Or => with_sc!(arena, "|"),
        SyntaxKind::OrOr => with_sc!(arena, "||"),
        SyntaxKind::ParenthesisLeft => with_sc!(arena, "("),
        SyntaxKind::ParenthesisRight => with_sc!(arena, ")"),
        SyntaxKind::Semicolon => with_sc!(arena, ";"),
        SyntaxKind::Star => with_sc!(arena, "*"),
        SyntaxKind::Tilde => with_sc!(arena, "~"),
        SyntaxKind::Underscore => with_sc!(arena, "_"),
        SyntaxKind::Xor => with_sc!(arena, "^"),
        SyntaxKind::Import => with_sc!(arena, "import"),
        SyntaxKind::Package => with_sc!(arena, "package"),
        SyntaxKind::Super => with_sc!(arena, "super"),
        SyntaxKind::As => with_sc!(arena, "as"),
        SyntaxKind::PlusEqual => with_sc!(arena, "+="),
        SyntaxKind::MinusEqual => with_sc!(arena, "-="),
        SyntaxKind::TimesEqual => with_sc!(arena, "*="),
        SyntaxKind::DivisionEqual => with_sc!(arena, "/="),
        SyntaxKind::ModuloEqual => with_sc!(arena, "%="),
        SyntaxKind::AndEqual => with_sc!(arena, "&="),
        SyntaxKind::OrEqual => with_sc!(arena, "|="),
        SyntaxKind::XorEqual => with_sc!(arena, "^="),
        SyntaxKind::ShiftRightEqual => with_sc!(arena, ">>="),
        SyntaxKind::ShiftLeftEqual => with_sc!(arena, "<<="),
        SyntaxKind::ShiftLeft => with_sc!(arena, "<<"),
        SyntaxKind::ShiftRight => with_sc!(arena, ">>"),

        SyntaxKind::LineEndingComment |
        SyntaxKind::BlockComment => {
            let comment = read_comment(node).expect("We just matched on the SyntaxKind");
            Ok(gen_comment(arena,&comment))
        },
        SyntaxKind::Blankspace => {
            let blankspace = read_blankspace(node).expect("We just matched on the SyntaxKind");
            gen_blankspace(arena,&blankspace)
        },
        SyntaxKind::Identifier
        | SyntaxKind::FloatLiteral
        | SyntaxKind::IntLiteral
        | SyntaxKind::StringLiteral => {
            let mut formatted = PrintItemBuffer::new(arena);
            formatted.push_string(arena,
                node
                    .as_token()
                    .ok_or_else(|| FormatDocumentError::UnexpectedNodeOrToken { received: Some(node.clone()) })?
                    .text()
                    .to_owned()
            );
            Ok(formatted)
        },


        SyntaxKind::Error |
        SyntaxKind::Reserved => {
            // We don't won't format a source that contains errors, or reserved keywords
            Err(FormatDocumentError::UnsupportedNodeOrToken  { received: node.clone() })
        },

        _ => {
            Err(FormatDocumentError::UnsupportedNodeOrToken  { received: node.clone() })
        },
    }
}

/// Generate only the preceding trivia of a [`NodeWithTrivia`].
#[inline]
pub(crate) fn gen_node_preceding_trivia<'buffer>(
    arena: &'buffer Arena,
    node: &NodeWithTrivia,
) -> FormatDocumentResult<PrintItemBuffer<'buffer>> {
    gen_node_trivia(arena, &node.preceding_trivia)
}

/// Generate only the succeeding trivia of a [`NodeWithTrivia`].
#[inline]
pub(crate) fn gen_node_succeeding_trivia<'buffer>(
    arena: &'buffer Arena,
    node: &NodeWithTrivia,
) -> FormatDocumentResult<PrintItemBuffer<'buffer>> {
    gen_node_trivia(arena, &node.succeeding_trivia)
}

/// Generate only the content of a [`NodeWithTrivia`].
#[inline]
pub(crate) fn gen_node_content<'buffer>(
    arena: &'buffer Arena,
    node: &NodeWithTrivia,
) -> FormatDocumentResult<PrintItemBuffer<'buffer>> {
    let mut formatted = PrintItemBuffer::new(arena);

    match &node.content {
        NodeWithTriviaContent::NoContent | NodeWithTriviaContent::End => {},
        NodeWithTriviaContent::Content(content) => {
            formatted.extend(arena, gen_node(arena, node, content)?);
        },
        NodeWithTriviaContent::IgnoredContent {
            ignore_pragma,
            ignored_preceding_trivia,
            ignored_content,
        } => {
            if let Some(ignore_pragma) = ignore_pragma {
                formatted.extend(arena, gen_ignore_pragma(arena, ignore_pragma));
            }
            for content in ignored_preceding_trivia {
                formatted.extend(arena, gen_node_syntax_verbatim(arena, &content.syntax())?);
            }
            if let Some(content) = ignored_content.as_content() {
                formatted.extend(arena, gen_node_syntax_verbatim(arena, content)?);
            }
        },
    }

    Ok(formatted)
}

/// Format [`NodeTriviaItem`]s.
#[inline]
pub(crate) fn gen_node_trivia<'buffer>(
    arena: &'buffer Arena,
    trivia: &[NodeTriviaItem],
) -> FormatDocumentResult<PrintItemBuffer<'buffer>> {
    let mut formatted = PrintItemBuffer::new(arena);
    for trivia in trivia {
        match trivia {
            NodeTriviaItem::LineSpacing(content) => {
                formatted.extend(arena, gen_blankspace(arena, content)?);
            },
            NodeTriviaItem::Comment(content) => {
                formatted.extend(arena, gen_comment(arena, content));
            },
            NodeTriviaItem::NewlinedComment(content) => {
                formatted.extend(arena, gen_comment(arena, content));
                formatted.request(Request::expect(RequestItem::LineBreak));
            },
            NodeTriviaItem::AttributeList(content) => {
                formatted.extend(arena, gen_attribute_list(arena, content)?);
            },
            NodeTriviaItem::Discarded(_) => {},
        }
    }

    Ok(formatted)
}

/// Generate a whole [`NodeWithTrivia`] including all trivia and content.
///
/// This respects ignore-pragmas by using [`NodeWithTrivia::format`], and will
/// output the node with all of its trivia verbatim, if the node is not supposed
/// to be formatted.
///
/// Depending on the `SyntaxKind` of the content, this delegates the actual formatting to
/// one of the many [`generators`](crate::generators).
pub(crate) fn gen_node_with_trivia<'buffer>(
    arena: &'buffer Arena,
    node: &NodeWithTrivia,
) -> FormatDocumentResult<PrintItemBuffer<'buffer>> {
    let mut formatted = PrintItemBuffer::new(arena);

    formatted.extend(arena, gen_node_preceding_trivia(arena, node)?);
    formatted.extend(arena, gen_node_content(arena, node)?);
    formatted.extend(arena, gen_node_succeeding_trivia(arena, node)?);

    Ok(formatted)
}

/// Like [`gen_node_with_trivia`] but strips surrounding newlines.
pub(crate) fn gen_node_with_trivia_no_newlines<'buffer>(
    arena: &'buffer Arena,
    trivia: &NodeWithTrivia,
) -> FormatDocumentResult<PrintItemBuffer<'buffer>> {
    let mut formatted = PrintItemBuffer::new(arena);
    formatted.request(Request::discourage(RequestItem::EmptyLine));
    formatted.extend(arena, gen_node_with_trivia(arena, trivia)?);
    formatted.request(Request::discourage(RequestItem::LineBreak));
    Ok(formatted)
}
