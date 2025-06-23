use std::sync::Arc;

use tracing::info;

use crate::{
    HirFileId, InFile,
    body::{
        BindingId,
        scope::{ExprScopes, ScopeId},
    },
    database::{DefDatabase, FunctionId, Location},
    hir_file_id::ImportFile,
    module_data::{
        Function, GlobalConstant, GlobalVariable, ModuleInfo, ModuleItem, Name, Override, Struct,
        TypeAlias,
    },
    type_ref::{TypeReference, VecDimensionality, VecType},
};

#[derive(Clone)]
pub enum Scope {
    /// The items inside a module
    ModuleScope(ModuleScope),
    /// Local bindings
    ExprScope(ExprScope),

    BuiltinScope,
}

#[derive(Clone)]
pub struct ModuleScope {
    module_info: Arc<ModuleInfo>,
    file_id: HirFileId,
}

#[derive(Clone)]
pub struct ExprScope {
    owner: FunctionId,
    expression_scopes: Arc<ExprScopes>,
    scope_id: ScopeId,
}

#[derive(Debug)]
pub enum ResolveValue {
    Local(BindingId),
    GlobalVariable(Location<GlobalVariable>),
    GlobalConstant(Location<GlobalConstant>),
    Override(Location<Override>),
}

#[derive(Debug)]
pub enum ResolveType {
    Struct(Location<Struct>),
    TypeAlias(Location<TypeAlias>),

    PredeclaredTypeAlias(TypeReference),
}

#[derive(Debug)]
pub enum ResolveCallable {
    Struct(Location<Struct>),
    TypeAlias(Location<TypeAlias>),
    Function(Location<Function>),
    // TODO: less special casing pls
    PredeclaredTypeAlias(TypeReference),
}

pub enum ScopeDef {
    Local(BindingId),
    ModuleItem(HirFileId, ModuleItem),
}

#[derive(Clone)]
pub struct Resolver {
    scopes: Vec<Scope>, // TODO: smallvec<2>
}

impl Default for Resolver {
    fn default() -> Self {
        Self {
            scopes: vec![Scope::BuiltinScope],
        }
    }
}

impl Resolver {
    #[must_use]
    pub fn push_scope(
        mut self,
        scope: Scope,
    ) -> Self {
        self.scopes.push(scope);
        self
    }

    #[must_use]
    pub fn push_module_scope(
        mut self,
        database: &dyn DefDatabase,
        file_id: HirFileId,
        module_info: Arc<ModuleInfo>,
    ) -> Self {
        for item in module_info.items() {
            if let ModuleItem::Import(import) = item {
                let loc = Location::new(file_id, *import);
                let import_id = database.intern_import(loc);
                let import_file = HirFileId::from(ImportFile { import_id });
                let import_module_info = database.module_info(import_file);
                // If we can find the original source file for this import, push its scope
                if let Some(original_file_id) = import_file.original_file(database) {
                    let original_file_id = HirFileId::from(original_file_id);
                    self = self.push_module_scope(database, original_file_id, import_module_info);
                } else {
                    info!("Failed to resolve import file for {file_id:?}");
                    // This import might be a custom import without a direct file
                    // For these cases, we'll use the imported module info with the original file ID
                    self = self.push_module_scope(database, file_id, import_module_info);
                    info!("Using module_info for import without resolving to a file: {file_id:?}");
                }
            }
        }

        self.scopes.push(Scope::ModuleScope(ModuleScope {
            module_info,
            file_id,
        }));
        self
    }

    #[must_use]
    pub fn push_expression_scope(
        mut self,
        owner: FunctionId,
        expression_scopes: Arc<ExprScopes>,
        scope_id: ScopeId,
    ) -> Self {
        self.scopes.push(Scope::ExprScope(ExprScope {
            owner,
            expression_scopes,
            scope_id,
        }));
        self
    }

    pub fn scopes(&self) -> impl Iterator<Item = &Scope> {
        self.scopes.iter().rev()
    }

    #[must_use]
    pub fn body_owner(&self) -> Option<FunctionId> {
        self.scopes().find_map(|scope| match scope {
            Scope::ExprScope(scope) => Some(scope.owner),
            Scope::ModuleScope(_) | Scope::BuiltinScope => None,
        })
    }

    /// calls f for every local, function, and global declaration, but not structs
    pub fn process_value_names<Function: FnMut(Name, ScopeDef)>(
        &self,
        mut function: Function,
    ) {
        self.scopes().for_each(|scope| match scope {
            Scope::ModuleScope(scope) => {
                scope
                    .module_info
                    .items()
                    .iter()
                    .for_each(|item| match item {
                        ModuleItem::Function(func) => function(
                            scope.module_info.data[func.index].name.clone(),
                            ScopeDef::ModuleItem(scope.file_id, *item),
                        ),
                        ModuleItem::GlobalVariable(var) => function(
                            scope.module_info.data[var.index].name.clone(),
                            ScopeDef::ModuleItem(scope.file_id, *item),
                        ),
                        ModuleItem::GlobalConstant(constant) => function(
                            scope.module_info.data[constant.index].name.clone(),
                            ScopeDef::ModuleItem(scope.file_id, *item),
                        ),
                        ModuleItem::Override(override_decl) => function(
                            scope.module_info.data[override_decl.index].name.clone(),
                            ScopeDef::ModuleItem(scope.file_id, *item),
                        ),
                        ModuleItem::Struct(_)
                        | ModuleItem::Import(_)
                        | ModuleItem::TypeAlias(_) => {},
                    });
            },
            Scope::ExprScope(expression_scope) => {
                expression_scope
                    .expression_scopes
                    .scope_chain(Some(expression_scope.scope_id))
                    .for_each(|id| {
                        let data = &expression_scope.expression_scopes[id];
                        data.entries.iter().for_each(|entry| {
                            function(entry.name.clone(), ScopeDef::Local(entry.binding));
                        });
                    });
            },
            Scope::BuiltinScope => {},
        });
    }

    #[must_use]
    pub fn resolve_value(
        &self,
        name: &Name,
    ) -> Option<ResolveValue> {
        self.scopes().find_map(|scope| -> Option<ResolveValue> {
            match scope {
                Scope::ExprScope(scope) => {
                    let entry = scope
                        .expression_scopes
                        .resolve_name_in_scope(scope.scope_id, name)?;
                    Some(ResolveValue::Local(entry.binding))
                },
                Scope::ModuleScope(scope) => {
                    scope
                        .module_info
                        .items()
                        .iter()
                        .find_map(|item| match item {
                            ModuleItem::GlobalVariable(variable)
                                if &scope.module_info.data[variable.index].name == name =>
                            {
                                Some(ResolveValue::GlobalVariable(Location::new(
                                    scope.file_id,
                                    *variable,
                                )))
                            },
                            ModuleItem::GlobalConstant(constant)
                                if &scope.module_info.data[constant.index].name == name =>
                            {
                                Some(ResolveValue::GlobalConstant(Location::new(
                                    scope.file_id,
                                    *constant,
                                )))
                            },
                            ModuleItem::Override(r#override)
                                if &scope.module_info.data[r#override.index].name == name =>
                            {
                                Some(ResolveValue::Override(Location::new(
                                    scope.file_id,
                                    *r#override,
                                )))
                            },
                            ModuleItem::Function(_)
                            | ModuleItem::Struct(_)
                            | ModuleItem::GlobalVariable(_)
                            | ModuleItem::GlobalConstant(_)
                            | ModuleItem::Override(_)
                            | ModuleItem::Import(_)
                            | ModuleItem::TypeAlias(_) => None,
                        })
                },
                Scope::BuiltinScope => None,
            }
        })
    }

    #[must_use]
    pub fn resolve_type(
        &self,
        name: &Name,
    ) -> Option<ResolveType> {
        self.scopes().find_map(|scope| match scope {
            Scope::ModuleScope(scope) => {
                scope
                    .module_info
                    .items()
                    .iter()
                    .find_map(|item| match item {
                        ModuleItem::Struct(id) => {
                            let r#struct = scope.module_info.get(*id);
                            (&r#struct.name == name)
                                .then(|| ResolveType::Struct(InFile::new(scope.file_id, *id)))
                        },
                        ModuleItem::TypeAlias(id) => {
                            let type_alias = scope.module_info.get(*id);
                            (&type_alias.name == name)
                                .then(|| ResolveType::TypeAlias(InFile::new(scope.file_id, *id)))
                        },
                        ModuleItem::Function(_)
                        | ModuleItem::GlobalVariable(_)
                        | ModuleItem::GlobalConstant(_)
                        | ModuleItem::Override(_)
                        | ModuleItem::Import(_) => None,
                    })
            },
            Scope::ExprScope(_) => None,

            Scope::BuiltinScope => {
                let r#type = vec_alias_typeref(name.as_str());
                r#type.map(ResolveType::PredeclaredTypeAlias)
            },
        })
    }

    #[must_use]
    pub fn resolve_callable(
        &self,
        name: &Name,
    ) -> Option<ResolveCallable> {
        self.scopes().find_map(|scope| match scope {
            Scope::ModuleScope(scope) => {
                scope
                    .module_info
                    .items()
                    .iter()
                    .find_map(|item| match item {
                        ModuleItem::Struct(id) => {
                            let r#struct = scope.module_info.get(*id);
                            (&r#struct.name == name)
                                .then(|| ResolveCallable::Struct(InFile::new(scope.file_id, *id)))
                        },
                        ModuleItem::TypeAlias(id) => {
                            let type_alias = scope.module_info.get(*id);
                            (&type_alias.name == name).then(|| {
                                ResolveCallable::TypeAlias(InFile::new(scope.file_id, *id))
                            })
                        },
                        ModuleItem::Function(id) => {
                            let function = scope.module_info.get(*id);
                            (&function.name == name)
                                .then(|| ResolveCallable::Function(InFile::new(scope.file_id, *id)))
                        },
                        ModuleItem::GlobalVariable(_)
                        | ModuleItem::GlobalConstant(_)
                        | ModuleItem::Override(_)
                        | ModuleItem::Import(_) => None,
                    })
            },
            Scope::ExprScope(_) => None,
            Scope::BuiltinScope => {
                let r#type = vec_alias_typeref(name.as_str());
                r#type.map(ResolveCallable::PredeclaredTypeAlias)
            },
        })
    }
}

fn vec_alias_typeref(name: &str) -> Option<TypeReference> {
    match name {
        "vec2i" => Some(TypeReference::Vec(VecType {
            size: VecDimensionality::Two,
            inner: Box::new(TypeReference::Scalar(crate::type_ref::ScalarType::Int32)),
        })),
        "vec3i" => Some(TypeReference::Vec(VecType {
            size: VecDimensionality::Three,
            inner: Box::new(TypeReference::Scalar(crate::type_ref::ScalarType::Int32)),
        })),
        "vec4i" => Some(TypeReference::Vec(VecType {
            size: VecDimensionality::Four,
            inner: Box::new(TypeReference::Scalar(crate::type_ref::ScalarType::Int32)),
        })),
        "vec2u" => Some(TypeReference::Vec(VecType {
            size: VecDimensionality::Two,
            inner: Box::new(TypeReference::Scalar(crate::type_ref::ScalarType::Uint32)),
        })),
        "vec3u" => Some(TypeReference::Vec(VecType {
            size: VecDimensionality::Three,
            inner: Box::new(TypeReference::Scalar(crate::type_ref::ScalarType::Uint32)),
        })),
        "vec4u" => Some(TypeReference::Vec(VecType {
            size: VecDimensionality::Four,
            inner: Box::new(TypeReference::Scalar(crate::type_ref::ScalarType::Uint32)),
        })),
        "vec2f" => Some(TypeReference::Vec(VecType {
            size: VecDimensionality::Two,
            inner: Box::new(TypeReference::Scalar(crate::type_ref::ScalarType::Float32)),
        })),
        "vec3f" => Some(TypeReference::Vec(VecType {
            size: VecDimensionality::Three,
            inner: Box::new(TypeReference::Scalar(crate::type_ref::ScalarType::Float32)),
        })),
        "vec4f" => Some(TypeReference::Vec(VecType {
            size: VecDimensionality::Four,
            inner: Box::new(TypeReference::Scalar(crate::type_ref::ScalarType::Float32)),
        })),
        // TODO float16
        _ => None,
    }
}
