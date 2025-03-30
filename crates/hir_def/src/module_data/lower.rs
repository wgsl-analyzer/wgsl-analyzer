use crate::HirFileId;
use crate::hir_file_id::relative_file;
use crate::module_data::{Function, ModuleData, ModuleItem, ModuleItemId, Parameter};
use crate::{ast_id::AstIdMap, db::DefDatabase, type_ref::TypeReference};
use la_arena::{Idx, IdxRange};
use std::sync::Arc;

use syntax::{
    AstNode, HasName,
    ast::{self, Item, SourceFile},
};

use super::{
    Field, GlobalConstant, GlobalVariable, Import, ImportValue, Name, Override, Struct, TypeAlias,
};

pub(crate) struct Ctx<'a> {
    db: &'a dyn DefDatabase,
    file_id: HirFileId,
    source_ast_id_map: Arc<AstIdMap>,
    pub module_data: ModuleData,
    pub items: Vec<ModuleItem>,
}

impl<'a> Ctx<'a> {
    pub(crate) fn new(
        db: &'a dyn DefDatabase,
        file_id: HirFileId,
    ) -> Self {
        Self {
            db,
            file_id,
            source_ast_id_map: db.ast_id_map(file_id),
            module_data: ModuleData::default(),
            items: vec![],
        }
    }

    pub(crate) fn lower_source_file(
        &mut self,
        source_file: SourceFile,
    ) {
        source_file.items().for_each(|item| {
            self.lower_item(item);
        })
    }

    fn lower_item(
        &mut self,
        item: Item,
    ) -> Option<()> {
        let item = match item {
            Item::Function(function) => ModuleItem::Function(self.lower_function(&function)?),
            Item::StructDeclaration(r#struct) => ModuleItem::Struct(self.lower_struct(&r#struct)?),
            Item::GlobalVariableDeclaration(var) => {
                ModuleItem::GlobalVariable(self.lower_global_var(&var)?)
            },
            Item::GlobalConstantDeclaration(constant) => {
                ModuleItem::GlobalConstant(self.lower_global_constant(&constant)?)
            },
            Item::OverrideDeclaration(override_declaration) => {
                ModuleItem::Override(self.lower_override(&override_declaration)?)
            },
            Item::Import(import) => ModuleItem::Import(self.lower_import(&import)?),
            Item::TypeAliasDeclaration(type_alias) => {
                ModuleItem::TypeAlias(self.lower_type_alias(&type_alias)?)
            },
        };
        self.items.push(item);
        Some(())
    }

    fn lower_import(
        &mut self,
        import: &syntax::ast::Import,
    ) -> Option<ModuleItemId<Import>> {
        let ast_id = self.source_ast_id_map.ast_id(import);

        let value = match import.import()? {
            ast::ImportKind::ImportPath(path) => {
                let import_path = path
                    .string_literal()?
                    .text()
                    .chars()
                    .filter(|&c| c != '"')
                    .collect();
                ImportValue::Path(import_path)
            },
            ast::ImportKind::ImportCustom(custom) => ImportValue::Custom(custom.key()),
        };

        let import = Import { value, ast_id };

        Some(self.module_data.imports.alloc(import).into())
    }

    fn lower_type_alias(
        &mut self,
        type_alias: &syntax::ast::TypeAliasDeclaration,
    ) -> Option<ModuleItemId<TypeAlias>> {
        let name = type_alias.name()?.text().into();

        let ty = type_alias
            .type_declaration()
            .and_then(|type_declaration| self.lower_type_ref(type_declaration))
            .unwrap_or(TypeReference::Error);

        let ty = self.db.intern_type_ref(ty);

        let ast_id = self.source_ast_id_map.ast_id(type_alias);
        Some(
            self.module_data
                .type_aliases
                .alloc(TypeAlias { name, ast_id, ty })
                .into(),
        )
    }

    fn lower_override(
        &mut self,
        override_declaration: &syntax::ast::OverrideDeclaration,
    ) -> Option<ModuleItemId<Override>> {
        let name = override_declaration.binding()?.name()?.text().into();
        let ast_id = self.source_ast_id_map.ast_id(override_declaration);

        let ty = override_declaration
            .ty()
            .map(|type_declaration| {
                self.lower_type_ref(type_declaration)
                    .unwrap_or(TypeReference::Error)
            })
            .map(|ty| self.db.intern_type_ref(ty));

        let override_declaration = Override { name, ty, ast_id };
        Some(
            self.module_data
                .overrides
                .alloc(override_declaration)
                .into(),
        )
    }

    fn lower_global_constant(
        &mut self,
        constant: &syntax::ast::GlobalConstantDeclaration,
    ) -> Option<ModuleItemId<GlobalConstant>> {
        let name = constant.binding()?.name()?.text().into();
        let ast_id = self.source_ast_id_map.ast_id(constant);

        let ty = constant
            .ty()
            .map(|type_declaration| {
                self.lower_type_ref(type_declaration)
                    .unwrap_or(TypeReference::Error)
            })
            .map(|ty| self.db.intern_type_ref(ty));

        let constant = GlobalConstant { name, ty, ast_id };
        Some(self.module_data.global_constants.alloc(constant).into())
    }

    fn lower_global_var(
        &mut self,
        var: &syntax::ast::GlobalVariableDeclaration,
    ) -> Option<ModuleItemId<GlobalVariable>> {
        let name = var.binding()?.name()?.text().into();
        let ast_id = self.source_ast_id_map.ast_id(var);

        let ty = var
            .ty()
            .and_then(|type_declaration| self.lower_type_ref(type_declaration))
            .map(|ty| self.db.intern_type_ref(ty));

        let storage_class = var
            .variable_qualifier()
            .and_then(|qualifier| qualifier.storage_class())
            .map(Into::into);
        let access_mode = var
            .variable_qualifier()
            .and_then(|qualifier| qualifier.access_mode())
            .map(Into::into);

        let var = GlobalVariable {
            name,
            ty,
            ast_id,
            storage_class,
            access_mode,
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
                let declaration = field.variable_ident_declaration()?;
                let name = Name::from(declaration.binding()?.name()?);
                let ty = self
                    .lower_type_ref(declaration.ty()?)
                    .unwrap_or(TypeReference::Error);
                let ty = self.db.intern_type_ref(ty);
                self.module_data.fields.alloc(Field { name, ty });
                Some(())
            })
            .for_each(drop);
        let end_field = self.next_field_index();

        let r#struct = Struct {
            name,
            fields: IdxRange::new(start_field..end_field),
            ast_id,
        };
        Some(self.module_data.structs.alloc(r#struct).into())
    }

    fn lower_function(
        &mut self,
        function: &syntax::ast::Function,
    ) -> Option<ModuleItemId<Function>> {
        let name = function.name()?.text().into();

        let ast_id = self.source_ast_id_map.ast_id(function);

        let start_parameter = self.next_param_index();
        self.lower_function_param_list(function.parameter_list()?);
        let end_parameter = self.next_param_index();
        let parameters = IdxRange::new(start_parameter..end_parameter);

        let return_type = function
            .return_type()
            .and_then(|ty| ty.ty())
            .map(|ty| self.lower_type_ref(ty).unwrap_or(TypeReference::Error))
            .map(|ty| self.db.intern_type_ref(ty));

        let function = Function {
            name,
            parameters,
            ast_id,
            return_type,
        };

        Some(self.module_data.functions.alloc(function).into())
    }

    fn lower_function_param_list(
        &mut self,
        function_param_list: ast::ParameterList,
    ) -> Option<()> {
        for parameter in function_param_list.parameters() {
            if let Some(parameter) = parameter.variable_ident_declaration() {
                let ty = parameter
                    .ty()
                    .and_then(|ty| self.lower_type_ref(ty))
                    .unwrap_or(TypeReference::Error);
                let ty = self.db.intern_type_ref(ty);
                let name = parameter
                    .binding()
                    .and_then(|binding| binding.name())
                    .map_or_else(Name::missing, Name::from);
                self.module_data.parameters.alloc(Parameter { ty, name });
            } else if let Some(import) = parameter.import() {
                let import = self.lower_import(&import)?;
                let import = &self.module_data.imports[import.index];
                let parse = match &import.value {
                    crate::module_data::ImportValue::Path(path) => {
                        tracing::info!("attempted import {:?}", path);
                        let file_id = relative_file(self.db, self.file_id, path)?;
                        Ok(self.db.parse(file_id))
                    },
                    crate::module_data::ImportValue::Custom(key) => self
                        .db
                        .parse_import(key.clone(), syntax::ParseEntryPoint::FunctionParameterList),
                };
                if let Ok(parse) = parse {
                    let param_list = ast::ParameterList::cast(parse.syntax())?;
                    self.lower_function_param_list(param_list)?;
                }
            }
        }

        Some(())
    }

    fn lower_type_ref(
        &self,
        ty: ast::Type,
    ) -> Option<TypeReference> {
        ty.try_into().ok()
    }

    fn next_param_index(&self) -> Idx<Parameter> {
        let index = self.module_data.parameters.len() as u32;
        Idx::from_raw(la_arena::RawIdx::from(index))
    }

    fn next_field_index(&self) -> Idx<Field> {
        let index = self.module_data.fields.len() as u32;
        Idx::from_raw(la_arena::RawIdx::from(index))
    }
}
