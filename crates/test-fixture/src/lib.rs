//! A set of high-level utility fixture methods to use in tests.
mod fixture;

use std::str::FromStr as _;

use base_db::{
    EditionedFileId, FileId, FilePosition, FileRange, FileSet, SourceDatabase, SourceRoot, VfsPath,
    change::Change,
    input::{Dependency, PackageData, PackageId, PackageName, PackageOrigin},
};
use edition::Edition;
use test_utils::{CURSOR_MARKER, ESCAPED_CURSOR_MARKER, RangeOrOffset, extract_range_or_offset};
use wgsl_std::StdLibrary;

pub use crate::fixture::{Fixture, FixtureWithProjectMeta};

type FxIndexMap<K, V> =
    indexmap::IndexMap<K, V, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>;
pub const WORKSPACE: base_db::SourceRootId = base_db::SourceRootId(0);

pub trait WithFixture: Default + SourceDatabase + 'static {
    #[must_use]
    #[track_caller]
    fn with_single_file(wa_fixture: &str) -> (Self, EditionedFileId) {
        let mut db = Self::default();
        let fixture = ChangeFixture::parse(wa_fixture);
        fixture.change.apply(&mut db);
        assert_eq!(
            fixture.files.len(),
            1,
            "Multiple files found in the fixture"
        );
        let file_id = EditionedFileId::from_file(&db, fixture.files[0]);
        (db, file_id)
    }

    #[must_use]
    #[track_caller]
    fn with_many_files(wa_fixture: &str) -> (Self, Vec<EditionedFileId>) {
        let mut db = Self::default();
        let fixture = ChangeFixture::parse(wa_fixture);
        fixture.change.apply(&mut db);
        assert!(fixture.file_position.is_none());
        let files = fixture
            .files
            .iter()
            .map(|file_id| EditionedFileId::from_file(&db, *file_id))
            .collect();
        (db, files)
    }

    #[must_use]
    #[track_caller]
    fn with_files(wa_fixture: &str) -> Self {
        let mut db = Self::default();
        let fixture = ChangeFixture::parse(wa_fixture);
        fixture.change.apply(&mut db);
        assert!(fixture.file_position.is_none());
        db
    }

    #[must_use]
    #[track_caller]
    fn with_position(wa_fixture: &str) -> (Self, FilePosition) {
        let (db, file_id, range_or_offset) = Self::with_range_or_offset(wa_fixture);
        let offset = range_or_offset.expect_offset();
        (db, FilePosition { file_id, offset })
    }

    #[must_use]
    #[track_caller]
    fn with_range(wa_fixture: &str) -> (Self, FileRange) {
        let (db, file_id, range_or_offset) = Self::with_range_or_offset(wa_fixture);
        let range = range_or_offset.expect_range();
        (db, FileRange { file_id, range })
    }

    #[must_use]
    #[track_caller]
    fn with_range_or_offset(wa_fixture: &str) -> (Self, FileId, RangeOrOffset) {
        let mut db = Self::default();
        let fixture = ChangeFixture::parse(wa_fixture);
        fixture.change.apply(&mut db);

        let (file_id, range_or_offset) = fixture
            .file_position
            .expect("Could not find file position in fixture. Did you forget to add an `$0`?");
        (db, file_id, range_or_offset)
    }
}

impl<Database: SourceDatabase + Default + 'static> WithFixture for Database {}

pub struct ChangeFixture {
    pub file_position: Option<(FileId, RangeOrOffset)>,
    pub file_lines: Vec<usize>,
    pub files: Vec<FileId>,
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

        let mut next_file_id = {
            let mut file_id = 0;
            move || {
                let id = file_id;
                file_id += 1;
                FileId::from_raw_usize(id)
            }
        };

        let mut files = Vec::new();
        let mut file_lines = Vec::new();
        let mut packages = FxIndexMap::default();
        let mut package_dependencies = Vec::new();

        let mut roots: Vec<(FileSet, PackageOrigin)> = Vec::new();

        // Add the standard library.
        // Don't add it to the files array, since that's for user files only.
        let std_file_set = {
            let std_library = StdLibrary::new();
            let mut file_set = FileSet::default();

            let manifest_file_id = next_file_id();
            source_change.change_file(
                manifest_file_id,
                Some(String::from_utf8(std_library.manifest.contents.to_vec()).unwrap()),
            );
            let path = VfsPath::new_virtual_path(std_library.manifest.path.clone());
            file_set.insert(manifest_file_id, path);

            let package_id = PackageId::from_raw_usize(packages.len());
            let package = PackageData {
                manifest_file_id,
                root: VfsPath::new_virtual_path("/std".to_owned()),
                edition: std_library.edition,
                display_name: Some("std".to_owned()),
                dependencies: Vec::new(),
                origin: PackageOrigin::Language,
            };
            let previous = packages.insert(
                PackageName::normalize_dashes("wa_test_std"),
                (package_id, package),
            );
            assert!(previous.is_none(), "multiple std packages");

            for file in std_library.files {
                let file_id = next_file_id();
                source_change.change_file(
                    file_id,
                    Some(String::from_utf8(file.contents.to_vec()).unwrap()),
                );
                let path = VfsPath::new_virtual_path(file.path);
                file_set.insert(file_id, path);
            }

            file_set
        };

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
            let mut meta_package = meta.package;
            if meta_package.is_none() && roots.is_empty() {
                // Support tests that have a single file or a few files without setting up a package
                meta_package = Some(PackageMeta {
                    name: "wa_test_fixture".to_owned(),
                    origin: PackageOrigin::Local,
                    root: None,
                    dependencies: Vec::new(),
                });
            }

            if let Some(meta_package) = meta_package {
                let package_name = PackageName::normalize_dashes(&meta_package.name);
                let root = match meta_package.root {
                    Some(root) => VfsPath::new_virtual_path(root),
                    None => VfsPath::new_virtual_path(meta.path.clone())
                        .parent()
                        .unwrap(),
                };

                let manifest_file_id = next_file_id();
                source_change.change_file(manifest_file_id, Some(String::new()));

                let package = PackageData {
                    manifest_file_id,
                    root: root.clone(),
                    edition: meta.edition,
                    display_name: Some(meta_package.name.clone()),
                    dependencies: Vec::new(),
                    origin: meta_package.origin,
                };
                let mut file_set = FileSet::default();
                file_set.insert(manifest_file_id, root.join("wesl.toml").unwrap());
                roots.push((file_set, package.origin));

                let package_id = PackageId::from_raw_usize(packages.len());
                let previous = packages.insert(package_name.clone(), (package_id, package));
                assert!(
                    previous.is_none(),
                    "multiple packages with same name: {package_name}"
                );
                for dep in meta_package.dependencies {
                    let dep = PackageName::normalize_dashes(&dep);
                    package_dependencies.push((package_name.clone(), dep));
                }
            }

            // We use raw file IDs here and then let the packages determine the editions.
            let file_id = next_file_id();
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

        // Push the root later, so that it doesn't mess with the user created files.
        roots.push((std_file_set, PackageOrigin::Language));

        source_change.set_roots(
            roots
                .into_iter()
                .map(|(file_set, origin)| match origin {
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
    edition: Edition,
    package: Option<PackageMeta>,
}

#[derive(Debug)]
struct PackageMeta {
    name: String,
    origin: PackageOrigin,
    root: Option<String>,
    dependencies: Vec<String>,
}

impl FileMeta {
    fn from_fixture(fixture: Fixture) -> Self {
        let edition = fixture.edition.map_or(Edition::CURRENT, |version| {
            Edition::from_str(&version).unwrap()
        });

        let package = if let Some(package_name) = fixture.package {
            let (name, origin) = parse_package(package_name, fixture.library);

            Some(PackageMeta {
                name,
                origin,
                root: fixture.root,
                dependencies: fixture.dependencies,
            })
        } else {
            assert!(
                !fixture.library,
                "cannot specify library mode without naming the package"
            );
            assert!(
                fixture.root.is_none(),
                "cannot specify package root without naming the package"
            );
            assert!(
                fixture.dependencies.is_empty(),
                "cannot specify dependencies without naming the package"
            );
            None
        };

        Self {
            path: fixture.path,
            edition,
            package,
        }
    }
}

const fn parse_package(
    name: String,
    explicit_non_workspace_member: bool,
) -> (String, PackageOrigin) {
    // syntax:
    //   "my_awesome_package"

    let origin = if explicit_non_workspace_member {
        PackageOrigin::Library
    } else {
        PackageOrigin::Local
    };

    (name, origin)
}
