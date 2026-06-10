use rowan::TokenAtOffset;
use syntax::{SyntaxKind, SyntaxToken};

pub(crate) fn pick_best_token<Scorer>(
    tokens: TokenAtOffset<SyntaxToken>,
    scorer: Scorer,
) -> Option<SyntaxToken>
where
    Scorer: Fn(SyntaxKind) -> usize,
{
    tokens.max_by_key(move |token| scorer(token.kind()))
}
