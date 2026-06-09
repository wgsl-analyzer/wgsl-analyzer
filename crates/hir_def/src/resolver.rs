use std::ops::ControlFlow;

use base_db::{EditionedFileId, Intern as _};
use triomphe::Arc;

use crate::{
    InFile,
    body::{
        BindingId,
        scope::{ExprScopes, ScopeId},
    },
    database::{
        self, DefDatabase, FunctionId, GlobalConstantId, GlobalVariableId, Location, OverrideId,
        StructId, TypeAliasId,
    },
    expression_store::path::Path,
    item_tree::{
        Function, GlobalConstant, GlobalVariable, ItemTree, ModuleItemId, Name, Override, Struct,
        TypeAlias,
    },
};

#[derive(Clone)]
pub enum Scope {
    /// The items inside a module.
    Module(ModuleScope),
    /// Local bindings.
    Expression(ExpressionScope),

    Builtin,
}

#[derive(Clone)]
pub struct ModuleScope {
    module_info: Arc<ItemTree>,
    file_id: EditionedFileId,
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
    Struct(StructId),
    TypeAlias(TypeAliasId),
    GlobalVariable(GlobalVariableId),
    GlobalConstant(GlobalConstantId),
    Override(OverrideId),
    Function(FunctionId),
}

pub enum ScopeDef {
    Local(BindingId),
    ModuleItem(EditionedFileId, ModuleItemId),
}

#[derive(Clone)]
pub struct Resolver {
    scopes: Vec<Scope>,
}

impl Resolver {
    #[must_use]
    pub fn new(
        file_id: EditionedFileId,
        module_info: Arc<ItemTree>,
    ) -> Self {
        let module_scope = ModuleScope {
            module_info,
            file_id,
        };
        Self {
            scopes: vec![Scope::Builtin, Scope::Module(module_scope)],
        }
    }

    #[must_use]
    pub fn push_scope(
        mut self,
        scope: Scope,
    ) -> Self {
        self.scopes.push(scope);
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
    pub fn process_all_names<Function>(
        &self,
        mut function: Function,
    ) where
        Function: FnMut(Name, ScopeDef),
    {
        self.scopes().for_each(|scope| match scope {
            Scope::Module(scope) => {
                scope
                    .module_info
                    .top_level_items()
                    .iter()
                    .for_each(|item| match item {
                        ModuleItemId::Function(id) => function(
                            scope.module_info[*id].name.clone(),
                            ScopeDef::ModuleItem(scope.file_id, *item),
                        ),
                        ModuleItemId::GlobalVariable(id) => function(
                            scope.module_info[*id].name.clone(),
                            ScopeDef::ModuleItem(scope.file_id, *item),
                        ),
                        ModuleItemId::GlobalConstant(id) => function(
                            scope.module_info[*id].name.clone(),
                            ScopeDef::ModuleItem(scope.file_id, *item),
                        ),
                        ModuleItemId::Override(id) => function(
                            scope.module_info[*id].name.clone(),
                            ScopeDef::ModuleItem(scope.file_id, *item),
                        ),
                        ModuleItemId::Struct(id) => function(
                            scope.module_info[*id].name.clone(),
                            ScopeDef::ModuleItem(scope.file_id, *item),
                        ),
                        ModuleItemId::TypeAlias(id) => function(
                            scope.module_info[*id].name.clone(),
                            ScopeDef::ModuleItem(scope.file_id, *item),
                        ),
                        ModuleItemId::GlobalAssertStatement(_) => {},
                        ModuleItemId::ImportStatement(id) => {
                            // The leaves of the tree are in scope
                            scope.module_info[*id].expand::<(), _>(|flat_import| {
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
        database: &dyn DefDatabase,
    ) -> Option<ResolveKind> {
        let mod_path = path.mod_path();
        let leaf_name = match mod_path.kind() {
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
            Scope::Module(scope) => {
                scope
                    .module_info
                    .top_level_items()
                    .iter()
                    .find_map(|item| match item {
                        ModuleItemId::Struct(id) => {
                            let r#struct = &scope.module_info[*id];
                            (&r#struct.name == leaf_name).then(|| {
                                ResolveKind::Struct(
                                    InFile::new(scope.file_id, *id).intern(database),
                                )
                            })
                        },
                        ModuleItemId::TypeAlias(id) => {
                            let type_alias = &scope.module_info[*id];
                            (&type_alias.name == leaf_name).then(|| {
                                ResolveKind::TypeAlias(
                                    InFile::new(scope.file_id, *id).intern(database),
                                )
                            })
                        },
                        ModuleItemId::GlobalVariable(id) => {
                            let variable = &scope.module_info[*id];
                            (&variable.name == leaf_name).then(|| {
                                ResolveKind::GlobalVariable(
                                    Location::new(scope.file_id, *id).intern(database),
                                )
                            })
                        },
                        ModuleItemId::GlobalConstant(id) => {
                            let constant = &scope.module_info[*id];
                            (&constant.name == leaf_name).then(|| {
                                ResolveKind::GlobalConstant(
                                    Location::new(scope.file_id, *id).intern(database),
                                )
                            })
                        },
                        ModuleItemId::Override(id) => {
                            let r#override = &scope.module_info[*id];
                            (&r#override.name == leaf_name).then(|| {
                                ResolveKind::Override(
                                    Location::new(scope.file_id, *id).intern(database),
                                )
                            })
                        },
                        ModuleItemId::Function(id) => {
                            let function = &scope.module_info[*id];
                            (&function.name == leaf_name).then(|| {
                                ResolveKind::Function(
                                    InFile::new(scope.file_id, *id).intern(database),
                                )
                            })
                        },
                        ModuleItemId::GlobalAssertStatement(_) => None,
                        ModuleItemId::ImportStatement(_id) => None,
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
                    })
            },
            Scope::Builtin => {
                // TODO: Match against "name.as_str()" and then point at a "builtin" file
                // See: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/559
                None
            },
        })
    }
}
