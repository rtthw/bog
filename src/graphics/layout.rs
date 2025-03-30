//! Visual layouts



use three_d::Mat3;

use super::{scene::Scene, Render, RenderOne};



pub struct Ui {
    scene: Scene,
    tree: taffy::TaffyTree<(usize, bool)>,
    root: taffy::NodeId,
}

impl Ui {
    pub fn new(layout: Layout) -> Self {
        let mut tree = taffy::TaffyTree::new();
        let root = tree.new_with_children(layout.into(), &[]).unwrap();

        Self {
            scene: Scene::default(),
            tree,
            root,
        }
    }

    // SAFETY: It appears that none of `taffy`'s methods can fail, so the unwraps are fine.
    pub fn push_to(
        &mut self,
        layout: Layout,
        parent: taffy::NodeId,
        object: impl RenderOne,
        resize: bool,
    ) -> taffy::NodeId {
        let (mesh, material) = object.destructure();
        let id = self.scene.append(mesh, material);
        let node = self.tree.new_leaf_with_context(layout.into(), (id, resize)).unwrap();
        self.tree.add_child(parent, node).unwrap();
        node
    }

    // SAFETY: It appears that none of `taffy`'s methods can fail, so the unwraps are fine.
    pub fn push_to_root(
        &mut self,
        layout: Layout,
        object: impl RenderOne,
        resize: bool,
    ) -> taffy::NodeId {
        self.push_to(layout, self.root, object, resize)
    }

    // SAFETY: It appears that none of `taffy`'s methods can fail, so the unwraps are fine.
    pub fn resize(&mut self, width: f32, height: f32) {
        self.tree.compute_layout(
            self.root,
            taffy::Size {
                width: taffy::AvailableSpace::Definite(width),
                height: taffy::AvailableSpace::Definite(height),
            },
        ).unwrap();

        let root_layout = self.tree.layout(self.root).unwrap();
        for node in self.tree.children(self.root).unwrap() {
            do_layout(
                &mut self.scene,
                &self.tree,
                node,
                (root_layout.content_box_x(), root_layout.content_box_y()),
                height,
            );
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
        layout.size.width,
        layout.size.height,
    );
    let real_pos = (
        layout.content_box_x() + position.0,
        layout.content_box_y() + position.1,
    );
    let pos3d = (
        real_pos.0,
        // NOTE: `taffy` uses the top-left corner as the origin, but `three-d` uses
        //       the bottom left corner as the origin. So we need to convert here.
        screen_height - (real_pos.1 + node_height),
    );
    let center = (
        pos3d.0 + (node_width / 2.0),
        pos3d.1 + (node_height / 2.0),
    );

    // println!("Layout @ {id} = ");
    // println!("\tSize = {:?}", layout.content_size);
    // println!("\tLocation = {:?}", layout.location);
    // println!("...Updating UI object: {}x{}@{:?}&{:?}", node_width, node_height, pos3d, center);

    if *resize {
        mesh.transform_2d(Mat3::from_translation((center.0, center.1).into())
            * Mat3::from_nonuniform_scale(node_width, node_height));
    } else {
        mesh.transform_2d(Mat3::from_translation((pos3d.0, pos3d.1).into()));
    }

    let Ok(children) = tree.children(node) else {
        return;
    };
    for child in children {
        do_layout(scene, tree, child, real_pos, screen_height);
    }
}

impl Render for Ui {
    fn objects(&self) -> impl Iterator<Item = impl three_d::Object> {
        self.scene.objects()
    }
}

impl Ui {
    pub fn handle_animations(&mut self, seconds_since_start: f32) {
        for geom in self.scene.geometries() {
            geom.perform_animation(seconds_since_start);
        }
    }

    pub fn handle_cursor_moved(&mut self, x: f32, y: f32) {}
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
