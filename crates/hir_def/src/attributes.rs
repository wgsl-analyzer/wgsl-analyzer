use std::iter;

use base_db::{Lookup as _, SourceDatabase};
use either::Either;
use rustc_hash::FxHashMap;
use syntax::{
    HasAttributes,
    ast::{self, AttributeKind},
};
use triomphe::Arc;

use crate::{
    HasSource as _,
    db::{FunctionId, GlobalVariableId, StructId},
    expression::ExpressionId,
    expression_store::{
        ExpressionSourceMap, ExpressionStore, ExpressionStoreSource, lower::ExprCollector,
    },
    item_tree::Name,
    mod_path::ModPath,
};

// TODO: Properly model the attributes (not all of them have expressions)
// https://github.com/wgsl-analyzer/wgsl-analyzer/issues/614
// For example, `@builtin(position)`, `@compute`
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Attribute {
    pub name: Name,
    pub parameters: Vec<ExpressionId>,
}

// For example, @group(0) @location(0)
#[derive(PartialEq, Eq, Debug)]
pub struct AttributeList {
    pub attributes: Vec<Attribute>,
    pub store: Arc<ExpressionStore>,
}

impl AttributeList {
    #[must_use]
    pub fn has(
        &self,
        name: &str,
    ) -> bool {
        self.attributes
            .iter()
            .any(|attribute| attribute.name.as_str() == name)
    }
}

impl AttributeList {
    pub fn from_src(
        db: &dyn SourceDatabase,
        source: &dyn HasAttributes,
    ) -> (Self, ExpressionSourceMap) {
        let mut collector = ExprCollector::new(db, ExpressionStoreSource::Signature);
        let attributes = if let Some(attributes) = source.attributes() {
            attributes
                .map(|attribute| Attribute {
                    name: attribute
                        .name()
                        .map_or_else(Name::missing, |attribute| Name::from(attribute.text())),
                    parameters: get_attribute_parameters(&mut collector, &attribute),
                })
                .collect()
        } else {
            Vec::new()
        };
        let (store, source_map) = collector.finish();
        (
            Self {
                attributes,
                store: Arc::new(store),
            },
            source_map,
        )
    }

    #[must_use]
    pub fn empty() -> (Self, ExpressionSourceMap) {
        (
            Self {
                attributes: Vec::new(),
                store: Arc::new(ExpressionStore::default()),
            },
            ExpressionSourceMap::default(),
        )
    }
}

#[expect(clippy::min_ident_chars, reason = "function.tar.gz")]
fn get_attribute_parameters(
    collector: &mut ExprCollector<'_>,
    attribute: &ast::Attribute,
) -> Vec<la_arena::Idx<crate::expression::Expression>> {
    match attribute.kind() {
        // their arguments are not expressions
        AttributeKind::Diagnostic | AttributeKind::Builtin | AttributeKind::Interpolate => {
            Vec::new()
        },
        AttributeKind::Align
        | AttributeKind::Binding
        | AttributeKind::BlendSrc
        | AttributeKind::Const
        | AttributeKind::Group
        | AttributeKind::Id
        | AttributeKind::Invariant
        | AttributeKind::Location
        | AttributeKind::MustUse
        | AttributeKind::Size
        | AttributeKind::SubgroupSize
        | AttributeKind::WorkgroupSize
        | AttributeKind::Entrypoint(_)
        | AttributeKind::Conditional(_)
        | AttributeKind::Other => attribute
            .arguments()
            .map(|p| p.arguments().map(|e| collector.collect_expression(e)))
            .map_or_else(|| Either::Left(iter::empty()), Either::Right)
            .collect(),
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug, salsa::Supertype)]
pub enum AttributeDefId {
    Struct(StructId),
    // Field(FieldId),
    Function(FunctionId),
    GlobalVariable(GlobalVariableId),
}

#[derive(PartialEq, Eq, Debug)]
pub struct AttributesWithOwner {
    pub attribute_list: AttributeList,
    pub owner: AttributeDefId,
}
#[expect(
    clippy::drop_non_drop,
    reason = "Clippy has a false positive for the salsa::tracked macro, see: https://github.com/rust-lang/rust-clippy/issues/16753"
)]
#[salsa::tracked]
impl AttributesWithOwner {
    #[salsa::tracked(returns(deref))]
    pub fn of(
        db: &dyn SourceDatabase,
        definition: AttributeDefId,
    ) -> Arc<Self> {
        Self::with_source_map(db, definition).0.clone()
    }

    #[salsa::tracked(returns(ref))]
    pub fn with_source_map(
        db: &dyn SourceDatabase,
        definition: AttributeDefId,
    ) -> (Arc<Self>, Arc<ExpressionSourceMap>) {
        let (attributes, source_map) = match definition {
            AttributeDefId::Struct(id) => {
                AttributeList::from_src(db, &id.lookup(db).source(db).value)
            },
            AttributeDefId::Function(id) => {
                AttributeList::from_src(db, &id.lookup(db).source(db).value)
            },
            AttributeDefId::GlobalVariable(id) => {
                AttributeList::from_src(db, &id.lookup(db).source(db).value)
            },
        };

        (
            Arc::new(Self {
                attribute_list: attributes,
                owner: definition,
            }),
            Arc::new(source_map),
        )
    }
}

pub(crate) fn is_else_branch(definition: &dyn HasAttributes) -> bool {
    let Some(mut attributes) = definition.attributes() else {
        return false;
    };
    attributes.any(|attribute| {
        matches!(
            attribute.kind(),
            ast::AttributeKind::Conditional(
                ast::ConditionalAttributeKind::Elif | ast::ConditionalAttributeKind::Else,
            ),
        )
    })
}

/// An overly simplistic implementation of conditional compilation.
pub(crate) fn eval_cond_comp(
    definition: &dyn HasAttributes,
    features: &FxHashMap<String, bool>,
) -> Option<bool> {
    let attributes = definition
        .attributes()?
        .filter(|attribute| attribute.is_conditional_compilation());
    for attribute in attributes {
        // @else is always taken (if we try it)
        if attribute.kind() == ast::AttributeKind::Conditional(ast::ConditionalAttributeKind::Else)
        {
            return Some(true);
        }
        // @if and @elif need to be evaluated
        // we do the silly assumption of "all arguments evaluate to true"
        let value = attribute.arguments()?.arguments().next()?;
        return evaluate_cond_comp_expression(value, features);
    }

    fn evaluate_cond_comp_expression(
        value: ast::Expression,
        features: &FxHashMap<String, bool>,
    ) -> Option<bool> {
        match value {
            ast::Expression::IndexExpression(_)
            | ast::Expression::FieldExpression(_)
            | ast::Expression::FunctionCall(_) => None,
            ast::Expression::PrefixExpression(prefix_expression) => {
                match prefix_expression.operator_kind()? {
                    ast::operators::UnaryOperator::Negation
                    | ast::operators::UnaryOperator::AddressOf
                    | ast::operators::UnaryOperator::Indirection
                    | ast::operators::UnaryOperator::BitwiseComplement => None,
                    ast::operators::UnaryOperator::LogicalNegation => {
                        let result = evaluate_cond_comp_expression(
                            prefix_expression.expression()?,
                            features,
                        )?;
                        Some(!result)
                    },
                }
            },
            ast::Expression::InfixExpression(infix_expression) => {
                match infix_expression.operator_kind()? {
                    ast::operators::BinaryOperation::Logical(
                        ast::operators::LogicOperation::ShortCircuitAnd,
                    ) => {
                        let left =
                            evaluate_cond_comp_expression(infix_expression.left_side()?, features)?;
                        let right = evaluate_cond_comp_expression(
                            infix_expression.right_side()?,
                            features,
                        )?;
                        Some(left && right)
                    },
                    ast::operators::BinaryOperation::Logical(
                        ast::operators::LogicOperation::ShortCircuitOr,
                    ) => {
                        let left =
                            evaluate_cond_comp_expression(infix_expression.left_side()?, features)?;
                        let right = evaluate_cond_comp_expression(
                            infix_expression.right_side()?,
                            features,
                        )?;
                        Some(left || right)
                    },
                    ast::operators::BinaryOperation::Arithmetic(_)
                    | ast::operators::BinaryOperation::Comparison(_) => None,
                }
            },
            ast::Expression::IdentExpression(ident_expression) => {
                let path = ModPath::from_src(&ident_expression.path()?);
                let name = path.as_ident()?;
                match features.get(name.as_str()) {
                    Some(value) => Some(*value),
                    None => Some(false), // TODO: should I default to false?
                }
            },
            ast::Expression::ParenthesisExpression(parenthesis_expression) => {
                evaluate_cond_comp_expression(parenthesis_expression.inner()?, features)
            },
            ast::Expression::Literal(literal) => match literal.kind() {
                ast::LiteralKind::IntLiteral(_) | ast::LiteralKind::FloatLiteral(_) => None,
                ast::LiteralKind::True(_) => Some(true),
                ast::LiteralKind::False(_) => Some(false),
            },
        }
    }

    None
}
