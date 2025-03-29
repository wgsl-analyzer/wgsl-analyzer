use std::sync::Arc;

use either::Either;
use syntax::{
    HasAttributes, HasName,
    ast::{self, IdentOrLiteral},
};

use crate::{
    HasSource,
    data::FieldId,
    db::{DefDatabase, FunctionId, GlobalVariableId, Interned, Lookup, StructId},
    expression::{Literal, parse_literal},
    module_data::Name,
};

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub enum AttributeValue {
    Name(Name),
    Literal(Literal),
}

// e.g `builtin(position)`, `block`
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct Attribute {
    pub name: Name,
    pub parameters: smallvec::SmallVec<[AttributeValue; 1]>,
}

// e.g. [[group(0), location(0)]]
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct AttributeList {
    pub attributes: Vec<Interned<Attribute>>,
}

impl AttributeList {
    pub fn has(
        &self,
        db: &dyn DefDatabase,
        name: &str,
    ) -> bool {
        self.attributes.iter().any(|attribute| {
            let attribute = db.lookup_intern_attribute(*attribute);
            attribute.name.as_str() == name
        })
    }
}

impl AttributeList {
    pub fn from_src(
        db: &dyn DefDatabase,
        source: &dyn HasAttributes,
    ) -> AttributeList {
        let attrs = source
            .attributes()
            .map(|attribute| Attribute {
                name: attribute
                    .ident_token()
                    .map_or_else(Name::missing, |attribute| Name::from(attribute.text())),
                parameters: attribute
                    .parameters()
                    .map(|parameter| {
                        parameter.values().map(|value| match value {
                            IdentOrLiteral::Identifier(ident) => {
                                AttributeValue::Name(Name::from(ident))
                            },
                            IdentOrLiteral::Literal(lit) => {
                                AttributeValue::Literal(parse_literal(lit.kind()))
                            },
                        })
                    })
                    .map_or_else(|| Either::Left(std::iter::empty()), Either::Right)
                    .collect(),
            })
            .map(|attribute| db.intern_attribute(attribute))
            .collect();

        AttributeList { attributes: attrs }
    }

    fn empty() -> AttributeList {
        AttributeList {
            attributes: Vec::new(),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum AttributeDefId {
    StructId(StructId),
    FieldId(FieldId),
    FunctionId(FunctionId),
    GlobalVariableId(GlobalVariableId),
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct AttributesWithOwner {
    pub attribute_list: AttributeList,
    pub owner: AttributeDefId,
}

impl AttributesWithOwner {
    pub(crate) fn attrs_query(
        db: &dyn DefDatabase,
        def: AttributeDefId,
    ) -> Arc<Self> {
        let attrs = match def {
            AttributeDefId::StructId(id) => {
                AttributeList::from_src(db, &id.lookup(db).source(db).value)
            },
            AttributeDefId::FieldId(id) => {
                let location = id.r#struct.lookup(db).source(db);
                let struct_declaration: ast::StructDeclaration = location.value;
                let mut fields = struct_declaration.body().map_or_else(
                    || Either::Left(std::iter::empty::<ast::StructDeclarationField>()),
                    |body| Either::Right(body.fields()),
                );

                let strukt_data = db.struct_data(id.r#struct);
                let field_name = strukt_data.fields[id.field].name.as_str();

                // this is ugly but rust-analyzer is more complicated and this should work for now
                let attrs = fields.find_map(|field| {
                    let name = field
                        .variable_ident_declaration()
                        .and_then(|var| var.binding())
                        .and_then(|binding| binding.name())?;
                    if name.text().as_str() == field_name {
                        Some(field)
                    } else {
                        None
                    }
                });
                match attrs {
                    Some(field) => AttributeList::from_src(db, &field),
                    None => AttributeList::empty(),
                }
            },
            AttributeDefId::FunctionId(id) => {
                AttributeList::from_src(db, &id.lookup(db).source(db).value)
            },
            AttributeDefId::GlobalVariableId(id) => {
                AttributeList::from_src(db, &id.lookup(db).source(db).value)
            },
        };

        Arc::new(AttributesWithOwner {
            attribute_list: attrs,
            owner: def,
        })
    }
}
