use dprint_core::formatting::{PrintItems, StringContainer};
use itertools::{Itertools as _, Position};

use crate::{ast_parse::SyntaxIter, generators::comments::{Comment, gen_comments, parse_many_comments_and_blankspace}, multiline_group::MultilineGroup, print_item_buffer::PrintItemBuffer, reporting::FormatDocumentResult};

pub type SeparatedItems<T> = Vec<SeparatedItem<T>>;

pub struct SeparatedItem<T> {
    item: T,
    comments_after_item: Vec<Comment>,
    //separator: S,
    comments_after_separator: Vec<Comment>,
}

pub fn parse_separated_items<T, S>(
    syntax: &mut SyntaxIter,
    parse_item: impl Fn(&mut SyntaxIter) -> Option<T>,
    parse_separator: impl Fn(&mut SyntaxIter) -> S,
) -> SeparatedItems<T> {
    let mut items = Vec::new();
    loop {
        let Some(item_param) = parse_item(syntax) else {
            break;
        };
        let item_comments_after_param = parse_many_comments_and_blankspace(syntax).unwrap(); //TODO parse_comments cannot return err

        let _item_separator = parse_separator(syntax);
        let item_comments_after_comma = parse_many_comments_and_blankspace(syntax).unwrap(); //TODO parse_comments cannot return err

        items.push(SeparatedItem {
            item: item_param,
            comments_after_item: item_comments_after_param,
            //separator: item_separator,
            comments_after_separator: item_comments_after_comma,
        });
    }
    items
}

pub fn format_separated_items<'a, T>(
    multiline_group: &mut MultilineGroup<'a>,
    items: SeparatedItems<T>,
    gen_item: impl Fn(&T) -> FormatDocumentResult<PrintItemBuffer>,
    separator: &'static StringContainer,
) -> FormatDocumentResult<()> {
    for (pos, item) in items.into_iter().with_position() {
        //multiline_group.extend(gen_expression(&item.item, false)?);
        multiline_group.extend(gen_item(&item.item)?);
        if pos == Position::Last || pos == Position::Only {
            multiline_group.extend_if_multi_line({
                let mut pi = PrintItems::default();
                pi.push_sc(separator);
                pi
            });
        } else {
            multiline_group.push_sc(separator);
        }

        //The comma should be immediately after the item, we move the comment back
        multiline_group.extend(gen_comments(&item.comments_after_item));
        multiline_group.extend(gen_comments(&item.comments_after_separator));

        multiline_group.grouped_newline_or_space();
    }
    Ok(())
}
