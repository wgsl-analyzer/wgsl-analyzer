//! The core of the module-level name resolution algorithm.

use crate::{HirFileId, database::DefDatabase, nameres::DefMap};

pub(super) fn collect_defs(
    db: &dyn DefDatabase,
    def_map: DefMap,
    tree_id: HirFileId,
) -> DefMap {
    let item_tree = db.item_tree(tree_id);
    todo!()
}
