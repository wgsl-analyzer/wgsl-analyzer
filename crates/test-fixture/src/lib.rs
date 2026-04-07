//! A set of high-level utility fixture methods to use in tests.
mod fixture;

use std::{any::TypeId, mem, str::FromStr as _, sync};

pub use crate::fixture::{Fixture, FixtureWithProjectMeta};
use base_db::change::Change;
use base_db::input::{Dependency, PackageData, PackageId, PackageName, PackageOrigin};
use base_db::{
    FileId, FilePosition, FileRange, FileSet, RawEditionedFileId, SourceDatabase, SourceRoot,
    VfsPath,
};
use edition::Edition;
use test_utils::{CURSOR_MARKER, ESCAPED_CURSOR_MARKER, RangeOrOffset, extract_range_or_offset};
use triomphe::Arc;

type FxIndexMap<K, V> =
    indexmap::IndexMap<K, V, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>;
pub const WORKSPACE: base_db::SourceRootId = base_db::SourceRootId(0);

pub trait WithFixture: Default + SourceDatabase + 'static {
    #[must_use]
    #[track_caller]
    fn with_single_file(wa_fixture: &str) -> (Self, RawEditionedFileId) {
        let mut database = Self::default();
        let fixture = ChangeFixture::parse(wa_fixture);
        fixture.change.apply(&mut database);
        assert_eq!(
            fixture.files.len(),
            1,
            "Multiple files found in the fixture"
        );
        (database, fixture.files[0])
    }

    #[must_use]
    #[track_caller]
    fn with_many_files(wa_fixture: &str) -> (Self, Vec<RawEditionedFileId>) {
        let mut database = Self::default();
        let fixture = ChangeFixture::parse(wa_fixture);
        fixture.change.apply(&mut database);
        assert!(fixture.file_position.is_none());
        (database, fixture.files)
    }

    #[must_use]
    #[track_caller]
    fn with_files(wa_fixture: &str) -> Self {
        let mut database = Self::default();
        let fixture = ChangeFixture::parse(wa_fixture);
        fixture.change.apply(&mut database);
        assert!(fixture.file_position.is_none());
        database
    }

    #[must_use]
    #[track_caller]
    fn with_position(wa_fixture: &str) -> (Self, FilePosition) {
        let (database, file_id, range_or_offset) = Self::with_range_or_offset(wa_fixture);
        let offset = range_or_offset.expect_offset();
        (database, FilePosition { file_id, offset })
    }

    #[must_use]
    #[track_caller]
    fn with_range(wa_fixture: &str) -> (Self, FileRange) {
        let (database, file_id, range_or_offset) = Self::with_range_or_offset(wa_fixture);
        let range = range_or_offset.expect_range();
        (database, FileRange { file_id, range })
    }

    #[must_use]
    #[track_caller]
    fn with_range_or_offset(wa_fixture: &str) -> (Self, FileId, RangeOrOffset) {
        let mut database = Self::default();
        let fixture = ChangeFixture::parse(wa_fixture);
        fixture.change.apply(&mut database);

        let (file_id, range_or_offset) = fixture
            .file_position
            .expect("Could not find file position in fixture. Did you forget to add an `$0`?");
        (database, file_id, range_or_offset)
    }
}

impl<Database: SourceDatabase + Default + 'static> WithFixture for Database {}

pub struct ChangeFixture {
    pub file_position: Option<(FileId, RangeOrOffset)>,
    pub file_lines: Vec<usize>,
    pub files: Vec<RawEditionedFileId>,
    pub change: Change,
}

const SOURCE_ROOT_PREFIX: &str = "/";

impl ChangeFixture {
    /// # Panics
    /// Panics if an invalid fixture is passed to it. This function is used only in tests.
    #[expect(clippy::too_many_lines, reason = "keeping it similar to rust-analyzer")]
    #[must_use]
    pub fn parse(wa_fixture: &str) -> Self {
        let FixtureWithProjectMeta { fixture } = FixtureWithProjectMeta::parse(wa_fixture);
        let mut source_change = Change::default();

        let mut files = Vec::new();
        let mut file_lines = Vec::new();
        let mut packages = FxIndexMap::default();
        let mut package_dependencies = Vec::new();

        let mut file_id = FileId::from_raw(0);
        let mut roots: Vec<(FileSet, PackageData)> = Vec::new();

        let mut file_position = None;

        for entry in fixture {
            file_lines.push(entry.line);

            let mut range_or_offset = None;
            let text = if entry.text.contains(CURSOR_MARKER) {
                if entry.text.contains(ESCAPED_CURSOR_MARKER) {
                    entry.text.replace(ESCAPED_CURSOR_MARKER, CURSOR_MARKER)
                } else {
                    let (roo, text) = extract_range_or_offset(&entry.text);
                    assert!(file_position.is_none());
                    range_or_offset = Some(roo);
                    text
                }
            } else {
                entry.text.as_str().into()
            };

            let meta = FileMeta::from_fixture(entry);
            if let Some(range_or_offset) = range_or_offset {
                file_position = Some((file_id, range_or_offset));
            }

            assert!(meta.path.starts_with(SOURCE_ROOT_PREFIX));
            if !meta.deps.is_empty() {
                assert!(
                    meta.package.is_some(),
                    "can't specify deps without naming the crate"
                );
            }

            if let Some((krate, origin)) = meta.package {
                let crate_name = PackageName::normalize_dashes(&krate);
                let package = PackageData {
                    root_file_id: file_id,
                    edition: meta.edition,
                    display_name: Some(krate.clone()),
                    dependencies: Vec::new(),
                    cyclic_dependencies: Vec::new(),
                    origin,
                };
                let package_id = PackageId::from_raw(u32::try_from(roots.len()).unwrap());
                roots.push((FileSet::default(), package));

                let previous = packages.insert(crate_name.clone(), package_id);
                assert!(
                    previous.is_none(),
                    "multiple crates with same name: {crate_name}"
                );
                for dep in meta.deps {
                    let dep = PackageName::normalize_dashes(&dep);
                    package_dependencies.push((crate_name.clone(), dep));
                }
            }

            source_change.change_file(file_id, Some(text));
            let path = VfsPath::new_virtual_path(meta.path);
            if roots.is_empty() {
                // Support tests that have a single file or a few files without setting up a package
                let default_package = PackageData {
                    root_file_id: file_id,
                    edition: meta.edition,
                    display_name: Some("wa_test_fixture".into()),
                    dependencies: Vec::new(),
                    cyclic_dependencies: Vec::new(),
                    origin: PackageOrigin::Local,
                };
                roots.push((FileSet::default(), default_package));
            }
            roots.last_mut().unwrap().0.insert(file_id, path);
            files.push(RawEditionedFileId {
                file_id,
                edition: meta.edition,
            });
            file_id = FileId::from_raw(file_id.index() + 1);
        }

        for (from, to) in package_dependencies {
            let from_id = packages[&from];
            let to_id = packages[&to];
            roots[from_id.to_raw_usize()]
                .1
                .dependencies
                .push(Dependency {
                    name: to.clone(),
                    package_id: to_id,
                });
        }

        source_change.set_roots(
            roots
                .into_iter()
                .map(|(file_set, package)| match package.origin {
                    PackageOrigin::Local => SourceRoot::new_local(file_set),
                    PackageOrigin::Library | PackageOrigin::Language => {
                        SourceRoot::new_library(file_set)
                    },
                })
                .collect(),
        );

        Self {
            file_position,
            file_lines,
            files,
            change: source_change,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum SourceRootKind {
    Local,
    Library,
}

#[derive(Debug)]
struct FileMeta {
    path: String,
    package: Option<(String, PackageOrigin)>,
    deps: Vec<String>,
    edition: Edition,
}

impl FileMeta {
    fn from_fixture(fixture: Fixture) -> Self {
        let deps = fixture.deps;

        Self {
            path: fixture.path,
            package: fixture
                .package
                .map(|package_name| parse_package(package_name, fixture.library)),
            deps,
            edition: fixture.edition.map_or(Edition::CURRENT, |version| {
                Edition::from_str(&version).unwrap()
            }),
        }
    }
}

const fn parse_package(
    name: String,
    explicit_non_workspace_member: bool,
) -> (String, PackageOrigin) {
    // syntax:
    //   "my_awesome_crate"

    let origin = if explicit_non_workspace_member {
        PackageOrigin::Library
    } else {
        PackageOrigin::Local
    };

    (name, origin)
}
