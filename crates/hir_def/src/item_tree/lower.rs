use base_db::EditionedFileId;
use syntax::{
    HasName as _,
    ast::{Directive, Item, SourceFile},
};
use triomphe::Arc;

use super::{GlobalConstant, GlobalVariable, Override, Struct, TypeAlias};
use crate::{
    ast_id::AstIdMap,
    database::DefDatabase,
    item_tree::{
        self, BigModItem, Function, GlobalAssertStatement, ImportStatement, ImportTree, ItemTree,
        ItemTreeAstId, ModuleItemId, SmallModItem,
    },
    mod_path::{ModPath, PathKind},
};

pub(crate) struct Ctx<'database> {
    database: &'database dyn DefDatabase,
    file_id: EditionedFileId,
    source_ast_id_map: Arc<AstIdMap>,
    pub(crate) tree: ItemTree,
    pub(crate) items: Vec<ModuleItemId>,
}

impl<'database> Ctx<'database> {
    pub(crate) fn new(
        database: &'database dyn DefDatabase,
        file_id: EditionedFileId,
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
        self.tree.top_level = self.items;
        self.tree
    }

    fn lower_item(
        &mut self,
        item: Item,
    ) -> Option<()> {
        let item = match item {
            Item::ImportStatement(import_statement) => {
                ModuleItemId::ImportStatement(self.lower_import(&import_statement)?)
            },
            Item::FunctionDeclaration(function) => {
                ModuleItemId::Function(self.lower_function(&function)?)
            },
            Item::StructDeclaration(r#struct) => {
                ModuleItemId::Struct(self.lower_struct(&r#struct)?)
            },
            Item::VariableDeclaration(variable) => {
                ModuleItemId::GlobalVariable(self.lower_global_variable(&variable)?)
            },
            Item::ConstantDeclaration(constant) => {
                ModuleItemId::GlobalConstant(self.lower_global_constant(&constant)?)
            },
            Item::OverrideDeclaration(override_declaration) => {
                ModuleItemId::Override(self.lower_override(&override_declaration)?)
            },
            Item::TypeAliasDeclaration(type_alias) => {
                ModuleItemId::TypeAlias(self.lower_type_alias(&type_alias)?)
            },
            Item::AssertStatement(assert_statement) => ModuleItemId::GlobalAssertStatement(
                self.lower_global_assert_statement(&assert_statement)?,
            ),
        };
        self.items.push(item);
        Some(())
    }

    fn lower_import(
        &mut self,
        item: &syntax::ast::ImportStatement,
    ) -> Option<ItemTreeAstId<ImportStatement>> {
        let kind = PathKind::from_src(item.relative());
        let tree = Self::lower_import_tree(&item.item()?)?;
        let ast_id = self.source_ast_id_map.ast_id(item);
        let import_statement = ImportStatement { kind, tree };
        self.tree.big_data.insert(
            ast_id.upcast(),
            BigModItem::ImportStatement(import_statement),
        );
        Some(ast_id)
    }

    fn lower_import_tree(import_tree: &syntax::ast::ImportTree) -> Option<ImportTree> {
        Some(match import_tree {
            syntax::ast::ImportTree::ImportPath(import_path) => ImportTree::Path {
                name: import_path.name()?.text().into(),
                item: Box::new(Self::lower_import_tree(&import_path.item()?)?),
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
                        .filter_map(|item| Self::lower_import_tree(&item))
                        .collect(),
                }
            },
        })
    }

    fn lower_type_alias(
        &mut self,
        type_alias: &syntax::ast::TypeAliasDeclaration,
    ) -> Option<ItemTreeAstId<TypeAlias>> {
        let name = type_alias.name()?.text().into();
        let ast_id = self.source_ast_id_map.ast_id(type_alias);
        let type_alias = TypeAlias { name };
        self.tree
            .small_data
            .insert(ast_id.upcast(), SmallModItem::TypeAlias(type_alias));
        Some(ast_id)
    }

    fn lower_override(
        &mut self,
        override_declaration: &syntax::ast::OverrideDeclaration,
    ) -> Option<ItemTreeAstId<Override>> {
        let name = override_declaration.name()?.text().into();
        let ast_id = self.source_ast_id_map.ast_id(override_declaration);
        let override_declaration = Override { name };
        self.tree.small_data.insert(
            ast_id.upcast(),
            SmallModItem::Override(override_declaration),
        );
        Some(ast_id)
    }

    fn lower_global_constant(
        &mut self,
        constant: &syntax::ast::ConstantDeclaration,
    ) -> Option<ItemTreeAstId<GlobalConstant>> {
        let name = constant.name()?.text().into();
        let ast_id = self.source_ast_id_map.ast_id(constant);
        let constant = GlobalConstant { name };
        self.tree
            .small_data
            .insert(ast_id.upcast(), SmallModItem::GlobalConstant(constant));
        Some(ast_id)
    }

    fn lower_global_variable(
        &mut self,
        variable: &syntax::ast::VariableDeclaration,
    ) -> Option<ItemTreeAstId<GlobalVariable>> {
        let name = variable.name()?.text().into();
        let ast_id = self.source_ast_id_map.ast_id(variable);
        let variable = GlobalVariable { name };
        self.tree
            .small_data
            .insert(ast_id.upcast(), SmallModItem::GlobalVariable(variable));

        Some(ast_id)
    }

    fn lower_struct(
        &mut self,
        r#struct: &syntax::ast::StructDeclaration,
    ) -> Option<ItemTreeAstId<Struct>> {
        let name = r#struct.name()?.text().into();
        let ast_id = self.source_ast_id_map.ast_id(r#struct);
        let r#struct = Struct { name };
        self.tree
            .small_data
            .insert(ast_id.upcast(), SmallModItem::Struct(r#struct));
        Some(ast_id)
    }

    fn lower_function(
        &mut self,
        function: &syntax::ast::FunctionDeclaration,
    ) -> Option<ItemTreeAstId<Function>> {
        let name = function.name()?.text().into();
        let ast_id = self.source_ast_id_map.ast_id(function);
        let function = Function { name };
        self.tree
            .small_data
            .insert(ast_id.upcast(), SmallModItem::Function(function));
        Some(ast_id)
    }

    #[expect(
        clippy::unnecessary_wraps,
        reason = "Maintain uniformity with the other lower_* functions"
    )]
    fn lower_global_assert_statement(
        &mut self,
        assert_statement: &syntax::ast::AssertStatement,
    ) -> Option<ItemTreeAstId<GlobalAssertStatement>> {
        let ast_id = self.source_ast_id_map.ast_id(assert_statement);
        let assert_statement = GlobalAssertStatement {};
        self.tree.small_data.insert(
            ast_id.upcast(),
            SmallModItem::GlobalAssertStatement(assert_statement),
        );

        Some(ast_id)
    }
}
