use base_db::FileId;
use vfs::AnchoredPath;

use crate::{
    db::{DefDatabase, ImportId},
    module_data::ImportValue,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HirFileId(pub(crate) HirFileIdRepr);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum HirFileIdRepr {
    FileId(FileId),
    MacroFile(ImportFile),
}

impl From<FileId> for HirFileId {
    fn from(id: FileId) -> Self {
        HirFileId(HirFileIdRepr::FileId(id))
    }
}

impl From<ImportFile> for HirFileId {
    fn from(id: ImportFile) -> Self {
        HirFileId(HirFileIdRepr::MacroFile(id))
    }
}

impl HirFileId {
    /// For import files, returns the file id of the file that needs to be imported
    /// or `None` if that file has not been opened yet
    pub fn original_file(
        self,
        db: &dyn DefDatabase,
    ) -> Option<FileId> {
        match self.0 {
            HirFileIdRepr::FileId(id) => Some(id),
            HirFileIdRepr::MacroFile(ImportFile { import_id }) => {
                let import_loc = db.lookup_intern_import(import_id);
                let module_info = db.module_info(import_loc.file_id);
                let import = module_info.get(import_loc.value);

                match &import.value {
                    ImportValue::Path(path) => relative_file(db, import_loc.file_id, path),
                    ImportValue::Custom(key) => {
                        // Try to resolve the custom import as a file
                        let imports = db.custom_imports();
                        if imports.contains_key(key) {
                            // For custom imports, we might not have a direct file,
                            // but return the source file that imported it for now
                            import_loc.file_id.original_file(db)
                        } else {
                            None
                        }
                    }
                }
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ImportFile {
    pub import_id: ImportId,
}

pub fn relative_file(
    db: &dyn DefDatabase,
    call_id: HirFileId,
    path_str: &str,
) -> Option<FileId> {
    let call_site = call_id.original_file(db)?;
    let path = AnchoredPath {
        anchor: call_site,
        path: path_str,
    };
    match db.resolve_path(path) {
        // Prevent including itself
        Some(res) if res != call_site => Some(res),
        // Possibly file not imported yet
        _ => None,
    }
}
