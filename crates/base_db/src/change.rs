//! Defines a unit of change that can be applied to the database to get the next
//! state. Changes are transactional.

use rustc_hash::FxHashMap;
use salsa::{Durability, Id, Setter as _};
use std::{
    collections::{VecDeque, hash_map::Entry},
    fmt,
};
use triomphe::Arc;
use vfs::{FileId, VfsPath};

use crate::{
    Package, RootQueryDb,
    input::{Dependency, PackageData, PackageId, PackageOrigin, SourceRoot, SourceRootId},
};

/// Encapsulate a bunch of raw `.set` calls on the database.
#[derive(Default)]
pub struct Change {
    pub roots: Option<Vec<SourceRoot>>,
    pub files_changed: Vec<(FileId, Option<String>)>,
    pub packages_changed: Vec<(PackageId, Option<PackageData>)>,
}

impl fmt::Debug for Change {
    fn fmt(
        &self,
        fmt: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let mut debug = fmt.debug_struct("Change");
        if let Some(roots) = &self.roots {
            debug.field("roots", &roots);
        }
        if !self.files_changed.is_empty() {
            debug.field("files_changed", &self.files_changed.len());
        }
        if !self.packages_changed.is_empty() {
            debug.field("packages_changed", &self.packages_changed);
        }
        debug.finish()
    }
}

impl Change {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_roots(
        &mut self,
        roots: Vec<SourceRoot>,
    ) {
        self.roots = Some(roots);
    }

    pub fn change_file(
        &mut self,
        file_id: FileId,
        new_text: Option<String>,
    ) {
        self.files_changed.push((file_id, new_text));
    }

    pub fn change_package(
        &mut self,
        package_id: PackageId,
        new_data: Option<PackageData>,
    ) {
        self.packages_changed.push((package_id, new_data));
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.roots.is_none() && self.files_changed.is_empty() && self.packages_changed.is_empty()
    }

    /// Applies a change to the database.
    ///
    /// # Panics
    ///
    /// Panics if the number of source roots exceeds `u32::MAX`, as `SourceRootId` holds a `u32`.
    pub fn apply(
        self,
        database: &mut dyn RootQueryDb,
    ) {
        if let Some(roots) = self.roots {
            for (root, root_id) in roots.into_iter().zip(0_u32..) {
                let root_id = SourceRootId(root_id);
                let durability = source_root_durability(&root);
                for file_id in root.iter() {
                    database.set_file_source_root_with_durability(file_id, root_id, durability);
                }
                database.set_source_root_with_durability(root_id, Arc::new(root), durability);
            }
        }

        for (file_id, text) in self.files_changed {
            let source_root_id = database.file_source_root(file_id);
            let source_root = database.source_root(source_root_id.source_root_id(database));

            let durability = file_text_durability(&source_root.source_root(database));
            // Can't actually remove the file, just reset the text, see: https://github.com/salsa-rs/salsa/issues/37
            let text = text.unwrap_or_default();
            database.set_file_text_with_durability(file_id, &text, durability);
        }

        let mut package_graph = PackageGraph::new(&*database);
        package_graph.update(self.packages_changed);
        let (sorted_packages, errors) = package_graph.to_topological_order();
        package_graph.remove_cycles(&errors);

        // TODO: Report the errors?

        apply_package_graph(database, package_graph, sorted_packages);
    }
}

fn apply_package_graph(
    database: &mut dyn RootQueryDb,
    mut package_graph: PackageGraph,
    sorted_packages: Vec<PackageId>,
) {
    let mut old_packages: FxHashMap<PackageId, Package> = database
        .all_packages()
        .iter()
        .map(|package| (package.package_id(database), *package))
        .collect();

    let mut all_packages = Vec::with_capacity(sorted_packages.len());
    for package_id in sorted_packages {
        let new_package = package_graph.packages.remove(&package_id).unwrap();
        let durability = package_data_durability(&new_package);

        let package = match old_packages.remove(&package_id) {
            Some(old_package) => {
                if old_package.data(database).as_ref() != &new_package {
                    old_package
                        .set_data(database)
                        .with_durability(durability)
                        .to(Arc::new(new_package));
                }
                old_package
            },
            None => Package::builder(Arc::new(new_package), package_id)
                .durability(durability)
                .new(database),
        };
        all_packages.push(package);
    }

    for (_, remaining_package) in old_packages {
        let package_data = remaining_package.data(database);
        let dummy_package = Arc::new(PackageData {
            root_file_id: package_data.root_file_id,
            edition: package_data.edition,
            display_name: None,
            dependencies: Vec::new(),
            cyclic_dependencies: Vec::new(),
            origin: package_data.origin,
        });
        // Salsa does not have a removal API yet, see: https://github.com/salsa-rs/salsa/issues/37
        remaining_package.set_data(database).to(dummy_package);
    }

    database.set_all_packages(Arc::new(all_packages.into_boxed_slice()));
}

#[must_use]
const fn source_root_durability(source_root: &SourceRoot) -> Durability {
    if source_root.is_library() {
        Durability::MEDIUM
    } else {
        Durability::LOW
    }
}

#[must_use]
const fn file_text_durability(source_root: &SourceRoot) -> Durability {
    if source_root.is_library() {
        Durability::HIGH
    } else {
        Durability::LOW
    }
}

#[must_use]
const fn package_data_durability(package_data: &PackageData) -> Durability {
    match package_data.origin {
        PackageOrigin::Local => Durability::LOW,
        PackageOrigin::Library | PackageOrigin::Language => Durability::HIGH,
    }
}

/// Creates a new cycle free, topological sorting of packages.
struct PackageGraph {
    /// Tracking the IDs separately for a more stable ordering.
    ids: Vec<PackageId>,
    packages: FxHashMap<PackageId, PackageData>,
}

impl PackageGraph {
    pub fn new(database: &dyn RootQueryDb) -> Self {
        let (ids, packages): (Vec<_>, FxHashMap<_, _>) = database
            .all_packages()
            .iter()
            .map(|package| {
                let mut package_data = PackageData::clone(package.data(database));
                // Ensure that we view everything as a potential dependency
                package_data
                    .dependencies
                    .append(&mut package_data.cyclic_dependencies);
                let id = package.package_id(database);
                (id, (id, package_data))
            })
            .unzip();
        Self { ids, packages }
    }

    fn update(
        &mut self,
        packages_changed: Vec<(PackageId, Option<PackageData>)>,
    ) {
        for (package_id, package) in packages_changed {
            if let Some(package) = package {
                self.packages.insert(package_id, package);
                self.ids.push(package_id);
            } else {
                self.packages.remove(&package_id);
            }
        }
    }

    fn remove_cycles(
        &mut self,
        errors: &[CyclicDependenciesError],
    ) {
        for error in errors {
            let package_data = self.packages.get_mut(&error.to().package_id).unwrap();
            let previous_length = package_data.dependencies.len();
            package_data
                .dependencies
                .retain_mut(|dependency| dependency != error.from());
            package_data.cyclic_dependencies.push(error.from().clone());
            assert_eq!(
                previous_length - 1,
                package_data.dependencies.len(),
                "Expected to have removed exactly one cyclic dependency."
            );
        }
    }

    /// Returns all packages, sorted in topological order (ie. dependencies of a package
    /// come before the package itself).
    ///
    /// Also uses [a coloring algorithm to find and remove cycles](https://en.wikipedia.org/wiki/Cycle_(graph_theory)#Algorithm).
    fn to_topological_order(&self) -> (Vec<PackageId>, Vec<CyclicDependenciesError>) {
        let mut visited: FxHashMap<PackageId, CycleState> = self
            .packages
            .keys()
            .map(|id| (*id, CycleState::Unvisited))
            .collect();
        let mut result = Vec::with_capacity(self.packages.len());
        let mut errors = Vec::new();

        for id in &self.ids {
            self.check(&mut visited, &mut result, &mut errors, *id);
        }

        (result, errors)
    }

    fn check(
        &self,
        visited: &mut FxHashMap<PackageId, CycleState>,
        result: &mut Vec<PackageId>,
        errors: &mut Vec<CyclicDependenciesError>,
        source: PackageId,
    ) {
        match visited.get(&source) {
            Some(CycleState::Finished) | None => (),
            Some(CycleState::Visited) => {
                // The visited nodes are a part of the cycle
                let mut path = vec![];
                let mut current = source;
                while let Some(cycle_dependency) = self.visited_child(visited, current) {
                    current = cycle_dependency.package_id;
                    // mark as finished so that we do not loop endlessly
                    visited.insert(current, CycleState::Finished);
                    path.push(cycle_dependency);
                }
                errors.push(CyclicDependenciesError { path });
            },
            Some(CycleState::Unvisited) => {
                visited.insert(source, CycleState::Visited);
                for dependency in &self.packages[&source].dependencies {
                    let package_id = dependency.package_id;
                    self.check(visited, result, errors, package_id);
                }
                result.push(source);
                visited.insert(source, CycleState::Finished);
            },
        }
    }

    fn visited_child(
        &self,
        visited: &FxHashMap<PackageId, CycleState>,
        index: PackageId,
    ) -> Option<Dependency> {
        self.packages[&index]
            .dependencies
            .iter()
            .find(|dependency| visited[&dependency.package_id] == CycleState::Visited)
            .cloned()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CyclicDependenciesError {
    path: Vec<Dependency>,
}

impl CyclicDependenciesError {
    fn from(&self) -> &Dependency {
        self.path.first().unwrap()
    }
    fn to(&self) -> &Dependency {
        self.path.last().unwrap()
    }
}

impl fmt::Display for CyclicDependenciesError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let render = |Dependency { package_id, name }: &Dependency| {
            format!("Package {name}({})", package_id.index())
        };
        let path = self
            .path
            .iter()
            .chain(std::iter::once(self.from()))
            .map(render)
            .collect::<Vec<String>>()
            .join(" -> ");
        write!(
            f,
            "Cyclic dependency from {} to {}. Path: {path}",
            render(self.from()),
            render(self.to())
        )
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub(crate) enum CycleState {
    Unvisited,
    Visited,
    Finished,
}

#[cfg(test)]
mod tests {
    use super::{CyclicDependenciesError, FileId, PackageGraph};
    use crate::{
        SourceRoot,
        input::{Dependency, PackageData, PackageId, PackageName, PackageOrigin},
    };
    use edition::Edition;
    use expect_test::expect;
    use std::fmt::Write;
    use triomphe::Arc;
    use vfs::{AbsPathBuf, file_set::FileSet};

    fn check(
        packages: &[(PackageId, PackageData)],
        expect: expect_test::Expect,
    ) {
        let (sorted, errors) = PackageGraph {
            ids: packages.iter().map(|(id, _)| *id).collect(),
            packages: packages.into_iter().cloned().collect(),
        }
        .to_topological_order();
        let mut actual = sorted
            .iter()
            .map(|id| id.index().to_string())
            .collect::<Vec<String>>()
            .join(", ");
        writeln!(&mut actual, "");
        for error in errors {
            writeln!(&mut actual, "{error}");
        }

        expect.assert_eq(&actual);
    }

    fn new_package(
        id: u32,
        dependencies: Vec<Dependency>,
    ) -> (PackageId, PackageData) {
        (
            PackageId::from_raw(id),
            PackageData {
                root_file_id: FileId::from_raw(id),
                edition: Edition::LATEST,
                display_name: None,
                dependencies,
                cyclic_dependencies: Vec::new(),
                origin: PackageOrigin::Local,
            },
        )
    }

    fn dependency(id: u32) -> Dependency {
        Dependency {
            package_id: PackageId::from_raw(id),
            name: PackageName::new(&id.to_string()).unwrap(),
        }
    }

    #[test]
    fn detect_cyclic_dependency_indirect() {
        check(
            &[
                new_package(0, vec![]),
                new_package(1, vec![]),
                new_package(2, vec![]),
            ],
            expect![[r#"
                0, 1, 2
            "#]],
        );

        check(
            &[
                new_package(0, vec![dependency(1)]),
                new_package(1, vec![]),
                new_package(2, vec![]),
            ],
            expect![[r#"
                1, 0, 2
            "#]],
        );

        check(
            &[
                new_package(0, vec![dependency(1)]),
                new_package(1, vec![dependency(2)]),
                new_package(2, vec![]),
            ],
            expect![[r#"
                2, 1, 0
            "#]],
        );

        check(
            &[
                new_package(0, vec![dependency(1)]),
                new_package(1, vec![dependency(2)]),
                new_package(2, vec![dependency(0)]),
            ],
            expect![[r#"
                2, 1, 0
                Cyclic dependency from Package 1(1) to Package 0(0). Path: Package 1(1) -> Package 2(2) -> Package 0(0) -> Package 1(1)
            "#]],
        );
    }

    #[test]
    fn detect_cyclic_dependency_direct() {
        check(
            &[new_package(2, vec![dependency(5)]), new_package(5, vec![])],
            expect![[r#"
                5, 2
            "#]],
        );
        check(
            &[
                new_package(2, vec![dependency(5)]),
                new_package(5, vec![dependency(2)]),
            ],
            expect![[r#"
                5, 2
                Cyclic dependency from Package 5(5) to Package 2(2). Path: Package 5(5) -> Package 2(2) -> Package 5(5)
            "#]],
        );
    }

    #[test]
    fn detect_cyclic_dependency_self_reference() {
        check(
            &[new_package(2, vec![dependency(2)]), new_package(5, vec![])],
            expect![[r#"
                2, 5
                Cyclic dependency from Package 2(2) to Package 2(2). Path: Package 2(2) -> Package 2(2)
            "#]],
        );
        check(
            &[
                new_package(2, vec![dependency(2)]),
                new_package(5, vec![dependency(2)]),
            ],
            expect![[r#"
                2, 5
                Cyclic dependency from Package 2(2) to Package 2(2). Path: Package 2(2) -> Package 2(2)
            "#]],
        );
    }

    #[test]
    fn detect_cyclic_dependency_empty() {
        check(
            &[],
            expect![[r#"

"#]],
        );
    }
}
