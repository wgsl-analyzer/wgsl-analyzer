use hir_def::item_tree::Name;
use triomphe::Arc;

use crate::{database::HirDatabase, ty::Type};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionDetails {
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

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct ResolvedFunctionId(salsa::InternId);

impl salsa::InternKey for ResolvedFunctionId {
    fn from_intern_id(v: salsa::InternId) -> Self {
        Self(v)
    }

    fn as_intern_id(&self) -> salsa::InternId {
        self.0
    }
}

impl ResolvedFunctionId {
    pub fn lookup(
        self,
        database: &dyn HirDatabase,
    ) -> Arc<FunctionDetails> {
        database.lookup_intern_resolved_function(self)
    }
}

impl FunctionDetails {
    pub fn intern(
        self,
        database: &dyn HirDatabase,
    ) -> ResolvedFunctionId {
        database.intern_resolved_function(Arc::new(self))
    }
}
