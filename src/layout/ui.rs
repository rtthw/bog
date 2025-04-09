//! User interface



use three_d::Vec2;

use super::Layout;



pub struct Ui {
    tree: LayoutTree,

    // Known state.
    area: (f32, f32),
    mouse_pos: (f32, f32),
    lmb_down_at: Option<(f32, f32)>,
    lmb_down_time: std::time::Instant,
    lmb_down_node: Option<u64>,
    is_dragging: bool,
    hovered_node: Option<u64>,
}

impl Ui {
    pub fn new(root_layout: Layout) -> Self {
        Self {
            tree: LayoutTree::new(root_layout),

            area: (1.0, 1.0),
            mouse_pos: (0.0, 0.0),
            lmb_down_at: None,
            lmb_down_time: std::time::Instant::now(),
            lmb_down_node: None,
            is_dragging: false,
            hovered_node: None,
        }
    }

    pub fn tree(&mut self) -> &mut LayoutTree {
        &mut self.tree
    }

    pub fn handle_mouse_move(&mut self, handler: &mut impl UiHandler, x: f32, y: f32) {
        if self.mouse_pos == (x, y) {
            return;
        }
        self.mouse_pos = (x, y);

        if let Some((drag_origin_x, drag_origin_y)) = self.lmb_down_at {
            if let Some(dragging_node) = self.lmb_down_node {
                if !self.is_dragging {
                    let dur_since = std::time::Instant::now().duration_since(self.lmb_down_time);
                    if dur_since.as_secs_f64() > 0.1 {
                        // User is likely dragging.
                        self.is_dragging = true;
                        handler.on_drag_start(dragging_node, &mut self.tree.inner);
                    }
                }
                if self.is_dragging {
                    let delta_x = x - drag_origin_x;
                    let delta_y = y - drag_origin_y;
                    handler.on_drag_update(dragging_node, &mut self.tree.inner, delta_x, delta_y);
                }
            }
        }

        let mut hover_changed_to = None;
        for_each_node(&self.tree.inner, self.tree.root, &mut |node, placement| {
            let pos = placement.position();

            if pos.x > x
                || pos.y > y
                || pos.x
                    + placement.layout.size.width
                    + placement.layout.padding.horizontal_components().sum() < x
                || pos.y
                    + placement.layout.size.height
                    + placement.layout.padding.vertical_components().sum() < y
            {
                return; // Breaks out of `for_each_node`.
            }

            let accepts_input = self.tree.inner.get_node_context(node).unwrap();
            if !*accepts_input {
                return; // Breaks out of `for_each_node`.
            }

            // TODO: See if there should be some multi-hovering system.
            hover_changed_to = Some(node.into());
        });

        if self.hovered_node == hover_changed_to {
            return;
        }

        if let Some(left_node) = self.hovered_node.take() {
            handler.on_mouse_leave(left_node, &mut self.tree.inner);
        }
        if let Some(entered_node) = hover_changed_to {
            handler.on_mouse_enter(entered_node, &mut self.tree.inner);
            self.hovered_node = Some(entered_node);
        }
    }

    pub fn handle_mouse_down(
        &mut self,
        handler: &mut impl UiHandler,
        button: winit::event::MouseButton,
    ) {
        if let Some(node) = self.hovered_node {
            handler.on_mouse_down(node, &mut self.tree.inner);
        }
        match button {
            winit::event::MouseButton::Left => {
                self.lmb_down_time = std::time::Instant::now();
                self.lmb_down_at = Some(self.mouse_pos);
                self.lmb_down_node = self.hovered_node.clone();
            }
            _ => {}
        }
    }

    pub fn handle_mouse_up(
        &mut self,
        handler: &mut impl UiHandler,
        button: winit::event::MouseButton,
    ) {
        match button {
            winit::event::MouseButton::Left => {
                if let Some(node) = self.hovered_node {
                    handler.on_mouse_up(node, &mut self.tree.inner);
                }
                self.lmb_down_at = None;
                if let Some(node) = self.lmb_down_node.take() {
                    handler.on_click(node, &mut self.tree.inner);
                    if self.is_dragging {
                        self.is_dragging = false;
                        handler.on_drag_end(node, self.hovered_node, &mut self.tree.inner);
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
        self.tree.handle_resize(handler, width, height);
    }
}

fn for_each_node<F>(tree: &UiModel, node: taffy::NodeId, func: &mut F)
where F: FnMut(taffy::NodeId, &Placement),
{
    let top_layout = tree.layout(node).unwrap();

    for child in tree.children(node).unwrap().into_iter() {
        let placement = Placement {
            parent_position: Vec2::new(top_layout.location.x, top_layout.location.y),
            layout: *tree.layout(child).unwrap(),
        };
        func(child, &placement);
        for_each_node_inner(tree, child, &placement, func);
    }
}

fn for_each_node_inner<F>(
    tree: &UiModel,
    node: taffy::NodeId,
    placement: &Placement,
    func: &mut F,
)
where F: FnMut(taffy::NodeId, &Placement),
{
    for child in tree.children(node).unwrap().into_iter() {
        let layout = *tree.layout(child).unwrap();
        let child_placement = Placement {
            parent_position: placement.parent_position
                + Vec2::new(layout.location.x, layout.location.y),
            layout,
        };
        func(child, &child_placement);
        for_each_node_inner(tree, child, &child_placement, func);
    }
}



pub type UiModel = taffy::TaffyTree<bool>;
pub type UiLayout = taffy::Layout;

pub trait UiHandler {
    fn on_layout(&mut self, node: u64, placement: &Placement);
    fn on_mouse_enter(&mut self, node: u64, model: &mut UiModel);
    fn on_mouse_leave(&mut self, node: u64, model: &mut UiModel);
    fn on_mouse_down(&mut self, node: u64, model: &mut UiModel);
    fn on_mouse_up(&mut self, node: u64, model: &mut UiModel);
    fn on_drag_start(&mut self, node: u64, model: &mut UiModel);
    fn on_drag_end(&mut self, node: u64, other: Option<u64>, model: &mut UiModel);
    fn on_drag_update(&mut self, node: u64, model: &mut UiModel, delta_x: f32, delta_y: f32);
    fn on_click(&mut self, node: u64, model: &mut UiModel);
}



pub struct LayoutTree {
    inner: taffy::TaffyTree<bool>,
    root: taffy::NodeId,
    known_size: Vec2,
}

impl LayoutTree {
    pub fn new(root_layout: Layout) -> Self {
        let mut inner = taffy::TaffyTree::new();
        let root = inner.new_with_children(root_layout.into(), &[]).unwrap();

        Self {
            inner,
            root,
            known_size: Vec2::new(0.0, 0.0),
        }
    }

    pub fn push_to(&mut self, layout: Layout, parent: u64, accepts_input: bool) -> u64 {
        let id = self.inner.new_leaf_with_context(layout.into(), accepts_input).unwrap();
        self.inner.add_child(parent.into(), id).unwrap();

        id.into()
    }

    pub fn push_to_root(&mut self, layout: Layout, accepts_input: bool) -> u64 {
        let id = self.inner.new_leaf_with_context(layout.into(), accepts_input).unwrap();
        self.inner.add_child(self.root, id).unwrap();

        id.into()
    }

    pub fn handle_resize(&mut self, handler: &mut impl UiHandler, width: f32, height: f32) {
        self.known_size = Vec2::new(width, height);
        self.do_layout(handler);
    }

    pub fn do_layout(&mut self, handler: &mut impl UiHandler) {
        self.inner.compute_layout(
            self.root,
            taffy::Size {
                width: taffy::AvailableSpace::Definite(self.known_size.x),
                height: taffy::AvailableSpace::Definite(self.known_size.y),
            },
        ).unwrap();

        for_each_node(&self.inner, self.root, &mut |node, placement| {
            handler.on_layout(node.into(), placement);
        });
    }
}

pub struct Placement {
    parent_position: Vec2,
    pub layout: taffy::Layout,
}

impl Placement {
    pub fn position(&self) -> Vec2 {
        Vec2::new(
            self.parent_position.x + self.layout.location.x,
            self.parent_position.y + self.layout.location.y,
        )
    }

    pub fn content_position(&self) -> Vec2 {
        Vec2::new(
            self.parent_position.x + self.layout.content_box_x(),
            self.parent_position.y + self.layout.content_box_y(),
        )
    }
}
