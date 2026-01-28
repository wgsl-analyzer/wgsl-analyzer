use hir_def::{
    HirFileId, expression_store::path::Path, item_tree::Name, mod_path::ModPath,
    resolver::ResolveKind,
};
use syntax::{AstNode as _, SyntaxNode, SyntaxToken, ast, match_ast};

use crate::{Field, Local, ModuleDef, Semantics, Struct, TypeAlias};

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
                ast::Path(name_ref) => {
                    resolve_path(semantics, file_id, &name_ref)
                },
                ast::FieldExpression(field_expression) => {
                    resolve_field(semantics, file_id, field_expression)
                },
                _ => {
                    tracing::warn!("attempted to go to definition {:?}", node);
                    None
                }
            }
        }
    }
}

fn resolve_path(
    semantics: &Semantics<'_>,
    file_id: HirFileId,
    path: &ast::Path,
) -> Option<Definition> {
    let parent = path.syntax().parent()?;

    if let Some(expression) = ast::IdentExpression::cast(parent.clone()) {
        let path = Path(ModPath::from_src(path));
        let definition = semantics.find_container(file_id, expression.syntax())?;
        let expression_node =
            if let Some(function_call) = ast::FunctionCall::cast(expression.syntax().parent()?) {
                ast::Expression::cast(function_call.syntax().clone())?
            } else {
                ast::Expression::cast(expression.syntax().clone())?
            };
        let definition =
            semantics.resolve_path_in_container(definition, &expression_node, &path)?;

        Some(definition)
    } else if let Some(expression) = ast::FieldExpression::cast(parent.clone()) {
        resolve_field(semantics, file_id, expression)
    } else if let Some(r#type) = ast::TypeSpecifier::cast(parent) {
        let resolver = semantics.resolver(file_id, r#type.syntax());

        match resolver.resolve(&Path(ModPath::from_src(&r#type.path()?)))? {
            ResolveKind::Struct(location) => {
                let id = semantics.database.intern_struct(location);
                Some(Definition::ModuleDef(ModuleDef::Struct(Struct { id })))
            },
            ResolveKind::TypeAlias(location) => {
                let id = semantics.database.intern_type_alias(location);
                Some(Definition::ModuleDef(ModuleDef::TypeAlias(TypeAlias {
                    id,
                })))
            },
            // Type specifiers always represent types
            ResolveKind::Function(_)
            | ResolveKind::GlobalConstant(_)
            | ResolveKind::GlobalVariable(_)
            | ResolveKind::Override(_)
            | ResolveKind::Local(_) => None,
        }
    } else {
        None
    }
}

fn resolve_field(
    semantics: &Semantics<'_>,
    file_id: HirFileId,
    field_expression: ast::FieldExpression,
) -> Option<Definition> {
    let definition = semantics.find_container(file_id, field_expression.syntax())?;
    let field = semantics
        .analyze(definition.as_def_with_body_id()?)
        .resolve_field(field_expression)?;
    Some(Definition::Field(field))
}
