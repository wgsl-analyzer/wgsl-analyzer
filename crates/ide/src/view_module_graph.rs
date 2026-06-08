use std::borrow::Cow;

use base_db::{EditionedFileId, file_package};
use dot::{Id, LabelText};
use hir_def::{
    FxIndexMap,
    item_tree::Name,
    name_resolution::{ModuleData, modules_map_query},
};
use ide_db::RootDatabase;
use vfs::FileId;

/// # Feature: View Module Graph
///
/// Renders the currently loaded module graph as an SVG graphic.
/// Requires the `dot` tool, which is part of graphviz, to be installed.
///
/// Only renders a detailed graph for modules in the current package.
/// Only workspace packages are included, no packages.io dependencies or sysroot packages.
///
/// | Editor  | Action Name |
/// |---------|-------------|
/// | VS Code | **wgsl-analyzer: View Module Graph** |
pub(crate) fn view_module_graph(
    database: &RootDatabase,
    file_id: FileId,
) -> String {
    // TODO: This only renders the children. It should render an edge for each import and inline usage of another module.
    let package = file_package(database, file_id);
    let modules_to_render = if let Some(package) = package {
        Cow::Borrowed(&modules_map_query(database, package).modules)
    } else {
        let mut modules_to_render = FxIndexMap::default();
        modules_to_render.insert(
            file_id,
            ModuleData {
                name: Some(Name::from("[standalone file]")),
                origin: EditionedFileId::from_file(database, file_id),
                parent: None,
                children: FxIndexMap::default(),
            },
        );
        Cow::Owned(modules_to_render)
    };

    let graph = DotModuleGraph { modules_to_render };

    let mut dot = Vec::new();
    dot::render(&graph, &mut dot).unwrap();
    String::from_utf8(dot).unwrap()
}

struct DotModuleGraph<'db> {
    modules_to_render: Cow<'db, FxIndexMap<FileId, ModuleData>>,
}

type Edge<'edge> = (FileId, FileId);

impl<'edge> dot::GraphWalk<'edge, FileId, Edge<'edge>> for DotModuleGraph<'_> {
    fn nodes(&'edge self) -> dot::Nodes<'edge, FileId> {
        let modules: FxIndexMap<_, _> = self
            .modules_to_render
            .clone()
            .into_owned()
            .sorted_by(|file_a, module_a, file_b, module_b| {
                if let Some(name) = &module_a.name {
                    module_a.name.cmp(&module_b.name)
                } else {
                    file_a.cmp(file_b)
                }
            })
            .collect();
        modules.keys().copied().collect()
    }

    fn edges(&'edge self) -> dot::Edges<'edge, Edge<'edge>> {
        self.modules_to_render
            .iter()
            .flat_map(|(package, module_data)| {
                module_data
                    .children
                    .values()
                    .copied()
                    .filter(|dependency| self.modules_to_render.contains_key(dependency))
                    .map(move |dependency| (*package, dependency))
            })
            .collect()
    }

    fn source(
        &'edge self,
        edge: &Edge<'edge>,
    ) -> FileId {
        edge.0
    }

    fn target(
        &'edge self,
        edge: &Edge<'edge>,
    ) -> FileId {
        edge.1
    }
}

impl<'edge> dot::Labeller<'edge, FileId, Edge<'edge>> for DotModuleGraph<'_> {
    fn graph_id(&'edge self) -> Id<'edge> {
        Id::new("wgsl_analyzer_module_graph").unwrap()
    }

    fn node_id(
        &'edge self,
        n: &FileId,
    ) -> Id<'edge> {
        let id = n.index();
        Id::new(format!("_{id:?}")).unwrap()
    }

    fn node_shape(
        &'edge self,
        _node: &FileId,
    ) -> Option<LabelText<'edge>> {
        Some(LabelText::LabelStr("box".into()))
    }

    fn node_label(
        &'edge self,
        n: &FileId,
    ) -> LabelText<'edge> {
        let name = self.modules_to_render[n]
            .name
            .as_ref()
            .map_or("package", |name| name.as_str());
        LabelText::LabelStr(name.into())
    }
}
