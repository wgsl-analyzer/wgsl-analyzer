use la_arena::Idx;

use crate::{expression::ExpressionId, expression_store::path::Path};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct TypeSpecifier {
    pub path: Path,
    pub template_parameters: Vec<ExpressionId>,
}

pub type TypeSpecifierId = Idx<TypeSpecifier>;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct IdentExpression {
    pub path: Path,
    pub template_parameters: Vec<ExpressionId>,
}
