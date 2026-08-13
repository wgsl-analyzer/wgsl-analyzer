use dprint_core::formatting::StringContainer;
use dprint_core_macros::sc;
use parser::{SyntaxKind, SyntaxNode};
use syntax::{
    AstNode as _,
    ast::{self},
};

use crate::{
    ast_parse::{IgnoreBlankspace, NoTrivia, parse_end, parse_node_with, syntax_iter},
    context_policies::statement_needs_semicolon_policy,
    generators::node::gen_node_with_trivia,
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem},
    },
    reporting::FormatDocumentResult,
};

pub fn gen_const_declaration_statement(
    statement: &ast::ConstantDeclaration
) -> FormatDocumentResult<PrintItemBuffer> {
    gen_var_let_const_override_statement(BindingKind::Const, statement.syntax())
}

pub fn gen_let_declaration_statement(
    statement: &ast::LetDeclaration
) -> FormatDocumentResult<PrintItemBuffer> {
    gen_var_let_const_override_statement(BindingKind::Let, statement.syntax())
}

pub fn gen_var_declaration_statement(
    statement: &ast::VariableDeclaration
) -> FormatDocumentResult<PrintItemBuffer> {
    gen_var_let_const_override_statement(BindingKind::Var, statement.syntax())
}

pub fn gen_override_declaration_statement(
    statement: &ast::OverrideDeclaration
) -> FormatDocumentResult<PrintItemBuffer> {
    gen_var_let_const_override_statement(BindingKind::Override, statement.syntax())
}

#[derive(Clone, Copy, Debug)]
enum BindingKind {
    Var,
    Let,
    Const,

    // For now we have this here, because the override syntax is basically equivalent to a global const.
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

    const fn sc(self) -> &'static StringContainer {
        match self {
            Self::Var => sc!("var"),
            Self::Let => sc!("let"),
            Self::Const => sc!("const"),
            Self::Override => sc!("override"),
        }
    }
}

fn gen_var_let_const_override_statement(
    kind: BindingKind,
    syntax_node: &SyntaxNode,
) -> FormatDocumentResult<PrintItemBuffer> {
    // Note: When changing this function, should one of the three cases divert from the others more than
    // it already is, consider pulling it into a wholly separate function, instead of expanding this one with ifs

    // ==== Parse ====
    let mut syntax = syntax_iter(syntax_node);

    parse_node_with(&mut syntax, NoTrivia).expect_kind(kind.syntax_kind())?;
    let item_template_list = parse_node_with(&mut syntax, IgnoreBlankspace)
        .only_if_kind(SyntaxKind::TemplateList, &mut syntax);
    let item_name = parse_node_with(&mut syntax, IgnoreBlankspace).expect_kind(SyntaxKind::Name)?;

    let item_colon =
        parse_node_with(&mut syntax, NoTrivia).only_if_kind(SyntaxKind::Colon, &mut syntax);

    let items_type = if item_colon.is_some() {
        let item_type_specifier = parse_node_with(&mut syntax, IgnoreBlankspace)
            .expect_kind(SyntaxKind::TypeSpecifier)?;
        Some(item_type_specifier)
    } else {
        None
    };

    let item_equal =
        parse_node_with(&mut syntax, NoTrivia).only_if_kind(SyntaxKind::Equal, &mut syntax);

    let assignment = if item_equal.is_some() {
        let value =
            parse_node_with(&mut syntax, IgnoreBlankspace).expect_ast_node::<ast::Expression>()?;
        Some(value)
    } else {
        None
    };

    parse_node_with(&mut syntax, NoTrivia).expect_kind_optional(SyntaxKind::Semicolon)?; //Not all var-statements have a semicolon (e.g for loop)
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    formatted.push_sc(kind.sc());
    formatted.start_indent_before_requests();

    if let Some(item_template_list) = item_template_list {
        formatted.extend(gen_node_with_trivia(&item_template_list)?);
    }

    formatted.request(Request::expect(RequestItem::Space));
    formatted.extend(gen_node_with_trivia(&item_name)?);

    if let Some(type_specifier) = items_type {
        formatted.request(Request::discourage(RequestItem::Space));
        formatted.push_sc(sc!(":"));
        formatted.request(Request::expect(RequestItem::Space));
        formatted.extend(gen_node_with_trivia(&type_specifier)?);
    }

    if let Some(value) = assignment {
        formatted.request(Request::expect(RequestItem::Space));
        formatted.push_sc(sc!("="));
        formatted.request(Request::expect(RequestItem::Space));
        formatted.extend(gen_node_with_trivia(&value)?);
    }

    if statement_needs_semicolon_policy(syntax_node) {
        formatted.request(Request::discourage(RequestItem::Space));
        formatted.push_sc(sc!(";"));
    }
    formatted.finish_indent_before_requests();

    Ok(formatted)
}
