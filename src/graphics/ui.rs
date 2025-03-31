//! User interface




pub struct Ui {
    tree: taffy::TaffyTree<(usize, bool)>,
    root: taffy::NodeId,

    // Known state.
    area: (f32, f32),
    mouse_pos: (f32, f32),
    hovered_node: Option<taffy::NodeId>,
}

impl Ui {
    fn next_id(&self) -> usize {
        self.tree.total_node_count()
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
                let (_id, responsive) = self.tree.get_node_context(child).unwrap();
                if *responsive {
                    if let Some(hovered) = self.hovered_node.take() {
                        if hovered != child {
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
                    let (_id, responsive) = self.tree.get_node_context(nested).unwrap();
                    if *responsive {
                        if let Some(hovered) = self.hovered_node.take() {
                            if hovered != nested {
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
            handler.on_hover(newly_hovered, &mut self.tree);
            self.hovered_node = Some(newly_hovered);
        }
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        if self.area != (width, height) {
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
}



pub trait UiHandler {
    fn on_hover(&mut self, node: taffy::NodeId, tree: &mut taffy::TaffyTree<(usize, bool)>);
}
