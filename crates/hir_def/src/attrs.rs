use std::sync::Arc;

use either::Either;
use syntax::{
    HasAttrs, HasName,
    ast::{self, IdentOrLiteral},
};

use crate::{
    HasSource,
    data::FieldId,
    db::{DefDatabase, FunctionId, GlobalVariableId, Interned, Lookup, StructId},
    expr::{Literal, parse_literal},
    module_data::Name,
};

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub enum AttrValue {
    Name(Name),
    Literal(Literal),
}

// e.g `builtin(position)`, `block`
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct Attr {
    pub name: Name,
    pub parameters: smallvec::SmallVec<[AttrValue; 1]>,
}

// e.g. [[group(0), location(0)]]
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct AttrList {
    pub attrs: Vec<Interned<Attr>>,
}

impl AttrList {
    pub fn has(
        &self,
        db: &dyn DefDatabase,
        name: &str,
    ) -> bool {
        self.attrs.iter().any(|attr| {
            let attr = db.lookup_intern_attr(*attr);
            attr.name.as_str() == name
        })
    }
}

impl AttrList {
    pub fn from_src(
        db: &dyn DefDatabase,
        src: &dyn HasAttrs,
    ) -> AttrList {
        let attrs = src
            .attributes()
            .map(|attr| Attr {
                name: attr
                    .ident_token()
                    .map_or_else(Name::missing, |attr| Name::from(attr.text())),
                parameters: attr
                    .params()
                    .map(|param| {
                        param.values().map(|value| match value {
                            IdentOrLiteral::Ident(ident) => AttrValue::Name(Name::from(ident)),
                            IdentOrLiteral::Literal(lit) => {
                                AttrValue::Literal(parse_literal(lit.kind()))
                            },
                        })
                    })
                    .map_or_else(|| Either::Left(std::iter::empty()), Either::Right)
                    .collect(),
            })
            .map(|attr| db.intern_attr(attr))
            .collect();

        AttrList { attrs }
    }

    fn empty() -> AttrList {
        AttrList { attrs: Vec::new() }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum AttrDefId {
    StructId(StructId),
    FieldId(FieldId),
    FunctionId(FunctionId),
    GlobalVariableId(GlobalVariableId),
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct AttrsWithOwner {
    pub attribute_list: AttrList,
    pub owner: AttrDefId,
}

impl AttrsWithOwner {
    pub(crate) fn attrs_query(
        db: &dyn DefDatabase,
        def: AttrDefId,
    ) -> Arc<Self> {
        let attrs = match def {
            AttrDefId::StructId(id) => AttrList::from_src(db, &id.lookup(db).source(db).value),
            AttrDefId::FieldId(id) => {
                let location = id.r#struct.lookup(db).source(db);
                let struct_decl: ast::StructDecl = location.value;
                let mut fields = struct_decl.body().map_or_else(
                    || Either::Left(std::iter::empty::<ast::StructDeclField>()),
                    |body| Either::Right(body.fields()),
                );

                let strukt_data = db.struct_data(id.r#struct);
                let field_name = strukt_data.fields[id.field].name.as_str();

                // this is ugly but rust-analyzer is more complicated and this should work for now
                let attrs = fields.find_map(|field| {
                    let name = field
                        .variable_ident_decl()
                        .and_then(|var| var.binding())
                        .and_then(|binding| binding.name())?;
                    if name.text().as_str() == field_name {
                        Some(field)
                    } else {
                        None
                    }
                });
                match attrs {
                    Some(field) => AttrList::from_src(db, &field),
                    None => AttrList::empty(),
                }
            },
            AttrDefId::FunctionId(id) => AttrList::from_src(db, &id.lookup(db).source(db).value),
            AttrDefId::GlobalVariableId(id) => {
                AttrList::from_src(db, &id.lookup(db).source(db).value)
            },
        };

        Arc::new(AttrsWithOwner {
            attribute_list: attrs,
            owner: def,
        })
    }
}
