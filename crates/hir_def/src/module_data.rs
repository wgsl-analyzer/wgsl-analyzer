mod lower;
pub mod pretty;

use std::{hash, marker::PhantomData, ops, sync::Arc};

use la_arena::{Arena, Idx, IdxRange};
use smol_str::SmolStr;
use syntax::{AstNode, TokenText, ast};

use crate::{
    HirFileId,
    ast_id::FileAstId,
    database::{DefDatabase, Interned},
    type_ref::{AccessMode, AddressSpace, TypeReference},
};

const MISSING_NAME_PLACEHOLDER: &str = "[missing name]";

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
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

impl From<ast::NameReference> for Name {
    fn from(name: ast::NameReference) -> Self {
        Self(name.text().as_str().into())
    }
}

impl From<&'_ str> for Name {
    fn from(text: &str) -> Self {
        Self(text.into())
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Function {
    pub name: Name,
    pub parameters: IdxRange<Parameter>,
    pub return_type: Option<TypeReference>,
    pub ast_id: FileAstId<ast::FunctionDeclaration>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Parameter {
    pub r#type: TypeReference,
    pub name: Name,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GlobalVariable {
    pub name: Name,
    pub r#type: Option<TypeReference>,
    pub ast_id: FileAstId<ast::VariableDeclaration>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GlobalConstant {
    pub name: Name,
    pub r#type: Option<TypeReference>,
    pub ast_id: FileAstId<ast::ConstantDeclaration>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Override {
    pub name: Name,
    pub r#type: Option<TypeReference>,
    pub ast_id: FileAstId<ast::OverrideDeclaration>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TypeAlias {
    pub name: Name,
    pub r#type: TypeReference,
    pub ast_id: FileAstId<ast::TypeAliasDeclaration>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Struct {
    pub name: Name,
    pub ast_id: FileAstId<ast::StructDeclaration>,
    pub fields: IdxRange<Field>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Field {
    pub r#type: TypeReference,
    pub name: Name,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Directive;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct ModuleInfo {
    pub(crate) data: ModuleData,
    items: Vec<ModuleItem>,
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct ModuleData {
    functions: Arena<Function>,
    parameters: Arena<Parameter>,
    global_variables: Arena<GlobalVariable>,
    global_constants: Arena<GlobalConstant>,
    overrides: Arena<Override>,
    type_aliases: Arena<TypeAlias>,
    structs: Arena<Struct>,
    pub(crate) fields: Arena<Field>,
    directives: Arena<Directive>,
}

impl ModuleInfo {
    pub fn module_info_query(
        database: &dyn DefDatabase,
        file_id: HirFileId,
    ) -> Arc<Self> {
        let source = match database.parse_or_resolve(file_id) {
            Ok(value) => value.tree(),
            Err(()) => return Arc::new(Self::default()),
        };

        let mut lower_ctx = lower::Ctx::new(database, file_id);
        lower_ctx.lower_source_file(&source);

        Arc::new(Self {
            data: lower_ctx.module_data,
            items: lower_ctx.items,
        })
    }

    #[must_use]
    pub fn items(&self) -> &[ModuleItem] {
        &self.items
    }

    pub fn structs(&self) -> impl Iterator<Item = ModuleItemId<Struct>> + '_ {
        self.items.iter().filter_map(|item| match item {
            ModuleItem::Struct(r#struct) => Some(*r#struct),
            ModuleItem::Function(_)
            | ModuleItem::GlobalVariable(_)
            | ModuleItem::GlobalConstant(_)
            | ModuleItem::Override(_)
            | ModuleItem::TypeAlias(_) => None,
        })
    }

    #[must_use]
    pub fn get<M: ModuleDataNode>(
        &self,
        id: ModuleItemId<M>,
    ) -> &M {
        M::lookup(&self.data, id.index)
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

impl<N: ModuleDataNode> Copy for ModuleItemId<N> {}

pub trait ModuleDataNode: Clone {
    type Source: AstNode + Into<ast::Item>;

    fn ast_id(&self) -> FileAstId<Self::Source>;

    /// Looks up an instance of `Self` in an item tree.
    fn lookup(
        data: &ModuleData,
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

        $(impl core::ops::Index<la_arena::Idx<$r#type>> for ModuleData {
            type Output = $r#type;

            fn index(&self, index: la_arena::Idx<$r#type>) -> &Self::Output {
                &self.$fld[index]
            }
        })*

        $(
        $(impl ModuleDataNode for $r#type {
                type Source = $ast;

                fn ast_id(&self) -> FileAstId<Self::Source> {
                    self.ast_id
                }

                fn lookup(data: &ModuleData, index: Idx<Self>) -> &Self {
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
        )*
        )?
    };
}

impl ops::Index<Idx<Field>> for ModuleData {
    type Output = Field;

    fn index(
        &self,
        index: Idx<Field>,
    ) -> &Self::Output {
        &self.fields[index]
    }
}

impl ops::Index<Idx<Parameter>> for ModuleData {
    type Output = Parameter;

    fn index(
        &self,
        index: Idx<Parameter>,
    ) -> &Self::Output {
        &self.parameters[index]
    }
}

mod_items! {
    Function in functions -> ast::FunctionDeclaration,
    Struct in structs -> ast::StructDeclaration,
    GlobalVariable in global_variables -> ast::VariableDeclaration,
    GlobalConstant in global_constants -> ast::ConstantDeclaration,
    Override in overrides -> ast::OverrideDeclaration,
    TypeAlias in type_aliases -> ast::TypeAliasDeclaration,
}

pub fn find_item<M: ModuleDataNode>(
    database: &dyn DefDatabase,
    file_id: HirFileId,
    source: &M::Source,
) -> Option<ModuleItemId<M>> {
    let module_info = database.module_info(file_id);
    module_info.items().iter().find_map(|&item| {
        let id = M::id_from_mod_item(item)?;
        let data = M::lookup(&module_info.data, id.index);
        let def_map = database.ast_id_map(file_id);

        let source_ast_id = def_map.ast_id(source);
        let item_ast_id = M::ast_id(data);

        (source_ast_id == item_ast_id).then_some(id)
    })
}
