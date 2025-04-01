use std::{fmt::Debug, marker::PhantomData, sync::Arc};

use base_db::{FileId, SourceDatabase, TextRange, TextSize, Upcast};
use salsa::InternKey;
use syntax::{
    AstNode, Parse,
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
pub trait DefDatabase: InternDatabase + Upcast<dyn SourceDatabase> {
    fn parse_or_resolve(
        &self,
        file_id: HirFileId,
    ) -> Result<Parse, ()>;

    fn get_path(
        &self,
        file_id: HirFileId,
    ) -> Result<VfsPath, ()>;

    fn get_file_id(
        &self,
        path: VfsPath,
    ) -> Result<FileId, ()>;

    fn ast_id_map(
        &self,
        file_id: HirFileId,
    ) -> Arc<AstIdMap>;

    fn resolve_full_source(
        &self,
        file_id: HirFileId,
    ) -> Result<String, ()>;

    fn text_range_from_full(
        &self,
        file_id: HirFileId,
        range: TextRange,
    ) -> Result<TextRange, ()>;

    #[salsa::invoke(ModuleInfo::module_info_query)]
    fn module_info(
        &self,
        file_id: HirFileId,
    ) -> Arc<ModuleInfo>;

    #[salsa::invoke(Body::body_with_source_map_query)]
    fn body_with_source_map(
        &self,
        def: DefinitionWithBodyId,
    ) -> (Arc<Body>, Arc<BodySourceMap>);

    #[salsa::invoke(Body::body_query)]
    fn body(
        &self,
        def: DefinitionWithBodyId,
    ) -> Arc<Body>;

    #[salsa::invoke(ExprScopes::expression_scopes_query)]
    fn expression_scopes(
        &self,
        def: DefinitionWithBodyId,
    ) -> Arc<ExprScopes>;

    #[salsa::invoke(FunctionData::fn_data_query)]
    fn fn_data(
        &self,
        def: FunctionId,
    ) -> Arc<FunctionData>;

    #[salsa::invoke(StructData::struct_data_query)]
    fn struct_data(
        &self,
        r#struct: StructId,
    ) -> Arc<StructData>;

    #[salsa::invoke(TypeAliasData::type_alias_data_query)]
    fn type_alias_data(
        &self,
        type_alias: TypeAliasId,
    ) -> Arc<TypeAliasData>;

    #[salsa::invoke(GlobalVariableData::global_var_data_query)]
    fn global_var_data(
        &self,
        def: GlobalVariableId,
    ) -> Arc<GlobalVariableData>;

    #[salsa::invoke(GlobalConstantData::global_constant_data_query)]
    fn global_constant_data(
        &self,
        def: GlobalConstantId,
    ) -> Arc<GlobalConstantData>;

    #[salsa::invoke(OverrideData::override_data_query)]
    fn override_data(
        &self,
        def: OverrideId,
    ) -> Arc<OverrideData>;

    #[salsa::invoke(AttributesWithOwner::attrs_query)]
    fn attrs(
        &self,
        def: AttributeDefId,
    ) -> Arc<AttributesWithOwner>;
}

fn get_path(
    db: &dyn DefDatabase,
    file_id: HirFileId,
) -> Result<VfsPath, ()> {
    match file_id.0 {
        HirFileIdRepr::FileId(file_id) => Ok(db.file_path(file_id)),
        _ => Err(()),
    }
}

fn get_file_id(
    db: &dyn DefDatabase,
    path: VfsPath,
) -> Result<FileId, ()> {
    Ok(db.file_id(path))
}

fn parse_or_resolve(
    db: &dyn DefDatabase,
    file_id: HirFileId,
) -> Result<Parse, ()> {
    match file_id.0 {
        HirFileIdRepr::FileId(file_id) => Ok(db.parse(file_id)),
        HirFileIdRepr::MacroFile(import_file) => {
            let import_loc = db.lookup_intern_import(import_file.import_id);
            let module_info = db.module_info(import_loc.file_id);
            let import: &Import = module_info.get(import_loc.value);

            match &import.value {
                crate::module_data::ImportValue::Path(path) => {
                    let file_id = relative_file(db, import_loc.file_id, path).ok_or(())?;
                    Ok(db.parse(file_id))
                },
                crate::module_data::ImportValue::Custom(key) => {
                    db.parse_import(key.clone(), syntax::ParseEntryPoint::File)
                },
            }
        },
    }
}

#[allow(clippy::needless_collect)] // false positive
fn resolve_full_source(
    db: &dyn DefDatabase,
    file_id: HirFileId,
) -> Result<String, ()> {
    let parse = db.parse_or_resolve(file_id)?;

    let root = ast::SourceFile::cast(parse.syntax().clone_for_update()).unwrap();

    let imports: Vec<_> = root
        .items()
        .filter_map(|item| match item {
            Item::Import(import) => Some(import),
            _ => None,
        })
        .filter_map(|import| {
            let import_mod_id = crate::module_data::find_item(db, file_id, &import)?;
            let import_id = db.intern_import(Location::new(file_id, import_mod_id));
            let import_file = HirFileId::from(ImportFile { import_id });

            Some((import.syntax().clone(), import_file))
        })
        .collect();

    for (import, import_file) in imports.into_iter().rev() {
        let import_source = match db.parse_or_resolve(import_file) {
            Ok(it) => it.syntax().clone_for_update(),
            Err(_) => continue,
        };

        let import_whitespace = import
            .last_token()
            .filter(|token| token.kind().is_whitespace());
        let to_insert = match import_whitespace {
            Some(whitespace) => vec![import_source.into(), whitespace.into()],
            None => vec![import_source.into()],
        };

        let index = import.index();
        import
            .parent()
            .unwrap()
            .splice_children(index..index + 1, to_insert);
    }

    Ok(root.syntax().to_string())
}

fn text_range_from_full(
    db: &dyn DefDatabase,
    file_id: HirFileId,
    mut range: TextRange,
) -> Result<TextRange, ()> {
    let root = db.parse_or_resolve(file_id)?.tree();

    let imports = root
        .items()
        .filter_map(|item| match item {
            Item::Import(import) => Some(import),
            _ => None,
        })
        .filter_map(|import| {
            let import_mod_id = crate::module_data::find_item(db, file_id, &import)?;
            let import_id = db.intern_import(Location::new(file_id, import_mod_id));
            let import_file = HirFileId::from(ImportFile { import_id });

            Some((import.syntax().clone(), import_file))
        });

    for (import, import_file) in imports {
        if import.text_range().start() > range.end() {
            break;
        }

        let import_length = match db.parse_or_resolve(import_file) {
            Ok(it) => it.syntax().text().len(),
            Err(_) => continue,
        };

        let import_whitespace = import
            .last_token()
            .filter(|token| token.kind().is_whitespace())
            .map_or(0, |ws| ws.text().len());

        let to_remove = import_length + TextSize::from(import_whitespace as u32);

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
    db: &dyn DefDatabase,
    file_id: HirFileId,
) -> Arc<AstIdMap> {
    let map = db
        .parse_or_resolve(file_id)
        .map(|source| AstIdMap::from_source(source.tree()))
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
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        f.debug_tuple("Interned").field(&self.0).finish()
    }
}

impl<T> InternKey for Interned<T> {
    fn from_intern_id(v: salsa::InternId) -> Self {
        Interned(v, PhantomData)
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
            fn from_intern_id(v: salsa::InternId) -> Self {
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
                db: &dyn DefDatabase,
            ) -> $loc {
                db.$lookup(*self)
            }
        }
    };
}

pub trait Lookup: Sized {
    type Data;
    fn lookup(
        &self,
        db: &dyn DefDatabase,
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
        db: &dyn DefDatabase,
    ) -> HirFileId {
        match self {
            DefinitionWithBodyId::Function(id) => id.lookup(db).file_id,
            DefinitionWithBodyId::GlobalVariable(id) => id.lookup(db).file_id,
            DefinitionWithBodyId::GlobalConstant(id) => id.lookup(db).file_id,
            DefinitionWithBodyId::Override(id) => id.lookup(db).file_id,
        }
    }

    pub fn resolver(
        &self,
        db: &dyn DefDatabase,
    ) -> Resolver {
        let file_id = self.file_id(db);
        let module_info = db.module_info(file_id);
        Resolver::default().push_module_scope(db, file_id, module_info)
    }
}
