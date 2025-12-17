use base_db::{FilePosition, SourceDatabase as _};
use hir::{HasSource as _, Local, Semantics, definition::Definition};
use hir_def::InFile;
use ide_db::RootDatabase;
use syntax::{AstNode as _, HasName as _, SyntaxKind};

use crate::{NavigationTarget, helpers};

pub(crate) fn goto_definition(
    database: &RootDatabase,
    file_position: FilePosition,
) -> Option<NavigationTarget> {
    let semantics = &Semantics::new(database);
    let file_id = file_position.file_id;
    let file = database.parse(file_id).tree();
    let token = file.syntax().token_at_offset(file_position.offset);

    #[expect(
        clippy::wildcard_enum_match_arm,
        reason = "infeasible to list all cases"
    )]
    let token = helpers::pick_best_token(token, |token| match token {
        SyntaxKind::Identifier => 2,
        kind if kind.is_trivia() => 0,
        _ => 1,
    })?;

    let definition = Definition::from_token(semantics, file_id.into(), &token)?;
    InFile::new(file_id.into(), definition).try_to_navigation_target(database)
}

pub(crate) trait ToNavigationTarget {
    fn to_navigation_target(
        &self,
        database: &RootDatabase,
    ) -> NavigationTarget;
}

pub trait TryToNavigationTarget {
    fn try_to_navigation_target(
        &self,
        database: &RootDatabase,
    ) -> Option<NavigationTarget>;
}

impl TryToNavigationTarget for InFile<Local> {
    fn try_to_navigation_target(
        &self,
        database: &RootDatabase,
    ) -> Option<NavigationTarget> {
        let binding = self.value.source(database)?;

        let file_range = binding.original_file_range(database);
        // let name: SmolStr = binding.value.name()?.text().into();
        let navigation = NavigationTarget::from_syntax(file_range.file_id, file_range.range, None);
        Some(navigation)
    }
}

impl TryToNavigationTarget for InFile<Definition> {
    fn try_to_navigation_target(
        &self,
        database: &RootDatabase,
    ) -> Option<NavigationTarget> {
        let navigation = match &self.value {
            Definition::Local(local) => {
                InFile::new(self.file_id, *local).try_to_navigation_target(database)?
            },
            Definition::ModuleDef(definition) => match definition {
                hir::ModuleDef::Function(function) => {
                    let declaration = function.source(database)?;

                    let frange = declaration.original_file_range(database);
                    let focus_range = declaration.value.name().map(|name| {
                        declaration
                            .with_value(name)
                            .original_file_range(database)
                            .range
                    });

                    NavigationTarget::from_syntax(frange.file_id, frange.range, focus_range)
                },
                hir::ModuleDef::GlobalVariable(variable) => {
                    let declaration = variable.source(database)?;

                    let frange = declaration.original_file_range(database);
                    let focus_range = declaration.value.name().map(|name| {
                        declaration
                            .with_value(name)
                            .original_file_range(database)
                            .range
                    });

                    NavigationTarget::from_syntax(frange.file_id, frange.range, focus_range)
                },
                hir::ModuleDef::GlobalConstant(constant) => {
                    let declaration = constant.source(database)?;

                    let frange = declaration.original_file_range(database);
                    let focus_range = declaration.value.name().map(|name| {
                        declaration
                            .with_value(name)
                            .original_file_range(database)
                            .range
                    });

                    NavigationTarget::from_syntax(frange.file_id, frange.range, focus_range)
                },
                hir::ModuleDef::Override(override_declaration) => {
                    let declaration = override_declaration.source(database)?;

                    let frange = declaration.original_file_range(database);
                    let focus_range = declaration.value.name().map(|name| {
                        declaration
                            .with_value(name)
                            .original_file_range(database)
                            .range
                    });

                    NavigationTarget::from_syntax(frange.file_id, frange.range, focus_range)
                },
                hir::ModuleDef::Struct(r#struct) => {
                    let declaration = r#struct.source(database)?;

                    let frange = declaration.original_file_range(database);
                    let focus_range = declaration.value.name().map(|name| {
                        declaration
                            .with_value(name)
                            .original_file_range(database)
                            .range
                    });

                    NavigationTarget::from_syntax(frange.file_id, frange.range, focus_range)
                },
                hir::ModuleDef::TypeAlias(type_alias) => {
                    let declaration = type_alias.source(database)?;

                    let frange = declaration.original_file_range(database);
                    let focus_range = declaration.value.name().map(|name| {
                        declaration
                            .with_value(name)
                            .original_file_range(database)
                            .range
                    });

                    NavigationTarget::from_syntax(frange.file_id, frange.range, focus_range)
                },
                hir::ModuleDef::GlobalAssertStatement(global_assert_statement) => todo!(),
            },
            Definition::Field(field) => {
                let declaration = field.source(database)?;

                let frange = declaration.original_file_range(database);
                let focus_range = declaration.value.name().map(|name| {
                    declaration
                        .with_value(name)
                        .original_file_range(database)
                        .range
                });

                NavigationTarget::from_syntax(frange.file_id, frange.range, focus_range)
            },
        };
        Some(navigation)
    }
}
