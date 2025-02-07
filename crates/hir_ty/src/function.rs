use std::sync::Arc;

use hir_def::module_data::Name;

use crate::{ty::Ty, HirDatabase};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionDetails {
    pub return_type: Option<Ty>,
    pub parameters: Vec<(Ty, Name)>,
}

impl FunctionDetails {
    pub fn parameters(&self) -> impl Iterator<Item = Ty> + '_ {
        self.parameters.iter().map(|(ty, _)| *ty)
    }

    pub fn parameter_names(&self) -> impl Iterator<Item = &str> + '_ {
        self.parameters.iter().map(|(_, name)| name.as_str())
    }

    pub fn parameters_with_names(&self) -> impl Iterator<Item = (Ty, &str)> + '_ {
        self.parameters
            .iter()
            .map(|(ty, name)| (*ty, name.as_str()))
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct ResolvedFunctionId(salsa::InternId);
impl salsa::InternKey for ResolvedFunctionId {
    fn from_intern_id(id: salsa::InternId) -> Self {
        ResolvedFunctionId(id)
    }

    fn as_intern_id(&self) -> salsa::InternId {
        self.0
    }
}

impl ResolvedFunctionId {
    pub fn lookup(
        self,
        db: &dyn HirDatabase,
    ) -> Arc<FunctionDetails> {
        db.lookup_intern_resolved_function(self)
    }
}

impl FunctionDetails {
    pub fn intern(
        self,
        db: &dyn HirDatabase,
    ) -> ResolvedFunctionId {
        db.intern_resolved_function(Arc::new(self))
    }
}
