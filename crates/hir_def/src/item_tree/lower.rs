use syntax::{
    HasName as _,
    ast::{Directive, Item, SourceFile},
};
use triomphe::Arc;

use super::{GlobalConstant, GlobalVariable, Override, Struct, TypeAlias};
use crate::{
    HirFileId,
    ast_id::AstIdMap,
    database::DefDatabase,
    item_tree::{
        self, Function, GlobalAssertStatement, ImportStatement, ImportTree, ItemTree, ModuleItem,
        ModuleItemId,
    },
    mod_path::{ModPath, PathKind},
};

pub(crate) struct Ctx<'database> {
    database: &'database dyn DefDatabase,
    file_id: HirFileId,
    source_ast_id_map: Arc<AstIdMap>,
    pub(crate) tree: ItemTree,
    pub(crate) items: Vec<ModuleItem>,
}

impl<'database> Ctx<'database> {
    pub(crate) fn new(
        database: &'database dyn DefDatabase,
        file_id: HirFileId,
    ) -> Self {
        Self {
            database,
            file_id,
            source_ast_id_map: database.ast_id_map(file_id),
            tree: ItemTree::default(),
            items: vec![],
        }
    }

    pub(crate) fn lower_source_file(
        mut self,
        source_file: &SourceFile,
    ) -> ItemTree {
        source_file.items().for_each(|item| {
            self.lower_item(item);
        });
        self.tree
    }

    fn lower_item(
        &mut self,
        item: Item,
    ) -> Option<()> {
        let item = match item {
            Item::ImportStatement(import_statement) => {
                ModuleItem::ImportStatement(self.lower_import(&import_statement)?)
            },
            Item::FunctionDeclaration(function) => {
                ModuleItem::Function(self.lower_function(&function)?)
            },
            Item::StructDeclaration(r#struct) => ModuleItem::Struct(self.lower_struct(&r#struct)?),
            Item::VariableDeclaration(variable) => {
                ModuleItem::GlobalVariable(self.lower_global_variable(&variable)?)
            },
            Item::ConstantDeclaration(constant) => {
                ModuleItem::GlobalConstant(self.lower_global_constant(&constant)?)
            },
            Item::OverrideDeclaration(override_declaration) => {
                ModuleItem::Override(self.lower_override(&override_declaration)?)
            },
            Item::TypeAliasDeclaration(type_alias) => {
                ModuleItem::TypeAlias(self.lower_type_alias(&type_alias)?)
            },
            Item::AssertStatement(assert_statement) => ModuleItem::GlobalAssertStatement(
                self.lower_global_assert_statement(&assert_statement)?,
            ),
        };
        self.items.push(item);
        Some(())
    }

    fn lower_import(
        &mut self,
        item: &syntax::ast::ImportStatement,
    ) -> Option<ModuleItemId<ImportStatement>> {
        let kind = PathKind::from_src(item.relative());
        let tree = self.lower_import_tree(&item.item()?)?;
        let ast_id = self.source_ast_id_map.ast_id(item);
        Some(
            self.tree
                .imports
                .alloc(ImportStatement { kind, tree, ast_id })
                .into(),
        )
    }

    fn lower_import_tree(
        &mut self,
        import_tree: &syntax::ast::ImportTree,
    ) -> Option<ImportTree> {
        Some(match import_tree {
            syntax::ast::ImportTree::ImportPath(import_path) => ImportTree::Path {
                name: import_path.name()?.text().into(),
                item: Box::new(self.lower_import_tree(&import_path.item()?)?),
            },
            syntax::ast::ImportTree::ImportItem(import_item) => ImportTree::Item {
                name: import_item.name()?.text().into(),
                alias: import_item
                    .alias()
                    .map(|alias| item_tree::Name::from(alias.text())),
            },
            syntax::ast::ImportTree::ImportCollection(import_collection) => {
                ImportTree::Collection {
                    list: import_collection
                        .items()
                        .filter_map(|item| self.lower_import_tree(&item))
                        .collect(),
                }
            },
        })
    }

    fn lower_type_alias(
        &mut self,
        type_alias: &syntax::ast::TypeAliasDeclaration,
    ) -> Option<ModuleItemId<TypeAlias>> {
        let name = type_alias.name()?.text().into();
        let ast_id = self.source_ast_id_map.ast_id(type_alias);
        Some(
            self.tree
                .type_aliases
                .alloc(TypeAlias { name, ast_id })
                .into(),
        )
    }

    fn lower_override(
        &mut self,
        override_declaration: &syntax::ast::OverrideDeclaration,
    ) -> Option<ModuleItemId<Override>> {
        let name = override_declaration.name()?.text().into();
        let ast_id = self.source_ast_id_map.ast_id(override_declaration);

        let override_declaration = Override { name, ast_id };
        Some(self.tree.overrides.alloc(override_declaration).into())
    }

    fn lower_global_constant(
        &mut self,
        constant: &syntax::ast::ConstantDeclaration,
    ) -> Option<ModuleItemId<GlobalConstant>> {
        let name = constant.name()?.text().into();
        let ast_id = self.source_ast_id_map.ast_id(constant);
        let constant = GlobalConstant { name, ast_id };
        Some(self.tree.global_constants.alloc(constant).into())
    }

    fn lower_global_variable(
        &mut self,
        variable: &syntax::ast::VariableDeclaration,
    ) -> Option<ModuleItemId<GlobalVariable>> {
        let name = variable.name()?.text().into();
        let ast_id = self.source_ast_id_map.ast_id(variable);
        let variable = GlobalVariable { name, ast_id };
        Some(self.tree.global_variables.alloc(variable).into())
    }

    fn lower_struct(
        &mut self,
        r#struct: &syntax::ast::StructDeclaration,
    ) -> Option<ModuleItemId<Struct>> {
        let name = r#struct.name()?.text().into();
        let ast_id = self.source_ast_id_map.ast_id(r#struct);
        let r#struct = Struct { name, ast_id };
        Some(self.tree.structs.alloc(r#struct).into())
    }

    fn lower_function(
        &mut self,
        function: &syntax::ast::FunctionDeclaration,
    ) -> Option<ModuleItemId<Function>> {
        let name = function.name()?.text().into();
        let ast_id = self.source_ast_id_map.ast_id(function);
        let function = Function { name, ast_id };
        Some(self.tree.functions.alloc(function).into())
    }

    #[expect(
        clippy::unnecessary_wraps,
        reason = "Maintain uniformity with the other lower_* functions"
    )]
    fn lower_global_assert_statement(
        &mut self,
        assert_statement: &syntax::ast::AssertStatement,
    ) -> Option<ModuleItemId<GlobalAssertStatement>> {
        let ast_id = self.source_ast_id_map.ast_id(assert_statement);
        let assert_statement = GlobalAssertStatement { ast_id };
        Some(
            self.tree
                .global_assert_statements
                .alloc(assert_statement)
                .into(),
        )
    }
}
