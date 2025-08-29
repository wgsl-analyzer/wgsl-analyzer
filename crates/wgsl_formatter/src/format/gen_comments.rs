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

use crate::format::print_item_buffer::{PrintItemBuffer, PrintItemRequest, SeparationPolicy};

pub fn gen_comments(comments: Vec<SyntaxToken>) -> PrintItemBuffer {
    let mut formatted = PrintItemBuffer::new();
    for item in comments {
        formatted.extend(gen_comment(&item));
    }
    formatted
}
pub fn gen_comment(item: &SyntaxToken) -> PrintItemBuffer {
    let mut formatted = PrintItemBuffer::new();
    if item.kind() == SyntaxKind::BlockComment {
        formatted.request_single_space();
        formatted.push_string(item.to_string());
        formatted.request_single_space();
    } else if item.kind() == SyntaxKind::LineEndingComment {
        formatted.request_single_space();
        formatted.push_string(item.to_string());
        //TODO This should be a request, but for now we have no way of encoding a "forced newline no matter what"
        formatted.request(PrintItemRequest {
            line_break: SeparationPolicy::Forced,
            ..Default::default()
        });
    } else {
        //TODO Make this unrepresentable
        unreachable!("Non comment entry found in comments Vec");
    }
    formatted
}
