use std::ops::ControlFlow;

use triomphe::Arc;
use vfs::AnchoredPath;

use crate::{
    HirFileId, InFile,
    body::{
        BindingId,
        scope::{ExprScopes, ScopeId},
    },
    database::{DefDatabase, FunctionId, Location},
    expression_store::path::Path,
    item_tree::{
        Function, GlobalConstant, GlobalVariable, ImportStatement, ImportTree, ItemTree,
        ModuleItem, Name, Override, Struct, TypeAlias,
    },
    mod_path::PathKind,
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
    pub fn process_all_names(
        &self,
        database: &dyn DefDatabase,
        mut function: impl FnMut(Name, ScopeDef),
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
                            let import = scope.module_info.get(*id);
                            import.expand::<(), _>(|flat_import| {
                                if let Some(leaf_name) = flat_import.leaf_name() {
                                    // Try to resolve the import to the actual item in the target file
                                    if let Some((target_file_id, target_item)) =
                                        resolve_import_item(
                                            database,
                                            scope.file_id,
                                            import,
                                            &flat_import,
                                        )
                                    {
                                        function(
                                            leaf_name.clone(),
                                            ScopeDef::ModuleItem(target_file_id, target_item),
                                        );
                                    } else {
                                        // Fallback: expose the import itself
                                        function(
                                            leaf_name.clone(),
                                            ScopeDef::ModuleItem(scope.file_id, *item),
                                        );
                                    }
                                }
                                ControlFlow::Continue(())
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
        database: &dyn DefDatabase,
        path: &Path,
    ) -> Option<ResolveKind> {
        let mod_path = path.mod_path();
        let leaf_name = match mod_path.kind() {
            crate::mod_path::PathKind::Plain => mod_path.as_ident()?,
            crate::mod_path::PathKind::Super(_) | crate::mod_path::PathKind::Package => {
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
                    ModuleItem::ImportStatement(id) => {
                        let import = scope.module_info.get(*id);
                        import.expand::<ResolveKind, _>(|flat_import| {
                            if flat_import.leaf_name() == Some(leaf_name) {
                                if let Some((target_file_id, target_item)) = resolve_import_item(
                                    database,
                                    scope.file_id,
                                    import,
                                    &flat_import,
                                ) {
                                    if let Some(kind) =
                                        resolve_kind_from_item(target_file_id, &target_item)
                                    {
                                        return ControlFlow::Break(kind);
                                    }
                                }
                            }
                            ControlFlow::Continue(())
                        })
                    },
                }),
            Scope::Builtin => {
                // TODO: Match against "name.as_str()" and then point at a "builtin" file
                // See: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/559
                None
            },
        })
    }
}

/// Convert a `ModuleItem` to a `ResolveKind`.
fn resolve_kind_from_item(
    file_id: HirFileId,
    item: &ModuleItem,
) -> Option<ResolveKind> {
    match item {
        ModuleItem::Function(id) => Some(ResolveKind::Function(InFile::new(file_id, *id))),
        ModuleItem::Struct(id) => Some(ResolveKind::Struct(InFile::new(file_id, *id))),
        ModuleItem::TypeAlias(id) => Some(ResolveKind::TypeAlias(InFile::new(file_id, *id))),
        ModuleItem::GlobalVariable(id) => {
            Some(ResolveKind::GlobalVariable(Location::new(file_id, *id)))
        },
        ModuleItem::GlobalConstant(id) => {
            Some(ResolveKind::GlobalConstant(Location::new(file_id, *id)))
        },
        ModuleItem::Override(id) => Some(ResolveKind::Override(Location::new(file_id, *id))),
        ModuleItem::GlobalAssertStatement(_) | ModuleItem::ImportStatement(_) => None,
    }
}

/// Resolve an import's leaf to the actual item in the target file.
///
/// Given an import like `import package::shared::normal::compute_tbn;`,
/// this resolves `shared/normal.wesl` and finds `compute_tbn` in that file.
fn resolve_import_item(
    database: &dyn DefDatabase,
    anchor_file_id: HirFileId,
    import: &ImportStatement,
    flat_import: &crate::item_tree::FlatImport,
) -> Option<(HirFileId, ModuleItem)> {
    let segments = flat_import.path.segments();
    if segments.len() < 2 {
        return None;
    }

    // All segments except the last are the module path (directories + file name).
    // The last segment is the item name to find in that file.
    let module_segments = &segments[..segments.len() - 1];
    let item_name = &segments[segments.len() - 1];

    let anchor_file = anchor_file_id.original_file(database).file_id;
    let target_file = resolve_import_to_file(database, anchor_file, import.kind, module_segments)?;
    let target_editioned = database.editioned_file_id(target_file);
    let target_hir_file = HirFileId::from(target_editioned);
    let target_tree = database.item_tree(target_hir_file);

    // Find the matching item in the target file
    let target_item = target_tree
        .items()
        .iter()
        .find(|target| item_name_matches(target, &target_tree, item_name))?;

    Some((target_hir_file, *target_item))
}

/// Check if a module item's name matches the given name.
fn item_name_matches(
    item: &ModuleItem,
    tree: &ItemTree,
    name: &Name,
) -> bool {
    match item {
        ModuleItem::Function(id) => &tree.get(*id).name == name,
        ModuleItem::Struct(id) => &tree.get(*id).name == name,
        ModuleItem::TypeAlias(id) => &tree.get(*id).name == name,
        ModuleItem::GlobalVariable(id) => &tree.get(*id).name == name,
        ModuleItem::GlobalConstant(id) => &tree.get(*id).name == name,
        ModuleItem::Override(id) => &tree.get(*id).name == name,
        ModuleItem::GlobalAssertStatement(_) | ModuleItem::ImportStatement(_) => false,
    }
}

/// Resolve an import to the FileId of the imported module file.
///
/// `kind` is the import path kind (Plain, Super, Package).
/// `module_segments` are all path segments except the leaf item name.
/// For `import package::shared::normal::compute_tbn`, module_segments = ["shared", "normal"].
pub fn resolve_import_to_file(
    database: &dyn DefDatabase,
    anchor_file: base_db::FileId,
    kind: PathKind,
    module_segments: &[Name],
) -> Option<base_db::FileId> {
    if module_segments.is_empty() {
        return None;
    }

    // Build the file path from module segments: shared/normal → shared/normal.wesl
    let module_path = module_segments
        .iter()
        .map(|s| s.as_str())
        .collect::<Vec<_>>()
        .join("/");

    // AnchoredPath resolution: pops the anchor filename (leaving its directory),
    // then joins the relative path. So paths are relative to the anchor file's directory.
    //
    // For `package::` imports, files are relative to the package root.
    // Currently we approximate the package root as the anchor file's directory.
    // For `super::` imports, each `super` goes up one additional directory level.
    // For plain imports, files are siblings of the anchor file.
    let relative_path = match kind {
        PathKind::Plain | PathKind::Package => {
            format!("{module_path}.wesl")
        },
        PathKind::Super(levels) => {
            let mut path = String::new();
            for _ in 0..levels {
                path.push_str("../");
            }
            path.push_str(&format!("{module_path}.wesl"));
            path
        },
    };

    let path = AnchoredPath {
        anchor: anchor_file,
        path: &relative_path,
    };
    database.resolve_path(path)
}

/// Extract module segments (all segments except the leaf) from an ImportTree.
/// For `shared::normal::compute_tbn`, returns ["shared", "normal"].
pub fn import_tree_module_segments(tree: &ImportTree) -> Vec<Name> {
    let mut segments = Vec::new();
    collect_path_segments(tree, &mut segments);
    // Remove the last segment (the leaf item name)
    if !segments.is_empty() {
        segments.pop();
    }
    segments
}

fn collect_path_segments(
    tree: &ImportTree,
    segments: &mut Vec<Name>,
) {
    match tree {
        ImportTree::Path { name, item } => {
            segments.push(name.clone());
            collect_path_segments(item, segments);
        },
        ImportTree::Item { name, .. } => {
            segments.push(name.clone());
        },
        ImportTree::Collection { list } => {
            // For collections, use the first item to determine the path
            if let Some(first) = list.first() {
                collect_path_segments(first, segments);
            }
        },
    }
}
