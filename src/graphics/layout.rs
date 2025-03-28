//! Visual layouts



pub struct Ui {
    tree: taffy::TaffyTree<usize>,
    root: taffy::NodeId,
}

impl Ui {
    pub fn new(layout: Layout) -> Self {
        let mut tree = taffy::TaffyTree::new();
        let root = tree.new_with_children(layout.into(), &[]).unwrap();

        Self {
            tree,
            root,
        }
    }

    pub fn push_to_root(&mut self, layout: Layout, id: usize) {
        let node = self.tree.new_leaf_with_context(layout.into(), id).unwrap(); // Cannot fail.
        self.tree.add_child(self.root, node).unwrap(); // Cannot fail.
    }
}



#[derive(Clone, Debug, Default, PartialEq)]
pub struct Layout(taffy::Style);

impl From<taffy::Style> for Layout {
    fn from(value: taffy::Style) -> Self {
        Self(value)
    }
}

impl Into<taffy::Style> for Layout {
    fn into(self) -> taffy::Style {
        self.0
    }
}
