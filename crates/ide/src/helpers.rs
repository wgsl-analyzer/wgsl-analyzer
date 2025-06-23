use rowan::TokenAtOffset;
use syntax::{SyntaxKind, SyntaxToken};

pub(crate) fn pick_best_token(
    tokens: TokenAtOffset<SyntaxToken>,
    scorer: impl Fn(SyntaxKind) -> usize,
) -> Option<SyntaxToken> {
    tokens.max_by_key(move |token| scorer(token.kind()))
}
