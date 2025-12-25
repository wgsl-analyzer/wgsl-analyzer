use base_db::change::Change;
use ide_db::RootDatabase;
use triomphe::Arc;
use vfs::VfsPath;

use crate::{Analysis, AnalysisHost, FileId, FilePosition, FileRange};

/// Creates analysis for a single file.
pub(crate) fn single_file_db(source: &str) -> (Analysis, FileId) {
    let mut host = AnalysisHost::default();
    let mut change = Change::new();
    let file_id = FileId::from_raw(0);
    change.change_file(
        file_id,
        Some(Arc::new(source.to_owned())),
        VfsPath::new_virtual_path("/".into()),
    );
    host.apply_change(change);

    (host.analysis(), file_id)
}
