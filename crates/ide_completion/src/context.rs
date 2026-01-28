use base_db::{FilePosition, TextRange};
use either::Either;
use hir::{ChildContainer, Semantics};
use hir_def::{HirFileId, database::DefDatabase as _, resolver::Resolver};
use ide_db::RootDatabase;
use rowan::NodeOrToken;
use syntax::{AstNode as _, Direction, SyntaxKind, SyntaxToken, ast};

use crate::{config::CompletionConfig, patterns::determine_location};

type ExprOrStatement = Either<ast::Expression, ast::Statement>;

/// `CompletionContext` is created early during completion to figure out, where
/// exactly is the cursor, syntax-wise.
pub(crate) struct CompletionContext<'database> {
    pub(crate) semantics: Semantics<'database>,
    pub(crate) file_id: HirFileId,
    pub(crate) database: &'database RootDatabase,
    pub(crate) position: FilePosition,
    pub(crate) token: SyntaxToken,
    pub(crate) file: ast::SourceFile,
    pub(crate) container: Option<ChildContainer>,
    pub(crate) completion_location: Option<ImmediateLocation>,
    pub(crate) resolver: Resolver,
}

impl<'database> CompletionContext<'database> {
    pub(crate) fn new(
        database: &'database RootDatabase,
        position @ FilePosition { file_id, offset }: FilePosition,
        config: &'database CompletionConfig,
    ) -> Option<Self> {
        let semantics = Semantics::new(database);
        let file_id = database.editioned_file_id(file_id);
        let file = semantics.parse(file_id);
        let token = file
            .syntax()
            .token_at_offset(position.offset)
            .left_biased()?;

        let file_id = HirFileId::from(file_id);

        let container = token
            .parent()
            .and_then(|parent| semantics.find_container(file_id, &parent));

        let completion_location =
            determine_location(&semantics, file.syntax(), position.offset, &token);

        let module_info = database.item_tree(file_id);
        let mut resolver = Resolver::default().push_module_scope(file_id, module_info);

        let nearest_scope = token
            .siblings_with_tokens(Direction::Prev) // spellchecker:disable-line
            .find_map(|sib| match sib {
                NodeOrToken::Node(node) if ExprOrStatement::can_cast(node.kind()) => {
                    ExprOrStatement::cast(node)
                },
                NodeOrToken::Node(_) | NodeOrToken::Token(_) => None,
            })
            .or_else(|| token.parent_ancestors().find_map(ExprOrStatement::cast));

        if let Some(scope) = nearest_scope
            && let Some(definition) = container
            && let Some(definition) = definition.as_def_with_body_id()
        {
            resolver = semantics.analyze(definition).resolver_for(scope);
        }

        let context = Self {
            semantics,
            file_id,
            database,
            position,
            token,
            file,
            container,
            completion_location,
            resolver,
        };
        Some(context)
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
    FieldAccess { expression: ast::FieldExpression },
}
