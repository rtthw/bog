//! GUI



use crate::{layout::*, math::{vec2, Vec2}};



pub trait UserInterfaceHandler {
    fn on_mouse_move(&mut self, pos: Vec2);
    fn on_mouse_enter(&mut self, node: Node, cx: UserInterfaceContext);
    fn on_mouse_leave(&mut self, node: Node, cx: UserInterfaceContext);
    fn on_mouse_down(&mut self, node: Node, cx: UserInterfaceContext);
    fn on_mouse_up(&mut self, node: Node, cx: UserInterfaceContext);
    fn on_drag_move(&mut self, node: Node, cx: UserInterfaceContext, delta: Vec2, over: Option<Node>);
    fn on_drag_start(&mut self, node: Node, cx: UserInterfaceContext);
    fn on_drag_end(&mut self, node: Node, cx: UserInterfaceContext, over: Option<Node>);
    fn on_resize(&mut self, size: Vec2);
    fn on_node_layout(&mut self, node: Node, placement: &Placement);
}



pub struct UserInterface {
    state: UserInterfaceState,
    layout_tree: LayoutTree,
    hovered_node: Option<Node>,
    drag_start_pos: Option<Vec2>,
    drag_start_time: std::time::Instant,
    drag_start_node: Option<Node>,
}

impl UserInterface {
    pub fn new(root_layout: Layout) -> Self {
        Self {
            state: UserInterfaceState {
                size: vec2(0.0, 0.0),
                mouse_pos: vec2(0.0, 0.0),
                is_dragging: false,
            },
            layout_tree: LayoutTree::new(root_layout),
            hovered_node: None,
            drag_start_pos: None,
            drag_start_time: std::time::Instant::now(),
            drag_start_node: None,
        }
    }

    pub fn tree(&mut self) -> &mut LayoutTree {
        &mut self.layout_tree
    }

    pub fn push_node_to_root(&mut self, layout: Layout) -> Node {
        self.layout_tree.push_to_root(layout, true)
    }

    pub fn push_node(&mut self, parent: Node, layout: Layout) -> Node {
        self.layout_tree.push(layout, parent, true)
    }

    pub fn handle_resize(&mut self, handler: &mut impl UserInterfaceHandler, size: Vec2) {
        if size == self.state.size {
            return;
        }
        self.state.size = size;
        self.layout_tree.resize(size);
        self.layout_tree.do_layout(&mut Proxy {
            handler,
        });
        handler.on_resize(size);
    }

    pub fn handle_mouse_move(&mut self, handler: &mut impl UserInterfaceHandler, pos: Vec2) {
        if pos == self.state.mouse_pos {
            return;
        }
        self.state.mouse_pos = pos;

        handler.on_mouse_move(pos);

        let mut hovered = Vec::with_capacity(3);
        self.layout_tree.iter_placements(&mut |node, placement| {
            let place_pos = placement.position();

            if place_pos.x > pos.x
                || place_pos.y > pos.y
                || place_pos.x + placement.layout.size.width < pos.x
                || place_pos.y + placement.layout.size.height < pos.y
            {
                return; // Breaks out of `iter_placements`.
            }

            if !self.layout_tree.is_interactable(node) {
                return; // Breaks out of `iter_placements`.
            }

            hovered.push(node.into());
        });

        let topmost_hovered = hovered.last().copied();

        if let Some(drag_origin_pos) = self.drag_start_pos {
            if let Some(drag_node) = self.drag_start_node {
                if !self.state.is_dragging {
                    let dur_since = std::time::Instant::now()
                        .duration_since(self.drag_start_time);
                    if dur_since.as_secs_f64() > 0.1 {
                        // User is likely dragging.
                        self.state.is_dragging = true;
                        handler.on_drag_start(drag_node, UserInterfaceContext {
                            state: &self.state,
                            tree: &mut self.layout_tree,
                        });
                    }
                }
                if self.state.is_dragging {
                    let delta = pos - drag_origin_pos;
                    handler.on_drag_move(
                        drag_node,
                        UserInterfaceContext {
                            state: &self.state,
                            tree: &mut self.layout_tree,
                        },
                        delta,
                        topmost_hovered,
                    );
                }
            }
        }

        if self.hovered_node != topmost_hovered {
            if let Some(left_node) = self.hovered_node.take() {
                handler.on_mouse_leave(left_node, UserInterfaceContext {
                    state: &self.state,
                    tree: &mut self.layout_tree,
                });
            }
            if let Some(entered_node) = topmost_hovered {
                handler.on_mouse_enter(entered_node, UserInterfaceContext {
                    state: &self.state,
                    tree: &mut self.layout_tree,
                });
                self.hovered_node = Some(entered_node);
            }
        }
    }

    pub fn handle_mouse_down(&mut self, handler: &mut impl UserInterfaceHandler) {
        if let Some(node) = self.hovered_node {
            handler.on_mouse_down(node, UserInterfaceContext {
                state: &self.state,
                tree: &mut self.layout_tree,
            });
        }
        self.drag_start_time = std::time::Instant::now();
        self.drag_start_pos = Some(self.state.mouse_pos);
        self.drag_start_node = self.hovered_node.clone();
    }

    pub fn handle_mouse_up(&mut self, handler: &mut impl UserInterfaceHandler) {
        if let Some(node) = self.hovered_node {
            handler.on_mouse_up(node, UserInterfaceContext {
                state: &self.state,
                tree: &mut self.layout_tree,
            });
        }
        self.drag_start_pos = None;
        if let Some(node) = self.drag_start_node.take() {
            if self.state.is_dragging {
                self.state.is_dragging = false;
                handler.on_drag_end(node, UserInterfaceContext {
                    state: &self.state,
                    tree: &mut self.layout_tree,
                }, self.hovered_node);
            }
        }
    }

    pub fn handle_wheel_down(&mut self, handler: &mut impl UserInterfaceHandler) {

    }

    pub fn handle_wheel_up(&mut self, handler: &mut impl UserInterfaceHandler) {

    }
}

pub struct UserInterfaceState {
    pub size: Vec2,
    pub mouse_pos: Vec2,
    pub is_dragging: bool,
}

pub struct UserInterfaceContext<'a> {
    pub state: &'a UserInterfaceState,
    pub tree: &'a mut LayoutTree,
}

struct Proxy<'a> {
    handler: &'a mut dyn UserInterfaceHandler,
}

impl<'a> LayoutHandler for Proxy<'a> {
    fn on_layout(&mut self, node: Node, placement: &Placement) {
        self.handler.on_node_layout(node, placement);
    }
}
