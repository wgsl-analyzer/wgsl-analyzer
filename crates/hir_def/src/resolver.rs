use std::sync::Arc;

use crate::{
    body::{
        scope::{ExprScopes, ScopeId},
        BindingId,
    },
    db::{DefDatabase, FunctionId, Location},
    hir_file_id::ImportFile,
    module_data::{Function, GlobalConstant, GlobalVariable, ModuleInfo, ModuleItem, Name, Struct},
    HirFileId, InFile,
};

#[derive(Clone)]
pub enum Scope {
    /// The items inside a module
    ModuleScope(ModuleScope),
    /// Local bindings
    ExprScope(ExprScope),
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
    Function(Location<Function>),
}

#[derive(Debug)]
pub enum ResolveType {
    Struct(Location<Struct>),
}

pub enum ScopeDef {
    Local(BindingId),
    ModuleItem(HirFileId, ModuleItem),
}

#[derive(Default, Clone)]
pub struct Resolver {
    scopes: Vec<Scope>, // TODO: smallvec<2>
}
impl Resolver {
    #[must_use]
    pub fn push_scope(mut self, scope: Scope) -> Resolver {
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

                self = self.push_module_scope(db, import_file, module_info);
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
    pub fn process_value_names(&self, mut f: impl FnMut(Name, ScopeDef)) {
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
                        ModuleItem::Struct(_) => {}
                        ModuleItem::Import(_) => {}
                        ModuleItem::TypeAlias(_) => {} // TODO: ?
                    });
            }
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
            }
        });
    }

    pub fn resolve_value(&self, name: &Name) -> Option<ResolveValue> {
        self.scopes().find_map(|scope| match scope {
            Scope::ExprScope(scope) => {
                let entry = scope
                    .expr_scopes
                    .resolve_name_in_scope(scope.scope_id, name)?;
                Some(ResolveValue::Local(entry.binding))
            }
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
                        }
                        ModuleItem::GlobalConstant(c)
                            if &scope.module_info.data[c.index].name == name =>
                        {
                            Some(ResolveValue::GlobalConstant(Location::new(
                                scope.file_id,
                                *c,
                            )))
                        }
                        ModuleItem::Function(f)
                            if &scope.module_info.data[f.index].name == name =>
                        {
                            Some(ResolveValue::Function(Location::new(scope.file_id, *f)))
                        }
                        _ => None,
                    })
            }
        })
    }

    pub fn resolve_type(&self, name: &Name) -> Option<ResolveType> {
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
                        }
                        _ => None,
                    })
            }
            Scope::ExprScope(_) => None,
        })
    }
}
