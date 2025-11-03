use la_arena::{Arena, Idx};
use triomphe::Arc;

use crate::{
    HasSource as _,
    database::{
        DefDatabase, FunctionId, GlobalConstantId, GlobalVariableId, Interned, Lookup as _,
        OverrideId, StructId, TypeAliasId,
    },
    expression::ExpressionId,
    expression_store::{
        ExpressionSourceMap, ExpressionStore,
        lower::{
            lower_constant, lower_function, lower_override, lower_struct, lower_type_alias,
            lower_variable,
        },
    },
    module_data::Name,
    type_specifier::{TypeSpecifier, TypeSpecifierId},
};

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub struct ParameterId {
    pub function: FunctionId,
    pub param: LocalParameterId,
}

pub type LocalParameterId = Idx<ParamData>;

#[derive(Debug, PartialEq, Eq)]
pub struct FunctionData {
    pub name: Name,
    pub store: Arc<ExpressionStore>,
    pub parameters: Arena<ParamData>,
    pub return_type: Option<TypeSpecifierId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParamData {
    pub name: Name,
    pub r#type: TypeSpecifierId,
}

impl FunctionData {
    pub fn query(
        database: &dyn DefDatabase,
        func: FunctionId,
    ) -> (Arc<Self>, Arc<ExpressionSourceMap>) {
        let loc = func.lookup(database);
        let source = loc.source(database);
        let (function_data, source_map) = lower_function(database, &source);
        (Arc::new(function_data), Arc::new(source_map))
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub struct FieldId {
    pub r#struct: StructId,
    pub field: LocalFieldId,
}

pub type LocalFieldId = Idx<FieldData>;

#[derive(Debug, PartialEq, Eq)]
pub struct StructData {
    pub name: Name,
    pub store: Arc<ExpressionStore>,
    pub fields: Arena<FieldData>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldData {
    pub name: Name,
    pub r#type: TypeSpecifierId,
}

impl StructData {
    pub fn query(
        database: &dyn DefDatabase,
        func: StructId,
    ) -> (Arc<Self>, Arc<ExpressionSourceMap>) {
        let loc = func.lookup(database);
        let source = loc.source(database);
        let (struct_data, source_map) = lower_struct(database, &source);
        (Arc::new(struct_data), Arc::new(source_map))
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

#[derive(Debug, PartialEq, Eq)]
pub struct TypeAliasData {
    pub name: Name,
    pub store: Arc<ExpressionStore>,
    pub r#type: TypeSpecifierId,
}

impl TypeAliasData {
    pub fn type_alias_data_query(
        database: &dyn DefDatabase,
        func: TypeAliasId,
    ) -> (Arc<Self>, Arc<ExpressionSourceMap>) {
        let loc = func.lookup(database);
        let source = loc.source(database);

        let (type_alias, source_map) = lower_type_alias(database, &source);
        (Arc::new(type_alias), Arc::new(source_map))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct GlobalVariableData {
    pub name: Name,
    pub store: Arc<ExpressionStore>,
    pub r#type: Option<TypeSpecifierId>,
    pub template_parameters: Vec<ExpressionId>,
}

impl GlobalVariableData {
    pub fn global_var_data_query(
        database: &dyn DefDatabase,
        var: GlobalVariableId,
    ) -> (Arc<Self>, Arc<ExpressionSourceMap>) {
        let loc = database.lookup_intern_global_variable(var);
        let source = loc.source(database);

        let (global_variable, source_map) = lower_variable(database, &source);
        (Arc::new(global_variable), Arc::new(source_map))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct GlobalConstantData {
    pub name: Name,
    pub store: Arc<ExpressionStore>,
    pub r#type: Option<TypeSpecifierId>,
}

impl GlobalConstantData {
    pub fn global_constant_data_query(
        database: &dyn DefDatabase,
        constant: GlobalConstantId,
    ) -> (Arc<Self>, Arc<ExpressionSourceMap>) {
        let loc = database.lookup_intern_global_constant(constant);
        let source = loc.source(database);

        let (global_constant, source_map) = lower_constant(database, &source);
        (Arc::new(global_constant), Arc::new(source_map))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct OverrideData {
    pub name: Name,
    pub store: Arc<ExpressionStore>,
    pub r#type: Option<TypeSpecifierId>,
}

impl OverrideData {
    pub fn override_data_query(
        database: &dyn DefDatabase,
        override_decl: OverrideId,
    ) -> (Arc<Self>, Arc<ExpressionSourceMap>) {
        let loc = database.lookup_intern_override(override_decl);
        let source = loc.source(database);

        let (global_override, source_map) = lower_override(database, &source);
        (Arc::new(global_override), Arc::new(source_map))
    }
}
