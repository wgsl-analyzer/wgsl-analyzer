use base_db::impl_intern_key;
use hir_def::item_tree::Name;

use crate::{database::HirDatabase, ty::Type};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionDetails {
    pub name: Name,
    pub return_type: Option<Type>,
    pub parameters: Vec<(Type, Name)>,
}

impl FunctionDetails {
    pub fn parameters(&self) -> impl Iterator<Item = Type> + '_ {
        self.parameters.iter().map(|(r#type, _)| *r#type)
    }

    pub fn parameter_names(&self) -> impl Iterator<Item = &str> + '_ {
        self.parameters.iter().map(|(_, name)| name.as_str())
    }

    pub fn parameters_with_names(&self) -> impl Iterator<Item = (Type, &str)> + '_ {
        self.parameters
            .iter()
            .map(|(r#type, name)| (*r#type, name.as_str()))
    }
}

impl_intern_key!(ResolvedFunctionId, FunctionDetails);

impl ResolvedFunctionId {
    pub fn lookup(
        self,
        database: &dyn HirDatabase,
    ) -> FunctionDetails {
        database.lookup_intern_resolved_function(self)
    }
}

impl FunctionDetails {
    pub fn intern(
        self,
        database: &dyn HirDatabase,
    ) -> ResolvedFunctionId {
        database.intern_resolved_function(self)
    }
}
