use dprint_core_macros::sc;
use syntax::{AstNode as _, SyntaxKind, SyntaxNode, ast};

use crate::{
    FormattingOptions,
    ast_parse::{DiscardBlankspace, NoTrivia, parse_end, parse_node_with, syntax_iter},
    context_policies::statement_needs_semicolon_policy,
    generators::node::gen_node_with_trivia,
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem},
    },
    reporting::FormatDocumentResult,
};

pub(crate) fn gen_const_declaration_statement(
    options: &FormattingOptions,
    statement: &ast::ConstantDeclaration,
) -> FormatDocumentResult<PrintItemBuffer> {
    gen_var_let_const_override_statement(options, BindingKind::Const, statement.syntax())
}

pub(crate) fn gen_let_declaration_statement(
    options: &FormattingOptions,
    statement: &ast::LetDeclaration,
) -> FormatDocumentResult<PrintItemBuffer> {
    gen_var_let_const_override_statement(options, BindingKind::Let, statement.syntax())
}

pub(crate) fn gen_var_declaration_statement(
    options: &FormattingOptions,
    statement: &ast::VariableDeclaration,
) -> FormatDocumentResult<PrintItemBuffer> {
    gen_var_let_const_override_statement(options, BindingKind::Var, statement.syntax())
}

pub(crate) fn gen_override_declaration_statement(
    options: &FormattingOptions,
    statement: &ast::OverrideDeclaration,
) -> FormatDocumentResult<PrintItemBuffer> {
    gen_var_let_const_override_statement(options, BindingKind::Override, statement.syntax())
}

#[derive(Clone, Copy, Debug)]
enum BindingKind {
    Var,
    Let,
    Const,

    // For now we have this here because the override syntax is basically equivalent to a global const.
    // If the override should diverge from that, extract it into its own file instead of branching around.
    Override,
}

impl BindingKind {
    const fn syntax_kind(self) -> SyntaxKind {
        match self {
            Self::Var => SyntaxKind::Var,
            Self::Let => SyntaxKind::Let,
            Self::Const => SyntaxKind::Const,
            Self::Override => SyntaxKind::Override,
        }
    }
}

fn gen_var_let_const_override_statement(
    options: &FormattingOptions,
    kind: BindingKind,
    syntax_node: &SyntaxNode,
) -> FormatDocumentResult<PrintItemBuffer> {
    // Note: When changing this function, should one of the three cases divert from the others more than
    // it already is, consider pulling it into a wholly separate function, instead of expanding this one with ifs

    // ==== Parse ====
    let mut syntax = syntax_iter(syntax_node);

    let item_visibility =
        parse_node_with(&mut syntax, NoTrivia).only_if_kind(SyntaxKind::Visibility, &mut syntax);
    let item_binding_token =
        parse_node_with(&mut syntax, DiscardBlankspace).expect_kind(kind.syntax_kind())?;
    let item_template_list = parse_node_with(&mut syntax, DiscardBlankspace)
        .only_if_kind(SyntaxKind::TemplateList, &mut syntax);
    let item_name =
        parse_node_with(&mut syntax, DiscardBlankspace).expect_kind(SyntaxKind::Name)?;

    let item_colon =
        parse_node_with(&mut syntax, NoTrivia).only_if_kind(SyntaxKind::Colon, &mut syntax);

    let items_type = if item_colon.is_some() {
        let item_type_specifier = parse_node_with(&mut syntax, DiscardBlankspace)
            .expect_kind(SyntaxKind::TypeSpecifier)?;
        Some(item_type_specifier)
    } else {
        None
    };

    let item_equal =
        parse_node_with(&mut syntax, NoTrivia).only_if_kind(SyntaxKind::Equal, &mut syntax);

    let assignment = if item_equal.is_some() {
        let value =
            parse_node_with(&mut syntax, DiscardBlankspace).expect_ast_node::<ast::Expression>()?;
        Some(value)
    } else {
        None
    };

    parse_node_with(&mut syntax, NoTrivia).expect_kind_optional(SyntaxKind::Semicolon)?; //Not all var-statements have a semicolon (for example, `for` loop)
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    if let Some(item_visibility) = item_visibility {
        formatted.extend(gen_node_with_trivia(options, &item_visibility)?);
        formatted.request(Request::expect(RequestItem::Space));
    }
    formatted.extend(gen_node_with_trivia(options, &item_binding_token)?);
    formatted.start_indent_before_requests();

    if let Some(item_template_list) = item_template_list {
        formatted.extend(gen_node_with_trivia(options, &item_template_list)?);
    }

    formatted.request(Request::expect(RequestItem::Space));
    formatted.extend(gen_node_with_trivia(options, &item_name)?);

    if let Some(type_specifier) = items_type {
        formatted.request(Request::discourage(RequestItem::Space));
        formatted.push_sc(sc!(":"));
        formatted.request(Request::expect(RequestItem::Space));
        formatted.extend(gen_node_with_trivia(options, &type_specifier)?);
    }

    if let Some(value) = assignment {
        formatted.start_new_line_group_before_requests();
        formatted.request(Request::expect(RequestItem::Space));
        formatted.push_sc(sc!("="));
        formatted.request(Request::expect(RequestItem::Space).or_newline());
        formatted.finish_new_line_group_after_requests();
        formatted.extend(gen_node_with_trivia(options, &value)?);
    }

    if statement_needs_semicolon_policy(syntax_node) {
        formatted.request(Request::discourage(RequestItem::Space));
        formatted.push_sc(sc!(";"));
    }
    formatted.finish_indent_before_requests();

    Ok(formatted)
}

#[must_use]
pub(crate) fn template_must_be_on_one_line_rule(node: &syntax::SyntaxNode) -> bool {
    let Some(parent) = node.parent() else {
        return false;
    };

    matches!(
        parent.kind(),
        SyntaxKind::VariableDeclaration
            | SyntaxKind::OverrideDeclaration
            | SyntaxKind::LetDeclaration
            | SyntaxKind::ConstantDeclaration
    )
}
