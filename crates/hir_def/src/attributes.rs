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
                parameters: match attribute {
                    ast::Attribute::ConstantAttribute(constant_attribute) => Vec::new(),
                    ast::Attribute::DiagnosticAttribute(diagnostic_attribute) => Vec::new(), // these controls are not expressions
                    ast::Attribute::OtherAttribute(other_attribute) => other_attribute
                        .parameters()
                        .map(|parameter| {
                            parameter
                                .arguments()
                                .map(|expression| collector.collect_expression(expression))
                        })
                        .map_or_else(|| Either::Left(iter::empty()), Either::Right)
                        .collect(),
                },
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
