//! GUI



use crate::{layout::*, math::{vec2, Vec2}};



pub trait UserInterfaceHandler {
    fn on_mouse_move(&mut self, pos: Vec2);
    fn on_mouse_enter(&mut self, node: u64, cx: UserInterfaceContext);
    fn on_mouse_leave(&mut self, node: u64, cx: UserInterfaceContext);
    fn on_mouse_down(&mut self, node: u64, cx: UserInterfaceContext);
    fn on_mouse_up(&mut self, node: u64, cx: UserInterfaceContext);
    fn on_drag_move(&mut self, node: u64, cx: UserInterfaceContext, delta: Vec2, over: Option<u64>);
    fn on_drag_start(&mut self, node: u64, cx: UserInterfaceContext);
    fn on_drag_end(&mut self, node: u64, cx: UserInterfaceContext, over: Option<u64>);
    fn on_resize(&mut self, size: Vec2);
    fn on_node_layout(&mut self, node: u64, placement: &Placement);
}



pub struct UserInterface {
    state: UserInterfaceState,
    layout_map: LayoutMap,
    root_node: u64,
    hovered_node: Option<u64>,
    drag_start_pos: Option<Vec2>,
    drag_start_time: std::time::Instant,
    drag_start_node: Option<u64>,
}

impl UserInterface {
    pub fn new(layout_map: LayoutMap, root_node: u64) -> Self {
        Self {
            state: UserInterfaceState {
                size: vec2(0.0, 0.0),
                mouse_pos: vec2(0.0, 0.0),
                is_dragging: false,
            },
            layout_map,
            root_node,
            hovered_node: None,
            drag_start_pos: None,
            drag_start_time: std::time::Instant::now(),
            drag_start_node: None,
        }
    }

    pub fn layout_map(&mut self) -> &mut LayoutMap {
        &mut self.layout_map
    }

    pub fn root_placement(&self) -> Placement {
        self.layout_map.placement(self.root_node, Vec2::ZERO)
    }

    pub fn handle_resize(&mut self, handler: &mut impl UserInterfaceHandler, size: Vec2) {
        if size == self.state.size {
            return;
        }
        self.state.size = size;
        self.layout_map.compute_layout(self.root_node, size);
        handler.on_resize(size);
    }

    pub fn handle_mouse_move(&mut self, handler: &mut impl UserInterfaceHandler, pos: Vec2) {
        if pos == self.state.mouse_pos {
            return;
        }
        self.state.mouse_pos = pos;

        handler.on_mouse_move(pos);

        let mut hovered = Vec::with_capacity(3);

        fn find_hovered(placement: Placement<'_>, hovered: &mut Vec<u64>, pos: Vec2) {
            if !placement.rect().contains(pos) {
                return;
            }

            for child_placement in placement.children() {
                find_hovered(child_placement, hovered, pos);
            }

            hovered.push(placement.node());
        }

        find_hovered(self.root_placement(), &mut hovered, pos);

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
                        });
                    }
                }
                if self.state.is_dragging {
                    let delta = pos - drag_origin_pos;
                    handler.on_drag_move(
                        drag_node,
                        UserInterfaceContext {
                            state: &self.state,
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
                });
            }
            if let Some(entered_node) = topmost_hovered {
                handler.on_mouse_enter(entered_node, UserInterfaceContext {
                    state: &self.state,
                });
                self.hovered_node = Some(entered_node);
            }
        }
    }

    pub fn handle_mouse_down(&mut self, handler: &mut impl UserInterfaceHandler) {
        if let Some(node) = self.hovered_node {
            handler.on_mouse_down(node, UserInterfaceContext {
                state: &self.state,
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
            });
        }
        self.drag_start_pos = None;
        if let Some(node) = self.drag_start_node.take() {
            if self.state.is_dragging {
                self.state.is_dragging = false;
                handler.on_drag_end(node, UserInterfaceContext {
                    state: &self.state,
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
}
