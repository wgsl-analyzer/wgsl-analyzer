use vfs::FileId;

use crate::{
    database::{DefDatabase, ModuleDefinitionId},
    mod_path::ModPath,
    nameres::DefMap,
};

#[derive(Debug, Clone)]
pub(super) struct ResolvePathResult {
    pub(super) resolved_def: ModuleDefinitionId,
    /// The index of the last resolved segment, or `None` if the full path has been resolved.
    pub(super) segment_index: Option<usize>,
}

impl DefMap {
    // Returns Yes if we are sure that additions to `ItemMap` wouldn't change
    // the result.
    pub(super) fn resolve_path_fp_with_macro(
        &self,
        db: &dyn DefDatabase,
        mut original_module: FileId,
        path: &ModPath,
    ) -> ResolvePathResult {
        todo!()
    }
}
