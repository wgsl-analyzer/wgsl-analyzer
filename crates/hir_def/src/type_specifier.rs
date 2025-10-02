use crate::{expression::ExpressionId, module_data::Name};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct TypeSpecifier {
    pub path: Name,
    pub generics: Vec<ExpressionId>,
}
