use dprint_core::formatting::{Signal, StringContainer};
use dprint_core_macros::sc;
use itertools::put_back;
use parser::{SyntaxKind, SyntaxNode};
use syntax::{
    AstNode as _,
    ast::{self, TemplateList},
};

use crate::format::{
    ast_parse::{
        parse_end, parse_many_comments_and_blankspace, parse_node, parse_node_optional,
        parse_token, parse_token_optional,
    },
    gen_attributes::{gen_attributes, parse_many_attributes},
    gen_comments::gen_comments,
    gen_expression::gen_expression,
    gen_types::{gen_template_list, gen_type_specifier},
    print_item_buffer::{PrintItemBuffer, SeparationPolicy},
    reporting::FormatDocumentResult,
};

pub fn gen_const_declaration_statement(
    statement: &ast::ConstantDeclaration,
    include_semicolon: bool,
) -> FormatDocumentResult<PrintItemBuffer> {
    gen_var_let_const_override_statement(BindingKind::Const, statement.syntax(), include_semicolon)
}

pub fn gen_let_declaration_statement(
    statement: &ast::LetDeclaration,
    include_semicolon: bool,
) -> FormatDocumentResult<PrintItemBuffer> {
    gen_var_let_const_override_statement(BindingKind::Let, statement.syntax(), include_semicolon)
}

pub fn gen_var_declaration_statement(
    statement: &ast::VariableDeclaration,
    include_semicolon: bool,
) -> FormatDocumentResult<PrintItemBuffer> {
    gen_var_let_const_override_statement(BindingKind::Var, statement.syntax(), include_semicolon)
}

pub fn gen_override_declaration_statement(
    statement: &ast::OverrideDeclaration,
    include_semicolon: bool,
) -> FormatDocumentResult<PrintItemBuffer> {
    gen_var_let_const_override_statement(
        BindingKind::Override,
        statement.syntax(),
        include_semicolon,
    )
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
            Self::Const => SyntaxKind::Constant,
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
    include_semicolon: bool,
) -> FormatDocumentResult<PrintItemBuffer> {
    // Note: When changing this function, should one of the three cases divert from the others more than
    // it already is, consider pulling it into a wholly separate function, instead of expanding this one with ifs

    // ==== Parse ====
    let mut syntax = put_back(syntax_node.children_with_tokens());

    let item_attributes = parse_many_attributes(&mut syntax)?;

    parse_token(&mut syntax, kind.syntax_kind())?;
    let item_comments_after_let = parse_many_comments_and_blankspace(&mut syntax)?;

    let item_template_list = if let Some(template_list) =
        parse_node_optional::<TemplateList>(&mut syntax)
    {
        let item_comments_after_template_list = parse_many_comments_and_blankspace(&mut syntax)?;
        Some((template_list, item_comments_after_template_list))
    } else {
        None
    };

    let item_name = parse_node::<ast::Name>(&mut syntax)?;
    let item_comments_after_name = parse_many_comments_and_blankspace(&mut syntax)?;

    let items_type = if parse_token_optional(&mut syntax, SyntaxKind::Colon).is_some() {
        let item_comments_after_colon = parse_many_comments_and_blankspace(&mut syntax)?;
        let item_type_specifier = parse_node::<ast::TypeSpecifier>(&mut syntax)?;
        let item_comments_after_type = parse_many_comments_and_blankspace(&mut syntax)?;
        Some((
            item_comments_after_colon,
            item_type_specifier,
            item_comments_after_type,
        ))
    } else {
        None
    };

    let assignment = if parse_token_optional(&mut syntax, SyntaxKind::Equal).is_some() {
        let item_comments_after_equal = parse_many_comments_and_blankspace(&mut syntax)?;

        let value = parse_node::<ast::Expression>(&mut syntax)?;
        let item_comments_after_value = parse_many_comments_and_blankspace(&mut syntax)?;
        Some((item_comments_after_equal, value, item_comments_after_value))
    } else {
        None
    };

    parse_token_optional(&mut syntax, SyntaxKind::Semicolon); //Not all var-statements have a semicolon (e.g for loop)
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    formatted.extend(gen_attributes(&item_attributes)?);
    formatted.push_sc(kind.sc());
    formatted.push_signal(Signal::StartIndent);

    formatted.extend(gen_comments(&item_comments_after_let));
    if let Some((item_template_list, item_comments_after_template_list)) = item_template_list {
        formatted.extend(gen_template_list(&item_template_list)?);
        formatted.extend(gen_comments(&item_comments_after_template_list));
    }

    formatted.expect_single_space();
    formatted.push_string(item_name.text().to_string());
    formatted.extend(gen_comments(&item_comments_after_name));

    if let Some((comments_after_colon, type_specifier, comments_after_type)) = items_type {
        formatted.request_space(SeparationPolicy::Discouraged);
        formatted.push_sc(sc!(":"));
        formatted.expect_single_space();
        formatted.extend(gen_comments(&comments_after_colon));
        formatted.extend(gen_type_specifier(&type_specifier)?);
        formatted.extend(gen_comments(&comments_after_type));
    }

    if let Some((comments_after_equal, value, comments_after_value)) = assignment {
        formatted.expect_single_space();
        formatted.push_sc(sc!("="));
        formatted.expect_single_space();
        formatted.extend(gen_comments(&comments_after_equal));
        formatted.extend(gen_expression(&value, false)?);
        formatted.extend(gen_comments(&comments_after_value));
        formatted.request_space(SeparationPolicy::Discouraged);
    }

    if include_semicolon {
        formatted.push_sc(sc!(";"));
    }
    formatted.push_signal(Signal::FinishIndent);

    Ok(formatted)
}
