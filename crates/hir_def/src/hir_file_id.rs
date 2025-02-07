use base_db::FileId;

use crate::db::ImportId;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ImportFile {
	pub import_id: ImportId,
}
