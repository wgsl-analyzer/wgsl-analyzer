use vfs::FileId;

use crate::{
    database::{DefDatabase, ModuleDefinitionId},
    mod_path::ModPath,
    nameres::DefMap,
};

#[derive(Debug, Clone)]
pub(crate) struct ResolvePathResult {
    pub resolved_def: ModuleDefinitionId,
    /// The index of the last resolved segment, or `None` if the full path has been resolved.
    /// TODO: I don't think that I need this
    pub segment_index: Option<usize>,
}

impl DefMap {
    pub(crate) fn resolve_path(
        &self,
        db: &dyn DefDatabase,
        mut original_module: FileId,
        path: &ModPath,
    ) -> ResolvePathResult {
        // Look at the code in fn resolve_path_fp_with_macro_single
        // Which ends up calling resolve_name_in_module
        todo!()
    }
}
