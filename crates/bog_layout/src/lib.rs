//! Bog Layout

// TODO: #![no_std]



use bog_math::{vec2, Vec2};
use taffy::TraversePartialTree;



pub type Node = u64;

pub struct LayoutTree {
    tree: taffy::TaffyTree<bool>,
    root: Node,
    available_space: Vec2,
}

impl LayoutTree {
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

    // TODO: Node context should be generic.
    pub fn is_interactable(&self, node: Node) -> bool {
        *self.tree.get_node_context(node.into()).unwrap()
    }

    pub fn push(&mut self, layout: Layout, parent: Node, interactable: bool) -> Node {
        let id = self.tree.new_leaf_with_context(layout.into(), interactable)
            .unwrap(); // Cannot fail.
        self.tree.add_child(parent.into(), id)
            .unwrap(); // Cannot fail.

        id.into()
    }

    pub fn push_to_root(&mut self, layout: Layout, interactable: bool) -> Node {
        let id = self.tree.new_leaf_with_context(layout.into(), interactable)
            .unwrap(); // Cannot fail.
        self.tree.add_child(self.root.into(), id)
            .unwrap(); // Cannot fail.

        id.into()
    }

    /// # Panics
    ///
    /// - If either node is the root node.
    // FIXME: This can probably be optimized.
    pub fn try_swap_nodes(&mut self, node_a: Node, node_b: Node) {
        if node_a == node_b {
            return;
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
        } else {
            // Don't panic here in case the user makes a mistake.
            println!("[TODO]: Support swapping nodes with different parents");
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

fn for_each_node<F>(tree: &taffy::TaffyTree<bool>, node: Node, func: &mut F)
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

fn for_each_node_inner<F>(
    tree: &taffy::TaffyTree<bool>,
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

    pub fn content_position(&self) -> Vec2 {
        Vec2::new(
            self.parent_pos.x + self.layout.content_box_x(),
            self.parent_pos.y + self.layout.content_box_y(),
        )
    }

    pub fn content_size(&self) -> Vec2 {
        Vec2::new(self.layout.content_box_width(), self.layout.content_box_height())
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

// Sizing.
impl Layout {
    pub fn width(mut self, width: f32) -> Self {
        self.0.size.width = taffy::prelude::length(width);
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.0.size.height = taffy::prelude::length(height);
        self
    }

    pub fn width_percent(mut self, width: f32) -> Self {
        self.0.size.width = taffy::prelude::percent(width);
        self
    }

    pub fn height_percent(mut self, height: f32) -> Self {
        self.0.size.height = taffy::prelude::percent(height);
        self
    }

    pub fn fill_width(mut self) -> Self {
        self.0.size.width = taffy::prelude::percent(1.0);
        self
    }

    pub fn fill_height(mut self) -> Self {
        self.0.size.height = taffy::prelude::percent(1.0);
        self
    }
}

// Margin, spacing, and padding.
impl Layout {
    pub fn gap_x(mut self, width: f32) -> Self {
        self.0.gap.width = taffy::LengthPercentage::Length(width);
        self
    }

    pub fn gap_y(mut self, height: f32) -> Self {
        self.0.gap.height = taffy::LengthPercentage::Length(height);
        self
    }

    pub fn padding(mut self, amount: f32) -> Self {
        self.0.padding.left = taffy::LengthPercentage::Length(amount);
        self.0.padding.right = taffy::LengthPercentage::Length(amount);
        self.0.padding.top = taffy::LengthPercentage::Length(amount);
        self.0.padding.bottom = taffy::LengthPercentage::Length(amount);
        self
    }

    pub fn margin(mut self, amount: f32) -> Self {
        self.0.margin.left = taffy::LengthPercentageAuto::Length(amount);
        self.0.margin.right = taffy::LengthPercentageAuto::Length(amount);
        self.0.margin.top = taffy::LengthPercentageAuto::Length(amount);
        self.0.margin.bottom = taffy::LengthPercentageAuto::Length(amount);
        self
    }
}

// Display attribute.
impl Layout {
    pub fn display_none(mut self) -> Self {
        self.0.display = taffy::Display::None;
        self
    }

    pub fn display_block(mut self) -> Self {
        self.0.display = taffy::Display::Block;
        self
    }

    pub fn display_grid(mut self) -> Self {
        self.0.display = taffy::Display::Grid;
        self
    }

    pub fn display_flex(mut self) -> Self {
        self.0.display = taffy::Display::Flex;
        self
    }

    pub fn flex_row(mut self) -> Self {
        self.0.flex_direction = taffy::FlexDirection::Row;
        self
    }

    pub fn flex_column(mut self) -> Self {
        self.0.flex_direction = taffy::FlexDirection::Column;
        self
    }

    pub fn flex_row_reverse(mut self) -> Self {
        self.0.flex_direction = taffy::FlexDirection::RowReverse;
        self
    }

    pub fn flex_column_reverse(mut self) -> Self {
        self.0.flex_direction = taffy::FlexDirection::ColumnReverse;
        self
    }

    pub fn flex_wrap(mut self) -> Self {
        self.0.flex_wrap = taffy::FlexWrap::Wrap;
        self
    }

    pub fn flex_nowrap(mut self) -> Self {
        self.0.flex_wrap = taffy::FlexWrap::NoWrap;
        self
    }

    pub fn flex_wrap_reverse(mut self) -> Self {
        self.0.flex_wrap = taffy::FlexWrap::WrapReverse;
        self
    }
}

// Alignment and justification.
impl Layout {
    pub fn align_items_center(mut self) -> Self {
        self.0.align_items = Some(taffy::AlignItems::Center);
        self
    }

    pub fn align_content_center(mut self) -> Self {
        self.0.align_content = Some(taffy::AlignContent::Center);
        self
    }

    pub fn align_self_center(mut self) -> Self {
        self.0.align_self = Some(taffy::AlignItems::Center);
        self
    }

    pub fn justify_items_center(mut self) -> Self {
        self.0.justify_items = Some(taffy::AlignItems::Center);
        self
    }

    pub fn justify_content_center(mut self) -> Self {
        self.0.justify_content = Some(taffy::AlignContent::Center);
        self
    }

    pub fn justify_self_center(mut self) -> Self {
        self.0.justify_self = Some(taffy::AlignItems::Center);
        self
    }

    pub fn align_items_start(mut self) -> Self {
        self.0.align_items = Some(taffy::AlignItems::Start);
        self
    }

    pub fn align_content_start(mut self) -> Self {
        self.0.align_content = Some(taffy::AlignContent::Start);
        self
    }

    pub fn align_self_start(mut self) -> Self {
        self.0.align_self = Some(taffy::AlignItems::Start);
        self
    }

    pub fn justify_items_start(mut self) -> Self {
        self.0.justify_items = Some(taffy::AlignItems::Start);
        self
    }

    pub fn justify_content_start(mut self) -> Self {
        self.0.justify_content = Some(taffy::AlignContent::Start);
        self
    }

    pub fn justify_self_start(mut self) -> Self {
        self.0.justify_self = Some(taffy::AlignItems::Start);
        self
    }
}
