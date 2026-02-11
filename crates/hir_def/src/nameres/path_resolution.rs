use base_db::EditionedFileId;
use vfs::FileId;

use crate::{
    database::{DefDatabase, ModuleDefinitionId},
    mod_path::{ModPath, PathKind},
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
    /// Keep this in sync with [`DefCollector::resolve_import_with_modules`]
    pub(crate) fn resolve_path(
        &self,
        mut file_id: FileId,
        path: &ModPath,
    ) -> Option<ResolvePathResult> {
        file_id = match path.kind() {
            PathKind::Plain => {
                // TODO:
                return None;
            },
            PathKind::Super(levels) => {
                // Parent modules are guaranteed to exist and be loaded all the way until the root.
                for _ in 0..levels {
                    file_id = self.modules[file_id].parent?;
                }
                file_id
            },
            PathKind::Package => self.crate_root(),
        };

        for (index, segment) in path.segments().iter().enumerate() {
            // Check in current module
            let module_data = &self.modules[file_id];
            if let Some(resolved_def) = module_data.scope.get(segment) {
                if index < path.segments().len() - 1 {
                    // Not at the last segment
                    return None;
                }
                return Some(ResolvePathResult {
                    resolved_def,
                    segment_index: Some(index),
                });
            }
            // Otherwise go to the child file
            file_id = *module_data.children.get(segment)?;
        }
        // We got to the end of the resolution
        Some(ResolvePathResult {
            resolved_def: ModuleDefinitionId::Module(EditionedFileId {
                file_id,
                edition: self.edition(),
            }),
            segment_index: None,
        })
    }
}
