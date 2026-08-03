use dprint_core::formatting::{PrintItems, StringContainer};
use parser::SyntaxKind;

use crate::{
    ast_parse::{SyntaxIter, parse_token_optional},
    generators::comments::{Comment, gen_comment, parse_comment_optional},
    multiline_group::MultilineGroup,
    print_item_buffer::PrintItemBuffer,
    reporting::FormatDocumentResult,
};

use super::{LineSpacing, parse_line_spacing};

pub struct SeparatedItems<T> {
    pub is_blank: bool,
    pub last_item_index: usize,
    pub items: Vec<SeparatedItem<T>>,
}

pub enum SeparatedItem<T> {
    Item(T),
    Separator,
    Comment(Comment),
    LineSpacing(LineSpacing),
}

pub fn parse_separated_items<T, S>(
    syntax: &mut SyntaxIter,
    parse_item: impl Fn(&mut SyntaxIter) -> Option<T>,
    parse_separator: impl Fn(&mut SyntaxIter) -> Option<S>,
) -> SeparatedItems<T> {
    let mut items = Vec::new();
    let mut is_blank = true;
    let mut last_item_index = 0;

    loop {
        if let Some(spacing) = parse_line_spacing(syntax) {
            // Currently we only respect line_spacings if they occur directly before a comment
            items.push(SeparatedItem::LineSpacing(spacing));
        } else if let Some(_statement) = parse_token_optional(syntax, SyntaxKind::Blankspace) {
            // If its not a line_spacing blankspace, then we simply discard it
        } else if let Some(item) = parse_item(syntax) {
            last_item_index = items.len();
            items.push(SeparatedItem::Item(item));
            is_blank = false;
        } else if let Some(_separator) = parse_separator(syntax) {
            items.push(SeparatedItem::Separator);
        } else if let Some(comment) = parse_comment_optional(syntax) {
            items.push(SeparatedItem::Comment(comment));
            is_blank = false;
        } else {
            break;
        }
    }
    SeparatedItems {
        is_blank,
        last_item_index,
        items,
    }
}

pub fn format_separated_items<'a, T>(
    multiline_group: &mut MultilineGroup<'a>,
    items: SeparatedItems<T>,
    gen_item: impl Fn(&T) -> FormatDocumentResult<PrintItemBuffer>,
    separator: &'static StringContainer,
) -> FormatDocumentResult<()> {
    for (index, item) in items.items.into_iter().enumerate() {
        match item {
            SeparatedItem::Item(item) => {
                // Separated Items always start on a new line
                multiline_group.grouped_newline_or_space();
                multiline_group.extend(gen_item(&item)?);

                // The separator is always immediately after the item
                if index == items.last_item_index {
                    multiline_group.extend_if_multi_line({
                        let mut pi = PrintItems::default();
                        pi.push_sc(separator);
                        pi
                    });
                } else {
                    multiline_group.push_sc(separator);
                }
            },
            SeparatedItem::Separator => {
                // The separator is always immediately after the item
            },
            SeparatedItem::Comment(comment) => {
                multiline_group.extend(gen_comment(&comment));
            },
            SeparatedItem::LineSpacing(_line_spacing) => {
                // We discard empty lines
            },
        }
    }
    Ok(())
}
