use base_db::change::Change;
use ide_db::RootDatabase;
use test_fixture::ChangeFixture;
use triomphe::Arc;
use vfs::VfsPath;

use crate::{Analysis, AnalysisHost, FileId, FilePosition, FileRange};

/// Creates analysis for a single file.
pub(crate) fn single_file_db(source: &str) -> (Analysis, FileId) {
    let mut host = AnalysisHost::default();
    let change_fixture = ChangeFixture::parse(source);
    host.apply_change(change_fixture.change);

    (host.analysis(), change_fixture.files[0].file_id)
}
