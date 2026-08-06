use base_db::{Lookup as _, SourceDatabase};
use la_arena::{Arena, Idx};
use triomphe::Arc;

use crate::{
    HasSource as _,
    database::{
        FunctionId, GlobalAssertStatementId, GlobalConstantId, GlobalVariableId, OverrideId,
        StructId, TypeAliasId,
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

#[salsa::tracked]
impl FunctionSignature {
    #[salsa::tracked(returns(deref))]
    pub fn of(
        db: &dyn SourceDatabase,
        id: FunctionId,
    ) -> Arc<Self> {
        Self::with_source_map(db, id).0.clone()
    }

    #[salsa::tracked(returns(ref))]

    pub fn with_source_map(
        db: &dyn SourceDatabase,
        id: FunctionId,
    ) -> (Arc<Self>, Arc<ExpressionSourceMap>) {
        let source = id.lookup(db).source(db);
        let (function_data, source_map) = lower_function(db, &source);
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

#[salsa::tracked]
impl StructSignature {
    #[salsa::tracked(returns(deref))]
    pub fn of(
        db: &dyn SourceDatabase,
        id: StructId,
    ) -> Arc<Self> {
        Self::with_source_map(db, id).0.clone()
    }

    #[salsa::tracked(returns(ref))]
    pub fn with_source_map(
        db: &dyn SourceDatabase,
        id: StructId,
    ) -> (Arc<Self>, Arc<ExpressionSourceMap>) {
        let source = id.lookup(db).source(db);
        let (struct_data, source_map) = lower_struct(db, &source);
        (Arc::new(struct_data), Arc::new(source_map))
    }
}

impl StructSignature {
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

    #[must_use]
    pub fn field_data(
        &self,
        field: LocalFieldId,
    ) -> Option<&FieldData> {
        self.fields()
            .iter()
            .find_map(|(id, data)| (id == field).then_some(data))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TypeAliasSignature {
    pub name: Name,
    pub store: Arc<ExpressionStore>,
    pub r#type: TypeSpecifierId,
}

#[salsa::tracked]
impl TypeAliasSignature {
    #[salsa::tracked(returns(deref))]
    pub fn of(
        db: &dyn SourceDatabase,
        id: TypeAliasId,
    ) -> Arc<Self> {
        Self::with_source_map(db, id).0.clone()
    }

    #[salsa::tracked(returns(ref))]
    pub fn with_source_map(
        db: &dyn SourceDatabase,
        id: TypeAliasId,
    ) -> (Arc<Self>, Arc<ExpressionSourceMap>) {
        let source = id.lookup(db).source(db);
        let (type_alias, source_map) = lower_type_alias(db, &source);
        (Arc::new(type_alias), Arc::new(source_map))
    }
}

/// The signature of a global variable.
#[derive(Debug, PartialEq, Eq)]
pub struct VariableSignature {
    pub name: Name,
    pub store: Arc<ExpressionStore>,
    pub r#type: Option<TypeSpecifierId>,
    pub template_parameters: Vec<ExpressionId>,
}

#[salsa::tracked]
impl VariableSignature {
    #[salsa::tracked(returns(deref))]
    pub fn of(
        db: &dyn SourceDatabase,
        id: GlobalVariableId,
    ) -> Arc<Self> {
        Self::with_source_map(db, id).0.clone()
    }

    #[salsa::tracked(returns(ref))]
    pub fn with_source_map(
        db: &dyn SourceDatabase,
        id: GlobalVariableId,
    ) -> (Arc<Self>, Arc<ExpressionSourceMap>) {
        let source = id.lookup(db).source(db);
        let (global_variable, source_map) = lower_variable(db, &source);
        (Arc::new(global_variable), Arc::new(source_map))
    }
}

/// The signature of a global constant.
#[derive(Debug, PartialEq, Eq)]
pub struct ConstantSignature {
    pub name: Name,
    pub store: Arc<ExpressionStore>,
    pub r#type: Option<TypeSpecifierId>,
}

#[salsa::tracked]
impl ConstantSignature {
    #[salsa::tracked(returns(deref))]
    pub fn of(
        db: &dyn SourceDatabase,
        id: GlobalConstantId,
    ) -> Arc<Self> {
        Self::with_source_map(db, id).0.clone()
    }

    #[salsa::tracked(returns(ref))]
    pub fn with_source_map(
        db: &dyn SourceDatabase,
        id: GlobalConstantId,
    ) -> (Arc<Self>, Arc<ExpressionSourceMap>) {
        let source = id.lookup(db).source(db);
        let (global_constant, source_map) = lower_constant(db, &source);
        (Arc::new(global_constant), Arc::new(source_map))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct OverrideSignature {
    pub name: Name,
    pub store: Arc<ExpressionStore>,
    pub r#type: Option<TypeSpecifierId>,
}

#[salsa::tracked]
impl OverrideSignature {
    #[salsa::tracked(returns(deref))]
    pub fn of(
        db: &dyn SourceDatabase,
        id: OverrideId,
    ) -> Arc<Self> {
        Self::with_source_map(db, id).0.clone()
    }

    #[salsa::tracked(returns(ref))]
    pub fn with_source_map(
        db: &dyn SourceDatabase,
        id: OverrideId,
    ) -> (Arc<Self>, Arc<ExpressionSourceMap>) {
        let source = id.lookup(db).source(db);
        let (global_override, source_map) = lower_override(db, &source);
        (Arc::new(global_override), Arc::new(source_map))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct AssertStatementSignature {
    pub store: Arc<ExpressionStore>,
}

#[salsa::tracked]
impl AssertStatementSignature {
    #[salsa::tracked(returns(deref))]
    pub fn of(
        db: &dyn SourceDatabase,
        id: GlobalAssertStatementId,
    ) -> Arc<Self> {
        Self::with_source_map(db, id).0.clone()
    }

    #[salsa::tracked(returns(ref))]
    pub fn with_source_map(
        db: &dyn SourceDatabase,
        id: GlobalAssertStatementId,
    ) -> (Arc<Self>, Arc<ExpressionSourceMap>) {
        let source = id.lookup(db).source(db);
        let (global_assert_statement, source_map) = lower_global_assert_statement(db, &source);
        (Arc::new(global_assert_statement), Arc::new(source_map))
    }
}
