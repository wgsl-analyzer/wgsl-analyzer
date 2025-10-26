use la_arena::Idx;

use crate::{expression::ExpressionId, module_data::Name};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct TypeSpecifier {
    pub path: Name,
    pub generics: Vec<ExpressionId>,
}

pub type TypeSpecifierId = Idx<TypeSpecifier>;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct IdentExpression {
    pub path: Name,
    pub generics: Vec<ExpressionId>,
}
