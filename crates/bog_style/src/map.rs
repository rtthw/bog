//! Style map



use bog_alloc::alloc::vec::Vec;

use crate::Style;



pub struct StyleMap {
    nodes: slotmap::SlotMap<slotmap::DefaultKey, NodeInfo>,
    children: slotmap::SlotMap<slotmap::DefaultKey, Vec<u64>>,
    parents: slotmap::SlotMap<slotmap::DefaultKey, Option<u64>>,
}



// --- Private



struct NodeInfo {
    style: Style,
}
