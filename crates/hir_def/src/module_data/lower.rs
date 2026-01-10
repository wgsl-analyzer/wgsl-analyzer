use syntax::{
    HasName as _,
    ast::{Item, SourceFile},
};
use triomphe::Arc;

use super::{GlobalConstant, GlobalVariable, Override, Struct, TypeAlias};
use crate::{
    HirFileId,
    ast_id::AstIdMap,
    database::DefDatabase,
    module_data::{Function, GlobalAssertStatement, ModuleData, ModuleItem, ModuleItemId},
};

pub(crate) struct Ctx<'database> {
    database: &'database dyn DefDatabase,
    file_id: HirFileId,
    source_ast_id_map: Arc<AstIdMap>,
    pub(crate) module_data: ModuleData,
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
            module_data: ModuleData::default(),
            items: vec![],
        }
    }

    pub(crate) fn lower_source_file(
        &mut self,
        source_file: &SourceFile,
    ) {
        source_file.items().for_each(|item| {
            self.lower_item(item);
        });
    }

    fn lower_item(
        &mut self,
        item: Item,
    ) -> Option<()> {
        let item = match item {
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

    fn lower_type_alias(
        &mut self,
        type_alias: &syntax::ast::TypeAliasDeclaration,
    ) -> Option<ModuleItemId<TypeAlias>> {
        let name = type_alias.name()?.text().into();
        let ast_id = self.source_ast_id_map.ast_id(type_alias);
        Some(
            self.module_data
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
        Some(
            self.module_data
                .overrides
                .alloc(override_declaration)
                .into(),
        )
    }

    fn lower_global_constant(
        &mut self,
        constant: &syntax::ast::ConstantDeclaration,
    ) -> Option<ModuleItemId<GlobalConstant>> {
        let name = constant.name()?.text().into();
        let ast_id = self.source_ast_id_map.ast_id(constant);
        let constant = GlobalConstant { name, ast_id };
        Some(self.module_data.global_constants.alloc(constant).into())
    }

    fn lower_global_variable(
        &mut self,
        variable: &syntax::ast::VariableDeclaration,
    ) -> Option<ModuleItemId<GlobalVariable>> {
        let name = variable.name()?.text().into();
        let ast_id = self.source_ast_id_map.ast_id(variable);
        let variable = GlobalVariable { name, ast_id };
        Some(self.module_data.global_variables.alloc(variable).into())
    }

    fn lower_struct(
        &mut self,
        r#struct: &syntax::ast::StructDeclaration,
    ) -> Option<ModuleItemId<Struct>> {
        let name = r#struct.name()?.text().into();
        let ast_id = self.source_ast_id_map.ast_id(r#struct);
        let r#struct = Struct { name, ast_id };
        Some(self.module_data.structs.alloc(r#struct).into())
    }

    fn lower_function(
        &mut self,
        function: &syntax::ast::FunctionDeclaration,
    ) -> Option<ModuleItemId<Function>> {
        let name = function.name()?.text().into();
        let ast_id = self.source_ast_id_map.ast_id(function);
        let function = Function { name, ast_id };
        Some(self.module_data.functions.alloc(function).into())
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
            self.module_data
                .global_assert_statements
                .alloc(assert_statement)
                .into(),
        )
    }
}
