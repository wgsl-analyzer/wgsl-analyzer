use base_db::input::{Dependency, PackageData, PackageId};
use dot::{Id, LabelText};
use ide_db::base_db::all_packages;
use ide_db::base_db::salsa::plumbing::AsId;
use ide_db::{
    FxHashMap, RootDatabase,
    base_db::{ExtraPackageData, Package, SourceDatabase as _},
};

/// # Feature: View Package Graph
///
/// Renders the currently loaded package graph as an SVG graphic.
/// Requires the `dot` tool, which is part of graphviz, to be installed.
///
/// Only workspace packages are included, no packages.io dependencies or sysroot packages.
///
/// | Editor  | Action Name |
/// |---------|-------------|
/// | VS Code | **wgsl-analyzer: View Package Graph** |
pub(crate) fn view_package_graph(
    database: &RootDatabase,
    full: bool,
) -> String {
    let all_packages = all_packages(database);
    let packages_to_render: FxHashMap<PackageId, (&PackageData, &())> = all_packages
        .iter()
        .copied()
        .map(|package| (package.package_id(database), (package.data(database), &())))
        .filter(|(_, (package_data, ()))| {
            if full {
                true
            } else {
                // Only render workspace packages
                let root_id = database
                    .file_source_root(package_data.root_file_id)
                    .source_root_id(database);
                !database
                    .source_root(root_id)
                    .source_root(database)
                    .is_library()
            }
        })
        .collect();
    let graph = DotPackageGraph { packages_to_render };

    let mut dot = Vec::new();
    dot::render(&graph, &mut dot).unwrap();
    String::from_utf8(dot).unwrap()
}

struct DotPackageGraph<'db> {
    packages_to_render: FxHashMap<PackageId, (&'db PackageData, &'db ())>,
}

type Edge<'edge> = (PackageId, &'edge Dependency);

impl<'edge> dot::GraphWalk<'edge, PackageId, Edge<'edge>> for DotPackageGraph<'_> {
    fn nodes(&'edge self) -> dot::Nodes<'edge, PackageId> {
        self.packages_to_render.keys().copied().collect()
    }

    fn edges(&'edge self) -> dot::Edges<'edge, Edge<'edge>> {
        self.packages_to_render
            .iter()
            .flat_map(|(package, (package_data, ()))| {
                package_data
                    .dependencies
                    .iter()
                    .filter(|dependency| {
                        self.packages_to_render.contains_key(&dependency.package_id)
                    })
                    .map(move |dependency| (*package, dependency))
            })
            .collect()
    }

    fn source(
        &'edge self,
        edge: &Edge<'edge>,
    ) -> PackageId {
        edge.0
    }

    fn target(
        &'edge self,
        edge: &Edge<'edge>,
    ) -> PackageId {
        edge.1.package_id
    }
}

impl<'edge> dot::Labeller<'edge, PackageId, Edge<'edge>> for DotPackageGraph<'_> {
    fn graph_id(&'edge self) -> Id<'edge> {
        Id::new("wgsl_analyzer_package_graph").unwrap()
    }

    fn node_id(
        &'edge self,
        n: &PackageId,
    ) -> Id<'edge> {
        let id = n.index();
        Id::new(format!("_{id:?}")).unwrap()
    }

    fn node_shape(
        &'edge self,
        _node: &PackageId,
    ) -> Option<LabelText<'edge>> {
        Some(LabelText::LabelStr("box".into()))
    }

    fn node_label(
        &'edge self,
        n: &PackageId,
    ) -> LabelText<'edge> {
        let name = self.packages_to_render[n]
            .0
            .display_name
            .as_ref()
            .map_or("(unnamed package)", |name| name.as_str());
        LabelText::LabelStr(name.into())
    }
}
