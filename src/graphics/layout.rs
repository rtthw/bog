//! Visual layouts



use super::scene::Scene;



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
            let layout = self.tree.layout(node).unwrap();
            let Some(id) = self.tree.get_node_context(node) else {
                println!("ERROR: Attempted to update nonexistent node in UI");
                continue;
            };
            let Some(mesh) = scene.geometry(*id) else {
                println!("ERROR: Attempted to update nonexistent scene object in UI");
                continue;
            };
            let (_node_width, node_height) = (
                layout.content_box_width(),
                layout.content_box_height(),
            );
            let center = (
                layout.content_box_x(),
                // NOTE: `taffy` uses the top-left corner as the origin, but `three-d` uses
                //       the bottom left corner as the origin. So we need to convert here. See
                //       also that `content_box_x` is unchanged.
                height - (layout.content_box_y() + node_height),
            );

            // println!("Updating UI object: {}x{}@{:?}", node_width, node_height, center);

            let transformation = three_d::Mat3::from_translation(center.into());
                // * three_d::Mat3::from_nonuniform_scale(width, height);

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
        }
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
