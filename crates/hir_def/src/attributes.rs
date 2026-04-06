use std::iter;

use base_db::Lookup as _;
use either::Either;
use syntax::{HasAttributes, HasName as _, ast};
use triomphe::Arc;

use crate::{
    HasSource as _,
    database::{DefDatabase, FunctionId, GlobalVariableId, StructId},
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
        database: &dyn DefDatabase,
        source: &dyn HasAttributes,
    ) -> (Self, ExpressionSourceMap) {
        let mut collector = ExprCollector::new(database, ExpressionStoreSource::Signature);
        let attributes = source
            .attributes()
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

    fn empty() -> (Self, ExpressionSourceMap) {
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
        ast::Attribute::ConstantAttribute(inner) => Vec::new(),
        ast::Attribute::DiagnosticAttribute(inner) => Vec::new(), // these controls are not expressions
        ast::Attribute::OtherAttribute(inner) => inner
            .parameters()
            .map(|p| p.arguments().map(|e| collector.collect_expression(e)))
            .map_or_else(|| Either::Left(iter::empty()), Either::Right)
            .collect(),
        ast::Attribute::AlignAttribute(inner) => inner
            .parameters()
            .map(|p| p.arguments().map(|e| collector.collect_expression(e)))
            .map_or_else(|| Either::Left(iter::empty()), Either::Right)
            .collect(),
        ast::Attribute::BindingAttribute(inner) => inner
            .parameters()
            .map(|p| p.arguments().map(|e| collector.collect_expression(e)))
            .map_or_else(|| Either::Left(iter::empty()), Either::Right)
            .collect(),
        ast::Attribute::BlendSrcAttribute(inner) => inner
            .parameters()
            .map(|p| p.arguments().map(|e| collector.collect_expression(e)))
            .map_or_else(|| Either::Left(iter::empty()), Either::Right)
            .collect(),
        ast::Attribute::BuiltinAttribute(inner) => inner
            .parameters()
            .map(|p| p.arguments().map(|e| collector.collect_expression(e)))
            .map_or_else(|| Either::Left(iter::empty()), Either::Right)
            .collect(),
        ast::Attribute::GroupAttribute(inner) => inner
            .parameters()
            .map(|p| p.arguments().map(|e| collector.collect_expression(e)))
            .map_or_else(|| Either::Left(iter::empty()), Either::Right)
            .collect(),
        ast::Attribute::IdAttribute(inner) => inner
            .parameters()
            .map(|p| p.arguments().map(|e| collector.collect_expression(e)))
            .map_or_else(|| Either::Left(iter::empty()), Either::Right)
            .collect(),
        ast::Attribute::InterpolateAttribute(inner) => Vec::new(), // these arguments are not expressions
        ast::Attribute::InvariantAttribute(inner) => inner
            .parameters()
            .map(|p| p.arguments().map(|e| collector.collect_expression(e)))
            .map_or_else(|| Either::Left(iter::empty()), Either::Right)
            .collect(),
        ast::Attribute::LocationAttribute(inner) => inner
            .parameters()
            .map(|p| p.arguments().map(|e| collector.collect_expression(e)))
            .map_or_else(|| Either::Left(iter::empty()), Either::Right)
            .collect(),
        ast::Attribute::MustUseAttribute(inner) => Vec::new(),
        ast::Attribute::SizeAttribute(inner) => inner
            .parameters()
            .map(|p| p.arguments().map(|e| collector.collect_expression(e)))
            .map_or_else(|| Either::Left(iter::empty()), Either::Right)
            .collect(),
        ast::Attribute::WorkgroupSizeAttribute(inner) => inner
            .parameters()
            .map(|p| p.arguments().map(|e| collector.collect_expression(e)))
            .map_or_else(|| Either::Left(iter::empty()), Either::Right)
            .collect(),
        ast::Attribute::VertexAttribute(inner) => Vec::new(),
        ast::Attribute::FragmentAttribute(inner) => Vec::new(),
        ast::Attribute::ComputeAttribute(inner) => Vec::new(),
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug, salsa_macros::Supertype)]
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

impl AttributesWithOwner {
    pub(crate) fn attrs_query(
        database: &dyn DefDatabase,
        definition: AttributeDefId,
    ) -> (Arc<Self>, Arc<ExpressionSourceMap>) {
        let (attributes, source_map) = match definition {
            AttributeDefId::Struct(id) => {
                AttributeList::from_src(database, &id.lookup(database).source(database).value)
            },
            AttributeDefId::Function(id) => {
                AttributeList::from_src(database, &id.lookup(database).source(database).value)
            },
            AttributeDefId::GlobalVariable(id) => {
                AttributeList::from_src(database, &id.lookup(database).source(database).value)
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
