use base_db::FileId;
use vfs::AnchoredPath;

use crate::database::DefDatabase;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HirFileId(pub(crate) HirFileIdRepr);

#[expect(
    clippy::enum_variant_names,
    reason = "Keep same as upstream to avoid churn"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum HirFileIdRepr {
    FileId(FileId),
}

impl From<FileId> for HirFileId {
    fn from(id: FileId) -> Self {
        Self(HirFileIdRepr::FileId(id))
    }
}

impl HirFileId {
    pub fn original_file(
        self,
        _database: &dyn DefDatabase,
    ) -> base_db::FileId {
        match self.0 {
            HirFileIdRepr::FileId(id) => id,
        }
    }
}

pub fn relative_file(
    database: &dyn DefDatabase,
    call_id: HirFileId,
    path_str: &str,
) -> Option<FileId> {
    let call_site = call_id.original_file(database);
    let path = AnchoredPath {
        anchor: call_site,
        path: path_str,
    };
    match database.resolve_path(path) {
        // Prevent including itself
        Some(result) if result != call_site => Some(result),
        // Possibly file not imported yet
        _ => None,
    }
}
