mod lower;

#[cfg(test)]
pub mod pretty;

use std::{hash, marker::PhantomData, ops::ControlFlow};

use la_arena::{Arena, Idx};
use smol_str::SmolStr;
use syntax::{AstNode, TokenText, ast};
use triomphe::Arc;

use crate::{
    HirFileId,
    ast_id::FileAstId,
    database::DefDatabase,
    mod_path::{ModPath, PathKind},
};

const MISSING_NAME_PLACEHOLDER: &str = "[missing name]";

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Name(SmolStr);

impl Name {
    #[must_use]
    pub fn missing() -> Self {
        Self(MISSING_NAME_PLACEHOLDER.into())
    }

    #[must_use]
    pub fn is_missing(value: &str) -> bool {
        value == MISSING_NAME_PLACEHOLDER
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<TokenText<'_>> for Name {
    fn from(text: TokenText<'_>) -> Self {
        Self(text.as_str().into())
    }
}

impl From<ast::Name> for Name {
    fn from(name: ast::Name) -> Self {
        Self(name.text().as_str().into())
    }
}

impl From<&'_ str> for Name {
    fn from(text: &str) -> Self {
        Self(text.into())
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ImportStatement {
    pub kind: PathKind,
    pub tree: ImportTree,
    pub ast_id: FileAstId<ast::ImportStatement>,
}

impl ImportStatement {
    /// Expands the `UseTree` into individually imported `FlatImport`s.
    pub fn expand<T, Callback: FnMut(FlatImport) -> ControlFlow<T>>(
        &self,
        mut callback: Callback,
    ) -> Option<T> {
        self.tree
            .expand_impl(ModPath::from_kind(self.kind), &mut callback)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct FlatImport {
    pub path: ModPath,
    pub alias: Option<Name>,
}

impl FlatImport {
    #[must_use]
    pub fn leaf_name(&self) -> Option<&Name> {
        self.alias.as_ref().or_else(|| self.path.segments().last())
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ImportTree {
    Path {
        name: Name,
        item: Box<Self>,
    },
    /// ```ignore
    /// foo as bar
    /// ```
    Item {
        name: Name,
        alias: Option<Name>,
    },
    /// ```ignore
    /// {Foo, Bar, Baz};
    /// ```
    Collection {
        list: Vec<Self>,
    },
}

impl ImportTree {
    fn expand_impl<T>(
        &self,
        mut prefix: ModPath,
        callback: &mut impl FnMut(FlatImport) -> ControlFlow<T>,
    ) -> Option<T> {
        match self {
            Self::Path { name, item } => {
                prefix.push_segment(name.clone());
                item.expand_impl(prefix, callback)
            },
            Self::Item { name, alias } => {
                prefix.push_segment(name.clone());
                callback(FlatImport {
                    path: prefix,
                    alias: alias.clone(),
                })
                .break_value()
            },
            Self::Collection { list } => {
                for tree in list {
                    if let Some(value) = tree.expand_impl(prefix.clone(), callback) {
                        return Some(value);
                    }
                }
                None
            },
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Directive {
    pub ast_id: FileAstId<ast::Directive>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Function {
    pub name: Name,
    pub ast_id: FileAstId<ast::FunctionDeclaration>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GlobalVariable {
    pub name: Name,
    pub ast_id: FileAstId<ast::VariableDeclaration>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GlobalConstant {
    pub name: Name,
    pub ast_id: FileAstId<ast::ConstantDeclaration>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Override {
    pub name: Name,
    pub ast_id: FileAstId<ast::OverrideDeclaration>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TypeAlias {
    pub name: Name,
    pub ast_id: FileAstId<ast::TypeAliasDeclaration>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GlobalAssertStatement {
    pub ast_id: FileAstId<ast::AssertStatement>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Struct {
    pub name: Name,
    pub ast_id: FileAstId<ast::StructDeclaration>,
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct ItemTree {
    top_level: Vec<ModuleItem>,
    imports: Arena<ImportStatement>,
    functions: Arena<Function>,
    global_variables: Arena<GlobalVariable>,
    global_constants: Arena<GlobalConstant>,
    overrides: Arena<Override>,
    type_aliases: Arena<TypeAlias>,
    structs: Arena<Struct>,
    directives: Arena<Directive>,
    global_assert_statements: Arena<GlobalAssertStatement>,
}

impl ItemTree {
    pub fn query(
        database: &dyn DefDatabase,
        file_id: HirFileId,
    ) -> Arc<Self> {
        let source = database.parse_or_resolve(file_id).tree();

        let lower_ctx = lower::Ctx::new(database, file_id);
        let tree = lower_ctx.lower_source_file(&source);

        Arc::new(tree)
    }

    #[must_use]
    pub fn top_level_items(&self) -> &[ModuleItem] {
        &self.top_level
    }

    pub fn structs(&self) -> impl Iterator<Item = ModuleItemId<Struct>> + '_ {
        self.top_level.iter().filter_map(|item| match item {
            ModuleItem::Struct(r#struct) => Some(*r#struct),
            ModuleItem::ImportStatement(_)
            | ModuleItem::Function(_)
            | ModuleItem::GlobalVariable(_)
            | ModuleItem::GlobalConstant(_)
            | ModuleItem::Override(_)
            | ModuleItem::GlobalAssertStatement(_)
            | ModuleItem::TypeAlias(_) => None,
        })
    }

    #[must_use]
    pub fn get<M: ItemTreeNode>(
        &self,
        id: ModuleItemId<M>,
    ) -> &M {
        M::lookup(self, id.index)
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct ModuleItemId<N> {
    pub(crate) index: Idx<N>,
    _marker: PhantomData<N>,
}

impl<N> From<Idx<N>> for ModuleItemId<N> {
    fn from(index: Idx<N>) -> Self {
        Self {
            index,
            _marker: PhantomData,
        }
    }
}

// If we automatically derive this trait, ModuleItemId<N> where N does not implement Hash cannot compile
impl<N> hash::Hash for ModuleItemId<N> {
    fn hash<H: hash::Hasher>(
        &self,
        state: &mut H,
    ) {
        self.index.hash(state);
    }
}

impl<N> Clone for ModuleItemId<N> {
    fn clone(&self) -> Self {
        Self {
            index: self.index,
            _marker: PhantomData,
        }
    }
}

impl<N: ItemTreeNode> Copy for ModuleItemId<N> {}

pub trait ItemTreeNode: Clone {
    type Source: AstNode + Into<ast::Item>;

    fn ast_id(&self) -> FileAstId<Self::Source>;

    /// Looks up an instance of `Self` in an item tree.
    fn lookup(
        data: &ItemTree,
        index: Idx<Self>,
    ) -> &Self;

    /// Downcasts a `ModItem` to a `FileItemTreeId` specific to this type.
    fn id_from_mod_item(mod_item: ModuleItem) -> Option<ModuleItemId<Self>>;

    /// Upcasts a `FileItemTreeId` to a generic `ModuleItem`.
    fn id_to_mod_item(id: ModuleItemId<Self>) -> ModuleItem;
}

macro_rules! mod_items {
    ( $( $r#type:ident in $fld:ident $(-> $ast:ty)? ),+ $(,)? ) => {
        #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
        pub enum ModuleItem {
            $($r#type(ModuleItemId<$r#type>),)+
        }

        $(impl From<ModuleItemId<$r#type>> for ModuleItem {
            fn from(id: ModuleItemId<$r#type>) -> ModuleItem {
                ModuleItem::$r#type(id)
            }
        })+

        $(impl core::ops::Index<la_arena::Idx<$r#type>> for ItemTree {
            type Output = $r#type;

            fn index(&self, index: la_arena::Idx<$r#type>) -> &Self::Output {
                &self.$fld[index]
            }
        })*

        $(
        $(impl ItemTreeNode for $r#type {
                type Source = $ast;

                fn ast_id(&self) -> FileAstId<Self::Source> {
                    self.ast_id
                }

                fn lookup(data: &ItemTree, index: Idx<Self>) -> &Self {
                    &data.$fld[index]
                }

                #[allow(clippy::allow_attributes, unreachable_patterns, reason = "macros should not leak lints")]
                fn id_from_mod_item(mod_item: ModuleItem) -> Option<ModuleItemId<Self>> {
                    match mod_item {
                        ModuleItem::$r#type(id) => Some(id),
                        _ => None,
                    }
                }

                fn id_to_mod_item(id: ModuleItemId<Self>) -> ModuleItem {
                    ModuleItem::$r#type(id)
                }
            }
        )+
        )*
    };
}

mod_items! {
    ImportStatement in imports -> ast::ImportStatement,
    Function in functions -> ast::FunctionDeclaration,
    Struct in structs -> ast::StructDeclaration,
    GlobalVariable in global_variables -> ast::VariableDeclaration,
    GlobalConstant in global_constants -> ast::ConstantDeclaration,
    Override in overrides -> ast::OverrideDeclaration,
    TypeAlias in type_aliases -> ast::TypeAliasDeclaration,
    GlobalAssertStatement in global_assert_statements -> ast::AssertStatement,
}

pub fn find_item<M: ItemTreeNode>(
    database: &dyn DefDatabase,
    file_id: HirFileId,
    source: &M::Source,
) -> Option<ModuleItemId<M>> {
    let item_tree = database.item_tree(file_id);
    item_tree.top_level_items().iter().find_map(|&item| {
        let id = M::id_from_mod_item(item)?;
        let data = M::lookup(&item_tree, id.index);
        let def_map = database.ast_id_map(file_id);

        let source_ast_id = def_map.try_ast_id(source)?;
        let item_ast_id = M::ast_id(data);

        (source_ast_id == item_ast_id).then_some(id)
    })
}
