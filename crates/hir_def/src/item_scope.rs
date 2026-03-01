use crate::{
    database::{DefDatabase, ModuleDefinitionId},
    item_tree::Name,
};
use rustc_hash::FxHashMap;
use std::fmt::Write as _;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct ItemScope {
    /// Items visible in this scope. Includes both declarations and imports.
    items: FxHashMap<Name, ModuleDefinitionId>,
    declarations: Vec<ModuleDefinitionId>,
}

impl ItemScope {
    pub(crate) fn declare(
        &mut self,
        definition: ModuleDefinitionId,
    ) {
        self.declarations.push(definition);
    }
    pub(crate) fn push_item(
        &mut self,
        name: Name,
        definition: ModuleDefinitionId,
    ) {
        // TODO: Check if item is already present
        self.items.insert(name, definition);
    }

    /// Get a name from current module scope.
    #[must_use]
    pub fn get(
        &self,
        name: &Name,
    ) -> Option<ModuleDefinitionId> {
        self.items.get(name).copied()
    }

    pub(crate) fn dump(
        &self,
        buffer: &mut String,
    ) {
        let mut entries: Vec<_> = self.items.iter().collect();
        entries.sort_by_key(|(name, _)| *name);

        for (name, _) in entries {
            writeln!(buffer, "{}: v", name.as_str());
        }
    }

    pub(crate) fn shrink_to_fit(&mut self) {
        // Exhaustive match to require handling new fields.
        let Self {
            items,
            declarations,
        } = self;
        items.shrink_to_fit();
        declarations.shrink_to_fit();
    }
}
