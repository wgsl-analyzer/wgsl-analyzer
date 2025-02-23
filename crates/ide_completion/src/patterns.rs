use base_db::TextSize;
use hir::Semantics;
use syntax::{AstNode, SyntaxKind, SyntaxNode, SyntaxToken, ast};

use crate::context::ImmediateLocation;

pub fn determine_location(
    _sema: &Semantics,
    _file: &SyntaxNode,
    _offset: TextSize,
    token: SyntaxToken,
) -> Option<ImmediateLocation> {
    let node = token.parent()?;
    let parent = node.parent()?;

    if let Some(expr) = ast::FieldExpr::cast(node.clone()) {
        Some(ImmediateLocation::FieldAccess { expr })
    } else if let Some(expr) = ast::FieldExpr::cast(parent.clone()) {
        Some(ImmediateLocation::FieldAccess { expr })
    } else if node.kind() == SyntaxKind::SourceFile {
        Some(ImmediateLocation::ItemList)
    } else if node.kind() == SyntaxKind::Import || parent.kind() == SyntaxKind::Import {
        Some(ImmediateLocation::Import)
    } else if node.ancestors().find_map(ast::Statement::cast).is_some() {
        Some(ImmediateLocation::InsideStatement)
    } else {
        None
    }
}
