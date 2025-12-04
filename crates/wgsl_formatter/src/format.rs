mod ast_parse;
mod helpers;
mod print_item_buffer;

pub mod gen_assignment_statement;
mod gen_attributes;
mod gen_comments;
mod gen_expression;
mod gen_function;
mod gen_function_call;
mod gen_if_statement;
mod gen_statement;
mod gen_struct;
pub mod gen_switch_statement;
mod gen_types;
mod gen_var_let_const_statement;
pub mod multiline_group;
mod reporting;

use std::{alloc::alloc, iter::repeat_with, rc::Rc};

use dprint_core::formatting::{
    ConditionResolver, ConditionResolverContext, LineNumber, LineNumberAnchor, PrintItem,
    PrintItems, PrintOptions, Signal, actions, condition_helpers, condition_resolvers, conditions,
    ir_helpers,
};
use dprint_core_macros::sc;
use itertools::{Itertools as _, Position, PutBack, put_back};
use parser::{SyntaxKind, SyntaxNode, SyntaxToken, WeslLanguage};
use rowan::{NodeOrToken, SyntaxElementChildren};
use syntax::{
    AstNode as _, HasName as _,
    ast::{self},
    match_ast,
};

use crate::{
    FormattingOptions,
    format::{
        self,
        ast_parse::{
            parse_end, parse_end_optional, parse_many_comments_and_blankspace, parse_node,
            parse_node_optional, parse_token, parse_token_optional,
        },
        gen_comments::gen_comment,
        gen_function::gen_function_declaration,
        gen_statement::gen_const_assert_statement,
        gen_struct::gen_struct_declaration,
        helpers::{gen_spaced_lines, into_items},
        print_item_buffer::{PrintItemBuffer, SeparationPolicy, SeparationRequest},
        reporting::{
            FormatDocumentError, FormatDocumentErrorKind, FormatDocumentResult, UnwrapIfPreferCrash,
        },
    },
};

pub fn format_str(
    input: &str,
    options: &FormattingOptions,
) -> FormatDocumentResult<String> {
    let parse = syntax::parse(input);
    //TODO Return error if the syntax could not parse.
    let file = parse.tree();
    format_tree(&file, options)
}

pub fn format_tree(
    syntax: &ast::SourceFile,
    options: &FormattingOptions,
) -> FormatDocumentResult<String> {
    let mut error = None;

    let formatted = dprint_core::formatting::format(
        || match gen_source_file(syntax) {
            Ok(items) => items.finish(),
            Err(err) => {
                //We seem to have to do it this weird way, because
                // a) We can't return the error from the closure because of dprint's api
                // b) We can't call gen_source_file outside of the closure because
                //    dprint requires the gen_items to be allocated using a thread local
                //    allocator that only exists within the closure.
                error.insert(err);

                //TODO maybe we should instead output the whole source verbatim
                // so that if many things go wrong and this value does somehow
                // reach the user's file, it doesn't just delete it all.
                let mut items = PrintItemBuffer::new();
                items.push_string("ERROR".into());
                items.finish()
            },
        },
        PrintOptions {
            //TODO Populate these from options
            max_width: options.width,
            indent_width: 4,
            use_tabs: false,
            new_line_text: "\n",
        },
    );

    match error {
        Some(err) => Err(err),
        None => Ok(formatted),
    }
}

fn gen_item(node: &ast::Item) -> FormatDocumentResult<PrintItemBuffer> {
    match node {
        ast::Item::FunctionDeclaration(function_declaration) => {
            gen_function_declaration(function_declaration)
        },
        ast::Item::StructDeclaration(struct_declaration) => {
            gen_struct_declaration(struct_declaration)
        },
        ast::Item::VariableDeclaration(variable_declaration) => todo!(),
        ast::Item::ConstantDeclaration(constant_declaration) => todo!(),
        ast::Item::OverrideDeclaration(override_declaration) => todo!(),
        ast::Item::TypeAliasDeclaration(type_alias_declaration) => todo!(),
        ast::Item::AssertStatement(assert_statement) => {
            gen_const_assert_statement(assert_statement, true)
        },
    }
}

fn gen_source_file(node: &ast::SourceFile) -> FormatDocumentResult<PrintItemBuffer> {
    dbg!(node.syntax());

    let mut formatted = PrintItemBuffer::new();
    formatted.request(SeparationRequest::discouraged());

    let lines = gen_spaced_lines(node.syntax(), |child| {
        let mut formatted = PrintItemBuffer::new();

        //TODO This clone is unnecessary if we had a cast that returned the passed in node
        // on a failure like std::any::Any (SyntaxNode -> Result<Item, Syntaxnode>)
        if let NodeOrToken::Node(child) = child
            && let Some(item) = ast::Item::cast(child.clone())
        {
            formatted.extend(gen_item(&item)?);
        } else if let NodeOrToken::Token(child) = child
            && (child.kind() == SyntaxKind::BlockComment
                || child.kind() == SyntaxKind::LineEndingComment)
        {
            formatted.extend(gen_comment(child));
        } else {
            return Err(FormatDocumentErrorKind::UnexpectedModuleNode.at(child.text_range()))
                .expect_if_prefer_crash();
        }

        // In a source file there will be a newline after *every* item.
        formatted.request(SeparationRequest {
            line_break: SeparationPolicy::Expected,
            ..Default::default()
        });
        Ok(formatted)
    })?;

    formatted.extend(lines);
    //There should be a newline, but no empty line at the end of the file
    formatted.request(SeparationRequest {
        empty_line: SeparationPolicy::Discouraged,
        line_break: SeparationPolicy::Expected,
        ..Default::default()
    });

    Ok(formatted)
}
