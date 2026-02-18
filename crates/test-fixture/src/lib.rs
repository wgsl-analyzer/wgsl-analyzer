//! A set of high-level utility fixture methods to use in tests.
mod fixture;

use std::{any::TypeId, mem, str::FromStr, sync};

pub use crate::fixture::{Fixture, FixtureWithProjectMeta};
use base_db::change::Change;
use base_db::{
    Dependency, EditionedFileId, FileId, FilePosition, FileRange, FileSet, LanguagePackageOrigin,
    PackageGraph, PackageName, PackageOrigin, SourceDatabase, SourceRoot, VfsPath,
};
use edition::Edition;
use stdx::itertools::Itertools;
use test_utils::{CURSOR_MARKER, ESCAPED_CURSOR_MARKER, RangeOrOffset, extract_range_or_offset};
use triomphe::Arc;

type FxIndexMap<K, V> =
    indexmap::IndexMap<K, V, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>;
pub const WORKSPACE: base_db::SourceRootId = base_db::SourceRootId(0);

pub trait WithFixture: Default + SourceDatabase + 'static {
    #[track_caller]
    fn with_single_file(
        #[rust_analyzer::rust_fixture] ra_fixture: &str
    ) -> (Self, EditionedFileId) {
        let mut db = Self::default();
        let fixture = ChangeFixture::parse(ra_fixture);
        fixture.change.apply(&mut db);
        assert_eq!(fixture.files.len(), 1, "Multiple file found in the fixture");
        (db, fixture.files[0])
    }

    #[track_caller]
    fn with_many_files(
        #[rust_analyzer::rust_fixture] ra_fixture: &str
    ) -> (Self, Vec<EditionedFileId>) {
        let mut db = Self::default();
        let fixture = ChangeFixture::parse(ra_fixture);
        fixture.change.apply(&mut db);
        assert!(fixture.file_position.is_none());
        (db, fixture.files)
    }

    #[track_caller]
    fn with_files(#[rust_analyzer::rust_fixture] ra_fixture: &str) -> Self {
        let mut db = Self::default();
        let fixture = ChangeFixture::parse(ra_fixture);
        fixture.change.apply(&mut db);
        assert!(fixture.file_position.is_none());
        db
    }

    #[track_caller]
    fn with_position(#[rust_analyzer::rust_fixture] ra_fixture: &str) -> (Self, FilePosition) {
        let (db, file_id, range_or_offset) = Self::with_range_or_offset(ra_fixture);
        let offset = range_or_offset.expect_offset();
        (db, FilePosition { file_id, offset })
    }

    #[track_caller]
    fn with_range(#[rust_analyzer::rust_fixture] ra_fixture: &str) -> (Self, FileRange) {
        let (db, file_id, range_or_offset) = Self::with_range_or_offset(ra_fixture);
        let range = range_or_offset.expect_range();
        (db, FileRange { file_id, range })
    }

    #[track_caller]
    fn with_range_or_offset(
        #[rust_analyzer::rust_fixture] ra_fixture: &str
    ) -> (Self, FileId, RangeOrOffset) {
        let mut db = Self::default();
        let fixture = ChangeFixture::parse(ra_fixture);
        fixture.change.apply(&mut db);

        let (file_id, range_or_offset) = fixture
            .file_position
            .expect("Could not find file position in fixture. Did you forget to add an `$0`?");
        (db, file_id, range_or_offset)
    }

    // fn test_crate(&self) -> Crate {
    //     self.all_crates()
    //         .iter()
    //         .copied()
    //         .find(|&krate| !krate.data(self).origin.is_lang())
    //         .unwrap()
    // }
}

impl<DB: SourceDatabase + Default + 'static> WithFixture for DB {}

pub struct ChangeFixture {
    pub file_position: Option<(FileId, RangeOrOffset)>,
    pub file_lines: Vec<usize>,
    pub files: Vec<EditionedFileId>,
    pub change: Change,
}

const SOURCE_ROOT_PREFIX: &str = "/";

impl ChangeFixture {
    pub fn parse(#[rust_analyzer::rust_fixture] ra_fixture: &str) -> ChangeFixture {
        let FixtureWithProjectMeta { fixture } = FixtureWithProjectMeta::parse(ra_fixture);
        let mut source_change = Change::default();

        let mut files = Vec::new();
        let mut file_lines = Vec::new();
        let mut crate_graph = PackageGraph::default();
        let mut crates = FxIndexMap::default();
        let mut crate_deps = Vec::new();
        let mut default_crate_root: Option<FileId> = None;
        let mut default_edition = Edition::CURRENT;

        let mut file_set = FileSet::default();
        let mut current_source_root_kind = SourceRootKind::Local;
        let mut file_id = FileId::from_raw(0);
        let mut roots = Vec::new();

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

            let meta = FileMeta::from_fixture(entry, current_source_root_kind);
            if let Some(range_or_offset) = range_or_offset {
                file_position = Some((file_id, range_or_offset));
            }

            assert!(meta.path.starts_with(SOURCE_ROOT_PREFIX));
            if !meta.deps.is_empty() {
                assert!(
                    meta.package.is_some(),
                    "can't specify deps without naming the crate"
                )
            }

            if let Some(kind) = meta.introduce_new_source_root {
                assert!(
                    meta.package.is_some(),
                    "new_source_root meta doesn't make sense without crate meta"
                );
                let previous_kind = mem::replace(&mut current_source_root_kind, kind);
                let previous_root = match previous_kind {
                    SourceRootKind::Local => SourceRoot::new_local(mem::take(&mut file_set)),
                    SourceRootKind::Library => SourceRoot::new_library(mem::take(&mut file_set)),
                };
                roots.push(previous_root);
            }

            if let Some((krate, origin, version)) = meta.package {
                let crate_name = PackageName::normalize_dashes(&krate);
                let crate_id = crate_graph.add_package_root(
                    file_id,
                    meta.edition,
                    Some(crate_name.clone().into()),
                    version,
                    origin,
                );
                let previous = crates.insert(crate_name.clone(), crate_id);
                assert!(
                    previous.is_none(),
                    "multiple crates with same name: {crate_name}"
                );
                for dep in meta.deps {
                    let dep = PackageName::normalize_dashes(&dep);
                    crate_deps.push((crate_name.clone(), dep))
                }
            } else if meta.path == "/main.rs" || meta.path == "/lib.rs" {
                assert!(default_crate_root.is_none());
                default_crate_root = Some(file_id);
                default_edition = meta.edition;
            }

            let path = VfsPath::new_virtual_path(meta.path);
            source_change.change_file(file_id, Some(text.into()), path.clone());
            file_set.insert(file_id, path);
            files.push(EditionedFileId {
                file_id,
                edition: meta.edition,
            });
            file_id = FileId::from_raw(file_id.index() + 1);
        }

        if crates.is_empty() {
            let crate_root = default_crate_root
                .expect("missing default crate root, specify a main.rs or lib.rs");
            crate_graph.add_package_root(
                crate_root,
                default_edition,
                Some(PackageName::new("ra_test_fixture").unwrap().into()),
                None,
                PackageOrigin::Local {
                    repository: None,
                    name: None,
                },
            );
            for (from, to) in crate_deps {
                let from_id = crates[&from];
                let to_id = crates[&to];
                let _sysroot = crate_graph[to_id].origin.is_lang();
                crate_graph
                    .add_dep(from_id, Dependency::new(to.clone(), to_id))
                    .unwrap();
            }
        }

        let _ = file_id;

        let root = match current_source_root_kind {
            SourceRootKind::Local => SourceRoot::new_local(mem::take(&mut file_set)),
            SourceRootKind::Library => SourceRoot::new_library(mem::take(&mut file_set)),
        };
        roots.push(root);

        source_change.set_roots(roots);
        source_change.set_package_graph(crate_graph);

        ChangeFixture {
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
    package: Option<(String, PackageOrigin, Option<String>)>,
    deps: Vec<String>,
    edition: Edition,
    introduce_new_source_root: Option<SourceRootKind>,
}

impl FileMeta {
    fn from_fixture(
        f: Fixture,
        current_source_root_kind: SourceRootKind,
    ) -> Self {
        let introduce_new_source_root = f.introduce_new_source_root.map(|kind| match &*kind {
            "local" => SourceRootKind::Local,
            "library" => SourceRootKind::Library,
            invalid => panic!("invalid source root kind '{invalid}'"),
        });
        let current_source_root_kind =
            introduce_new_source_root.unwrap_or(current_source_root_kind);

        let deps = f.deps;
        Self {
            path: f.path,
            package: f
                .package
                .map(|it| parse_crate(it, current_source_root_kind, f.library)),
            deps,
            edition: f
                .edition
                .map_or(Edition::CURRENT, |v| Edition::from_str(&v).unwrap()),
            introduce_new_source_root,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ForceNoneLangOrigin {
    Yes,
    No,
}

fn parse_crate(
    crate_str: String,
    current_source_root_kind: SourceRootKind,
    explicit_non_workspace_member: bool,
) -> (String, PackageOrigin, Option<String>) {
    let (crate_str, force_non_lang_origin) = if let Some(s) = crate_str.strip_prefix("r#") {
        (s.to_owned(), ForceNoneLangOrigin::Yes)
    } else {
        (crate_str, ForceNoneLangOrigin::No)
    };

    // syntax:
    //   "my_awesome_crate"
    //   "my_awesome_crate@0.0.1,http://example.com"
    let (name, repository, version) = if let Some((name, remain)) = crate_str.split_once('@') {
        let (version, repository) = remain
            .split_once(',')
            .expect("crate meta: found '@' without version and url");
        (
            name.to_owned(),
            Some(repository.to_owned()),
            Some(version.to_owned()),
        )
    } else {
        (crate_str, None, None)
    };

    let non_workspace_member = explicit_non_workspace_member
        || matches!(current_source_root_kind, SourceRootKind::Library);

    let origin = if force_non_lang_origin == ForceNoneLangOrigin::Yes {
        let name = name.clone();
        if non_workspace_member {
            PackageOrigin::Library { repository, name }
        } else {
            PackageOrigin::Local {
                repository,
                name: Some(name),
            }
        }
    } else {
        match LanguagePackageOrigin::from(&*name) {
            LanguagePackageOrigin::Other => {
                let name = name.clone();
                if non_workspace_member {
                    PackageOrigin::Library { repository, name }
                } else {
                    PackageOrigin::Local {
                        repository,
                        name: Some(name),
                    }
                }
            },
            origin => PackageOrigin::Language(origin),
        }
    };

    (name, origin, version)
}
