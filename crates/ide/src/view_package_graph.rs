use base_db::input::{Dependency, PackageData, PackageId};
use dot::{Id, LabelText};
use ide_db::base_db::all_packages;
use ide_db::base_db::salsa::plumbing::AsId;
use ide_db::{
    FxHashMap, RootDatabase,
    base_db::{ExtraPackageData, Package, SourceDatabase},
};

/// Feature: View Package Graph
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
) -> Result<String, String> {
    let all_packages = all_packages(database);
    let packages_to_render: FxHashMap<PackageId, (&PackageData, &())> = all_packages
        .iter()
        .copied()
        .map(|package| (package.package_id(database), (package.data(database), &())))
        .filter(|(_, (package_data, _))| {
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
    Ok(String::from_utf8(dot).unwrap())
}

struct DotPackageGraph<'db> {
    packages_to_render: FxHashMap<PackageId, (&'db PackageData, &'db ())>,
}

type Edge<'a> = (PackageId, &'a Dependency);

impl<'a> dot::GraphWalk<'a, PackageId, Edge<'a>> for DotPackageGraph<'_> {
    fn nodes(&'a self) -> dot::Nodes<'a, PackageId> {
        self.packages_to_render.keys().copied().collect()
    }

    fn edges(&'a self) -> dot::Edges<'a, Edge<'a>> {
        self.packages_to_render
            .iter()
            .flat_map(|(package, (package_data, _))| {
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
        &'a self,
        edge: &Edge<'a>,
    ) -> PackageId {
        edge.0
    }

    fn target(
        &'a self,
        edge: &Edge<'a>,
    ) -> PackageId {
        edge.1.package_id
    }
}

impl<'a> dot::Labeller<'a, PackageId, Edge<'a>> for DotPackageGraph<'_> // spellchecker:disable-line
{
    fn graph_id(&'a self) -> Id<'a> {
        Id::new("wgsl_analyzer_package_graph").unwrap()
    }

    fn node_id(
        &'a self,
        n: &PackageId,
    ) -> Id<'a> {
        let id = n.index();
        Id::new(format!("_{id:?}")).unwrap()
    }

    fn node_shape(
        &'a self,
        _node: &PackageId,
    ) -> Option<LabelText<'a>> {
        Some(LabelText::LabelStr("box".into()))
    }

    fn node_label(
        &'a self,
        n: &PackageId,
    ) -> LabelText<'a> {
        let name = self.packages_to_render[n]
            .0
            .display_name
            .as_ref()
            .map_or("(unnamed package)", |name| name.as_str());
        LabelText::LabelStr(name.into())
    }
}
