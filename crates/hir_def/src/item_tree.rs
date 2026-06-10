mod lower;

#[cfg(test)]
mod pretty;
#[cfg(test)]
mod tests;

use std::{hash, marker::PhantomData};

use base_db::EditionedFileId;
use rustc_hash::FxHashMap;
use smol_str::SmolStr;
use syntax::{
    AstNode, TokenText,
    ast::{self, StructDeclaration},
};
use triomphe::Arc;

use crate::{
    ast_id::FileAstId,
    database::{DefDatabase, ModuleDefinitionId},
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
}

impl ImportStatement {
    /// Expands the `UseTree` into individually imported `FlatImport`s.
    pub fn expand<Callback: FnMut(FlatImport)>(
        &self,
        mut callback: Callback,
    ) {
        self.tree
            .expand_impl(ModPath::from_kind(self.kind), &mut callback);
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
    fn expand_impl<Callback: FnMut(FlatImport)>(
        &self,
        mut prefix: ModPath,
        callback: &mut Callback,
    ) {
        match self {
            Self::Path { name, item } => {
                prefix.push_segment(name.clone());
                item.expand_impl(prefix, callback);
            },
            Self::Item { name, alias } => {
                prefix.push_segment(name.clone());
                callback(FlatImport {
                    path: prefix,
                    alias: alias.clone(),
                });
            },
            Self::Collection { list } => {
                for tree in list {
                    tree.expand_impl(prefix.clone(), callback);
                }
            },
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Directive;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Function {
    pub name: Name,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GlobalVariable {
    pub name: Name,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GlobalConstant {
    pub name: Name,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Override {
    pub name: Name,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TypeAlias {
    pub name: Name,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GlobalAssertStatement;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Struct {
    pub name: Name,
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct ItemTree {
    top_level: Vec<ModuleItemId>,

    big_data: FxHashMap<FileAstId<ast::Item>, BigModItem>,
    small_data: FxHashMap<FileAstId<ast::Item>, SmallModItem>,
}

impl ItemTree {
    pub fn query(
        database: &dyn DefDatabase,
        file_id: EditionedFileId,
    ) -> Arc<Self> {
        let source = file_id.parse(database).tree();

        let lower_ctx = lower::Ctx::new(database, file_id);
        let mut tree = lower_ctx.lower_source_file(&source);
        tree.shrink_to_fit();
        Arc::new(tree)
    }

    #[must_use]
    pub fn top_level_items(&self) -> &[ModuleItemId] {
        &self.top_level
    }

    pub fn structs(&self) -> impl Iterator<Item = FileAstId<StructDeclaration>> + '_ {
        self.top_level.iter().filter_map(|item| match item {
            ModuleItemId::Struct(r#struct) => Some(*r#struct),
            ModuleItemId::ImportStatement(_)
            | ModuleItemId::Function(_)
            | ModuleItemId::GlobalVariable(_)
            | ModuleItemId::GlobalConstant(_)
            | ModuleItemId::Override(_)
            | ModuleItemId::GlobalAssertStatement(_)
            | ModuleItemId::TypeAlias(_) => None,
        })
    }

    fn shrink_to_fit(&mut self) {
        let Self {
            top_level: _,
            big_data,
            small_data,
        } = self;
        big_data.shrink_to_fit();
        small_data.shrink_to_fit();
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum SmallModItem {
    Function(Function),
    GlobalVariable(GlobalVariable),
    GlobalConstant(GlobalConstant),
    Override(Override),
    TypeAlias(TypeAlias),
    Struct(Struct),
    Directive(Directive),
    GlobalAssertStatement(GlobalAssertStatement),
}

#[expect(
    clippy::enum_variant_names,
    reason = "Match Rust-Analyzer, we might get more big module items in the future"
)]
#[derive(Debug, Clone, Eq, PartialEq)]
enum BigModItem {
    ImportStatement(ImportStatement),
}

pub trait ItemTreeNode: Clone {
    type Source: AstNode + Into<ast::Item>;
}

#[expect(type_alias_bounds, reason = "TODO:")]
pub(crate) type ItemTreeAstId<T: ItemTreeNode> = FileAstId<T::Source>;

macro_rules! mod_items {
    ( $( $r#type:ident in $fld:ident -> $ast:ty ),+ $(,)? ) => {
        #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
        pub enum ModuleItemId {
            $($r#type(FileAstId<$ast>),)+
        }

        impl ModuleItemId {
            pub(crate) fn ast_id(self) -> FileAstId<ast::Item> {
                match self {
                    $(ModuleItemId::$r#type(it) => it.upcast()),+
                }
            }
        }

        $(impl From<FileAstId<$ast>> for ModuleItemId {
            fn from(id: FileAstId<$ast>) -> ModuleItemId {
                ModuleItemId::$r#type(id)
            }
        })+

        $(
            impl ItemTreeNode for $r#type {
                type Source = $ast;
            }

            impl std::ops::Index<FileAstId<$ast>> for ItemTree {
                type Output = $r#type;

                #[expect(unused_imports, reason = "Either a BigModItem or a SmallModItem will be used here.")]
                fn index(&self, index: FileAstId<$ast>) -> &Self::Output {
                    use BigModItem::*;
                    use SmallModItem::*;
                    match &self.$fld[&index.upcast()] {
                        $r#type(item) => item,
                        _ => panic!("expected item of type `{}` at index `{:?}`", stringify!($r#type), index),
                    }
                }
            }

        )+
    };
}

mod_items! {
    ImportStatement in big_data -> ast::ImportStatement,
    Function in small_data -> ast::FunctionDeclaration,
    Struct in small_data -> ast::StructDeclaration,
    GlobalVariable in small_data -> ast::VariableDeclaration,
    GlobalConstant in small_data -> ast::ConstantDeclaration,
    Override in small_data -> ast::OverrideDeclaration,
    TypeAlias in small_data -> ast::TypeAliasDeclaration,
    GlobalAssertStatement in small_data -> ast::AssertStatement,
}
