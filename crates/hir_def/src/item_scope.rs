use crate::{
    database::{DefDatabase, ModuleDefinitionId},
    item_tree::Name,
};
use rustc_hash::FxHashMap;
use std::fmt::Write as _;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct ItemScope {
    items: FxHashMap<Name, ModuleDefinitionId>,
    declarations: Vec<ModuleDefinitionId>,
}

impl ItemScope {
    pub(crate) fn dump(
        &self,
        db: &dyn DefDatabase,
        buf: &mut String,
    ) {
        let mut entries: Vec<_> = self.items.iter().collect();
        entries.sort_by_key(|(name, _)| name.clone());

        for (name, def) in entries {
            write!(buf, "{}: v\n", name.as_str());
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
