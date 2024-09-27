use std::sync::Arc;

use la_arena::{Arena, Idx};

use crate::{
    db::{
        DefDatabase, FunctionId, GlobalConstantId, GlobalVariableId, Interned, Lookup, OverrideId,
        StructId, TypeAliasId,
    },
    module_data::Name,
    type_ref::{AccessMode, StorageClass, TypeRef},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionData {
    pub name: Name,
    pub params: Vec<(Interned<TypeRef>, Name)>,
    pub return_type: Option<Interned<TypeRef>>,
}

impl FunctionData {
    pub fn fn_data_query(db: &dyn DefDatabase, func: FunctionId) -> Arc<FunctionData> {
        let loc = func.lookup(db);
        let module_info = db.module_info(loc.file_id);
        let function = &module_info.data[loc.value.index];

        Arc::new(FunctionData {
            name: function.name.clone(),
            params: function
                .params
                .clone()
                .map(|param| {
                    let param = &module_info.data[param];
                    (param.ty, param.name.clone())
                })
                .collect(),
            return_type: function.return_type,
        })
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub struct FieldId {
    pub strukt: StructId,
    pub field: LocalFieldId,
}
pub type LocalFieldId = Idx<FieldData>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructData {
    pub name: Name,
    pub fields: Arena<FieldData>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldData {
    pub name: Name,
    pub ty: Interned<TypeRef>,
}

impl StructData {
    pub fn struct_data_query(db: &dyn DefDatabase, func: StructId) -> Arc<StructData> {
        let loc = func.lookup(db);
        let module_info = db.module_info(loc.file_id);
        let strukt = &module_info.data[loc.value.index];

        let mut fields = Arena::new();
        strukt
            .fields
            .clone()
            .map(|field| &module_info.data[field])
            .map(|field| FieldData {
                name: field.name.clone(),
                ty: field.ty,
            })
            .for_each(|field| {
                fields.alloc(field);
            });

        Arc::new(StructData {
            name: strukt.name.clone(),
            fields,
        })
    }
    pub fn fields(&self) -> &Arena<FieldData> {
        &self.fields
    }

    pub fn field(&self, name: &Name) -> Option<LocalFieldId> {
        self.fields()
            .iter()
            .find_map(|(id, data)| if &data.name == name { Some(id) } else { None })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeAliasData {
    pub name: Name,
    pub ty: Interned<TypeRef>,
}

impl TypeAliasData {
    pub fn type_alias_data_query(db: &dyn DefDatabase, func: TypeAliasId) -> Arc<TypeAliasData> {
        let loc = func.lookup(db);
        let module_info = db.module_info(loc.file_id);
        let type_alias = &module_info.data[loc.value.index];

        Arc::new(TypeAliasData {
            name: type_alias.name.clone(),
            ty: type_alias.ty,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlobalVariableData {
    pub name: Name,
    pub ty: Option<Interned<TypeRef>>,
    pub storage_class: Option<StorageClass>,
    pub access_mode: Option<AccessMode>,
}
impl GlobalVariableData {
    pub fn global_var_data_query(
        db: &dyn DefDatabase,
        var: GlobalVariableId,
    ) -> Arc<GlobalVariableData> {
        let loc = db.lookup_intern_global_variable(var);
        let module_info = db.module_info(loc.file_id);
        let var = &module_info.data[loc.value.index];

        Arc::new(GlobalVariableData {
            name: var.name.clone(),
            ty: var.ty,
            storage_class: var.storage_class,
            access_mode: var.access_mode,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlobalConstantData {
    pub name: Name,
    pub ty: Option<Interned<TypeRef>>,
}
impl GlobalConstantData {
    pub fn global_constant_data_query(
        db: &dyn DefDatabase,
        constant: GlobalConstantId,
    ) -> Arc<GlobalConstantData> {
        let loc = db.lookup_intern_global_constant(constant);
        let module_info = db.module_info(loc.file_id);
        let constant = &module_info.data[loc.value.index];

        Arc::new(GlobalConstantData {
            name: constant.name.clone(),
            ty: constant.ty,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OverrideData {
    pub name: Name,
    pub ty: Option<Interned<TypeRef>>,
}
impl OverrideData {
    pub fn override_data_query(
        db: &dyn DefDatabase,
        override_decl: OverrideId,
    ) -> Arc<OverrideData> {
        let loc = db.lookup_intern_override(override_decl);
        let module_info = db.module_info(loc.file_id);
        let constant = &module_info.data[loc.value.index];

        Arc::new(OverrideData {
            name: constant.name.clone(),
            ty: constant.ty,
        })
    }
}
