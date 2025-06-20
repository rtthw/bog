//! Styling



use crate::Element;



pub struct Style {
    inner: taffy::Style,
    cache: taffy::Cache,
    layout: taffy::Layout,
}

// Builder.
impl Style {
    pub fn new() -> Self {
        Self {
            inner: taffy::Style::DEFAULT,
            cache: taffy::Cache::new(),
            layout: taffy::Layout::new(), // TODO: Ordered?
        }
    }
}



// --- Internal



struct Node<'e> {
    element: &'e mut Box<dyn Element>,
}

struct ChildIter(core::ops::Range<usize>);

impl Iterator for ChildIter {
    type Item = taffy::NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(taffy::NodeId::from)
    }
}

impl<'e> taffy::TraversePartialTree for Node<'e> {
    type ChildIter<'a> = ChildIter where Self: 'a;

    fn child_ids(&self, _parent_node_id: taffy::NodeId) -> Self::ChildIter<'_> {
        ChildIter(0..self.element.num_children())
    }

    fn child_count(&self, _parent_node_id: taffy::NodeId) -> usize {
        self.element.num_children()
    }

    fn get_child_id(&self, _parent_node_id: taffy::NodeId, child_index: usize) -> taffy::NodeId {
        taffy::NodeId::from(child_index)
    }
}
