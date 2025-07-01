//! User interface



slotmap::new_key_type! { pub struct Node; }

pub struct UserInterface {
    elements: slotmap::SlotMap<Node, ElementInfo>,
    children: slotmap::SecondaryMap<Node, Vec<Node>>,
    parents: slotmap::SecondaryMap<Node, Option<Node>>,
}

impl Default for UserInterface {
    fn default() -> Self {
        Self {
            elements: slotmap::SlotMap::with_capacity_and_key(16),
            children: slotmap::SecondaryMap::with_capacity(16),
            parents: slotmap::SecondaryMap::with_capacity(16),
        }
    }
}

impl UserInterface {
    pub fn new(root: Element) -> Self {
        let mut ui = Self::default();

        ui
    }
}

pub struct Element {
    pub style: Style,
    pub children: Vec<Element>,
}



pub struct Style {}



// ---



struct ElementInfo {
    style: Style,
}
