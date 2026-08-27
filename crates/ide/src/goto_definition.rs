use base_db::{EditionedFileId, FilePosition, RangeInfo, SourceDatabase as _};
use hir::{HasSource as _, Local, Semantics, definition::Definition};
use hir_def::InFile;
use ide_db::RootDatabase;
use syntax::{AstNode as _, HasName as _, SyntaxKind};

use crate::{NavigationTarget, helpers};

pub(crate) fn goto_definition(
    db: &RootDatabase,
    file_position: FilePosition,
) -> Option<RangeInfo<NavigationTarget>> {
    let semantics = &Semantics::new(db);
    let file_id = EditionedFileId::from_file(db, file_position.file_id);
    let file = file_id.parse(db).tree();
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

    let definition = Definition::from_token(semantics, file_id, &token)?;
    Some(RangeInfo::new(
        token.text_range(),
        definition.try_to_navigation_target(db)?,
    ))
}

pub(crate) trait ToNavigationTarget {
    fn to_navigation_target(
        &self,
        db: &RootDatabase,
    ) -> NavigationTarget;
}

pub trait TryToNavigationTarget {
    fn try_to_navigation_target(
        &self,
        db: &RootDatabase,
    ) -> Option<NavigationTarget>;
}

impl TryToNavigationTarget for Local {
    fn try_to_navigation_target(
        &self,
        db: &RootDatabase,
    ) -> Option<NavigationTarget> {
        let binding = self.source(db)?;

        let file_range = binding.original_file_range(db);
        // let name: SmolStr = binding.value.name()?.text().into();
        let navigation = NavigationTarget::from_syntax(file_range.file_id, file_range.range, None);
        Some(navigation)
    }
}

impl TryToNavigationTarget for Definition {
    fn try_to_navigation_target(
        &self,
        db: &RootDatabase,
    ) -> Option<NavigationTarget> {
        let navigation =
            match self {
                Self::BuiltinFunction(name) => None?,
                Self::BuiltinType(name) => None?,
                Self::BuiltinTypeGenerator(name) => None?,
                Self::BuiltinTypeConstructor(name) => None?,
                Self::BuiltinEnumerant(name) => None?,
                Self::BuiltinDeclaration(name) => None?,
                Self::Local(local) => local.try_to_navigation_target(db)?,
                Self::ModuleDef(definition) => {
                    match definition {
                        hir::ModuleDef::Function(function) => {
                            let declaration = function.source(db)?;

                            let frange = declaration.original_file_range(db);
                            let focus_range = declaration.value.name().map(|name| {
                                declaration.with_value(name).original_file_range(db).range
                            });

                            NavigationTarget::from_syntax(frange.file_id, frange.range, focus_range)
                        },
                        hir::ModuleDef::GlobalVariable(variable) => {
                            let declaration = variable.source(db)?;

                            let frange = declaration.original_file_range(db);
                            let focus_range = declaration.value.name().map(|name| {
                                declaration.with_value(name).original_file_range(db).range
                            });

                            NavigationTarget::from_syntax(frange.file_id, frange.range, focus_range)
                        },
                        hir::ModuleDef::GlobalConstant(constant) => {
                            let declaration = constant.source(db)?;

                            let frange = declaration.original_file_range(db);
                            let focus_range = declaration.value.name().map(|name| {
                                declaration.with_value(name).original_file_range(db).range
                            });

                            NavigationTarget::from_syntax(frange.file_id, frange.range, focus_range)
                        },
                        hir::ModuleDef::Override(override_declaration) => {
                            let declaration = override_declaration.source(db)?;

                            let frange = declaration.original_file_range(db);
                            let focus_range = declaration.value.name().map(|name| {
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
                        hir::ModuleDef::GlobalAssertStatement(global_assert_statement) => {
                            // Goto definition makes little sense for global assert statements - but we implement it anyway to have some parity.
                            let statement = global_assert_statement.source(db)?;

                            let frange = statement.original_file_range(db);
                            let focus_range = statement.value.expression().map(|expression| {
                                statement
                                    .with_value(expression)
                                    .original_file_range(db)
                                    .range
                            });

                            NavigationTarget::from_syntax(frange.file_id, frange.range, focus_range)
                        },
                    }
                },
                Self::Field(field) => {
                    let declaration = field.source(db)?;

                    let frange = declaration.original_file_range(db);
                    let focus_range = declaration
                        .value
                        .name()
                        .map(|name| declaration.with_value(name).original_file_range(db).range);

                    NavigationTarget::from_syntax(frange.file_id, frange.range, focus_range)
                },
            };
        Some(navigation)
    }
}
