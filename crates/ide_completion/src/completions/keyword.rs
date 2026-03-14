use super::Completions;
use crate::{
    context::{CompletionContext, ImmediateLocation},
    item::{CompletionItem, CompletionItemKind},
};

/// Top-level keywords (valid at module scope).
const ITEM_KEYWORDS: &[&str] = &[
    "fn",
    "struct",
    "var",
    "const",
    "let",
    "override",
    "alias",
    "const_assert",
    "enable",
    "requires",
    "diagnostic",
];

/// Statement-level keywords (valid inside function bodies).
const STATEMENT_KEYWORDS: &[&str] = &[
    "let",
    "var",
    "const",
    "if",
    "else",
    "for",
    "while",
    "loop",
    "switch",
    "return",
    "break",
    "continue",
    "continuing",
    "discard",
    "const_assert",
];

pub(crate) fn complete_keywords(
    accumulator: &mut Completions,
    context: &CompletionContext<'_>,
) -> Option<()> {
    let keywords = match context.completion_location {
        Some(ImmediateLocation::ItemList) => ITEM_KEYWORDS,
        Some(ImmediateLocation::InsideStatement | ImmediateLocation::StatementList) => {
            STATEMENT_KEYWORDS
        },
        _ => return None,
    };

    for keyword in keywords {
        CompletionItem::new(
            CompletionItemKind::Keyword,
            context.source_range(),
            *keyword,
        )
        .add_to(accumulator, context.database);
    }

    Some(())
}
