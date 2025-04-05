//! User interface



use super::layout::Layout;



pub struct Ui {
    tree: taffy::TaffyTree<ElementObject>,
    root: taffy::NodeId,

    // Known state.
    area: (f32, f32),
    mouse_pos: (f32, f32),
    hovered_element: Option<u64>,
}

impl Ui {
    fn next_id(&self) -> usize {
        self.tree.total_node_count()
    }

    pub fn new(layout: Layout) -> Self {
        let mut tree = taffy::TaffyTree::new();
        let root = tree.new_with_children(layout.into(), &[]).unwrap();

        Self {
            tree,
            root,

            area: (1.0, 1.0),
            mouse_pos: (0.0, 0.0),
            hovered_element: None,
        }
    }

    pub fn push_to(&mut self, layout: Layout, parent: u64, element: ElementObject) -> u64 {
        let id = self.tree.new_leaf_with_context(layout.into(), element).unwrap();
        self.tree.add_child(parent.into(), id).unwrap();

        id.into()
    }

    pub fn push_to_root(&mut self, layout: Layout, element: ElementObject) -> u64 {
        let id = self.tree.new_leaf_with_context(layout.into(), element).unwrap();
        self.tree.add_child(self.root, id).unwrap();

        id.into()
    }

    pub fn mouse_moved(&mut self, handler: &mut impl UiHandler, x: f32, y: f32) {
        if self.mouse_pos == (x, y) {
            return;
        }
        self.mouse_pos = (x, y);
        let mut hover_changed_to = None;
        for child in self.tree.children(self.root).unwrap() {
            let layout = self.tree.layout(child).unwrap();
            if layout.location.x > x
                || layout.location.y > y
                || layout.location.x + layout.size.width < x
                || layout.location.y + layout.size.height < y
            {
                continue;
            }
            let nested = self.tree.children(child).unwrap();
            if nested.is_empty() {
                let elem = self.tree.get_node_context(child).unwrap();
                if elem.is_responsive() {
                    if let Some(hovered) = self.hovered_element.take() {
                        if hovered != child.into() {
                            hover_changed_to = Some(child);
                        }
                    } else {
                        hover_changed_to = Some(child);
                    }
                }
            } else {
                for nested in nested {
                    let layout = self.tree.layout(nested).unwrap();
                    if layout.location.x > x
                        || layout.location.y > y
                        || layout.location.x + layout.size.width < x
                        || layout.location.y + layout.size.height < y
                    {
                        continue;
                    }
                    let elem = self.tree.get_node_context(nested).unwrap();
                    if elem.is_responsive() {
                        if let Some(hovered) = self.hovered_element.take() {
                            if hovered != nested.into() {
                                hover_changed_to = Some(nested);
                            }
                        } else {
                            hover_changed_to = Some(nested);
                        }
                    }
                }
            }
        }
        if let Some(newly_hovered) = hover_changed_to {
            handler.on_hover(newly_hovered.into(), &mut self.tree);
            self.hovered_element = Some(newly_hovered.into());
        }
    }

    pub fn resized(&mut self, width: f32, height: f32) {
        if self.area == (width, height) {
            return;
        }
        self.area = (width, height);
        self.tree.compute_layout(
            self.root,
            taffy::Size {
                width: taffy::AvailableSpace::Definite(width),
                height: taffy::AvailableSpace::Definite(height),
            },
        ).unwrap();
    }
}



pub trait UiHandler {
    fn on_resize(&mut self, element: u64, tree: &mut taffy::TaffyTree<ElementObject>);
    fn on_hover(&mut self, element: u64, tree: &mut taffy::TaffyTree<ElementObject>);
    fn on_click(&mut self, element: u64, tree: &mut taffy::TaffyTree<ElementObject>);
}



pub trait Element {
    fn is_responsive(&self) -> bool;
}

pub type ElementObject = Box<dyn Element>;
