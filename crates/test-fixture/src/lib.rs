//! A set of high-level utility fixture methods to use in tests.
mod fixture;

use std::str::FromStr as _;

use base_db::{
    change::Change,
    input::{Dependency, PackageData, PackageId, PackageName, PackageOrigin},
    EditionedFileId, FileId, FilePosition, FileRange, FileSet, SourceDatabase, SourceRoot, VfsPath,
};
use edition::Edition;
use test_utils::{extract_range_or_offset, RangeOrOffset, CURSOR_MARKER, ESCAPED_CURSOR_MARKER};

pub use crate::fixture::{Fixture, FixtureWithProjectMeta};

type FxIndexMap<K, V> =
    indexmap::IndexMap<K, V, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>;
pub const WORKSPACE: base_db::SourceRootId = base_db::SourceRootId(0);

pub trait WithFixture: Default + SourceDatabase + 'static {
    #[must_use]
    #[track_caller]
    fn with_single_file(wa_fixture: &str) -> (Self, EditionedFileId) {
        let mut database = Self::default();
        let fixture = ChangeFixture::parse(wa_fixture);
        fixture.change.apply(&mut database);
        assert_eq!(
            fixture.files.len(),
            1,
            "Multiple files found in the fixture"
        );
        let file_id = EditionedFileId::from_file(&database, fixture.files[0]);
        (database, file_id)
    }

    #[must_use]
    #[track_caller]
    fn with_many_files(wa_fixture: &str) -> (Self, Vec<EditionedFileId>) {
        let mut database = Self::default();
        let fixture = ChangeFixture::parse(wa_fixture);
        fixture.change.apply(&mut database);
        assert!(fixture.file_position.is_none());
        let files = fixture
            .files
            .iter()
            .map(|file_id| EditionedFileId::from_file(&database, *file_id))
            .collect();
        (database, files)
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
    pub files: Vec<FileId>,
    pub manifest_files: Vec<FileId>,
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
        let mut manifest_files = Vec::new();
        let mut next_file_id = (0..).map(FileId::from_raw);
        let mut file_lines = Vec::new();
        let mut packages = FxIndexMap::default();
        let mut package_dependencies = Vec::new();

        let mut roots: Vec<(FileSet, PackageOrigin)> = Vec::new();

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
            assert!(
                meta.package_root.is_none() || meta.package.is_some(),
                "cannot specify package root without naming the package"
            );
            assert!(
                meta.dependencies.is_empty() || meta.package.is_some(),
                "cannot specify dependencies without naming the package"
            );

            let mut meta_package = meta.package.or(roots.is_empty().then(||
                    // Support tests that have a single file or a few files without setting up a package
                    ("wa_test_fixture".to_owned(), PackageOrigin::Local)));

            if let Some((package, origin)) = meta_package {
                let package_name = PackageName::normalize_dashes(&package);
                let root = VfsPath::new_virtual_path(meta.package_root.unwrap_or_default());

                let manifest_file_id = next_file_id.next().unwrap();
                manifest_files.push(manifest_file_id);
                source_change.change_file(manifest_file_id, Some(String::new()));

                let package = PackageData {
                    manifest_file_id,
                    root: package_root.clone(),
                    edition: meta.edition,
                    display_name: Some(package.clone()),
                    dependencies: Vec::new(),
                    origin,
                };
                let mut file_set = FileSet::default();
                file_set.insert(manifest_file_id, package_root.join("wesl.toml").unwrap());
                roots.push((file_set, origin));

                let package_id = PackageId::from_raw(u32::try_from(packages.len()).unwrap());
                let previous = packages.insert(package_name.clone(), (package_id, package));
                assert!(
                    previous.is_none(),
                    "multiple packages with same name: {package_name}"
                );
                for dep in meta.dependencies {
                    let dep = PackageName::normalize_dashes(&dep);
                    package_dependencies.push((package_name.clone(), dep));
                }
            }

            // We use raw file IDs here and then let the packages determine the editions.
            let file_id = next_file_id.next().unwrap();
            files.push(file_id);

            source_change.change_file(file_id, Some(text));

            assert!(meta.path.starts_with(SOURCE_ROOT_PREFIX));
            let path = VfsPath::new_virtual_path(meta.path);
            roots.last_mut().unwrap().0.insert(file_id, path);

            if let Some(range_or_offset) = range_or_offset {
                file_position = Some((file_id, range_or_offset));
            }
        }

        for (from, to) in package_dependencies {
            let (to_id, _) = packages[&to];
            let (_, from_data) = &mut packages[&from];
            from_data.dependencies.push(Dependency {
                name: to.clone(),
                package_id: to_id,
            });
        }

        for (package_id, package_data) in packages.into_values() {
            source_change.change_package(package_id, Some(package_data));
        }

        source_change.set_roots(
            roots
                .into_iter()
                .map(|(file_set, origin)| match origin {
                    PackageOrigin::Local => SourceRoot::new_local(file_set),
                    PackageOrigin::Library | PackageOrigin::Language => {
                        SourceRoot::new_library(file_set)
                    }
                })
                .collect(),
        );

        Self {
            file_position,
            file_lines,
            files,
            manifest_files,
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
    package_root: Option<String>,
    dependencies: Vec<String>,
    edition: Edition,
}

impl FileMeta {
    fn from_fixture(fixture: Fixture) -> Self {
        let dependencies = fixture.dependencies;

        Self {
            path: fixture.path,
            package: fixture
                .package
                .map(|package_name| parse_package(package_name, fixture.library)),
            package_root: fixture.package_root,
            dependencies,
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
    //   "my_awesome_package"
    //   "my_awesome_package"

    let origin = if explicit_non_workspace_member {
        PackageOrigin::Library
    } else {
        PackageOrigin::Local
    };

    (name, origin)
}
