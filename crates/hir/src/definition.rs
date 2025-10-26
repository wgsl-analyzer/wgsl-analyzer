use hir_def::{HirFileId, module_data::Name, resolver::ResolveType};
use syntax::{AstNode as _, SyntaxNode, SyntaxToken, ast, match_ast};

use crate::{Field, Function, Local, ModuleDef, Semantics, Struct, TypeAlias};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Definition {
    Local(Local),
    Field(Field),
    ModuleDef(ModuleDef),
}

impl Definition {
    #[must_use]
    pub fn from_token(
        semantics: &Semantics<'_>,
        file_id: HirFileId,
        token: &SyntaxToken,
    ) -> Option<Self> {
        let parent = token.parent()?;
        Self::from_node(semantics, file_id, &parent)
    }

    pub fn from_node(
        semantics: &Semantics<'_>,
        file_id: HirFileId,
        node: &SyntaxNode,
    ) -> Option<Self> {
        match_ast! {
            match node {
                ast::NameReference(name_ref) => {
                    resolve_name_ref(semantics, file_id, &name_ref)
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
    semantics: &Semantics<'_>,
    file_id: HirFileId,
    name_ref: &ast::NameReference,
) -> Option<Definition> {
    let parent = name_ref.syntax().parent()?;

    if let Some(expression) = ast::IdentExpression::cast(parent.clone()) {
        let name = Name::from(expression.name_ref()?);
        let definition = semantics.find_container(file_id, expression.syntax())?;
        let definition =
            semantics.resolve_name_in_expression_scope(definition, expression.syntax(), &name)?;

        Some(definition)
    } else if let Some(expression) = ast::FieldExpression::cast(parent.clone()) {
        let definition = semantics.find_container(file_id, expression.syntax())?;
        let field = semantics.analyze(definition).resolve_field(expression)?;

        Some(Definition::Field(field))
    } else if let Some(r#type) = ast::TypeSpecifier::cast(parent) {
        let resolver = semantics.resolver(file_id, r#type.syntax());

        match resolver.resolve(&r#type.name_ref()?.into())? {
            ResolveType::Struct(loc) => {
                let id = semantics.database.intern_struct(loc);
                Some(Definition::ModuleDef(ModuleDef::Struct(Struct { id })))
            },
            ResolveType::TypeAlias(loc) => {
                let id = semantics.database.intern_type_alias(loc);
                Some(Definition::ModuleDef(ModuleDef::TypeAlias(TypeAlias {
                    id,
                })))
            },
            // TODO: Why is it not doing anything here?
            ResolveType::Function(_)
            | ResolveType::GlobalConstant(_)
            | ResolveType::GlobalVariable(_)
            | ResolveType::Override(_)
            | ResolveType::Local(_) => None,
        }
    } else {
        None
    }
}
