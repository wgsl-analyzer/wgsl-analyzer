use base_db::{EditionedFileId, Package, file_package};
use dot::{Id, LabelText};
use hir_def::{
    FxIndexMap,
    item_tree::Name,
    mod_path::{AbsoluteModPath, ModPath, PathKind},
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
    let modules_to_render = modules_map(database, package);
    let graph = DotModuleGraph::new(database, modules_to_render);

    let mut dot = Vec::new();
    dot::render(&graph, &mut dot).unwrap();
    Some(String::from_utf8(dot).unwrap())
}

fn modules_map(
    database: &RootDatabase,
    package: Package,
) -> FxHashMap<AbsoluteModPath, ModuleData> {
    let package_data = package.data(database);
    let source_root = package_data.source_root(database);
    let modules: Vec<_> = source_root
        .iter()
        .filter_map(|file_id| {
            let (name, extension) = source_root.path_for_file(file_id)?.name_and_extension()?;
            let file_id = EditionedFileId::try_with_extension(database, file_id, extension?)?;
            let mod_path = AbsoluteModPath::for_file(database, package, file_id)?;
            Some(ModuleData {
                file_id: Some(file_id),
                mod_path,
            })
        })
        .collect();

    // Invariant: Given a ModPath, the parent ModPath exists in the folders
    let mut folders = FxHashMap::default();
    folders.insert(
        AbsoluteModPath::new_root(),
        ModuleData {
            file_id: None,
            mod_path: AbsoluteModPath::new_root(),
        },
    );

    for module in modules {
        if folders.contains_key(&module.mod_path) {
            continue;
        }
        let mut module_path = module.mod_path.clone();
        folders.insert(module.mod_path.clone(), module);

        while let Some(_) = module_path.pop_segment()
            && !folders.contains_key(&module_path)
        {
            folders.insert(
                module_path.clone(),
                ModuleData {
                    file_id: None,
                    mod_path: module_path.clone(),
                },
            );
        }
    }

    folders
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
        modules_map: FxHashMap<AbsoluteModPath, ModuleData>,
    ) -> Self {
        let modules: Vec<_> = modules_map
            .into_values()
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
