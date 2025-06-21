#![expect(clippy::empty_structs_with_brackets, reason = "salsa leaks a lint")]

use std::{fmt::Debug, marker::PhantomData, sync::Arc};

use base_db::{FileId, SourceDatabase, TextRange, TextSize};
use salsa::InternKey;
use syntax::{
    AstNode as _, Parse,
    ast::{self, Item},
};
use vfs::VfsPath;

use crate::{
    HirFileId, InFile,
    ast_id::AstIdMap,
    attributes::{Attribute, AttributeDefId, AttributesWithOwner},
    body::{Body, BodySourceMap, scope::ExprScopes},
    data::{
        FunctionData, GlobalConstantData, GlobalVariableData, OverrideData, StructData,
        TypeAliasData,
    },
    hir_file_id::{HirFileIdRepr, ImportFile, relative_file},
    module_data::{
        Function, GlobalConstant, GlobalVariable, Import, ModuleInfo, ModuleItemId, Override,
        Struct, TypeAlias,
    },
    resolver::Resolver,
    type_ref::TypeReference,
};

#[salsa::query_group(DefDatabaseStorage)]
pub trait DefDatabase: InternDatabase + SourceDatabase {
    fn parse_or_resolve(
        &self,
        key: HirFileId,
    ) -> Result<Parse, ()>;

    fn get_path(
        &self,
        key: HirFileId,
    ) -> Result<VfsPath, ()>;

    fn get_file_id(
        &self,
        key: VfsPath,
    ) -> Result<FileId, ()>;

    fn ast_id_map(
        &self,
        key: HirFileId,
    ) -> Arc<AstIdMap>;

    fn resolve_full_source(
        &self,
        key: HirFileId,
    ) -> Result<String, ()>;

    fn text_range_from_full(
        &self,
        key: HirFileId,
        range: TextRange,
    ) -> Result<TextRange, ()>;

    #[salsa::invoke(ModuleInfo::module_info_query)]
    fn module_info(
        &self,
        key: HirFileId,
    ) -> Arc<ModuleInfo>;

    #[salsa::invoke(Body::body_with_source_map_query)]
    fn body_with_source_map(
        &self,
        key: DefinitionWithBodyId,
    ) -> (Arc<Body>, Arc<BodySourceMap>);

    #[salsa::invoke(Body::body_query)]
    fn body(
        &self,
        key: DefinitionWithBodyId,
    ) -> Arc<Body>;

    #[salsa::invoke(ExprScopes::expression_scopes_query)]
    fn expression_scopes(
        &self,
        key: DefinitionWithBodyId,
    ) -> Arc<ExprScopes>;

    #[salsa::invoke(FunctionData::fn_data_query)]
    fn fn_data(
        &self,
        key: FunctionId,
    ) -> Arc<FunctionData>;

    #[salsa::invoke(StructData::struct_data_query)]
    fn struct_data(
        &self,
        key: StructId,
    ) -> Arc<StructData>;

    #[salsa::invoke(TypeAliasData::type_alias_data_query)]
    fn type_alias_data(
        &self,
        key: TypeAliasId,
    ) -> Arc<TypeAliasData>;

    #[salsa::invoke(GlobalVariableData::global_var_data_query)]
    fn global_var_data(
        &self,
        key: GlobalVariableId,
    ) -> Arc<GlobalVariableData>;

    #[salsa::invoke(GlobalConstantData::global_constant_data_query)]
    fn global_constant_data(
        &self,
        key: GlobalConstantId,
    ) -> Arc<GlobalConstantData>;

    #[salsa::invoke(OverrideData::override_data_query)]
    fn override_data(
        &self,
        key: OverrideId,
    ) -> Arc<OverrideData>;

    #[salsa::invoke(AttributesWithOwner::attrs_query)]
    fn attrs(
        &self,
        key: AttributeDefId,
    ) -> Arc<AttributesWithOwner>;
}

fn get_path(
    database: &dyn DefDatabase,
    file_id: HirFileId,
) -> Result<VfsPath, ()> {
    match file_id.0 {
        HirFileIdRepr::FileId(file_id) => Ok(database.file_path(file_id)),
        HirFileIdRepr::MacroFile(_) => Err(()),
    }
}

#[expect(clippy::unnecessary_wraps, reason = "Needed for salsa")]
fn get_file_id(
    database: &dyn DefDatabase,
    path: VfsPath,
) -> Result<FileId, ()> {
    Ok(database.file_id(path))
}

fn parse_or_resolve(
    database: &dyn DefDatabase,
    file_id: HirFileId,
) -> Result<Parse, ()> {
    match file_id.0 {
        HirFileIdRepr::FileId(file_id) => Ok(database.parse(file_id)),
        HirFileIdRepr::MacroFile(import_file) => {
            let import_loc = database.lookup_intern_import(import_file.import_id);
            let module_info = database.module_info(import_loc.file_id);
            let import: &Import = module_info.get(import_loc.value);

            match &import.value {
                crate::module_data::ImportValue::Path(path) => {
                    let file_id = relative_file(database, import_loc.file_id, path).ok_or(())?;
                    Ok(database.parse(file_id))
                },
                crate::module_data::ImportValue::Custom(key) => {
                    database.parse_import(key.clone(), syntax::ParseEntryPoint::File)
                },
            }
        },
    }
}

fn resolve_full_source(
    database: &dyn DefDatabase,
    file_id: HirFileId,
) -> Result<String, ()> {
    let parse = database.parse_or_resolve(file_id)?;

    let root = ast::SourceFile::cast(parse.syntax().clone_for_update()).unwrap();

    let imports: Vec<_> = root
        .items()
        .filter_map(|item| match item {
            Item::Import(import) => Some(import),
            Item::Function(_)
            | Item::StructDeclaration(_)
            | Item::GlobalVariableDeclaration(_)
            | Item::GlobalConstantDeclaration(_)
            | Item::OverrideDeclaration(_)
            | Item::TypeAliasDeclaration(_) => None,
        })
        .filter_map(|import| {
            let import_mod_id = crate::module_data::find_item(database, file_id, &import)?;
            let import_id = database.intern_import(Location::new(file_id, import_mod_id));
            let import_file = HirFileId::from(ImportFile { import_id });

            Some((import.syntax().clone(), import_file))
        })
        .collect();

    for (import, import_file) in imports.into_iter().rev() {
        let import_source = match database.parse_or_resolve(import_file) {
            Ok(parse) => parse.syntax().clone_for_update(),
            Err(()) => continue,
        };

        let import_whitespace = import
            .last_token()
            .filter(|token| token.kind().is_whitespace());
        let to_insert = match import_whitespace {
            Some(whitespace) => vec![import_source.into(), whitespace.into()],
            None => vec![import_source.into()],
        };

        let index = import.index();
        #[expect(
            clippy::range_plus_one,
            reason = "rowan does not support generic ranges"
        )]
        import
            .parent()
            .unwrap()
            .splice_children(index..index + 1, to_insert);
    }

    Ok(root.syntax().to_string())
}

fn text_range_from_full(
    database: &dyn DefDatabase,
    file_id: HirFileId,
    mut range: TextRange,
) -> Result<TextRange, ()> {
    let root = database.parse_or_resolve(file_id)?.tree();

    let imports = root
        .items()
        .filter_map(|item| match item {
            Item::Import(import) => Some(import),
            Item::Function(_)
            | Item::StructDeclaration(_)
            | Item::GlobalVariableDeclaration(_)
            | Item::GlobalConstantDeclaration(_)
            | Item::OverrideDeclaration(_)
            | Item::TypeAliasDeclaration(_) => None,
        })
        .filter_map(|import| {
            let import_mod_id = crate::module_data::find_item(database, file_id, &import)?;
            let import_id = database.intern_import(Location::new(file_id, import_mod_id));
            let import_file = HirFileId::from(ImportFile { import_id });

            Some((import.syntax().clone(), import_file))
        });

    for (import, import_file) in imports {
        if import.text_range().start() > range.end() {
            break;
        }

        let import_length = match database.parse_or_resolve(import_file) {
            Ok(parse) => parse.syntax().text().len(),
            Err(()) => continue,
        };

        let import_whitespace = import
            .last_token()
            .filter(|token| token.kind().is_whitespace())
            .map_or(0, |ws| ws.text().len());

        let to_remove = import_length + TextSize::from(u32::try_from(import_whitespace).unwrap());

        if let Some(new_range) = range.checked_sub(to_remove) {
            range = new_range + import.syntax().text().len();
        } else {
            // original range is inside the import
            range = import.syntax().text_range();
            break;
        }
    }

    Ok(range)
}

fn ast_id_map(
    database: &dyn DefDatabase,
    file_id: HirFileId,
) -> Arc<AstIdMap> {
    let map = database
        .parse_or_resolve(file_id)
        .map(|source| AstIdMap::from_source(&source.tree()))
        .unwrap_or_default();
    Arc::new(map)
}

#[salsa::query_group(InternDatabaseStorage)]
pub trait InternDatabase: SourceDatabase {
    #[salsa::interned]
    fn intern_type_ref(
        &self,
        type_reference: TypeReference,
    ) -> Interned<TypeReference>;
    #[salsa::interned]
    fn intern_attribute(
        &self,
        attribute: Attribute,
    ) -> Interned<Attribute>;

    #[salsa::interned]
    fn intern_function(
        &self,
        loc: Location<Function>,
    ) -> FunctionId;
    #[salsa::interned]
    fn intern_global_variable(
        &self,
        location: Location<GlobalVariable>,
    ) -> GlobalVariableId;
    #[salsa::interned]
    fn intern_global_constant(
        &self,
        loc: Location<GlobalConstant>,
    ) -> GlobalConstantId;
    #[salsa::interned]
    fn intern_override(
        &self,
        loc: Location<Override>,
    ) -> OverrideId;
    #[salsa::interned]
    fn intern_struct(
        &self,
        loc: Location<Struct>,
    ) -> StructId;
    #[salsa::interned]
    fn intern_import(
        &self,
        loc: Location<Import>,
    ) -> ImportId;
    #[salsa::interned]
    fn intern_type_alias(
        &self,
        loc: Location<TypeAlias>,
    ) -> TypeAliasId;
}

pub type Location<T> = InFile<ModuleItemId<T>>;

pub struct Interned<T>(salsa::InternId, PhantomData<T>);

impl<T> std::hash::Hash for Interned<T> {
    fn hash<H: std::hash::Hasher>(
        &self,
        state: &mut H,
    ) {
        self.0.hash(state);
    }
}

impl<T> PartialEq for Interned<T> {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for Interned<T> {}

impl<T> Clone for Interned<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Interned<T> {}

impl<T> std::fmt::Debug for Interned<T> {
    fn fmt(
        &self,
        #[expect(clippy::min_ident_chars, reason = "trait impl")] f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        f.debug_tuple("Interned").field(&self.0).finish()
    }
}

impl<T> InternKey for Interned<T> {
    fn from_intern_id(
        #[expect(clippy::min_ident_chars, reason = "trait impl")] v: salsa::InternId
    ) -> Self {
        Self(v, PhantomData)
    }

    fn as_intern_id(&self) -> salsa::InternId {
        self.0
    }
}

macro_rules! intern_id {
    ($id:ident, $loc:ty, $lookup:ident) => {
        #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
        pub struct $id(salsa::InternId);
        impl InternKey for $id {
            fn from_intern_id(
                #[expect(clippy::min_ident_chars, reason = "trait impl")] v: salsa::InternId
            ) -> Self {
                $id(v)
            }

            fn as_intern_id(&self) -> salsa::InternId {
                self.0
            }
        }

        impl Lookup for $id {
            type Data = $loc;

            fn lookup(
                &self,
                database: &dyn DefDatabase,
            ) -> $loc {
                database.$lookup(*self)
            }
        }
    };
}

pub trait Lookup: Sized {
    type Data;
    fn lookup(
        &self,
        database: &dyn DefDatabase,
    ) -> Self::Data;
}

intern_id!(FunctionId, Location<Function>, lookup_intern_function);
intern_id!(
    GlobalVariableId,
    Location<GlobalVariable>,
    lookup_intern_global_variable
);
intern_id!(
    GlobalConstantId,
    Location<GlobalConstant>,
    lookup_intern_global_constant
);
intern_id!(OverrideId, Location<Override>, lookup_intern_override);
intern_id!(StructId, Location<Struct>, lookup_intern_struct);
intern_id!(ImportId, Location<Import>, lookup_intern_import);
intern_id!(TypeAliasId, Location<TypeAlias>, lookup_intern_type_alias);

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum DefinitionWithBodyId {
    Function(FunctionId),
    GlobalVariable(GlobalVariableId),
    GlobalConstant(GlobalConstantId),
    Override(OverrideId),
}

impl DefinitionWithBodyId {
    pub fn file_id(
        &self,
        database: &dyn DefDatabase,
    ) -> HirFileId {
        match self {
            Self::Function(id) => id.lookup(database).file_id,
            Self::GlobalVariable(id) => id.lookup(database).file_id,
            Self::GlobalConstant(id) => id.lookup(database).file_id,
            Self::Override(id) => id.lookup(database).file_id,
        }
    }

    pub fn resolver(
        &self,
        database: &dyn DefDatabase,
    ) -> Resolver {
        let file_id = self.file_id(database);
        let module_info = database.module_info(file_id);
        Resolver::default().push_module_scope(database, file_id, module_info)
    }
}
