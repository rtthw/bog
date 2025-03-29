//! Visual layouts



use super::scene::Scene;



pub struct Ui {
    tree: taffy::TaffyTree<(usize, bool)>,
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

    // SAFETY: It appears that none of `taffy`'s methods can fail, so the unwraps are fine.
    pub fn push_to(&mut self, layout: Layout, parent: taffy::NodeId, id: usize, resize: bool) -> taffy::NodeId {
        let node = self.tree.new_leaf_with_context(layout.into(), (id, resize)).unwrap();
        self.tree.add_child(parent, node).unwrap();
        node
    }

    // SAFETY: It appears that none of `taffy`'s methods can fail, so the unwraps are fine.
    pub fn push_to_root(&mut self, layout: Layout, id: usize, resize: bool) -> taffy::NodeId {
        let node = self.tree.new_leaf_with_context(layout.into(), (id, resize)).unwrap();
        self.tree.add_child(self.root, node).unwrap();
        node
    }

    // SAFETY: It appears that none of `taffy`'s methods can fail, so the unwraps are fine.
    pub fn resize(&mut self, scene: &mut Scene, width: f32, height: f32) {
        self.tree.compute_layout(
            self.root,
            taffy::Size {
                width: taffy::AvailableSpace::Definite(width),
                height: taffy::AvailableSpace::Definite(height),
            },
        ).unwrap();

        for node in self.tree.children(self.root).unwrap() {
            do_layout(scene, &self.tree, node, (0.0, 0.0), height);
        }
    }
}

fn do_layout(
    scene: &mut Scene,
    tree: &taffy::TaffyTree<(usize, bool)>,
    node: taffy::NodeId,
    position: (f32, f32),
    screen_height: f32,
) {
    let layout = tree.layout(node).unwrap();
    let Some((id, resize)) = tree.get_node_context(node) else {
        println!("ERROR: Attempted to update nonexistent node in UI");
        return;
    };
    let Some(mesh) = scene.geometry(*id) else {
        println!("ERROR: Attempted to update nonexistent scene object in UI");
        return;
    };
    let (node_width, node_height) = (
        layout.content_box_width(),
        layout.content_box_height(),
    );
    let real_y = layout.location.y + position.1 + node_height;
    let pos = (
        layout.location.x + position.0,
        // NOTE: `taffy` uses the top-left corner as the origin, but `three-d` uses
        //       the bottom left corner as the origin. So we need to convert here.
        screen_height - real_y,
        // layout.location.y + position.1,
    );

    // println!("Updating UI object: {}x{}@{:?}", node_width, node_height, pos);

    let mut transformation = three_d::Mat3::from_translation(pos.into());
    if *resize {
        transformation = transformation
            * three_d::Mat3::from_nonuniform_scale(node_width, node_height);
    }

    // See: `three_d::Mesh::set_transformation_2d`
    mesh.set_transformation(three_d::Mat4::new(
        transformation.x.x,
        transformation.x.y,
        0.0,
        transformation.x.z,
        transformation.y.x,
        transformation.y.y,
        0.0,
        transformation.y.z,
        0.0,
        0.0,
        1.0,
        0.0,
        transformation.z.x,
        transformation.z.y,
        0.0,
        transformation.z.z,
    ));

    let Ok(children) = tree.children(node) else {
        println!("Found no children for {id}");
        return;
    };
    for child in children {
        do_layout(scene, tree, child, (pos.0, real_y), screen_height);
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
