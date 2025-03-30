mod lower;
pub mod pretty;

use std::{marker::PhantomData, sync::Arc};

use la_arena::{Arena, Idx, IdxRange};
use smol_str::SmolStr;
use syntax::{AstNode, TokenText, ast};

use crate::{
    HirFileId,
    ast_id::FileAstId,
    db::{DefDatabase, Interned},
    type_ref::*,
};

const MISSING_NAME_PLACEHOLDER: &str = "[missing name]";

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Name(SmolStr);

impl Name {
    pub fn missing() -> Name {
        Name(MISSING_NAME_PLACEHOLDER.into())
    }

    pub fn is_missing(value: &str) -> bool {
        value == MISSING_NAME_PLACEHOLDER
    }

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
        Name(text.as_str().into())
    }
}

impl From<ast::Name> for Name {
    fn from(name: ast::Name) -> Self {
        Name(name.text().as_str().into())
    }
}

impl From<ast::NameReference> for Name {
    fn from(name: ast::NameReference) -> Self {
        Name(name.text().as_str().into())
    }
}

impl From<ast::Identifier> for Name {
    fn from(ident: ast::Identifier) -> Self {
        Name(ident.text().as_str().into())
    }
}

impl From<&'_ str> for Name {
    fn from(text: &str) -> Self {
        Name(text.into())
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Function {
    pub name: Name,
    pub parameters: IdxRange<Parameter>,
    pub return_type: Option<Interned<TypeReference>>,
    pub ast_id: FileAstId<ast::Function>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Parameter {
    pub ty: Interned<TypeReference>,
    pub name: Name,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GlobalVariable {
    pub name: Name,
    pub ty: Option<Interned<TypeReference>>,
    pub ast_id: FileAstId<ast::GlobalVariableDeclaration>,
    pub storage_class: Option<StorageClass>,
    pub access_mode: Option<AccessMode>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GlobalConstant {
    pub name: Name,
    pub ty: Option<Interned<TypeReference>>,
    pub ast_id: FileAstId<ast::GlobalConstantDeclaration>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Override {
    pub name: Name,
    pub ty: Option<Interned<TypeReference>>,
    pub ast_id: FileAstId<ast::OverrideDeclaration>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TypeAlias {
    pub name: Name,
    pub ty: Interned<TypeReference>,
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
    pub ty: Interned<TypeReference>,
    pub name: Name,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Directive;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Import {
    pub value: ImportValue,
    pub ast_id: FileAstId<ast::Import>,
}

// PERF: maybe intern string
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ImportValue {
    Path(String),
    Custom(String),
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct ModuleInfo {
    pub data: ModuleData,
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
    imports: Arena<Import>,
}

impl ModuleInfo {
    pub fn module_info_query(
        db: &dyn DefDatabase,
        file_id: HirFileId,
    ) -> Arc<ModuleInfo> {
        let source = match db.parse_or_resolve(file_id) {
            Ok(value) => value.tree(),
            Err(_) => return Arc::new(ModuleInfo::default()),
        };

        let mut lower_ctx = lower::Ctx::new(db, file_id);
        lower_ctx.lower_source_file(source);

        Arc::new(ModuleInfo {
            data: lower_ctx.module_data,
            items: lower_ctx.items,
        })
    }

    pub fn items(&self) -> &[ModuleItem] {
        &self.items
    }

    pub fn structs(&self) -> impl Iterator<Item = ModuleItemId<Struct>> + '_ {
        self.items.iter().filter_map(|item| match item {
            ModuleItem::Struct(r#struct) => Some(*r#struct),
            _ => None,
        })
    }

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
        ModuleItemId {
            index,
            _marker: PhantomData,
        }
    }
}

// If we automatically derive this trait, ModuleItemId<N> where N does not implement Hash cannot compile
#[allow(clippy::derived_hash_with_manual_eq)]
impl<N> std::hash::Hash for ModuleItemId<N> {
    fn hash<H: std::hash::Hasher>(
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
    fn id_from_mod_item(mod_item: &ModuleItem) -> Option<ModuleItemId<Self>>;

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

        $(impl std::ops::Index<la_arena::Idx<$r#type>> for ModuleData {
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

                fn id_from_mod_item(mod_item: &ModuleItem) -> Option<ModuleItemId<Self>> {
                    match mod_item {
                        ModuleItem::$r#type(id) => Some(*id),
                        #[allow(unreachable_patterns)]
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

impl std::ops::Index<Idx<Field>> for ModuleData {
    type Output = Field;

    fn index(
        &self,
        index: Idx<Field>,
    ) -> &Self::Output {
        &self.fields[index]
    }
}

impl std::ops::Index<Idx<Parameter>> for ModuleData {
    type Output = Parameter;

    fn index(
        &self,
        index: Idx<Parameter>,
    ) -> &Self::Output {
        &self.parameters[index]
    }
}

mod_items! {
    Function in functions -> ast::Function,
    Struct in structs -> ast::StructDeclaration,
    GlobalVariable in global_variables -> ast::GlobalVariableDeclaration,
    GlobalConstant in global_constants -> ast::GlobalConstantDeclaration,
    Override in overrides -> ast::OverrideDeclaration,
    Import in imports -> ast::Import,
    TypeAlias in type_aliases -> ast::TypeAliasDeclaration,
}

pub fn find_item<M: ModuleDataNode>(
    db: &dyn DefDatabase,
    file_id: HirFileId,
    source: &M::Source,
) -> Option<ModuleItemId<M>> {
    let module_info = db.module_info(file_id);
    module_info.items().iter().find_map(|item| {
        let id = M::id_from_mod_item(item)?;
        let data = M::lookup(&module_info.data, id.index);
        let def_map = db.ast_id_map(file_id);

        let source_ast_id = def_map.ast_id(source);
        let item_ast_id = M::ast_id(data);

        if source_ast_id == item_ast_id {
            Some(id)
        } else {
            None
        }
    })
}

// imports can be found not just in the items
pub fn find_import(
    db: &dyn DefDatabase,
    file_id: HirFileId,
    source: &syntax::ast::Import,
) -> Option<ModuleItemId<Import>> {
    let module_info = db.module_info(file_id);

    module_info.data.imports.iter().find_map(|(index, data)| {
        let id = ModuleItemId::from(index);
        let def_map = db.ast_id_map(file_id);

        let source_ast_id = def_map.ast_id(source);
        let item_ast_id = Import::ast_id(data);

        if source_ast_id == item_ast_id {
            Some(id)
        } else {
            None
        }
    })
}
