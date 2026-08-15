use std::iter;

use base_db::{Lookup as _, SourceDatabase};
use either::Either;
use syntax::{HasAttributes, ast};
use triomphe::Arc;

use crate::{
    HasSource as _,
    db::{FunctionId, GlobalVariableId, StructId},
    expression::ExpressionId,
    expression_store::{
        ExpressionSourceMap, ExpressionStore, ExpressionStoreSource, lower::ExprCollector,
    },
    item_tree::Name,
};

// TODO: Properly model the attributes (not all of them have expressions)
// https://github.com/wgsl-analyzer/wgsl-analyzer/issues/614
// e.g `@builtin(position)`, `@compute`
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Attribute {
    pub name: Name,
    pub parameters: Vec<ExpressionId>,
}

// for example, @group(0) @location(0)
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
        let attributes = source
            .attributes()
            .into_iter()
            .flat_map(std::iter::IntoIterator::into_iter)
            .map(|attribute| Attribute {
                name: attribute
                    .name()
                    .map_or_else(Name::missing, |attribute| Name::from(attribute.text())),
                parameters: get_attribute_parameters(&mut collector, attribute),
            })
            .collect();
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
    attribute: ast::Attribute,
) -> Vec<la_arena::Idx<crate::expression::Expression>> {
    match attribute {
        ast::Attribute::OtherAttribute(inner) => inner
            .parameters()
            .map(|p| p.arguments().map(|e| collector.collect_expression(e)))
            .map_or_else(|| Either::Left(iter::empty()), Either::Right)
            .collect(),
        ast::Attribute::AlignAttribute(inner) => inner
            .parameter()
            .into_iter()
            .map(|e| collector.collect_expression(e))
            .collect(),
        ast::Attribute::BindingAttribute(inner) => inner
            .parameter()
            .into_iter()
            .map(|e| collector.collect_expression(e))
            .collect(),
        ast::Attribute::BlendSrcAttribute(inner) => inner
            .parameter()
            .into_iter()
            .map(|e| collector.collect_expression(e))
            .collect(),
        ast::Attribute::GroupAttribute(inner) => inner
            .parameter()
            .into_iter()
            .map(|e| collector.collect_expression(e))
            .collect(),
        ast::Attribute::IdAttribute(inner) => inner
            .parameter()
            .into_iter()
            .map(|e| collector.collect_expression(e))
            .collect(),
        ast::Attribute::LocationAttribute(inner) => inner
            .parameter()
            .into_iter()
            .map(|e| collector.collect_expression(e))
            .collect(),
        ast::Attribute::SizeAttribute(inner) => inner
            .parameter()
            .into_iter()
            .map(|e| collector.collect_expression(e))
            .collect(),
        ast::Attribute::WorkgroupSizeAttribute(inner) => inner
            .parameters()
            .map(|e| collector.collect_expression(e))
            .collect(),
        ast::Attribute::ElifAttribute(inner) => inner
            .parameter()
            .into_iter()
            .map(|e| collector.collect_expression(e))
            .collect(),
        ast::Attribute::IfAttribute(inner) => inner
            .parameter()
            .into_iter()
            .map(|e| collector.collect_expression(e))
            .collect(),
        ast::Attribute::ConstantAttribute(_)
        | ast::Attribute::DiagnosticAttribute(_)
        | ast::Attribute::BuiltinAttribute(_)
        | ast::Attribute::InterpolateAttribute(_)
        | ast::Attribute::InvariantAttribute(_)
        | ast::Attribute::MustUseAttribute(_)
        | ast::Attribute::VertexAttribute(_)
        | ast::Attribute::FragmentAttribute(_)
        | ast::Attribute::ComputeAttribute(_)
        | ast::Attribute::ElseAttribute(_) => Vec::new(),
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
