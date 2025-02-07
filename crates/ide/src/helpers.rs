use rowan::TokenAtOffset;
use syntax::{SyntaxKind, SyntaxToken};

pub fn pick_best_token(
	tokens: TokenAtOffset<SyntaxToken>,
	f: impl Fn(SyntaxKind) -> usize,
) -> Option<SyntaxToken> {
	tokens.max_by_key(move |t| f(t.kind()))
}
