use base_db::{EditionedFileId, Package, file_package};
use dot::{Id, LabelText};
use hir_def::{
    FxIndexMap,
    item_tree::Name,
    mod_path::{AbsoluteModPath, ModPath, PathKind},
    name_resolution::ModulesMap,
};
use ide_db::{FxHashMap, RootDatabase};
use itertools::Itertools as _;
use triomphe::Arc;
use vfs::FileId;

/// # Feature: View Module Graph
///
/// Renders the currently loaded module graph as an SVG graphic.
/// Requires the `dot` tool, which is part of graphviz, to be installed.
///
/// Only renders a detailed graph for modules in the current package.
///
/// | Editor  | Action Name |
/// |---------|-------------|
/// | VS Code | **wgsl-analyzer: View Module Graph** |
pub(crate) fn view_module_graph(
    database: &RootDatabase,
    file_id: FileId,
) -> Option<String> {
    // TODO: This only renders the children. It should render an edge for each import and inline usage of another module.
    let package = file_package(database, file_id)?;
    let modules_to_render = ModulesMap::of(database, package);
    let graph = DotModuleGraph::new(database, modules_to_render);

    let mut dot = Vec::new();
    dot::render(&graph, &mut dot).unwrap();
    Some(String::from_utf8(dot).unwrap())
}

struct ModuleData {
    file_id: Option<EditionedFileId>,
    mod_path: AbsoluteModPath,
}
impl ModuleData {
    fn name(&self) -> Option<&Name> {
        self.mod_path.segments().first()
    }
}

struct DotModuleGraph<'db> {
    database: &'db RootDatabase,
    modules: Vec<ModuleData>,
}

impl<'db> DotModuleGraph<'db> {
    fn new(
        database: &'db RootDatabase,
        modules_map: &ModulesMap,
    ) -> Self {
        let modules: Vec<_> = modules_map
            .modules
            .iter()
            .map(|(mod_path, data)| ModuleData {
                file_id: data.file,
                mod_path: mod_path.clone(),
            })
            .sorted_by(|module_a, module_b| module_a.mod_path.cmp(&module_b.mod_path))
            .collect();

        Self { database, modules }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct ModKey(usize);

type Edge<'edge> = (ModKey, ModKey);

impl<'edge> dot::GraphWalk<'edge, ModKey, Edge<'edge>> for DotModuleGraph<'_> {
    fn nodes(&'edge self) -> dot::Nodes<'edge, ModKey> {
        (0..self.modules.len()).map(ModKey).collect()
    }

    fn edges(&'edge self) -> dot::Edges<'edge, Edge<'edge>> {
        let module_ids: FxHashMap<_, _> = self
            .modules
            .iter()
            .enumerate()
            .map(|(id, module)| (&module.mod_path, ModKey(id)))
            .collect();

        self.modules
            .iter()
            .filter_map(|module| {
                let mut parent_path = module.mod_path.clone();
                parent_path.pop_segment()?;

                Some((module_ids[&module.mod_path], module_ids[&parent_path]))
            })
            .collect()
    }

    fn source(
        &'edge self,
        edge: &Edge<'edge>,
    ) -> ModKey {
        edge.0
    }

    fn target(
        &'edge self,
        edge: &Edge<'edge>,
    ) -> ModKey {
        edge.1
    }
}

impl<'edge> dot::Labeller<'edge, ModKey, Edge<'edge>> for DotModuleGraph<'_> {
    fn graph_id(&'edge self) -> Id<'edge> {
        Id::new("wgsl_analyzer_module_graph").unwrap()
    }

    fn node_id(
        &'edge self,
        n: &ModKey,
    ) -> Id<'edge> {
        Id::new(format!("_{}", n.0)).unwrap()
    }

    fn node_shape(
        &'edge self,
        _node: &ModKey,
    ) -> Option<LabelText<'edge>> {
        Some(LabelText::LabelStr("box".into()))
    }

    fn node_style(
        &'edge self,
        node: &ModKey,
    ) -> dot::Style {
        let has_file = self.modules[node.0].file_id.is_some();
        if has_file {
            dot::Style::None
        } else {
            dot::Style::Dashed
        }
    }

    fn node_label(
        &'edge self,
        n: &ModKey,
    ) -> LabelText<'edge> {
        let name = self.modules[n.0]
            .name()
            .map_or("[package]", |name| name.as_str());
        LabelText::LabelStr(name.into())
    }
}
