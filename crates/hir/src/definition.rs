use hir_def::{
    HirFileId, InFile,
    database::{DefDatabase as _, DefinitionWithBodyId, Lookup as _},
    expression_store::path::Path,
    item_tree::Name,
    mod_path::ModPath,
    resolver::{ResolveKind, Resolver},
    signature::FieldId,
};
use hir_ty::{
    database::HirDatabase,
    infer::TypeLoweringContext,
    ty::pretty::{pretty_fn, pretty_type},
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
                ast::Name(_name) => {
                    resolve_name_at_declaration(semantics, file_id, node)
                },
                _ => {
                    tracing::warn!("attempted to go to definition {:?}", node);
                    None
                }
            }
        }
    }

    /// Returns a human-readable hover text for this definition.
    #[must_use]
    pub fn hover_text(
        &self,
        database: &dyn HirDatabase,
    ) -> Option<String> {
        match self {
            Self::Local(local) => {
                let infer = database.infer(DefinitionWithBodyId::Function(local.parent));
                let ty = infer[local.binding];
                let (body, _) =
                    database.body_with_source_map(DefinitionWithBodyId::Function(local.parent));
                let name = &body.bindings[local.binding].name;
                Some(format!(
                    "let {}: {}",
                    name.as_str(),
                    pretty_type(database, ty)
                ))
            },
            Self::Field(field) => {
                let field_types = database.field_types(field.id.r#struct);
                let ty = field_types.0.get(field.id.field)?;
                let struct_data = database.struct_data(field.id.r#struct).0;
                let field_data = &struct_data.fields()[field.id.field];
                Some(format!(
                    "{}: {}",
                    field_data.name.as_str(),
                    pretty_type(database, *ty)
                ))
            },
            Self::ModuleDef(module_def) => match module_def {
                ModuleDef::Function(function) => {
                    let resolved = database.function_type(function.id);
                    let details = resolved.lookup(database);
                    Some(pretty_fn(database, &details))
                },
                ModuleDef::GlobalVariable(var) => hover_global_variable(database, var.id),
                ModuleDef::GlobalConstant(constant) => hover_global_constant(database, constant.id),
                ModuleDef::Override(override_decl) => hover_override(database, override_decl.id),
                ModuleDef::Struct(s) => {
                    let data = database.struct_data(s.id).0;
                    Some(format!("struct {}", data.name.as_str()))
                },
                ModuleDef::TypeAlias(alias) => {
                    let resolved = database.type_alias_type(alias.id);
                    let ty = resolved.0;
                    let data = database.type_alias_data(alias.id).0;
                    Some(format!(
                        "alias {} = {}",
                        data.name.as_str(),
                        pretty_type(database, ty)
                    ))
                },
                ModuleDef::GlobalAssertStatement(_) => None,
            },
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

        match resolver.resolve(
            semantics.database,
            &Path(ModPath::from_src(&r#type.path()?)),
        )? {
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

/// Resolves an `ast::Name` at a declaration site to a `Definition`.
/// This handles hovering over the name in `fn myFunc(...)`, `struct MyStruct`, etc.
fn resolve_name_at_declaration(
    semantics: &Semantics<'_>,
    file_id: HirFileId,
    name_node: &SyntaxNode,
) -> Option<Definition> {
    let name = ast::Name::cast(name_node.clone())?;
    let parent = name_node.parent()?;
    match_ast! {
        match parent {
            ast::FunctionDeclaration(decl) => {
                let id = semantics.function_to_def(&InFile::new(file_id, decl))?;
                Some(Definition::ModuleDef(ModuleDef::Function(Function { id })))
            },
            ast::VariableDeclaration(decl) => {
                // Try global var first, then local binding
                if let Some(id) = semantics.global_variable_to_def(&InFile::new(file_id, decl)) {
                    Some(Definition::ModuleDef(ModuleDef::GlobalVariable(GlobalVariable { id })))
                } else {
                    resolve_local_binding(semantics, file_id, &name)
                }
            },
            ast::LetDeclaration(_decl) => {
                resolve_local_binding(semantics, file_id, &name)
            },
            ast::ConstantDeclaration(decl) => {
                // Try global const first, then local binding
                if let Some(id) = semantics.global_constant_to_def(&InFile::new(file_id, decl)) {
                    Some(Definition::ModuleDef(ModuleDef::GlobalConstant(GlobalConstant { id })))
                } else {
                    resolve_local_binding(semantics, file_id, &name)
                }
            },
            ast::OverrideDeclaration(decl) => {
                let id = semantics.global_override_to_def(&InFile::new(file_id, decl))?;
                Some(Definition::ModuleDef(ModuleDef::Override(Override { id })))
            },
            ast::StructDeclaration(decl) => {
                let id = semantics.global_struct_to_def(&InFile::new(file_id, decl))?;
                Some(Definition::ModuleDef(ModuleDef::Struct(Struct { id })))
            },
            ast::TypeAliasDeclaration(decl) => {
                let id = semantics.global_type_alias_to_def(&InFile::new(file_id, decl))?;
                Some(Definition::ModuleDef(ModuleDef::TypeAlias(TypeAlias { id })))
            },
            ast::StructMember(_member) => {
                resolve_struct_member_field(semantics, file_id, &name)
            },
            ast::Parameter(_param) => {
                resolve_local_binding(semantics, file_id, &name)
            },
            _ => None,
        }
    }
}

/// Resolves a local binding (let/var/const/parameter inside a function body) to a `Definition::Local`.
fn resolve_local_binding(
    semantics: &Semantics<'_>,
    file_id: HirFileId,
    name: &ast::Name,
) -> Option<Definition> {
    let container = semantics.find_container(file_id, name.syntax())?;
    if let DefinitionWithBodyId::Function(function_id) = container.as_def_with_body_id()? {
        let analyzer = semantics.analyze(DefinitionWithBodyId::Function(function_id));
        let binding_id = analyzer.binding_id(name)?;
        Some(Definition::Local(Local {
            parent: function_id,
            binding: binding_id,
        }))
    } else {
        None
    }
}

/// Resolves a struct member name to a `Definition::Field`.
fn resolve_struct_member_field(
    semantics: &Semantics<'_>,
    file_id: HirFileId,
    name: &ast::Name,
) -> Option<Definition> {
    // Walk up: Name -> StructMember -> StructBody -> StructDeclaration
    let member = name.syntax().parent()?;
    let body = member.parent()?;
    let struct_node = body.parent()?;
    let struct_decl = ast::StructDeclaration::cast(struct_node)?;
    let struct_id = semantics.global_struct_to_def(&InFile::new(file_id, struct_decl))?;

    // Find which field index this member is
    let struct_data = semantics.database.struct_data(struct_id).0;
    let name_text = name.ident_token()?.text().to_string();
    let field_id = struct_data
        .fields()
        .iter()
        .find(|(_idx, field_data)| field_data.name.as_str() == name_text)?
        .0;

    Some(Definition::Field(Field {
        id: FieldId {
            r#struct: struct_id,
            field: field_id,
        },
    }))
}

fn hover_global_variable(
    database: &dyn HirDatabase,
    id: hir_def::database::GlobalVariableId,
) -> Option<String> {
    let data = database.global_var_data(id).0;
    let file_id = id.lookup(database).file_id;
    let module_info = database.item_tree(file_id);
    let resolver = Resolver::default().push_module_scope(file_id, module_info);
    let mut type_context = TypeLoweringContext::new(database, &resolver, &data.store);
    if let Some(type_ref) = data.r#type {
        let ty = type_context.lower_type(type_ref);
        Some(format!(
            "var {}: {}",
            data.name.as_str(),
            pretty_type(database, ty)
        ))
    } else {
        Some(format!("var {}", data.name.as_str()))
    }
}

fn hover_global_constant(
    database: &dyn HirDatabase,
    id: hir_def::database::GlobalConstantId,
) -> Option<String> {
    let data = database.global_constant_data(id).0;
    let file_id = id.lookup(database).file_id;
    let module_info = database.item_tree(file_id);
    let resolver = Resolver::default().push_module_scope(file_id, module_info);
    let mut type_context = TypeLoweringContext::new(database, &resolver, &data.store);
    if let Some(type_ref) = data.r#type {
        let ty = type_context.lower_type(type_ref);
        Some(format!(
            "const {}: {}",
            data.name.as_str(),
            pretty_type(database, ty)
        ))
    } else {
        Some(format!("const {}", data.name.as_str()))
    }
}

fn hover_override(
    database: &dyn HirDatabase,
    id: hir_def::database::OverrideId,
) -> Option<String> {
    let data = database.override_data(id).0;
    let file_id = id.lookup(database).file_id;
    let module_info = database.item_tree(file_id);
    let resolver = Resolver::default().push_module_scope(file_id, module_info);
    let mut type_context = TypeLoweringContext::new(database, &resolver, &data.store);
    if let Some(type_ref) = data.r#type {
        let ty = type_context.lower_type(type_ref);
        Some(format!(
            "override {}: {}",
            data.name.as_str(),
            pretty_type(database, ty)
        ))
    } else {
        Some(format!("override {}", data.name.as_str()))
    }
}
