use crate::HirFileId;
use crate::hir_file_id::relative_file;
use crate::module_data::{Function, ModuleData, ModuleItem, ModuleItemId, Parameter};
use crate::{ast_id::AstIdMap, database::DefDatabase, type_ref::TypeReference};
use la_arena::{Idx, IdxRange};
use std::sync::Arc;

use syntax::{
    AstNode as _, HasName as _,
    ast::{self, Item, SourceFile},
};

use super::{Field, GlobalConstant, GlobalVariable, Name, Override, Struct, TypeAlias};

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
            Item::VariableDeclaration(var) => {
                ModuleItem::GlobalVariable(self.lower_global_var(&var)?)
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
        };
        self.items.push(item);
        Some(())
    }

    fn lower_type_alias(
        &mut self,
        type_alias: &syntax::ast::TypeAliasDeclaration,
    ) -> Option<ModuleItemId<TypeAlias>> {
        let name = type_alias.name()?.text().into();

        let r#type = type_alias
            .type_declaration()
            .and_then(|type_declaration| self.lower_type_ref(type_declaration))
            .unwrap_or(TypeReference::Error);

        let ast_id = self.source_ast_id_map.ast_id(type_alias);
        Some(
            self.module_data
                .type_aliases
                .alloc(TypeAlias {
                    name,
                    r#type,
                    ast_id,
                })
                .into(),
        )
    }

    fn lower_override(
        &mut self,
        override_declaration: &syntax::ast::OverrideDeclaration,
    ) -> Option<ModuleItemId<Override>> {
        let name = override_declaration.name()?.text().into();
        let ast_id = self.source_ast_id_map.ast_id(override_declaration);

        let r#type = override_declaration.ty().map(|type_declaration| {
            self.lower_type_ref(type_declaration)
                .unwrap_or(TypeReference::Error)
        });

        let override_declaration = Override {
            name,
            r#type,
            ast_id,
        };
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

        let r#type = constant.ty().map(|type_declaration| {
            self.lower_type_ref(type_declaration)
                .unwrap_or(TypeReference::Error)
        });

        let constant = GlobalConstant {
            name,
            r#type,
            ast_id,
        };
        Some(self.module_data.global_constants.alloc(constant).into())
    }

    fn lower_global_var(
        &mut self,
        var: &syntax::ast::VariableDeclaration,
    ) -> Option<ModuleItemId<GlobalVariable>> {
        let name = var.name()?.text().into();
        let ast_id = self.source_ast_id_map.ast_id(var);

        let r#type = var
            .ty()
            .and_then(|type_declaration| self.lower_type_ref(type_declaration));

        let var = GlobalVariable {
            name,
            r#type,
            ast_id,
        };
        Some(self.module_data.global_variables.alloc(var).into())
    }

    fn lower_struct(
        &mut self,
        r#struct: &syntax::ast::StructDeclaration,
    ) -> Option<ModuleItemId<Struct>> {
        let name = r#struct.name()?.text().into();
        let ast_id = self.source_ast_id_map.ast_id(r#struct);

        let start_field = self.next_field_index();
        r#struct
            .body()?
            .fields()
            .map(|field| {
                let name = Name::from(field.name()?);
                let r#type = self
                    .lower_type_ref(field.ty()?)
                    .unwrap_or(TypeReference::Error);
                self.module_data.fields.alloc(Field { r#type, name });
                Some(())
            })
            .for_each(drop);
        let end_field = self.next_field_index();

        let r#struct = Struct {
            name,
            ast_id,
            fields: IdxRange::new(start_field..end_field),
        };
        Some(self.module_data.structs.alloc(r#struct).into())
    }

    fn lower_function(
        &mut self,
        function: &syntax::ast::FunctionDeclaration,
    ) -> Option<ModuleItemId<Function>> {
        let name = function.name()?.text().into();

        let ast_id = self.source_ast_id_map.ast_id(function);

        let start_parameter = self.next_param_index();
        self.lower_function_param_list(&function.parameter_list()?);
        let end_parameter = self.next_param_index();
        let parameters = IdxRange::new(start_parameter..end_parameter);

        let return_type = function
            .return_type()
            .and_then(|r#type| r#type.ty())
            .map(|r#type| self.lower_type_ref(r#type).unwrap_or(TypeReference::Error));

        let function = Function {
            name,
            parameters,
            return_type,
            ast_id,
        };

        Some(self.module_data.functions.alloc(function).into())
    }

    fn lower_function_param_list(
        &mut self,
        function_param_list: &ast::FunctionParameters,
    ) -> Option<()> {
        for parameter in function_param_list.parameters() {
            let r#type = parameter
                .ty()
                .and_then(|r#type| self.lower_type_ref(r#type))
                .unwrap_or(TypeReference::Error);
            let name = parameter.name().map_or_else(Name::missing, Name::from);
            self.module_data
                .parameters
                .alloc(Parameter { r#type, name });
        }

        Some(())
    }

    #[expect(clippy::unused_self, reason = "intentional API")]
    fn lower_type_ref(
        &self,
        r#type: ast::TypeSpecifier,
    ) -> Option<TypeReference> {
        r#type.try_into().ok()
    }

    fn next_param_index(&self) -> Idx<Parameter> {
        let index = u32::try_from(self.module_data.parameters.len()).unwrap();
        Idx::from_raw(la_arena::RawIdx::from(index))
    }

    fn next_field_index(&self) -> Idx<Field> {
        let index = u32::try_from(self.module_data.fields.len()).unwrap();
        Idx::from_raw(la_arena::RawIdx::from(index))
    }
}
