//! User interface layouts



use crate::math::{vec2, Vec2};



pub type LayoutNode = taffy::NodeId;

pub struct LayoutTree {
    tree: taffy::TaffyTree<bool>,
    root: LayoutNode,
    available_space: Vec2,
}

impl LayoutTree {
    pub fn new(root_layout: Layout) -> Self {
        let mut tree = taffy::TaffyTree::new();
        let root = tree.new_with_children(root_layout.into(), &[])
            .unwrap(); // Cannot fail.

        Self {
            tree,
            root,
            available_space: vec2(0.0, 0.0),
        }
    }

    pub fn iter_placements(&self, func: &mut impl FnMut(LayoutNode, &Placement)) {
        for_each_node(&self.tree, self.root, func);
    }

    // TODO: Node context should be generic.
    pub fn is_interactable(&self, node: LayoutNode) -> bool {
        *self.tree.get_node_context(node).unwrap()
    }

    pub fn push(&mut self, layout: Layout, parent: LayoutNode, interactable: bool) -> LayoutNode {
        let id = self.tree.new_leaf_with_context(layout.into(), interactable)
            .unwrap(); // Cannot fail.
        self.tree.add_child(parent, id)
            .unwrap(); // Cannot fail.

        id
    }

    pub fn push_to_root(&mut self, layout: Layout, interactable: bool) -> LayoutNode {
        let id = self.tree.new_leaf_with_context(layout.into(), interactable)
            .unwrap(); // Cannot fail.
        self.tree.add_child(self.root, id)
            .unwrap(); // Cannot fail.

        id
    }

    pub fn resize(&mut self, available_space: Vec2) {
        self.available_space = available_space;
    }

    pub fn do_layout(&mut self, handler: &mut impl LayoutHandler) {
        self.tree
            .compute_layout(
                self.root,
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

fn for_each_node<F>(tree: &taffy::TaffyTree<bool>, node: LayoutNode, func: &mut F)
where F: FnMut(LayoutNode, &Placement),
{
    let top_layout = tree.layout(node).unwrap();

    for child in tree.children(node).unwrap().into_iter() {
        let placement = Placement {
            parent_pos: Vec2::new(top_layout.location.x, top_layout.location.y),
            layout: *tree.layout(child).unwrap(),
        };
        func(child, &placement);
        for_each_node_inner(tree, child, &placement, func);
    }
}

fn for_each_node_inner<F>(
    tree: &taffy::TaffyTree<bool>,
    node: LayoutNode,
    placement: &Placement,
    func: &mut F,
)
where F: FnMut(LayoutNode, &Placement),
{
    for child in tree.children(node).unwrap().into_iter() {
        let layout = *tree.layout(child).unwrap();
        let child_placement = Placement {
            parent_pos: placement.parent_pos
                + Vec2::new(layout.location.x, layout.location.y),
            layout,
        };
        func(child, &child_placement);
        for_each_node_inner(tree, child, &child_placement, func);
    }
}



pub trait LayoutHandler {
    fn on_layout(&mut self, node: LayoutNode, placement: &Placement);
}



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

impl Layout {
    pub fn width(mut self, width: f32) -> Self {
        self.0.size.width = taffy::prelude::length(width);
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.0.size.height = taffy::prelude::length(height);
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
