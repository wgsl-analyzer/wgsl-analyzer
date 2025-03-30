use base_db::{FilePosition, TextRange};
use either::Either;
use hir::{HirDatabase, Semantics};
use hir_def::{HirFileId, db::DefinitionWithBodyId, resolver::Resolver};
use rowan::NodeOrToken;
use syntax::{AstNode, Direction, SyntaxKind, SyntaxToken, ast};

use crate::{config::CompletionConfig, patterns::determine_location};

type ExprOrStatement = Either<ast::Expression, ast::Statement>;

/// `CompletionContext` is created early during completion to figure out, where
/// exactly is the cursor, syntax-wise.
pub(crate) struct CompletionContext<'a> {
    pub sema: Semantics<'a>,
    pub file_id: HirFileId,
    pub db: &'a dyn HirDatabase,
    pub position: FilePosition,
    pub token: SyntaxToken,
    pub file: ast::SourceFile,
    pub container: Option<DefinitionWithBodyId>,
    pub completion_location: Option<ImmediateLocation>,
    pub resolver: Resolver,
}

impl<'a> CompletionContext<'a> {
    pub(crate) fn new(
        db: &'a dyn HirDatabase,
        position @ FilePosition { file_id, offset }: FilePosition,
        config: &'a CompletionConfig<'a>,
    ) -> Option<Self> {
        let sema = Semantics::new(db);
        let file = sema.parse(position.file_id);
        let token = file
            .syntax()
            .token_at_offset(position.offset)
            .left_biased()?;

        let file_id = HirFileId::from(position.file_id);

        let container = token
            .parent()
            .and_then(|parent| sema.find_container(file_id, &parent));

        let completion_location =
            determine_location(&sema, file.syntax(), position.offset, token.clone());

        let module_info = db.module_info(file_id);
        let mut resolver = Resolver::default().push_module_scope(db.upcast(), file_id, module_info);

        let nearest_scope = token
            .siblings_with_tokens(Direction::Prev) // spellchecker:disable-line
            .find_map(|sib| match sib {
                NodeOrToken::Node(node) if ExprOrStatement::can_cast(node.kind()) => {
                    ExprOrStatement::cast(node)
                },
                _ => None,
            })
            .or_else(|| token.parent_ancestors().find_map(ExprOrStatement::cast));

        if let Some(scope) = nearest_scope {
            if let Some(def) = container {
                resolver = sema.analyze(def).resolver_for(scope);
            }
        }

        let ctx = Self {
            file_id,
            sema,
            db,
            position,
            token,
            file,
            container,
            completion_location,
            resolver,
        };
        Some(ctx)
    }

    pub(crate) fn source_range(&self) -> base_db::TextRange {
        let kind = self.token.kind();
        if kind == SyntaxKind::Identifier
        // || kind.is_keyword()
        {
            self.token.text_range()
        } else {
            TextRange::empty(self.position.offset)
        }
    }
}

#[derive(Debug)]
pub(crate) enum ImmediateLocation {
    ItemList,
    StatementList,
    InsideStatement,
    Import,
    FieldAccess { expression: ast::FieldExpression },
}
