use crate::module_data::{Function, ModuleData, ModuleItem, ModuleItemId, Param};
use crate::HirFileId;
use crate::{ast_id::AstIdMap, db::DefDatabase, type_ref::TypeRef};
use la_arena::{Idx, IdxRange};
use std::sync::Arc;
use syntax::ast::{self, Item, SourceFile};
use syntax::{AstNode, HasName};

use super::{Field, GlobalConstant, GlobalVariable, Import, ImportValue, Name, Struct, TypeAlias};

pub(crate) struct Ctx<'a> {
    db: &'a dyn DefDatabase,
    source_ast_id_map: Arc<AstIdMap>,
    pub module_data: ModuleData,
    pub items: Vec<ModuleItem>,
}

impl<'a> Ctx<'a> {
    pub(crate) fn new(db: &'a dyn DefDatabase, file_id: HirFileId) -> Self {
        Self {
            db,
            source_ast_id_map: db.ast_id_map(file_id),
            module_data: ModuleData::default(),
            items: vec![],
        }
    }

    pub(crate) fn lower_source_file(&mut self, source_file: SourceFile) {
        source_file.items().for_each(|item| {
            self.lower_item(item);
        })
    }
    fn lower_item(&mut self, item: Item) -> Option<()> {
        let item = match item {
            Item::Function(function) => ModuleItem::Function(self.lower_function(&function)?),
            Item::StructDecl(strukt) => ModuleItem::Struct(self.lower_struct(&strukt)?),
            Item::GlobalVariableDecl(var) => {
                ModuleItem::GlobalVariable(self.lower_global_var(&var)?)
            }
            Item::GlobalConstantDecl(constant) => {
                ModuleItem::GlobalConstant(self.lower_global_constant(&constant)?)
            }
            Item::Import(import) => ModuleItem::Import(self.lower_import(&import)?),
            Item::TypeAliasDecl(type_alias) => {
                ModuleItem::TypeAlias(self.lower_type_alias(&type_alias)?)
            }
        };
        self.items.push(item);
        Some(())
    }

    fn lower_import(&mut self, import: &syntax::ast::Import) -> Option<ModuleItemId<Import>> {
        let ast_id = self.source_ast_id_map.ast_id(import);

        let value = match import.import()? {
            ast::ImportKind::ImportPath(path) => {
                ImportValue::Path(path.string_literal()?.text().to_string())
            }
            ast::ImportKind::ImportCustom(custom) => ImportValue::Custom(custom.key()),
        };

        let import = Import { value, ast_id };

        Some(self.module_data.imports.alloc(import).into())
    }

    fn lower_type_alias(
        &mut self,
        type_alias: &syntax::ast::TypeAliasDecl,
    ) -> Option<ModuleItemId<TypeAlias>> {
        let name = type_alias.name()?.text().into();

        let ty = type_alias
            .type_decl()
            .and_then(|type_decl| self.lower_type_ref(type_decl))
            .unwrap_or(TypeRef::Error);

        let ty = self.db.intern_type_ref(ty);

        let ast_id = self.source_ast_id_map.ast_id(type_alias);
        Some(
            self.module_data
                .type_aliases
                .alloc(TypeAlias { name, ast_id, ty })
                .into(),
        )
    }

    fn lower_global_constant(
        &mut self,
        constant: &syntax::ast::GlobalConstantDecl,
    ) -> Option<ModuleItemId<GlobalConstant>> {
        let name = constant.binding()?.name()?.text().into();
        let ast_id = self.source_ast_id_map.ast_id(constant);

        let ty = constant
            .ty()
            .map(|type_decl| self.lower_type_ref(type_decl).unwrap_or(TypeRef::Error))
            .map(|ty| self.db.intern_type_ref(ty));

        let constant = GlobalConstant { name, ty, ast_id };
        Some(self.module_data.global_constants.alloc(constant).into())
    }
    fn lower_global_var(
        &mut self,
        var: &syntax::ast::GlobalVariableDecl,
    ) -> Option<ModuleItemId<GlobalVariable>> {
        let name = var.binding()?.name()?.text().into();
        let ast_id = self.source_ast_id_map.ast_id(var);

        let ty = var
            .ty()
            .and_then(|type_decl| self.lower_type_ref(type_decl))
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

    fn lower_struct(&mut self, strukt: &syntax::ast::StructDecl) -> Option<ModuleItemId<Struct>> {
        let name = strukt.name()?.text().into();
        let ast_id = self.source_ast_id_map.ast_id(strukt);

        let start_field = self.next_field_idx();
        strukt
            .body()?
            .fields()
            .map(|field| {
                let decl = field.variable_ident_decl()?;
                let name = Name::from(decl.binding()?.name()?);
                let ty = self.lower_type_ref(decl.ty()?).unwrap_or(TypeRef::Error);
                let ty = self.db.intern_type_ref(ty);
                self.module_data.fields.alloc(Field { name, ty });
                Some(())
            })
            .for_each(drop);
        let end_field = self.next_field_idx();

        let strukt = Struct {
            name,
            fields: IdxRange::new(start_field..end_field),
            ast_id,
        };
        Some(self.module_data.structs.alloc(strukt).into())
    }

    fn lower_function(
        &mut self,
        function: &syntax::ast::Function,
    ) -> Option<ModuleItemId<Function>> {
        let name = function.name()?.text().into();

        let ast_id = self.source_ast_id_map.ast_id(function);

        let start_param = self.next_param_idx();
        self.lower_function_param_list(function.param_list()?);
        let end_param = self.next_param_idx();
        let params = IdxRange::new(start_param..end_param);

        let return_type = function
            .return_type()
            .and_then(|ty| ty.ty())
            .map(|ty| self.lower_type_ref(ty).unwrap_or(TypeRef::Error))
            .map(|ty| self.db.intern_type_ref(ty));

        let function = Function {
            name,
            params,
            ast_id,
            return_type,
        };

        Some(self.module_data.functions.alloc(function).into())
    }

    fn lower_function_param_list(&mut self, function_param_list: ast::ParamList) -> Option<()> {
        for param in function_param_list.params() {
            if let Some(param) = param.variable_ident_declaration() {
                let ty = param
                    .ty()
                    .and_then(|ty| self.lower_type_ref(ty))
                    .unwrap_or(TypeRef::Error);
                let ty = self.db.intern_type_ref(ty);
                let name = param
                    .binding()
                    .and_then(|binding| binding.name())
                    .map_or_else(Name::missing, Name::from);
                self.module_data.params.alloc(Param { ty, name });
            } else if let Some(import) = param.import() {
                let import = self.lower_import(&import)?;
                let import = &self.module_data.imports[import.index];
                let parse = match &import.value {
                    crate::module_data::ImportValue::Path(_) => Err(()), // TODO: path imports
                    crate::module_data::ImportValue::Custom(key) => self
                        .db
                        .parse_import(key.clone(), syntax::ParseEntryPoint::FnParamList),
                };
                if let Ok(parse) = parse {
                    let param_list = ast::ParamList::cast(parse.syntax())?;
                    self.lower_function_param_list(param_list)?;
                }
            }
        }

        Some(())
    }

    fn lower_type_ref(&self, ty: ast::Type) -> Option<TypeRef> {
        ty.try_into().ok()
    }

    fn next_param_idx(&self) -> Idx<Param> {
        let idx = self.module_data.params.len() as u32;
        Idx::from_raw(la_arena::RawIdx::from(idx))
    }
    fn next_field_idx(&self) -> Idx<Field> {
        let idx = self.module_data.fields.len() as u32;
        Idx::from_raw(la_arena::RawIdx::from(idx))
    }
}
