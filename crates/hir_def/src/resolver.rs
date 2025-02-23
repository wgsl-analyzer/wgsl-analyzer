use std::sync::Arc;

use tracing::info;

use crate::{
    HirFileId, InFile,
    body::{
        BindingId,
        scope::{ExprScopes, ScopeId},
    },
    db::{DefDatabase, FunctionId, Location},
    hir_file_id::ImportFile,
    module_data::{
        Function, GlobalConstant, GlobalVariable, ModuleInfo, ModuleItem, Name, Override, Struct,
        TypeAlias,
    },
    type_ref::{TypeRef, VecDimensionality, VecType},
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
    expr_scopes: Arc<ExprScopes>,
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

    PredeclaredTypeAlias(TypeRef),
}

#[derive(Debug)]
pub enum ResolveCallable {
    Struct(Location<Struct>),
    TypeAlias(Location<TypeAlias>),
    Function(Location<Function>),
    // TODO: less special casing pls
    PredeclaredTypeAlias(TypeRef),
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
    ) -> Resolver {
        self.scopes.push(scope);
        self
    }

    #[must_use]
    pub fn push_module_scope(
        mut self,
        db: &dyn DefDatabase,
        file_id: HirFileId,
        module_info: Arc<ModuleInfo>,
    ) -> Resolver {
        for item in module_info.items() {
            if let ModuleItem::Import(import) = item {
                let loc = Location::new(file_id, *import);
                let import_id = db.intern_import(loc);
                let import_file = HirFileId::from(ImportFile { import_id });
                let module_info = db.module_info(import_file);
                if let Some(file_id) = import_file.original_file(db) {
                    self = self.push_module_scope(db, file_id.into(), module_info);
                } else {
                    info!("Failed to resolve import file for {file_id:?}");
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
    pub fn push_expr_scope(
        mut self,
        owner: FunctionId,
        expr_scopes: Arc<ExprScopes>,
        scope_id: ScopeId,
    ) -> Resolver {
        self.scopes.push(Scope::ExprScope(ExprScope {
            owner,
            expr_scopes,
            scope_id,
        }));
        self
    }

    pub fn scopes(&self) -> impl Iterator<Item = &Scope> {
        self.scopes.iter().rev()
    }

    pub fn body_owner(&self) -> Option<FunctionId> {
        self.scopes().find_map(|scope| match scope {
            Scope::ExprScope(scope) => Some(scope.owner),
            _ => None,
        })
    }

    /// calls f for every local, function and global declaration, but not structs
    pub fn process_value_names(
        &self,
        mut f: impl FnMut(Name, ScopeDef),
    ) {
        self.scopes().for_each(|scope| match scope {
            Scope::ModuleScope(scope) => {
                scope
                    .module_info
                    .items()
                    .iter()
                    .for_each(|item| match item {
                        ModuleItem::Function(func) => f(
                            scope.module_info.data[func.index].name.clone(),
                            ScopeDef::ModuleItem(scope.file_id, *item),
                        ),
                        ModuleItem::GlobalVariable(var) => f(
                            scope.module_info.data[var.index].name.clone(),
                            ScopeDef::ModuleItem(scope.file_id, *item),
                        ),
                        ModuleItem::GlobalConstant(constant) => f(
                            scope.module_info.data[constant.index].name.clone(),
                            ScopeDef::ModuleItem(scope.file_id, *item),
                        ),
                        ModuleItem::Override(override_decl) => f(
                            scope.module_info.data[override_decl.index].name.clone(),
                            ScopeDef::ModuleItem(scope.file_id, *item),
                        ),
                        ModuleItem::Struct(_) => {},
                        ModuleItem::Import(_) => {},
                        ModuleItem::TypeAlias(_) => {},
                    });
            },
            Scope::ExprScope(expr_scope) => {
                expr_scope
                    .expr_scopes
                    .scope_chain(Some(expr_scope.scope_id))
                    .for_each(|id| {
                        let data = &expr_scope.expr_scopes[id];
                        data.entries.iter().for_each(|entry| {
                            f(entry.name.clone(), ScopeDef::Local(entry.binding))
                        });
                    });
            },
            Scope::BuiltinScope => {},
        });
    }

    pub fn resolve_value(
        &self,
        name: &Name,
    ) -> Option<ResolveValue> {
        self.scopes().find_map(|scope| -> Option<ResolveValue> {
            match scope {
                Scope::ExprScope(scope) => {
                    let entry = scope
                        .expr_scopes
                        .resolve_name_in_scope(scope.scope_id, name)?;
                    Some(ResolveValue::Local(entry.binding))
                },
                Scope::ModuleScope(scope) => {
                    scope
                        .module_info
                        .items()
                        .iter()
                        .find_map(|item| match item {
                            ModuleItem::GlobalVariable(var)
                                if &scope.module_info.data[var.index].name == name =>
                            {
                                Some(ResolveValue::GlobalVariable(Location::new(
                                    scope.file_id,
                                    *var,
                                )))
                            },
                            ModuleItem::GlobalConstant(c)
                                if &scope.module_info.data[c.index].name == name =>
                            {
                                Some(ResolveValue::GlobalConstant(Location::new(
                                    scope.file_id,
                                    *c,
                                )))
                            },
                            ModuleItem::Override(c)
                                if &scope.module_info.data[c.index].name == name =>
                            {
                                Some(ResolveValue::Override(Location::new(scope.file_id, *c)))
                            },
                            _ => None,
                        })
                },
                Scope::BuiltinScope => None,
            }
        })
    }

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
                            let strukt = scope.module_info.get(*id);
                            (&strukt.name == name)
                                .then(|| ResolveType::Struct(InFile::new(scope.file_id, *id)))
                        },
                        ModuleItem::TypeAlias(id) => {
                            let type_alias = scope.module_info.get(*id);
                            (&type_alias.name == name)
                                .then(|| ResolveType::TypeAlias(InFile::new(scope.file_id, *id)))
                        },
                        _ => None,
                    })
            },
            Scope::ExprScope(_) => None,

            Scope::BuiltinScope => {
                let ty = vec_alias_typeref(name.as_str());
                ty.map(ResolveType::PredeclaredTypeAlias)
            },
        })
    }

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
                            let strukt = scope.module_info.get(*id);
                            (&strukt.name == name)
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
                        _ => None,
                    })
            },
            Scope::ExprScope(_) => None,
            Scope::BuiltinScope => {
                let ty = vec_alias_typeref(name.as_str());
                ty.map(ResolveCallable::PredeclaredTypeAlias)
            },
        })
    }
}

fn vec_alias_typeref(name: &str) -> Option<TypeRef> {
    match name {
        "vec2i" => Some(TypeRef::Vec(VecType {
            size: VecDimensionality::Two,
            inner: Box::new(TypeRef::Scalar(crate::type_ref::ScalarType::Int32)),
        })),
        "vec3i" => Some(TypeRef::Vec(VecType {
            size: VecDimensionality::Three,
            inner: Box::new(TypeRef::Scalar(crate::type_ref::ScalarType::Int32)),
        })),
        "vec4i" => Some(TypeRef::Vec(VecType {
            size: VecDimensionality::Four,
            inner: Box::new(TypeRef::Scalar(crate::type_ref::ScalarType::Int32)),
        })),
        "vec2u" => Some(TypeRef::Vec(VecType {
            size: VecDimensionality::Two,
            inner: Box::new(TypeRef::Scalar(crate::type_ref::ScalarType::Uint32)),
        })),
        "vec3u" => Some(TypeRef::Vec(VecType {
            size: VecDimensionality::Three,
            inner: Box::new(TypeRef::Scalar(crate::type_ref::ScalarType::Uint32)),
        })),
        "vec4u" => Some(TypeRef::Vec(VecType {
            size: VecDimensionality::Four,
            inner: Box::new(TypeRef::Scalar(crate::type_ref::ScalarType::Uint32)),
        })),
        "vec2f" => Some(TypeRef::Vec(VecType {
            size: VecDimensionality::Two,
            inner: Box::new(TypeRef::Scalar(crate::type_ref::ScalarType::Float32)),
        })),
        "vec3f" => Some(TypeRef::Vec(VecType {
            size: VecDimensionality::Three,
            inner: Box::new(TypeRef::Scalar(crate::type_ref::ScalarType::Float32)),
        })),
        "vec4f" => Some(TypeRef::Vec(VecType {
            size: VecDimensionality::Four,
            inner: Box::new(TypeRef::Scalar(crate::type_ref::ScalarType::Float32)),
        })),
        // TODO float16
        _ => None,
    }
}
