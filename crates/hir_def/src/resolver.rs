use std::sync::Arc;

use tracing::info;

use crate::{
    HirFileId, InFile,
    body::{
        BindingId,
        scope::{ExprScopes, ScopeId},
    },
    database::{DefDatabase, FunctionId, Location},
    module_data::{
        Function, GlobalConstant, GlobalVariable, ModuleInfo, ModuleItem, Name, Override, Struct,
        TypeAlias,
    },
};

#[derive(Clone)]
pub enum Scope {
    /// The items inside a module
    Module(ModuleScope),
    /// Local bindings
    Expression(ExpressionScope),

    Builtin,
}

#[derive(Clone)]
pub struct ModuleScope {
    module_info: Arc<ModuleInfo>,
    file_id: HirFileId,
}

#[derive(Clone)]
pub struct ExpressionScope {
    owner: FunctionId,
    expression_scopes: Arc<ExprScopes>,
    scope_id: ScopeId,
}

#[derive(Debug)]
pub enum ResolveType {
    Local(BindingId),
    Struct(Location<Struct>),
    TypeAlias(Location<TypeAlias>),
    GlobalVariable(Location<GlobalVariable>),
    GlobalConstant(Location<GlobalConstant>),
    Override(Location<Override>),
    Function(Location<Function>),
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
            scopes: vec![Scope::Builtin],
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
        file_id: HirFileId,
        module_info: Arc<ModuleInfo>,
    ) -> Self {
        self.scopes.push(Scope::Module(ModuleScope {
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
        self.scopes.push(Scope::Expression(ExpressionScope {
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
            Scope::Expression(scope) => Some(scope.owner),
            Scope::Module(_) | Scope::Builtin => None,
        })
    }

    /// calls function for every local, function, and global declaration, but not structs
    pub fn process_value_names<Function: FnMut(Name, ScopeDef)>(
        &self,
        mut function: Function,
    ) {
        self.scopes().for_each(|scope| match scope {
            Scope::Module(scope) => {
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
                        ModuleItem::Struct(_) | ModuleItem::TypeAlias(_) => {},
                    });
            },
            Scope::Expression(expression_scope) => {
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
            Scope::Builtin => {},
        });
    }

    #[must_use]
    pub fn resolve(
        &self,
        name: &Name,
    ) -> Option<ResolveType> {
        self.scopes().find_map(|scope| match scope {
            Scope::Expression(scope) => {
                let entry = scope
                    .expression_scopes
                    .resolve_name_in_scope(scope.scope_id, name)?;
                Some(ResolveType::Local(entry.binding))
            },
            Scope::Module(scope) => scope
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
                    ModuleItem::GlobalVariable(id) => {
                        let variable = scope.module_info.get(*id);
                        (&variable.name == name)
                            .then(|| ResolveType::GlobalVariable(Location::new(scope.file_id, *id)))
                    },
                    ModuleItem::GlobalConstant(id) => {
                        let constant = scope.module_info.get(*id);
                        (&constant.name == name)
                            .then(|| ResolveType::GlobalConstant(Location::new(scope.file_id, *id)))
                    },
                    ModuleItem::Override(id) => {
                        let r#override = scope.module_info.get(*id);
                        (&r#override.name == name)
                            .then(|| ResolveType::Override(Location::new(scope.file_id, *id)))
                    },
                    ModuleItem::Function(id) => {
                        let function = scope.module_info.get(*id);
                        (&function.name == name)
                            .then(|| ResolveType::Function(InFile::new(scope.file_id, *id)))
                    },
                }),
            Scope::Builtin => {
                // TODO: Match against "name.as_str()" and then point at a "builtin" file
                None
            },
        })
    }
}
