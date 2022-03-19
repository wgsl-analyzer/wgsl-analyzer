use hir_def::{module_data::Name, resolver::ResolveType, HirFileId};
use syntax::{ast, match_ast, AstNode, SyntaxNode, SyntaxToken};

use crate::{Field, Local, ModuleDef, Semantics, Struct, TypeAlias};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Definition {
    Local(Local),
    Field(Field),
    ModuleDef(ModuleDef),
    Struct(Struct),
    TypeAlias(TypeAlias),
}

impl Definition {
    pub fn from_token(
        sema: &Semantics<'_>,
        file_id: HirFileId,
        token: &SyntaxToken,
    ) -> Option<Definition> {
        let parent = token.parent()?;
        Self::from_node(sema, file_id, &parent)
    }

    pub fn from_node(
        sema: &Semantics<'_>,
        file_id: HirFileId,
        node: &SyntaxNode,
    ) -> Option<Definition> {
        match_ast! {
            match node {
                ast::NameRef(name_ref) => {
                    resolve_name_ref(sema, file_id, &name_ref)
                },
                _ => {
                    tracing::warn!("attempted to go to definition {:?}", node);
                    None
                }
            }
        }
    }
}

fn resolve_name_ref(
    sema: &Semantics<'_>,
    file_id: HirFileId,
    name_ref: &ast::NameRef,
) -> Option<Definition> {
    let parent = name_ref.syntax().parent()?;

    if let Some(expr) = ast::PathExpr::cast(parent.clone()) {
        let name = Name::from(expr.name_ref()?);
        let def = sema.find_container(file_id, expr.syntax())?;
        let def = sema.resolve_name_in_expr_scope(def, expr.syntax(), name)?;

        Some(def)
    } else if let Some(expr) = ast::FieldExpr::cast(parent.clone()) {
        let def = sema.find_container(file_id, expr.syntax())?;
        let field = sema.analyze(def).resolve_field(expr)?;

        Some(Definition::Field(field))
    } else if let Some(ty) = ast::PathType::cast(parent.clone()) {
        let resolver = sema.resolver(file_id, ty.syntax());

        match resolver.resolve_type(&ty.name()?.into())? {
            ResolveType::Struct(loc) => {
                let id = sema.db.intern_struct(loc);
                Some(Definition::Struct(Struct { id }))
            }
            ResolveType::TypeAlias(loc) => {
                let id = sema.db.intern_type_alias(loc);
                Some(Definition::TypeAlias(TypeAlias { id }))
            }
        }
    } else {
        None
    }
}
