//! This module implements import-resolution.
//!
//! The result of this module is `DefMap`: a data structure which contains:
//!
//!   * a tree of modules for the crate
//!   * for each module, a set of items visible in the module (directly declared
//!     or imported)
//!
//! Computing `DefMap` can be partitioned into several logically
//! independent "phases". The phases are mutually recursive though, there's no
//! strict ordering.
//!
//! ## Collecting RawItems
//!
//! This happens in the `raw` module, which parses a single source file into a
//! set of top-level items. Nested imports are desugared to flat imports in this
//! phase.
//!
//! ## Collecting Modules
//!
//! This happens in the `collector` module. In this phase, we recursively walk
//! tree of modules, collect items from submodules, populate module scopes
//! with defined items (so, we assign item ids in this phase) and record the set
//! of unresolved imports.
//!
//! ## Resolving Imports
//!
//! We maintain a list of currently unresolved imports. On every iteration, we
//! try to resolve some imports from this list. If the import is resolved, we
//! record it, by adding an item to current module scope and, if necessary, by
//! recursively populating glob imports.
//!
//! TODO: ^ check if the comments there make sense

mod collector;
mod diagnostics;
mod path_resolution;
#[cfg(test)]
mod tests;

use std::fmt::Write as _;
use std::ops::{Deref, DerefMut, Index, IndexMut};

use base_db::{EditionedFileId, FileId, PackageId};
use either::Either;
use rowan::TextRange;
use rustc_hash::{FxHashMap, FxHashSet};
use syntax::{AstNode, Edition, SyntaxNode, ast};
use triomphe::Arc;

use crate::database::{DefDatabase, ModuleDefinitionId};
use crate::item_scope::ItemScope;
use crate::item_tree::Name;

use crate::mod_path::ModPath;
use crate::nameres::diagnostics::DefDiagnostic;
use crate::{FileAstId, FxIndexMap, HirFileId, InFile};

/// Contains the results of (early) name resolution.
///
/// A `DefMap` stores the module tree and the definitions that are in scope in every module after
/// item-level macros have been expanded.
///
/// Every crate has a primary `DefMap` whose root is the crate's main file (`main.rs`/`lib.rs`),
/// computed by the `crate_def_map` query.
#[derive(Debug, PartialEq, Eq)]
pub struct DefMap {
    /// The package this `DefMap` belongs to.
    package: PackageId,
    pub root: FileId,
    /// The modules and their data declared in this crate.
    pub modules: ModulesMap,

    /// The diagnostics that need to be emitted for this crate.
    diagnostics: Vec<DefDiagnostic>,

    /// The crate data that is shared between a crate's def map and all its block def maps.
    data: Arc<DefMapCrateData>,
}

/// Data that belongs to a crate which is shared between a crate's def map and all its block def maps.
#[derive(Clone, Debug, PartialEq, Eq)]
struct DefMapCrateData {
    edition: Edition,
    recursion_limit: Option<u32>,
}

impl DefMapCrateData {
    fn new(edition: Edition) -> Self {
        Self {
            edition,
            recursion_limit: None,
        }
    }
}

impl std::ops::Index<FileId> for DefMap {
    type Output = ModuleData;

    fn index(
        &self,
        id: FileId,
    ) -> &ModuleData {
        self.modules
            .get(&id)
            .unwrap_or_else(|| panic!("FileId not found in ModulesMap {:#?}: {id:#?}", self.root))
    }
}

impl std::ops::IndexMut<FileId> for DefMap {
    fn index_mut(
        &mut self,
        id: FileId,
    ) -> &mut ModuleData {
        &mut self.modules[id]
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ModuleData {
    /// Where does this module come from?
    pub origin: EditionedFileId,
    /// Declared visibility of this module.
    // pub visibility: Visibility,
    /// Parent module in the same `DefMap`.
    ///
    /// [`None`] for block modules because they are always its `DefMap`'s root.
    pub parent: Option<FileId>,
    pub children: FxIndexMap<Name, FileId>,
    pub scope: ItemScope,
}

impl DefMap {
    pub fn edition(&self) -> Edition {
        self.data.edition
    }

    pub(crate) fn package_def_map_query(
        database: &dyn DefDatabase,
        package_id: PackageId,
    ) -> Arc<DefMap> {
        let package_graph = database.package_graph();

        let edition = package_graph[package_id].edition;
        let origin = EditionedFileId {
            file_id: package_graph[package_id].root_file_id,
            edition,
        };
        let def_map = DefMap::empty(
            package_id,
            edition,
            Arc::new(DefMapCrateData::new(edition)),
            ModuleData::new(origin),
        );
        let def_map = collector::collect_defs(database, def_map, origin.into());

        Arc::new(def_map)
    }

    fn empty(
        package_id: PackageId,
        edition: Edition,
        crate_data: Arc<DefMapCrateData>,
        module_data: ModuleData,
    ) -> DefMap {
        let mut modules = ModulesMap::new();
        let root = module_data.origin.file_id;
        modules.insert(root, module_data);

        DefMap {
            package: package_id,
            root,
            modules,
            diagnostics: Vec::new(),
            data: crate_data,
        }
    }
    fn shrink_to_fit(&mut self) {
        // Exhaustive match to require handling new fields.
        let Self {
            diagnostics,
            modules,
            root: _,
            package: _,
            data: _,
        } = self;

        diagnostics.shrink_to_fit();
        modules.shrink_to_fit();
        for (_, module) in modules.iter_mut() {
            module.children.shrink_to_fit();
            module.scope.shrink_to_fit();
        }
    }
}

impl DefMap {
    pub fn modules(&self) -> impl Iterator<Item = (FileId, &ModuleData)> + '_ {
        self.modules.iter()
    }

    pub fn package(&self) -> PackageId {
        self.package
    }

    #[inline]
    pub fn crate_root(&self) -> FileId {
        self.root
    }

    /// Returns the module containing `local_mod`, either the parent `mod`, or the module (or block) containing
    /// the block, if `self` corresponds to a block expression.
    pub fn containing_module(
        &self,
        local_mod: FileId,
    ) -> Option<FileId> {
        self[local_mod].parent
    }

    /// Get a reference to the def map's diagnostics.
    pub fn diagnostics(&self) -> &[DefDiagnostic] {
        self.diagnostics.as_slice()
    }

    pub fn recursion_limit(&self) -> u32 {
        // 128 is the default in rustc
        self.data.recursion_limit.unwrap_or(128)
    }

    // FIXME: this can use some more human-readable format (ideally, an IR
    // even), as this should be a great debugging aid.
    pub fn dump(
        &self,
        db: &dyn DefDatabase,
    ) -> String {
        let mut buf = String::new();
        let mut current_map = self;
        go(&mut buf, db, current_map, "crate", current_map.root);
        return buf;

        fn go(
            buf: &mut String,
            db: &dyn DefDatabase,
            map: &DefMap,
            path: &str,
            module: FileId,
        ) {
            write!(buf, "{}\n", path);

            map[module].scope.dump(db, buf);

            let mut child_modules = map[module].children.iter().collect::<Vec<_>>();
            child_modules.sort_by(|a, b| Ord::cmp(&a.0, &b.0));
            for (name, child) in child_modules {
                let path = format!("{path}::{}", name.as_str());
                buf.push('\n');
                go(buf, db, map, &path, *child);
            }
        }
    }
}

impl DefMap {
    pub(crate) fn resolve_path(
        &self,
        db: &dyn DefDatabase,
        original_module: FileId,
        path: &ModPath,
    ) -> (ModuleDefinitionId, Option<usize>) {
        let result = self.resolve_path_fp_with_macro(db, original_module, path);
        (result.resolved_def, result.segment_index)
    }
}

impl ModuleData {
    pub(crate) fn new(origin: EditionedFileId) -> Self {
        ModuleData {
            origin,
            parent: None,
            children: Default::default(),
            scope: ItemScope::default(),
        }
    }

    /// Same as [`ModuleData::definition_source`] but only returns the file id to prevent parsing the AST.
    pub fn definition_source_file_id(&self) -> HirFileId {
        self.origin.into()
    }
}

/// A newtype wrapper around `FxHashMap<FileId, ModuleData>` that implements `IndexMut`.
#[derive(Debug, PartialEq, Eq)]
pub struct ModulesMap {
    inner: FxIndexMap<FileId, ModuleData>,
}

impl ModulesMap {
    fn new() -> Self {
        Self {
            inner: FxIndexMap::default(),
        }
    }

    fn iter(&self) -> impl Iterator<Item = (FileId, &ModuleData)> + '_ {
        self.inner.iter().map(|(&k, v)| (k, v))
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = (FileId, &mut ModuleData)> + '_ {
        self.inner.iter_mut().map(|(&k, v)| (k, v))
    }
}

impl Deref for ModulesMap {
    type Target = FxIndexMap<FileId, ModuleData>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for ModulesMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Index<FileId> for ModulesMap {
    type Output = ModuleData;

    fn index(
        &self,
        id: FileId,
    ) -> &ModuleData {
        self.inner
            .get(&id)
            .unwrap_or_else(|| panic!("FileId not found in ModulesMap: {id:#?}"))
    }
}

impl IndexMut<FileId> for ModulesMap {
    fn index_mut(
        &mut self,
        id: FileId,
    ) -> &mut ModuleData {
        self.inner
            .get_mut(&id)
            .unwrap_or_else(|| panic!("FileId not found in ModulesMap: {id:#?}"))
    }
}
