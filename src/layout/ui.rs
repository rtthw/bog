//! User interface



use super::Layout;



pub struct Ui {
    tree: UiModel,
    root: taffy::NodeId,

    // Known state.
    area: (f32, f32),
    mouse_pos: (f32, f32),
    lmb_down_at: Option<(f32, f32)>,
    hovered_node: Option<u64>,
}

impl Ui {
    pub fn new(layout: Layout) -> Self {
        let mut tree = taffy::TaffyTree::new();
        let root = tree.new_with_children(layout.into(), &[]).unwrap();

        Self {
            tree,
            root,

            area: (1.0, 1.0),
            mouse_pos: (0.0, 0.0),
            lmb_down_at: None,
            hovered_node: None,
        }
    }

    pub fn push_to(&mut self, layout: Layout, parent: u64, accepts_input: bool) -> u64 {
        let id = self.tree.new_leaf_with_context(layout.into(), accepts_input).unwrap();
        self.tree.add_child(parent.into(), id).unwrap();

        id.into()
    }

    pub fn push_to_root(&mut self, layout: Layout, accepts_input: bool) -> u64 {
        let id = self.tree.new_leaf_with_context(layout.into(), accepts_input).unwrap();
        self.tree.add_child(self.root, id).unwrap();

        id.into()
    }

    pub fn handle_mouse_move(&mut self, handler: &mut impl UiHandler, x: f32, y: f32) {
        if self.mouse_pos == (x, y) {
            return;
        }
        self.mouse_pos = (x, y);

        let mut hover_changed_to = None;

        let mut parent_layout = self.tree.layout(self.root).unwrap();
        let mut known_parent = self.root;
        for_each_node(&self.tree, self.root, &mut |node, parent| {
            if parent != known_parent {
                parent_layout = self.tree.layout(parent).unwrap();
                known_parent = parent;
            }
            let layout = self.tree.layout(node).unwrap();
            let (abs_x, abs_y) = (
                parent_layout.location.x + layout.location.x,
                parent_layout.location.y + layout.location.y,
            );

            if abs_x > x
                || abs_y > y
                || abs_x + layout.size.width + layout.padding.horizontal_components().sum() < x
                || abs_y + layout.size.height + layout.padding.vertical_components().sum() < y
            {
                return; // Breaks out of `for_each_node`.
            }

            let accepts_input = self.tree.get_node_context(node).unwrap();
            if !*accepts_input {
                return; // Breaks out of `for_each_node`.
            }

            // TODO: See if there should be some multi-hovering system.
            hover_changed_to = Some(node);

            // if let Some(hovered) = self.hovered_node {
            //     if hovered != node.into() {
            //         hover_changed_to = Some(node);
            //     }
            // } else {
            //     hover_changed_to = Some(node);
            // }
        });

        if let Some(left_node) = self.hovered_node {
            handler.on_mouse_leave(left_node, &mut self.tree);
        }
        if let Some(entered_node) = hover_changed_to {
            handler.on_mouse_enter(entered_node.into(), &mut self.tree);
            self.hovered_node = Some(entered_node.into());
        }
    }

    pub fn handle_mouse_down(
        &mut self,
        handler: &mut impl UiHandler,
        button: winit::event::MouseButton,
    ) {
        if let Some(node) = self.hovered_node {
            handler.on_mouse_down(node, &mut self.tree);
        }
        match button {
            winit::event::MouseButton::Left => {
                self.lmb_down_at = Some(self.mouse_pos);
            }
            _ => {}
        }
    }

    pub fn handle_mouse_up(
        &mut self,
        handler: &mut impl UiHandler,
        button: winit::event::MouseButton,
    ) {
        if let Some(node) = self.hovered_node {
            handler.on_mouse_up(node, &mut self.tree);
        }
        match button {
            winit::event::MouseButton::Left => {
                if let Some(_down_pos) = self.lmb_down_at.take() {
                    if let Some(node) = self.hovered_node {
                        handler.on_click(node, &mut self.tree);
                    }
                }
            }
            _ => {}
        }
    }

    pub fn handle_resize(&mut self, handler: &mut impl UiHandler, width: f32, height: f32) {
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
        for node in self.tree.children(self.root).unwrap() {
            handler.on_resize(node.into(), &mut self.tree);
            for child_node in self.tree.children(node).unwrap() {
                handler.on_resize(child_node.into(), &mut self.tree);
            }
        }
    }
}

fn for_each_node<F>(tree: &UiModel, node: taffy::NodeId, func: &mut F)
where F: FnMut(taffy::NodeId, taffy::NodeId),
{
    for child in tree.children(node).unwrap().into_iter() {
        func(child, node);
        for_each_node(tree, child, func);
    }
}



pub type UiModel = taffy::TaffyTree<bool>;
pub type UiLayout = taffy::Layout;

pub trait UiHandler {
    fn on_resize(&mut self, node: u64, model: &mut UiModel);
    fn on_mouse_enter(&mut self, node: u64, model: &mut UiModel);
    fn on_mouse_leave(&mut self, node: u64, model: &mut UiModel);
    fn on_mouse_down(&mut self, node: u64, model: &mut UiModel);
    fn on_mouse_up(&mut self, node: u64, model: &mut UiModel);
    fn on_click(&mut self, node: u64, model: &mut UiModel);
}
