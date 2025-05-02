//! Bog Layout

// TODO: #![no_std]



mod layout;
mod tree;

use bog_math::{vec2, Rect, Vec2};
use taffy::TraversePartialTree;

pub use layout::Layout;



pub type Node = u64;

pub struct LayoutTree<T> {
    tree: taffy::TaffyTree<T>,
    root: Node,
    available_space: Vec2,
}

impl<T> LayoutTree<T> {
    pub fn new(root_layout: Layout) -> Self {
        let mut tree = taffy::TaffyTree::new();
        let root = tree.new_with_children(root_layout.into(), &[])
            .unwrap(); // Cannot fail.

        Self {
            tree,
            root: root.into(),
            available_space: vec2(0.0, 0.0),
        }
    }

    pub fn root_node(&self) -> Node {
        self.root
    }

    pub fn children_of(&self, node: Node) -> Vec<Node> {
        self.tree.child_ids(node.into()).map(|n| n.into()).collect()
    }

    // FIXME: This can be optimized.
    pub fn placement(&self, node: Node) -> Option<Placement> {
        let mut found = None;
        self.iter_placements(&mut |n, placement| {
            if n == node {
                found = Some(placement.clone());
            }
        });

        found
    }

    // TODO: Make the function return a bool to indicate if this should continue iterating.
    pub fn iter_placements(&self, func: &mut impl FnMut(Node, &Placement)) {
        for_each_node(&self.tree, self.root, func);
    }

    pub fn node_context(&self, node: Node) -> &T {
        self.tree.get_node_context(node.into()).unwrap()
    }

    pub fn node_context_mut(&mut self, node: Node) -> &mut T {
        self.tree.get_node_context_mut(node.into()).unwrap()
    }

    pub fn push(&mut self, layout: Layout, parent: Node, context: T) -> Node {
        let id = self.tree.new_leaf_with_context(layout.into(), context)
            .unwrap(); // Cannot fail.
        self.tree.add_child(parent.into(), id)
            .unwrap(); // Cannot fail.

        id.into()
    }

    pub fn push_to_root(&mut self, layout: Layout, context: T) -> Node {
        let id = self.tree.new_leaf_with_context(layout.into(), context)
            .unwrap(); // Cannot fail.
        self.tree.add_child(self.root.into(), id)
            .unwrap(); // Cannot fail.

        id.into()
    }

    pub fn set_node_layout(&mut self, node: Node, layout: Layout) {
        self.tree.set_style(node.into(), layout.0).unwrap();
    }

    pub fn get_node_layout(&mut self, node: Node) -> Layout {
        Layout(self.tree.style(node.into()).unwrap().clone())
    }

    /// # Panics
    ///
    /// - If either node is the root node.
    // FIXME: This can probably be optimized.
    pub fn try_swap_nodes(&mut self, node_a: Node, node_b: Node) -> bool {
        if node_a == node_b {
            return false;
        }

        let node_a_parent = self.tree.parent(node_a.into()).unwrap();
        let node_b_parent = self.tree.parent(node_b.into()).unwrap();

        if node_a_parent == node_b_parent {
            let mut children = self.tree.children(node_a_parent).unwrap();
            let index_of = |children: &Vec<taffy::NodeId>, node: taffy::NodeId| -> usize {
                children.iter()
                    .enumerate()
                    .find(|(_i, n)| *n == &node)
                    .unwrap()
                    .0
            };
            let index_a = index_of(&children, node_a.into());
            let index_b = index_of(&children, node_b.into());
            children.swap(index_a, index_b);
            self.tree.set_children(node_a_parent, &children).unwrap();

            true
        } else {
            println!("[TODO]: Support swapping nodes with different parents");

            false
        }
    }

    pub fn resize(&mut self, available_space: Vec2) {
        self.available_space = available_space;
    }

    pub fn do_layout(&mut self, handler: &mut impl LayoutHandler) {
        self.tree
            .compute_layout(
                self.root.into(),
                taffy::Size {
                    width: taffy::AvailableSpace::Definite(self.available_space.x),
                    height: taffy::AvailableSpace::Definite(self.available_space.y),
                },
            )
            .unwrap(); // Cannot fail.

        for_each_node(&self.tree, self.root, &mut |node, placement| {
            handler.on_layout(node.into(), placement);
        });
    }
}

fn for_each_node<F, T>(tree: &taffy::TaffyTree<T>, node: Node, func: &mut F)
where F: FnMut(Node, &Placement),
{
    let top_layout = tree.layout(node.into()).unwrap();
    let top_pos = Vec2::new(top_layout.location.x, top_layout.location.y);

    for child in tree.children(node.into()).unwrap().into_iter() {
        let placement = Placement {
            parent_pos: top_pos,
            layout: *tree.layout(child).unwrap(),
        };
        func(child.into(), &placement);
        for_each_node_inner(tree, child.into(), &placement, func);
    }
}

fn for_each_node_inner<F, T>(
    tree: &taffy::TaffyTree<T>,
    node: Node,
    placement: &Placement,
    func: &mut F,
)
where F: FnMut(Node, &Placement),
{
    for child in tree.children(node.into()).unwrap().into_iter() {
        let layout = *tree.layout(child).unwrap();
        let child_placement = Placement {
            parent_pos: placement.position(),
            layout,
        };
        func(child.into(), &child_placement);
        for_each_node_inner(tree, child.into(), &child_placement, func);
    }
}



pub trait LayoutHandler {
    fn on_layout(&mut self, node: Node, placement: &Placement);
}



#[derive(Clone)]
pub struct Placement {
    parent_pos: Vec2,
    pub layout: taffy::Layout,
}

impl Placement {
    pub fn position(&self) -> Vec2 {
        Vec2::new(
            self.parent_pos.x + self.layout.location.x,
            self.parent_pos.y + self.layout.location.y,
        )
    }

    pub fn size(&self) -> Vec2 {
        Vec2::new(self.layout.size.width, self.layout.size.height)
    }

    pub fn rect(&self) -> Rect {
        Rect::new(self.position(), self.size())
    }

    pub fn content_position(&self) -> Vec2 {
        Vec2::new(
            self.parent_pos.x + self.layout.content_box_x(),
            self.parent_pos.y + self.layout.content_box_y(),
        )
    }

    pub fn content_size(&self) -> Vec2 {
        Vec2::new(self.layout.content_box_width(), self.layout.content_box_height())
    }

    pub fn content_rect(&self) -> Rect {
        Rect::new(self.content_position(), self.content_size())
    }
}
