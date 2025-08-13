use std::{iter, sync::Arc};

use either::Either;
use la_arena::Arena;
use syntax::{
    HasAttributes, HasName as _,
    ast::{self},
};

use crate::{
    HasSource as _,
    data::FieldId,
    database::{DefDatabase, FunctionId, GlobalVariableId, Interned, Lookup as _, StructId},
    expression::{Expression, ExpressionId, Literal, parse_literal},
    module_data::Name,
};

// TODO: Properly model the attributes (not all of them have expressions)
// e.g `@builtin(position)`, `@compute`
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Attribute {
    pub name: Name,
    pub parameters: Vec<ExpressionId>,
}

// e.g. @group(0) @location(0)
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct AttributeList {
    pub attributes: Vec<Attribute>,
}

impl AttributeList {
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
    pub fn from_src(source: &dyn HasAttributes) -> Self {
        let attrs = source
            .attributes()
            .map(|attribute| Attribute {
                name: attribute
                    .ident_token()
                    .map_or_else(Name::missing, |attribute| Name::from(attribute.text())),
                parameters: attribute
                    .parameters()
                    .map(|parameter| parameter.arguments().map(|v| v))
                    .map_or_else(|| Either::Left(iter::empty()), Either::Right)
                    .collect(),
            })
            .collect();

        Self { attributes: attrs }
    }

    const fn empty() -> Self {
        Self {
            attributes: Vec::new(),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum AttributeDefId {
    Struct(StructId),
    Field(FieldId),
    Function(FunctionId),
    GlobalVariable(GlobalVariableId),
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct AttributesWithOwner {
    pub attribute_list: AttributeList,
    pub exprs: Arena<Expression>,
    pub owner: AttributeDefId,
}

impl AttributesWithOwner {
    pub(crate) fn attrs_query(
        database: &dyn DefDatabase,
        definition: AttributeDefId,
    ) -> Arc<Self> {
        let attrs = match definition {
            AttributeDefId::Struct(id) => {
                AttributeList::from_src(&id.lookup(database).source(database).value)
            },
            AttributeDefId::Field(id) => {
                let location = id.r#struct.lookup(database).source(database);
                let struct_declaration: ast::StructDeclaration = location.value;
                let mut fields = struct_declaration.body().map_or_else(
                    || Either::Left(iter::empty::<ast::StructMember>()),
                    |body| Either::Right(body.fields()),
                );

                let strukt_data = database.struct_data(id.r#struct);
                let field_name = strukt_data.fields[id.field].name.as_str();

                // this is ugly but rust-analyzer is more complicated and this should work for now
                let attrs = fields.find_map(|field| {
                    let name = field.name()?;
                    (name.text().as_str() == field_name).then_some(field)
                });
                attrs.map_or_else(AttributeList::empty, |field| {
                    AttributeList::from_src(&field)
                })
            },
            AttributeDefId::Function(id) => {
                AttributeList::from_src(&id.lookup(database).source(database).value)
            },
            AttributeDefId::GlobalVariable(id) => {
                AttributeList::from_src(&id.lookup(database).source(database).value)
            },
        };

        Arc::new(Self {
            attribute_list: attrs,
            owner: definition,
            exprs: Default::default(),
        })
    }
}
