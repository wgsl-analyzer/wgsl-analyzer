//! This module specifies the input to wgsl-analyzer. In some sense, this is
//! **the** most important module, because all other fancy stuff is strictly
//! derived from this input.
//!
//! Note that neither this module, nor any other part of the analyzer's core do
//! actual IO. See `vfs` and `project_model` in the `wgsl-analyzer` package for how
//! actual IO is done and lowered to input.

use std::{fmt, mem, ops};

use edition::Edition;
use la_arena::{Arena, Idx, RawIdx};
use rustc_hash::{FxHashMap, FxHashSet};
use triomphe::Arc;
use vfs::{AbsPathBuf, AnchoredPath, FileId, VfsPath, file_set::FileSet};

use crate::PackageWorkspaceData;

pub type ProcMacroPaths = FxHashMap<PackageId, Result<(String, AbsPathBuf), String>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SourceRootId(pub u32);

/// Files are grouped into source roots. A source root is a directory on the
/// file systems which is watched for changes.
///
/// Typically it corresponds to a WESL package.
/// Source roots *might* be nested: in this case, a file belongs to
/// the nearest enclosing source root.
/// Paths to files are always relative to a source root, and the analyzer does
/// not know the root path of the source root at all.
/// So, a file from one source root can't refer to a file in another source root
/// by path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceRoot {
    /// Sysroot or remote library.
    ///
    /// Libraries are considered mostly immutable, this assumption is used to
    /// optimize salsa's query structure
    pub is_library: bool,
    file_set: FileSet,
}

impl SourceRoot {
    #[must_use]
    pub const fn new_local(file_set: FileSet) -> Self {
        Self {
            is_library: false,
            file_set,
        }
    }

    #[must_use]
    pub const fn new_library(file_set: FileSet) -> Self {
        Self {
            is_library: true,
            file_set,
        }
    }

    pub fn path_for_file(
        &self,
        file: &FileId,
    ) -> Option<&VfsPath> {
        self.file_set.path_for_file(file)
    }

    pub fn file_for_path(
        &self,
        path: &VfsPath,
    ) -> Option<&FileId> {
        self.file_set.file_for_path(path)
    }

    pub fn resolve_path(
        &self,
        path: AnchoredPath<'_>,
    ) -> Option<FileId> {
        self.file_set.resolve_path(path)
    }

    pub fn iter(&self) -> impl Iterator<Item = FileId> + '_ {
        self.file_set.iter()
    }
}

/// `PackageGraph` is a bit of information which turns a set of text files into a
/// number of WESL packages.
///
/// Each package is defined by the `FileId` of its root module, the set of enabled
/// `cfg` flags and the set of dependencies.
///
/// Note that, due to cfg's, there might be several packages for a single `FileId`!
///
/// For the purposes of analysis, a package does not have a name. Instead, names
/// are specified on dependency edges. That is, a package might be known under
/// different names in different dependent packages.
///
/// Note that `PackageGraph` is build-system agnostic: it's a concept of the WESL
/// language proper, not a concept of the build system. In practice, we get
/// `PackageGraph` by lowering `wesl metadata` output.
///
/// `PackageGraph` is `!Serialize` by design, see
/// <https://github.com/wgsl-analyzer/wgsl-analyzer/blob/main/docs/dev/architecture.md#serialization>
#[derive(Clone, Default)]
pub struct PackageGraph {
    arena: Arena<PackageData>,
}

impl fmt::Debug for PackageGraph {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.debug_map()
            .entries(
                self.arena
                    .iter()
                    .map(|(id, data)| (u32::from(id.into_raw()), data)),
            )
            .finish()
    }
}

pub type PackageId = Idx<PackageData>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PackageName(String);

impl PackageName {
    /// Creates a package name, checking for dashes in the string provided.
    /// Dashes are not allowed in the package names,
    /// hence the input string is returned as `Err` for those cases.
    pub fn new(name: &str) -> Result<PackageName, &str> {
        if name.contains('-') {
            Err(name)
        } else {
            Ok(Self(name.to_owned()))
        }
    }

    /// Creates a package name, unconditionally replacing the dashes with underscores.
    pub fn normalize_dashes(name: &str) -> PackageName {
        Self(name.replace('-', "_"))
    }

    pub fn symbol(&self) -> &String {
        &self.0
    }
}

impl fmt::Display for PackageName {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl ops::Deref for PackageName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Origin of the packages.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PackageOrigin {
    /// Packages that are workspace members.
    Local {
        repository: Option<String>,
        name: Option<String>,
    },
    /// Packages that are non-member libraries.
    Library {
        repository: Option<String>,
        name: String,
    },
    /// Packages that are provided by the language, like builtins, ...
    Language(LanguagePackageOrigin),
}

impl PackageOrigin {
    pub fn is_local(&self) -> bool {
        matches!(self, PackageOrigin::Local { .. })
    }

    pub fn is_lib(&self) -> bool {
        matches!(self, PackageOrigin::Library { .. })
    }

    pub fn is_lang(&self) -> bool {
        matches!(self, PackageOrigin::Language { .. })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LanguagePackageOrigin {
    Core,
    Std,
    Other,
}

impl From<&str> for LanguagePackageOrigin {
    fn from(s: &str) -> Self {
        match s {
            "core" => LanguagePackageOrigin::Core,
            "std" => LanguagePackageOrigin::Std,
            _ => LanguagePackageOrigin::Other,
        }
    }
}

impl fmt::Display for LanguagePackageOrigin {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let text = match self {
            LanguagePackageOrigin::Core => "core",
            LanguagePackageOrigin::Std => "std",
            LanguagePackageOrigin::Other => "other",
        };
        f.write_str(text)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PackageDisplayName {
    // The name we use to display various paths (with `_`).
    package_name: PackageName,
    // The name as specified in Cargo.toml (with `-`).
    canonical_name: String,
}

impl PackageDisplayName {
    pub fn canonical_name(&self) -> &String {
        &self.canonical_name
    }
    pub fn package_name(&self) -> &PackageName {
        &self.package_name
    }
}

impl From<PackageName> for PackageDisplayName {
    fn from(package_name: PackageName) -> PackageDisplayName {
        let canonical_name = package_name.0.clone();
        PackageDisplayName {
            package_name,
            canonical_name,
        }
    }
}

impl fmt::Display for PackageDisplayName {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        self.package_name.fmt(f)
    }
}

impl ops::Deref for PackageDisplayName {
    type Target = String;
    fn deref(&self) -> &String {
        &self.package_name
    }
}

impl PackageDisplayName {
    pub fn from_canonical_name(canonical_name: &str) -> PackageDisplayName {
        let package_name = PackageName::normalize_dashes(canonical_name);
        Self {
            package_name,
            canonical_name: canonical_name.to_owned(),
        }
    }
}

pub type TargetLayoutLoadResult = Result<Arc<str>, Arc<str>>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ReleaseChannel {
    Stable,
    Beta,
    Nightly,
}

impl ReleaseChannel {
    pub fn as_str(self) -> &'static str {
        match self {
            ReleaseChannel::Stable => "stable",
            ReleaseChannel::Beta => "beta",
            ReleaseChannel::Nightly => "nightly",
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(str: &str) -> Option<Self> {
        Some(match str {
            "" | "stable" => ReleaseChannel::Stable,
            "nightly" => ReleaseChannel::Nightly,
            _ if str.starts_with("beta") => ReleaseChannel::Beta,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageData {
    pub root_file_id: FileId,
    pub edition: Edition,
    pub version: Option<String>,
    /// A name used in the package's project declaration: for Cargo projects,
    /// its `[package].name` can be different for other project types or even
    /// absent (a dummy package for the code snippet, for example).
    ///
    /// For purposes of analysis, packages are anonymous (only names in
    /// `Dependency` matters), this name should only be used for UI.
    pub display_name: Option<PackageDisplayName>,
    pub env: Environment,
    /// The dependencies of this package.
    ///
    /// Note that this may contain more dependencies than the package actually uses.
    /// A common example is the test package which is included but only actually is active when
    /// declared in source via `extern package test`.
    pub dependencies: Vec<Dependency>,
    pub origin: PackageOrigin,
}

#[derive(Default, Clone, PartialEq, Eq)]
pub struct Environment {
    entries: FxHashMap<String, String>,
}

impl fmt::Debug for Environment {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        struct EnvDebug<'s>(Vec<(&'s String, &'s String)>);

        impl fmt::Debug for EnvDebug<'_> {
            fn fmt(
                &self,
                f: &mut fmt::Formatter<'_>,
            ) -> fmt::Result {
                f.debug_map().entries(self.0.iter().copied()).finish()
            }
        }
        f.debug_struct("Env")
            .field("entries", &{
                let mut entries: Vec<_> = self.entries.iter().collect();
                entries.sort();
                EnvDebug(entries)
            })
            .finish()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Dependency {
    pub package_id: PackageId,
    pub name: PackageName,
    prelude: bool,
    sysroot: bool,
}

impl Dependency {
    pub fn new(
        name: PackageName,
        package_id: PackageId,
    ) -> Self {
        Self {
            name,
            package_id,
            prelude: true,
            sysroot: false,
        }
    }

    pub fn with_prelude(
        name: PackageName,
        package_id: PackageId,
        prelude: bool,
        sysroot: bool,
    ) -> Self {
        Self {
            name,
            package_id,
            prelude,
            sysroot,
        }
    }

    /// Whether this dependency is to be added to the depending package's extern prelude.
    #[must_use]
    pub const fn is_prelude(&self) -> bool {
        self.prelude
    }

    /// Whether this dependency is a sysroot injected one.
    #[must_use]
    pub const fn is_sysroot(&self) -> bool {
        self.sysroot
    }
}

impl PackageGraph {
    pub fn add_package_root(
        &mut self,
        root_file_id: FileId,
        edition: Edition,
        display_name: Option<PackageDisplayName>,
        version: Option<String>,
        mut environment: Environment,
        origin: PackageOrigin,
    ) -> PackageId {
        environment.entries.shrink_to_fit();
        let data = PackageData {
            root_file_id,
            edition,
            version,
            display_name,
            env: environment,
            dependencies: Vec::new(),
            origin,
        };
        self.arena.alloc(data)
    }

    pub fn add_dep(
        &mut self,
        from: PackageId,
        dep: Dependency,
    ) -> Result<(), CyclicDependenciesError> {
        let _p = tracing::info_span!("add_dep").entered();

        // Check if adding a dep from `from` to `to` creates a cycle. To figure
        // that out, look for a  path in the *opposite* direction, from `to` to
        // `from`.
        if let Some(path) = self.find_path(&mut FxHashSet::default(), dep.package_id, from) {
            let path = path
                .into_iter()
                .map(|it| (it, self[it].display_name.clone()))
                .collect();
            let error = CyclicDependenciesError { path };
            assert!(error.from().0 == from && error.to().0 == dep.package_id);
            return Err(error);
        }

        self.arena[from].add_dep(dep);
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.arena.is_empty()
    }

    pub fn len(&self) -> usize {
        self.arena.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = PackageId> + '_ {
        self.arena.iter().map(|(index, _)| index)
    }

    // FIXME: used for fixing up the toolchain sysroot, should be removed and done differently
    #[doc(hidden)]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (PackageId, &mut PackageData)> + '_ {
        self.arena.iter_mut()
    }

    /// Returns an iterator over all transitive dependencies of the given package,
    /// including the package itself.
    pub fn transitive_deps(
        &self,
        of: PackageId,
    ) -> impl Iterator<Item = PackageId> {
        let mut worklist = vec![of];
        let mut deps = FxHashSet::default();

        while let Some(package) = worklist.pop() {
            if !deps.insert(package) {
                continue;
            }

            worklist.extend(self[package].dependencies.iter().map(|dep| dep.package_id));
        }

        deps.into_iter()
    }

    /// Returns all transitive reverse dependencies of the given package,
    /// including the package itself.
    pub fn transitive_rev_deps(
        &self,
        of: PackageId,
    ) -> impl Iterator<Item = PackageId> {
        let mut worklist = vec![of];
        let mut rev_deps = FxHashSet::default();
        rev_deps.insert(of);

        let mut inverted_graph = FxHashMap::<_, Vec<_>>::default();
        self.arena.iter().for_each(|(package, data)| {
            data.dependencies.iter().for_each(|dep| {
                inverted_graph
                    .entry(dep.package_id)
                    .or_default()
                    .push(package)
            })
        });

        while let Some(package) = worklist.pop() {
            if let Some(krate_rev_deps) = inverted_graph.get(&package) {
                krate_rev_deps
                    .iter()
                    .copied()
                    .filter(|&rev_dep| rev_deps.insert(rev_dep))
                    .for_each(|rev_dep| worklist.push(rev_dep));
            }
        }

        rev_deps.into_iter()
    }

    /// Returns all packages in the graph, sorted in topological order (ie. dependencies of a package
    /// come before the package itself).
    pub fn packages_in_topological_order(&self) -> Vec<PackageId> {
        let mut result = Vec::new();
        let mut visited = FxHashSet::default();

        for package in self.iter() {
            go(self, &mut visited, &mut result, package);
        }

        return result;

        fn go(
            graph: &PackageGraph,
            visited: &mut FxHashSet<PackageId>,
            result: &mut Vec<PackageId>,
            source: PackageId,
        ) {
            if !visited.insert(source) {
                return;
            }
            for dep in graph[source].dependencies.iter() {
                go(graph, visited, result, dep.package_id)
            }
            result.push(source)
        }
    }

    /// Extends this package graph by adding a complete second package
    /// graph and adjust the ids in the [`ProcMacroPaths`] accordingly.
    ///
    /// This will deduplicate the packages of the graph where possible.
    /// Furthermore dependencies are sorted by package id to make deduplication easier.
    ///
    /// Returns a map mapping `other`'s IDs to the new IDs in `self`.
    pub fn extend(
        &mut self,
        mut other: PackageGraph,
    ) -> FxHashMap<PackageId, PackageId> {
        // Sorting here is a bit pointless because the input is likely already sorted.
        // However, the overhead is small and it makes the `extend` method harder to misuse.
        self.arena
            .iter_mut()
            .for_each(|(_, data)| data.dependencies.sort_by_key(|dep| dep.package_id));

        let m = self.len();
        let topo = other.packages_in_topological_order();
        let mut id_map: FxHashMap<PackageId, PackageId> = FxHashMap::default();
        for topo in topo {
            let package_data = &mut other.arena[topo];

            package_data
                .dependencies
                .iter_mut()
                .for_each(|dep| dep.package_id = id_map[&dep.package_id]);
            package_data.dependencies.sort_by_key(|dep| dep.package_id);

            let find = self
                .arena
                .iter()
                .take(m)
                .find_map(|(k, v)| (v == package_data).then_some(k));
            let new_id = find.unwrap_or_else(|| self.arena.alloc(package_data.clone()));
            id_map.insert(topo, new_id);
        }
        id_map
    }

    fn find_path(
        &self,
        visited: &mut FxHashSet<PackageId>,
        from: PackageId,
        to: PackageId,
    ) -> Option<Vec<PackageId>> {
        if !visited.insert(from) {
            return None;
        }

        if from == to {
            return Some(vec![to]);
        }

        for dep in &self[from].dependencies {
            let package_id = dep.package_id;
            if let Some(mut path) = self.find_path(visited, package_id, to) {
                path.push(from);
                return Some(path);
            }
        }

        None
    }

    /// Removes all packages from this package graph except for the ones in `to_keep` and fixes up the dependencies.
    /// Returns a mapping from old package ids to new package ids.
    pub fn remove_packages_except(
        &mut self,
        to_keep: &[PackageId],
    ) -> Vec<Option<PackageId>> {
        let mut id_map = vec![None; self.arena.len()];
        self.arena = std::mem::take(&mut self.arena)
            .into_iter()
            .filter_map(|(id, data)| {
                if to_keep.contains(&id) {
                    Some((id, data))
                } else {
                    None
                }
            })
            .enumerate()
            .map(|(new_id, (id, data))| {
                id_map[id.into_raw().into_u32() as usize] =
                    Some(PackageId::from_raw(RawIdx::from_u32(new_id as u32)));
                data
            })
            .collect();
        for (_, data) in self.arena.iter_mut() {
            data.dependencies.iter_mut().for_each(|dep| {
                dep.package_id = id_map[dep.package_id.into_raw().into_u32() as usize]
                    .expect("package was filtered")
            });
        }
        id_map
    }

    pub fn shrink_to_fit(&mut self) {
        self.arena.shrink_to_fit();
    }
}

impl ops::Index<PackageId> for PackageGraph {
    type Output = PackageData;
    fn index(
        &self,
        package_id: PackageId,
    ) -> &PackageData {
        &self.arena[package_id]
    }
}

impl PackageData {
    /// Add a dependency to `self` without checking if the dependency
    // is existent among `self.dependencies`.
    fn add_dep(
        &mut self,
        dep: Dependency,
    ) {
        self.dependencies.push(dep)
    }

    pub fn root_file_id(&self) -> (FileId, Edition) {
        (self.root_file_id, self.edition)
    }
}

impl Extend<(String, String)> for Environment {
    fn extend<T: IntoIterator<Item = (String, String)>>(
        &mut self,
        iter: T,
    ) {
        self.entries.extend(iter);
    }
}

impl FromIterator<(String, String)> for Environment {
    fn from_iter<T: IntoIterator<Item = (String, String)>>(iter: T) -> Self {
        Environment {
            entries: FromIterator::from_iter(iter),
        }
    }
}

impl Environment {
    pub fn set(
        &mut self,
        env: &str,
        value: impl Into<String>,
    ) {
        self.entries.insert(env.to_owned(), value.into());
    }

    pub fn get(
        &self,
        env: &str,
    ) -> Option<String> {
        self.entries.get(env).cloned()
    }

    pub fn extend_from_other(
        &mut self,
        other: &Environment,
    ) {
        self.entries.extend(
            other
                .entries
                .iter()
                .map(|(x, y)| (x.to_owned(), y.to_owned())),
        );
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn insert(
        &mut self,
        k: impl Into<String>,
        v: impl Into<String>,
    ) -> Option<String> {
        self.entries.insert(k.into(), v.into())
    }
}

impl From<Environment> for Vec<(String, String)> {
    fn from(env: Environment) -> Vec<(String, String)> {
        let mut entries: Vec<_> = env.entries.into_iter().collect();
        entries.sort();
        entries
    }
}

impl<'a> IntoIterator for &'a Environment {
    type Item = (&'a String, &'a String);
    type IntoIter = std::collections::hash_map::Iter<'a, String, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.iter()
    }
}

#[derive(Debug)]
pub struct CyclicDependenciesError {
    path: Vec<(PackageId, Option<PackageDisplayName>)>,
}

impl CyclicDependenciesError {
    fn from(&self) -> &(PackageId, Option<PackageDisplayName>) {
        self.path.first().unwrap()
    }
    fn to(&self) -> &(PackageId, Option<PackageDisplayName>) {
        self.path.last().unwrap()
    }
}

impl fmt::Display for CyclicDependenciesError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let render = |(id, name): &(PackageId, Option<PackageDisplayName>)| match name {
            Some(it) => format!("{it}({id:?})"),
            None => format!("{id:?}"),
        };
        let path = self
            .path
            .iter()
            .rev()
            .map(render)
            .collect::<Vec<String>>()
            .join(" -> ");
        write!(
            f,
            "cyclic deps: {} -> {}, alternative path: {}",
            render(self.from()),
            render(self.to()),
            path
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::PackageOrigin;

    use super::{
        Dependency, Edition::Wesl2025Unstable, Environment, FileId, PackageGraph, PackageName,
    };

    #[test]
    fn detect_cyclic_dependency_indirect() {
        let mut graph = PackageGraph::default();
        let package1 = graph.add_package_root(
            FileId::from_raw(1u32),
            Wesl2025Unstable,
            None,
            None,
            Environment::default(),
            PackageOrigin::Local {
                repository: None,
                name: None,
            },
        );
        let package2 = graph.add_package_root(
            FileId::from_raw(2u32),
            Wesl2025Unstable,
            None,
            None,
            Environment::default(),
            PackageOrigin::Local {
                repository: None,
                name: None,
            },
        );
        let package3 = graph.add_package_root(
            FileId::from_raw(3u32),
            Wesl2025Unstable,
            None,
            None,
            Environment::default(),
            PackageOrigin::Local {
                repository: None,
                name: None,
            },
        );
        assert!(
            graph
                .add_dep(
                    package1,
                    Dependency::new(PackageName::new("package2").unwrap(), package2,)
                )
                .is_ok()
        );
        assert!(
            graph
                .add_dep(
                    package2,
                    Dependency::new(PackageName::new("package3").unwrap(), package3,)
                )
                .is_ok()
        );
        assert!(
            graph
                .add_dep(
                    package3,
                    Dependency::new(PackageName::new("package1").unwrap(), package1,)
                )
                .is_err()
        );
    }

    #[test]
    fn detect_cyclic_dependency_direct() {
        let mut graph = PackageGraph::default();
        let package1 = graph.add_package_root(
            FileId::from_raw(1u32),
            Wesl2025Unstable,
            None,
            None,
            Environment::default(),
            PackageOrigin::Local {
                repository: None,
                name: None,
            },
        );
        let package2 = graph.add_package_root(
            FileId::from_raw(2u32),
            Wesl2025Unstable,
            None,
            None,
            Environment::default(),
            PackageOrigin::Local {
                repository: None,
                name: None,
            },
        );
        assert!(
            graph
                .add_dep(
                    package1,
                    Dependency::new(PackageName::new("package2").unwrap(), package2,)
                )
                .is_ok()
        );
        assert!(
            graph
                .add_dep(
                    package2,
                    Dependency::new(PackageName::new("package2").unwrap(), package2,)
                )
                .is_err()
        );
    }

    #[test]
    fn it_works() {
        let mut graph = PackageGraph::default();
        let package1 = graph.add_package_root(
            FileId::from_raw(1_u32),
            Wesl2025Unstable,
            None,
            None,
            Environment::default(),
            PackageOrigin::Local {
                repository: None,
                name: None,
            },
        );
        let package2 = graph.add_package_root(
            FileId::from_raw(2u32),
            Wesl2025Unstable,
            None,
            None,
            Environment::default(),
            PackageOrigin::Local {
                repository: None,
                name: None,
            },
        );
        let package3 = graph.add_package_root(
            FileId::from_raw(3u32),
            Wesl2025Unstable,
            None,
            None,
            Environment::default(),
            PackageOrigin::Local {
                repository: None,
                name: None,
            },
        );
        assert!(
            graph
                .add_dep(
                    package1,
                    Dependency::new(PackageName::new("package2").unwrap(), package2,)
                )
                .is_ok()
        );
        assert!(
            graph
                .add_dep(
                    package2,
                    Dependency::new(PackageName::new("package3").unwrap(), package3,)
                )
                .is_ok()
        );
    }

    #[test]
    fn dashes_are_normalized() {
        let mut graph = PackageGraph::default();
        let package1 = graph.add_package_root(
            FileId::from_raw(1u32),
            Wesl2025Unstable,
            None,
            None,
            Environment::default(),
            PackageOrigin::Local {
                repository: None,
                name: None,
            },
        );
        let package2 = graph.add_package_root(
            FileId::from_raw(2u32),
            Wesl2025Unstable,
            None,
            None,
            Environment::default(),
            PackageOrigin::Local {
                repository: None,
                name: None,
            },
        );
        assert!(
            graph
                .add_dep(
                    package1,
                    Dependency::new(
                        PackageName::normalize_dashes("package-name-with-dashes"),
                        package2,
                    )
                )
                .is_ok()
        );
        assert_eq!(
            graph[package1].dependencies,
            vec![Dependency::new(
                PackageName::new("package_name_with_dashes").unwrap(),
                package2,
            )]
        );
    }
}
