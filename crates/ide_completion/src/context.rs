use base_db::{FilePosition, TextRange};
use either::Either;
use hir::{HirDatabase, Semantics};
use hir_def::{
    HirFileId,
    database::{DefDatabase as _, DefinitionWithBodyId},
    resolver::Resolver,
};
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
    pub(crate) container: Option<DefinitionWithBodyId>,
    pub(crate) completion_location: Option<ImmediateLocation>,
    pub(crate) resolver: Resolver,
}

impl<'database> CompletionContext<'database> {
    pub(crate) fn new(
        database: &'database RootDatabase,
        position @ FilePosition { file_id, offset }: FilePosition,
        config: &'database CompletionConfig,
    ) -> Option<Self> {
        let sema = Semantics::new(database);
        let file = sema.parse(position.file_id);
        let token = file
            .syntax()
            .token_at_offset(position.offset)
            .left_biased()?;

        let file_id = HirFileId::from(position.file_id);

        let container = token
            .parent()
            .and_then(|parent| sema.find_container(file_id, &parent));

        let completion_location = determine_location(&sema, file.syntax(), position.offset, &token);

        let module_info = database.module_info(file_id);
        let mut resolver = Resolver::default().push_module_scope(database, file_id, module_info);

        let nearest_scope = token
            .siblings_with_tokens(Direction::Prev) // spellchecker:disable-line
            .find_map(|sib| match sib {
                NodeOrToken::Node(node) if ExprOrStatement::can_cast(node.kind()) => {
                    ExprOrStatement::cast(node)
                },
                NodeOrToken::Node(_) | NodeOrToken::Token(_) => None,
            })
            .or_else(|| token.parent_ancestors().find_map(ExprOrStatement::cast));

        if let Some(scope) = nearest_scope {
            if let Some(def) = container {
                resolver = sema.analyze(def).resolver_for(scope);
            }
        }

        let context = Self {
            semantics: sema,
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
    Import,
    FieldAccess { expression: ast::FieldExpression },
}
