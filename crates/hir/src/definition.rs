use hir_def::{
    HirFileId,
    module_data::Name,
    resolver::{ResolveCallable, ResolveType},
};
use syntax::{AstNode as _, SyntaxNode, SyntaxToken, ast, match_ast};

use crate::{Field, Function, Local, ModuleDef, Semantics, Struct, TypeAlias};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Definition {
    Local(Local),
    Field(Field),
    ModuleDef(ModuleDef),
    Struct(Struct),
    TypeAlias(TypeAlias),
}

impl Definition {
    #[must_use]
    pub fn from_token(
        sema: &Semantics<'_>,
        file_id: HirFileId,
        token: &SyntaxToken,
    ) -> Option<Self> {
        let parent = token.parent()?;
        Self::from_node(sema, file_id, &parent)
    }

    pub fn from_node(
        sema: &Semantics<'_>,
        file_id: HirFileId,
        node: &SyntaxNode,
    ) -> Option<Self> {
        match_ast! {
            match node {
                ast::NameReference(name_ref) => {
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
    name_ref: &ast::NameReference,
) -> Option<Definition> {
    let parent = name_ref.syntax().parent()?;

    if let Some(expression) = ast::PathExpression::cast(parent.clone()) {
        let name = Name::from(expression.name_ref()?);
        let def = sema.find_container(file_id, expression.syntax())?;
        let def = sema.resolve_name_in_expression_scope(def, expression.syntax(), name)?;

        Some(def)
    } else if let Some(expression) = ast::FieldExpression::cast(parent.clone()) {
        let def = sema.find_container(file_id, expression.syntax())?;
        let field = sema.analyze(def).resolve_field(expression)?;

        Some(Definition::Field(field))
    } else if let Some(expression) = ast::FunctionCall::cast(parent.clone()) {
        let resolver = sema.resolver(file_id, expression.syntax());

        match resolver.resolve_callable(&expression.name_ref()?.into())? {
            ResolveCallable::Struct(loc) => {
                let id = sema.database.intern_struct(loc);
                Some(Definition::Struct(Struct { id }))
            },
            ResolveCallable::TypeAlias(loc) => {
                let id = sema.database.intern_type_alias(loc);
                Some(Definition::TypeAlias(TypeAlias { id }))
            },
            ResolveCallable::Function(function) => {
                let id = sema.database.intern_function(function);
                Some(Definition::ModuleDef(ModuleDef::Function(Function { id })))
            },
            ResolveCallable::PredeclaredTypeAlias(_) => None,
        }
    } else if let Some(r#type) = ast::PathType::cast(parent) {
        let resolver = sema.resolver(file_id, r#type.syntax());

        match resolver.resolve_type(&r#type.name()?.into())? {
            ResolveType::Struct(loc) => {
                let id = sema.database.intern_struct(loc);
                Some(Definition::Struct(Struct { id }))
            },
            ResolveType::TypeAlias(loc) => {
                let id = sema.database.intern_type_alias(loc);
                Some(Definition::TypeAlias(TypeAlias { id }))
            },
            ResolveType::PredeclaredTypeAlias(_) => {
                // TODO: should this return something?
                None
            },
        }
    } else {
        None
    }
}
