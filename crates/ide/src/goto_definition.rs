use base_db::{FilePosition, SourceDatabase};
use hir::{HasSource, Local, Semantics, definition::Definition};
use hir_def::InFile;
use ide_db::RootDatabase;
use smol_str::SmolStr;
use syntax::{AstNode, HasName, SyntaxKind};

use crate::{NavigationTarget, helpers};

pub(crate) fn goto_definition(
    db: &RootDatabase,
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
    InFile::new(file_id.into(), definition).try_to_nav(db)
}

pub(crate) trait ToNav {
    fn to_nav(
        &self,
        db: &RootDatabase,
    ) -> NavigationTarget;
}

pub trait TryToNav {
    fn try_to_nav(
        &self,
        db: &RootDatabase,
    ) -> Option<NavigationTarget>;
}

impl TryToNav for InFile<Local> {
    fn try_to_nav(
        &self,
        db: &RootDatabase,
    ) -> Option<NavigationTarget> {
        let binding = self.value.source(db)?;

        let file_range = binding.original_file_range(db);
        // let name: SmolStr = binding.value.name()?.text().into();
        let nav = NavigationTarget::from_syntax(file_range.file_id, file_range.range, None);
        Some(nav)
    }
}

impl TryToNav for InFile<Definition> {
    fn try_to_nav(
        &self,
        db: &RootDatabase,
    ) -> Option<NavigationTarget> {
        let nav =
            match &self.value {
                Definition::Local(local) => InFile::new(self.file_id, *local).try_to_nav(db)?,
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
