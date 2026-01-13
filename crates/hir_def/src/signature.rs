use la_arena::{Arena, Idx};
use triomphe::Arc;

use crate::{
    HasSource as _,
    database::{
        DefDatabase, FunctionId, GlobalAssertStatementId, GlobalConstantId, GlobalVariableId,
        Lookup as _, OverrideId, StructId, TypeAliasId,
    },
    expression::ExpressionId,
    expression_store::{
        ExpressionSourceMap, ExpressionStore,
        lower::{
            lower_constant, lower_function, lower_global_assert_statement, lower_override,
            lower_struct, lower_type_alias, lower_variable,
        },
    },
    item_tree::Name,
    type_specifier::TypeSpecifierId,
};

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub struct ParameterId {
    pub function: FunctionId,
    pub param: LocalParameterId,
}

pub type LocalParameterId = Idx<ParamData>;

#[derive(Debug, PartialEq, Eq)]
pub struct FunctionSignature {
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

impl FunctionSignature {
    pub fn query(
        database: &dyn DefDatabase,
        function: FunctionId,
    ) -> (Arc<Self>, Arc<ExpressionSourceMap>) {
        let location = function.lookup(database);
        let source = location.source(database);
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
pub struct StructSignature {
    pub name: Name,
    pub store: Arc<ExpressionStore>,
    pub fields: Arena<FieldData>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldData {
    pub name: Name,
    pub r#type: TypeSpecifierId,
}

impl StructSignature {
    pub fn query(
        database: &dyn DefDatabase,
        function: StructId,
    ) -> (Arc<Self>, Arc<ExpressionSourceMap>) {
        let location = function.lookup(database);
        let source = location.source(database);
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
pub struct TypeAliasSignature {
    pub name: Name,
    pub store: Arc<ExpressionStore>,
    pub r#type: TypeSpecifierId,
}

impl TypeAliasSignature {
    pub fn query(
        database: &dyn DefDatabase,
        function: TypeAliasId,
    ) -> (Arc<Self>, Arc<ExpressionSourceMap>) {
        let location = function.lookup(database);
        let source = location.source(database);

        let (type_alias, source_map) = lower_type_alias(database, &source);
        (Arc::new(type_alias), Arc::new(source_map))
    }
}

/// The signature of a global variable
#[derive(Debug, PartialEq, Eq)]
pub struct VariableSignature {
    pub name: Name,
    pub store: Arc<ExpressionStore>,
    pub r#type: Option<TypeSpecifierId>,
    pub template_parameters: Vec<ExpressionId>,
}

impl VariableSignature {
    pub fn query(
        database: &dyn DefDatabase,
        variable: GlobalVariableId,
    ) -> (Arc<Self>, Arc<ExpressionSourceMap>) {
        let location = database.lookup_intern_global_variable(variable);
        let source = location.source(database);

        let (global_variable, source_map) = lower_variable(database, &source);
        (Arc::new(global_variable), Arc::new(source_map))
    }
}

/// The signature of a global constant
#[derive(Debug, PartialEq, Eq)]
pub struct ConstantSignature {
    pub name: Name,
    pub store: Arc<ExpressionStore>,
    pub r#type: Option<TypeSpecifierId>,
}

impl ConstantSignature {
    pub fn query(
        database: &dyn DefDatabase,
        constant: GlobalConstantId,
    ) -> (Arc<Self>, Arc<ExpressionSourceMap>) {
        let location = database.lookup_intern_global_constant(constant);
        let source = location.source(database);

        let (global_constant, source_map) = lower_constant(database, &source);
        (Arc::new(global_constant), Arc::new(source_map))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct GlobalAssertStatementSignature {
    pub store: Arc<ExpressionStore>,
}

impl GlobalAssertStatementSignature {
    pub fn query(
        database: &dyn DefDatabase,
        constant: GlobalAssertStatementId,
    ) -> (Arc<Self>, Arc<ExpressionSourceMap>) {
        let location = database.lookup_intern_global_assert_statement(constant);
        let source = location.source(database);

        let (global_constant, source_map) = lower_global_assert_statement(database, &source);
        (Arc::new(global_constant), Arc::new(source_map))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct OverrideSignature {
    pub name: Name,
    pub store: Arc<ExpressionStore>,
    pub r#type: Option<TypeSpecifierId>,
}

impl OverrideSignature {
    pub fn query(
        database: &dyn DefDatabase,
        override_declaration: OverrideId,
    ) -> (Arc<Self>, Arc<ExpressionSourceMap>) {
        let location = database.lookup_intern_override(override_declaration);
        let source = location.source(database);

        let (global_override, source_map) = lower_override(database, &source);
        (Arc::new(global_override), Arc::new(source_map))
    }
}
