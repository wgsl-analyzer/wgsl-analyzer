use base_db::{FilePosition, TextRange};
use hir::{HasSource, HirDatabase, Local, Semantics, definition::Definition};
use hir_def::{InFile, db::DefDatabase};
use syntax::{AstNode, HasName, SyntaxKind};
use vfs::FileId;

use crate::helpers;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct NavigationTarget {
    pub file_id: FileId,
    /// Range which encompasses the whole element.
    ///
    /// Should include body, doc comments, attributes, etc.
    ///
    /// Clients should use this range to answer "is the cursor inside the
    /// element?" question.
    pub full_range: TextRange,
    /// A "most interesting" range within the `full_range`.
    ///
    /// Typically, `full_range` is the whole syntax node, including doc
    /// comments, and `focus_range` is the range of the identifier.
    ///
    /// Clients should place the cursor on this range when navigating to this target.
    pub focus_range: Option<TextRange>,
    // pub name: SmolStr,
    // pub kind: Option<SymbolKind>,
    // pub container_name: Option<SmolStr>,
    // pub description: Option<String>,
    // pub docs: Option<Documentation>,
}

impl NavigationTarget {
    pub fn from_syntax(
        file_id: FileId,
        full_range: TextRange,
        focus_range: Option<TextRange>,
    ) -> Self {
        Self {
            file_id,
            full_range,
            focus_range,
        }
    }

    pub fn focus_or_full_range(&self) -> TextRange {
        self.focus_range.unwrap_or(self.full_range)
    }
}

pub(crate) fn goto_definition(
    db: &dyn HirDatabase,
    file_position: FilePosition,
) -> Option<NavigationTarget> {
    let sema = &Semantics::new(db);
    let file_id = file_position.file_id;
    let file = db.parse(file_id).tree();
    let token = file.syntax().token_at_offset(file_position.offset);

    let token = helpers::pick_best_token(token, |token| match token {
        SyntaxKind::Identifier => 2,
        kind if kind.is_trivia() => 0,
        _ => 1,
    })?;

    let definition = Definition::from_token(sema, file_id.into(), &token)?;
    InFile::new(file_id.into(), definition).to_nav(db.upcast())
}

trait ToNav {
    fn to_nav(
        &self,
        db: &dyn DefDatabase,
    ) -> Option<NavigationTarget>;
}

impl ToNav for InFile<Local> {
    fn to_nav(
        &self,
        db: &dyn DefDatabase,
    ) -> Option<NavigationTarget> {
        let binding = self.value.source(db)?;

        let frange = binding.original_file_range(db);
        let nav = NavigationTarget::from_syntax(frange.file_id, frange.range, None);
        Some(nav)
    }
}

impl ToNav for InFile<Definition> {
    fn to_nav(
        &self,
        db: &dyn DefDatabase,
    ) -> Option<NavigationTarget> {
        let nav =
            match &self.value {
                Definition::Local(local) => InFile::new(self.file_id, *local).to_nav(db)?,
                Definition::ModuleDef(def) => {
                    match def {
                        hir::ModuleDef::Function(function) => {
                            let declaration = function.source(db)?;

                            let frange = declaration.original_file_range(db);
                            let focus_range = declaration.value.name().map(|name| {
                                declaration.with_value(name).original_file_range(db).range
                            });

                            NavigationTarget::from_syntax(frange.file_id, frange.range, focus_range)
                        },
                        hir::ModuleDef::GlobalVariable(var) => {
                            let declaration = var.source(db)?;

                            let frange = declaration.original_file_range(db);
                            let focus_range = declaration.value.binding().map(|name| {
                                declaration.with_value(name).original_file_range(db).range
                            });

                            NavigationTarget::from_syntax(frange.file_id, frange.range, focus_range)
                        },
                        hir::ModuleDef::GlobalConstant(constant) => {
                            let declaration = constant.source(db)?;

                            let frange = declaration.original_file_range(db);
                            let focus_range = declaration.value.binding().map(|name| {
                                declaration.with_value(name).original_file_range(db).range
                            });

                            NavigationTarget::from_syntax(frange.file_id, frange.range, focus_range)
                        },
                        hir::ModuleDef::Override(override_declaration) => {
                            let declaration = override_declaration.source(db)?;

                            let frange = declaration.original_file_range(db);
                            let focus_range = declaration.value.binding().map(|name| {
                                declaration.with_value(name).original_file_range(db).range
                            });

                            NavigationTarget::from_syntax(frange.file_id, frange.range, focus_range)
                        },
                        hir::ModuleDef::Struct(r#struct) => {
                            let declaration = r#struct.source(db)?;

                            let frange = declaration.original_file_range(db);
                            let focus_range = declaration.value.name().map(|name| {
                                declaration.with_value(name).original_file_range(db).range
                            });

                            NavigationTarget::from_syntax(frange.file_id, frange.range, focus_range)
                        },
                        hir::ModuleDef::TypeAlias(type_alias) => {
                            let declaration = type_alias.source(db)?;

                            let frange = declaration.original_file_range(db);
                            let focus_range = declaration.value.name().map(|name| {
                                declaration.with_value(name).original_file_range(db).range
                            });

                            NavigationTarget::from_syntax(frange.file_id, frange.range, focus_range)
                        },
                    }
                },
                Definition::Field(field) => {
                    let declaration = field.source(db)?;

                    let frange = declaration.original_file_range(db);
                    let focus_range = declaration
                        .value
                        .variable_ident_declaration()
                        .map(|name| declaration.with_value(name).original_file_range(db).range);

                    NavigationTarget::from_syntax(frange.file_id, frange.range, focus_range)
                },
                Definition::Struct(r#struct) => {
                    let declaration = r#struct.source(db)?;
                    let frange = declaration.original_file_range(db);

                    let focus_range = declaration
                        .value
                        .name()
                        .map(|name| declaration.with_value(name).original_file_range(db).range);

                    NavigationTarget::from_syntax(frange.file_id, frange.range, focus_range)
                },
                Definition::TypeAlias(type_alias) => {
                    let declaration = type_alias.source(db)?;
                    let frange = declaration.original_file_range(db);

                    let focus_range = declaration
                        .value
                        .name()
                        .map(|name| declaration.with_value(name).original_file_range(db).range);

                    NavigationTarget::from_syntax(frange.file_id, frange.range, focus_range)
                },
            };
        Some(nav)
    }
}
