use base_db::EditionedFileId;
use hir_def::{
    expression_store::path::Path, item_tree::Name, mod_path::ModPath, resolver::ResolveKind,
};
use syntax::{AstNode as _, SyntaxNode, SyntaxToken, ast, match_ast};

use crate::{
    Field, Function, GlobalConstant, GlobalVariable, Local, ModuleDef, Override, Semantics, Struct,
    TypeAlias,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Definition {
    Local(Local),
    Field(Field),
    ModuleDef(ModuleDef),
    BuiltinFunction(Name),
    BuiltinType(Name),
    BuiltinTypeGenerator(Name),
    BuiltinTypeConstructor(Name),
    BuiltinEnumerant(Name),
    BuiltinDeclaration(Name),
}

impl Definition {
    #[must_use]
    pub fn from_token(
        semantics: &Semantics<'_>,
        file_id: EditionedFileId,
        token: &SyntaxToken,
    ) -> Option<Self> {
        let parent = token.parent()?;
        Self::from_node(semantics, file_id, &parent)
    }

    pub fn from_node(
        semantics: &Semantics<'_>,
        file_id: EditionedFileId,
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

impl From<ResolveKind> for Definition {
    fn from(value: ResolveKind) -> Self {
        match value {
            ResolveKind::Local(binding, parent) => Self::Local(Local { parent, binding }),
            ResolveKind::GlobalVariable(id) => {
                Self::ModuleDef(ModuleDef::GlobalVariable(GlobalVariable { id }))
            },
            ResolveKind::GlobalConstant(id) => {
                Self::ModuleDef(ModuleDef::GlobalConstant(GlobalConstant { id }))
            },
            ResolveKind::Override(id) => Self::ModuleDef(ModuleDef::Override(Override { id })),
            ResolveKind::Struct(id) => Self::ModuleDef(ModuleDef::Struct(Struct { id })),
            ResolveKind::TypeAlias(id) => Self::ModuleDef(ModuleDef::TypeAlias(TypeAlias { id })),
            ResolveKind::Function(id) => Self::ModuleDef(ModuleDef::Function(Function { id })),
            ResolveKind::BuiltinFunction(name) => Self::BuiltinFunction(name),
            ResolveKind::BuiltinType(name) => Self::BuiltinType(name),
            ResolveKind::BuiltinTypeGenerator(name) => Self::BuiltinTypeGenerator(name),
            ResolveKind::BuiltinTypeConstructor(name) => Self::BuiltinTypeConstructor(name),
            ResolveKind::BuiltinEnumerant(name) => Self::BuiltinEnumerant(name),
            ResolveKind::BuiltinDeclaration(name) => Self::BuiltinDeclaration(name),
        }
    }
}

fn resolve_path(
    semantics: &Semantics<'_>,
    file_id: EditionedFileId,
    path: &ast::Path,
) -> Option<Definition> {
    let parent = path.syntax().parent()?;

    if ast::IdentExpression::can_cast(parent.kind()) || ast::TypeSpecifier::can_cast(parent.kind())
    {
        let resolver = semantics.resolver(file_id, path.syntax());
        resolver
            .resolve(semantics.db, &Path(ModPath::from_src(path)))
            .ok()
            .map(Definition::from)
    } else if let Some(expression) = ast::FieldExpression::cast(parent) {
        resolve_field(semantics, file_id, expression)
    } else {
        None
    }
}

fn resolve_field(
    semantics: &Semantics<'_>,
    file_id: EditionedFileId,
    field_expression: ast::FieldExpression,
) -> Option<Definition> {
    let definition = semantics.find_container(file_id, field_expression.syntax())?;
    let field = semantics
        .analyze(definition.as_def_with_body_id()?)
        .resolve_field(field_expression)?;
    Some(Definition::Field(field))
}
