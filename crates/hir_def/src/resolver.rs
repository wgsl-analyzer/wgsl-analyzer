use std::ops::ControlFlow;

use triomphe::Arc;

use crate::{
    HirFileId, InFile,
    body::{
        BindingId,
        scope::{ExprScopes, ScopeId},
    },
    database::{FunctionId, Location},
    expression_store::path::Path,
    item_tree::{
        Function, GlobalConstant, GlobalVariable, ItemTree, ModuleItem, Name, Override, Struct,
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
    module_info: Arc<ItemTree>,
    file_id: HirFileId,
}

#[derive(Clone)]
pub struct ExpressionScope {
    owner: FunctionId,
    expression_scopes: Arc<ExprScopes>,
    scope_id: ScopeId,
}

#[derive(Debug)]
pub enum ResolveKind {
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
    scopes: Vec<Scope>,
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
        item_tree: Arc<ItemTree>,
    ) -> Self {
        self.scopes.push(Scope::Module(ModuleScope {
            module_info: item_tree,
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

    /// Calls the passed closure `function` on all names in scope.
    pub fn process_all_names<Function: FnMut(Name, ScopeDef)>(
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
                        ModuleItem::Function(id) => function(
                            scope.module_info.get(*id).name.clone(),
                            ScopeDef::ModuleItem(scope.file_id, *item),
                        ),
                        ModuleItem::GlobalVariable(id) => function(
                            scope.module_info.get(*id).name.clone(),
                            ScopeDef::ModuleItem(scope.file_id, *item),
                        ),
                        ModuleItem::GlobalConstant(id) => function(
                            scope.module_info.get(*id).name.clone(),
                            ScopeDef::ModuleItem(scope.file_id, *item),
                        ),
                        ModuleItem::Override(id) => function(
                            scope.module_info.get(*id).name.clone(),
                            ScopeDef::ModuleItem(scope.file_id, *item),
                        ),
                        ModuleItem::Struct(id) => function(
                            scope.module_info.get(*id).name.clone(),
                            ScopeDef::ModuleItem(scope.file_id, *item),
                        ),
                        ModuleItem::TypeAlias(id) => function(
                            scope.module_info.get(*id).name.clone(),
                            ScopeDef::ModuleItem(scope.file_id, *item),
                        ),
                        ModuleItem::GlobalAssertStatement(_) => {},
                        ModuleItem::ImportStatement(id) => {
                            // The leaves of the tree are in scope
                            scope.module_info.get(*id).expand::<()>(|flat_import| {
                                if let Some(name) = flat_import.leaf_name() {
                                    function(
                                        name.clone(),
                                        ScopeDef::ModuleItem(scope.file_id, *item),
                                    );
                                }
                                std::ops::ControlFlow::Continue(())
                            });
                        },
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
            Scope::Builtin => {
                // TODO: Match against "name.as_str()" and then point at a "builtin" file
                // See: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/559
            },
        });
    }

    #[must_use]
    pub fn resolve(
        &self,
        path: &Path,
    ) -> Option<ResolveKind> {
        let mod_path = path.mod_path();
        let leaf_name = match mod_path.kind {
            crate::mod_path::PathKind::Plain => {
                mod_path.as_ident()?
                // TODO: If the option fails, then we have to import it https://github.com/wgsl-analyzer/wgsl-analyzer/issues/632
            },
            crate::mod_path::PathKind::Super(_) | crate::mod_path::PathKind::Package => {
                // TODO: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/632
                return None;
            },
        };

        self.scopes().find_map(|scope| match scope {
            Scope::Expression(scope) => {
                let entry = scope
                    .expression_scopes
                    .resolve_name_in_scope(scope.scope_id, leaf_name)?;
                Some(ResolveKind::Local(entry.binding))
            },
            Scope::Module(scope) => scope
                .module_info
                .items()
                .iter()
                .find_map(|item| match item {
                    ModuleItem::Struct(id) => {
                        let r#struct = scope.module_info.get(*id);
                        (&r#struct.name == leaf_name)
                            .then(|| ResolveKind::Struct(InFile::new(scope.file_id, *id)))
                    },
                    ModuleItem::TypeAlias(id) => {
                        let type_alias = scope.module_info.get(*id);
                        (&type_alias.name == leaf_name)
                            .then(|| ResolveKind::TypeAlias(InFile::new(scope.file_id, *id)))
                    },
                    ModuleItem::GlobalVariable(id) => {
                        let variable = scope.module_info.get(*id);
                        (&variable.name == leaf_name)
                            .then(|| ResolveKind::GlobalVariable(Location::new(scope.file_id, *id)))
                    },
                    ModuleItem::GlobalConstant(id) => {
                        let constant = scope.module_info.get(*id);
                        (&constant.name == leaf_name)
                            .then(|| ResolveKind::GlobalConstant(Location::new(scope.file_id, *id)))
                    },
                    ModuleItem::Override(id) => {
                        let r#override = scope.module_info.get(*id);
                        (&r#override.name == leaf_name)
                            .then(|| ResolveKind::Override(Location::new(scope.file_id, *id)))
                    },
                    ModuleItem::Function(id) => {
                        let function = scope.module_info.get(*id);
                        (&function.name == leaf_name)
                            .then(|| ResolveKind::Function(InFile::new(scope.file_id, *id)))
                    },
                    ModuleItem::GlobalAssertStatement(_) => None,
                    ModuleItem::ImportStatement(_id) => None,
                    // TODO: Support import statements https://github.com/wgsl-analyzer/wgsl-analyzer/issues/632
                    /*scope
                    .module_info
                    .get(*id)
                    .expand::<ResolveKind>(|flat_import| {
                        if flat_import.leaf_name() == Some(leaf_name) {
                            ControlFlow::Break(ResolveKind::Function(InFile::new(
                                scope.file_id,
                                *id,
                            )))
                        } else {
                            ControlFlow::Continue(())
                        }
                    }),*/
                }),
            Scope::Builtin => {
                // TODO: Match against "name.as_str()" and then point at a "builtin" file
                // See: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/559
                None
            },
        })
    }
}
