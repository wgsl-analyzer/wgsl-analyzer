use std::sync::Arc;

use la_arena::{Arena, Idx};

use crate::{
    database::{
        DefDatabase, FunctionId, GlobalConstantId, GlobalVariableId, Interned, Lookup as _,
        OverrideId, StructId, TypeAliasId,
    },
    module_data::Name,
    type_ref::{AccessMode, AddressSpace, TypeReference},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionData {
    pub name: Name,
    pub parameters: Vec<(Interned<TypeReference>, Name)>,
    pub return_type: Option<Interned<TypeReference>>,
}

impl FunctionData {
    pub fn fn_data_query(
        database: &dyn DefDatabase,
        func: FunctionId,
    ) -> Arc<Self> {
        let loc = func.lookup(database);
        let module_info = database.module_info(loc.file_id);
        let function = &module_info.data[loc.value.index];

        Arc::new(Self {
            name: function.name.clone(),
            parameters: function
                .parameters
                .clone()
                .map(|parameter| {
                    let parameter = &module_info.data[parameter];
                    (parameter.r#type, parameter.name.clone())
                })
                .collect(),
            return_type: function.return_type,
        })
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub struct FieldId {
    pub r#struct: StructId,
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
    pub r#type: Interned<TypeReference>,
}

impl StructData {
    pub fn struct_data_query(
        database: &dyn DefDatabase,
        func: StructId,
    ) -> Arc<Self> {
        let loc = func.lookup(database);
        let module_info = database.module_info(loc.file_id);
        let r#struct = &module_info.data[loc.value.index];

        let mut fields = Arena::new();
        r#struct
            .fields
            .clone()
            .map(|field| &module_info.data[field])
            .map(|field| FieldData {
                name: field.name.clone(),
                r#type: field.r#type,
            })
            .for_each(|field| {
                fields.alloc(field);
            });

        Arc::new(Self {
            name: r#struct.name.clone(),
            fields,
        })
    }

    #[must_use]
    pub const fn fields(&self) -> &Arena<FieldData> {
        &self.fields
    }

    #[must_use]
    pub fn field(
        &self,
        name: &Name,
    ) -> Option<LocalFieldId> {
        self.fields()
            .iter()
            .find_map(|(id, data)| (&data.name == name).then_some(id))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeAliasData {
    pub name: Name,
    pub r#type: Interned<TypeReference>,
}

impl TypeAliasData {
    pub fn type_alias_data_query(
        database: &dyn DefDatabase,
        func: TypeAliasId,
    ) -> Arc<Self> {
        let loc = func.lookup(database);
        let module_info = database.module_info(loc.file_id);
        let type_alias = &module_info.data[loc.value.index];

        Arc::new(Self {
            name: type_alias.name.clone(),
            r#type: type_alias.r#type,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlobalVariableData {
    pub name: Name,
    pub r#type: Option<Interned<TypeReference>>,
    pub address_space: Option<AddressSpace>,
    pub access_mode: Option<AccessMode>,
}

impl GlobalVariableData {
    pub fn global_var_data_query(
        database: &dyn DefDatabase,
        var: GlobalVariableId,
    ) -> Arc<Self> {
        let loc = database.lookup_intern_global_variable(var);
        let module_info = database.module_info(loc.file_id);
        let var = &module_info.data[loc.value.index];

        Arc::new(Self {
            name: var.name.clone(),
            r#type: var.r#type,
            address_space: var.address_space,
            access_mode: var.access_mode,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlobalConstantData {
    pub name: Name,
    pub r#type: Option<Interned<TypeReference>>,
}

impl GlobalConstantData {
    pub fn global_constant_data_query(
        database: &dyn DefDatabase,
        constant: GlobalConstantId,
    ) -> Arc<Self> {
        let loc = database.lookup_intern_global_constant(constant);
        let module_info = database.module_info(loc.file_id);
        let constant = &module_info.data[loc.value.index];

        Arc::new(Self {
            name: constant.name.clone(),
            r#type: constant.r#type,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OverrideData {
    pub name: Name,
    pub r#type: Option<Interned<TypeReference>>,
}

impl OverrideData {
    pub fn override_data_query(
        database: &dyn DefDatabase,
        override_decl: OverrideId,
    ) -> Arc<Self> {
        let loc = database.lookup_intern_override(override_decl);
        let module_info = database.module_info(loc.file_id);
        let constant = &module_info.data[loc.value.index];

        Arc::new(Self {
            name: constant.name.clone(),
            r#type: constant.r#type,
        })
    }
}
