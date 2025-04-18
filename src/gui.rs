//! GUI



use crate::{layout::*, math::{vec2, Vec2}};



pub type Element = LayoutNode;

pub struct Gui {
    state: GuiState,
    layout_tree: LayoutTree,
    hovered_element: Option<LayoutNode>,
    drag_start_pos: Option<Vec2>,
    drag_start_time: std::time::Instant,
    drag_start_element: Option<LayoutNode>,
}

impl Gui {
    pub fn new(root_layout: Layout) -> Self {
        Self {
            state: GuiState {
                size: vec2(0.0, 0.0),
                mouse_pos: vec2(0.0, 0.0),
                is_dragging: false,
            },
            layout_tree: LayoutTree::new(root_layout),
            hovered_element: None,
            drag_start_pos: None,
            drag_start_time: std::time::Instant::now(),
            drag_start_element: None,
        }
    }

    pub fn push_element_to_root(&mut self, layout: Layout) -> Element {
        self.layout_tree.push_to_root(layout, true)
    }

    pub fn handle_resize(&mut self, handler: &mut impl GuiHandler, size: Vec2) {
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

    pub fn handle_mouse_move(&mut self, handler: &mut impl GuiHandler, pos: Vec2) {
        if pos == self.state.mouse_pos {
            return;
        }
        self.state.mouse_pos = pos;

        handler.on_mouse_move(pos);

        let mut hover_changed_to = None;
        self.layout_tree.iter_placements(&mut |node, placement| {
            let place_pos = placement.position();

            if place_pos.x > pos.x
                || place_pos.y > pos.y
                || place_pos.x
                    + placement.layout.size.width
                    + placement.layout.padding.horizontal_components().sum() < pos.x
                || place_pos.y
                    + placement.layout.size.height
                    + placement.layout.padding.vertical_components().sum() < pos.y
            {
                return; // Breaks out of `iter_placements`.
            }

            if !self.layout_tree.is_interactable(node) {
                return; // Breaks out of `iter_placements`.
            }

            // TODO: See if there should be some multi-hovering system.
            hover_changed_to = Some(node.into());
        });

        if let Some(drag_origin_pos) = self.drag_start_pos {
            if let Some(drag_element) = self.drag_start_element {
                if !self.state.is_dragging {
                    let dur_since = std::time::Instant::now()
                        .duration_since(self.drag_start_time);
                    if dur_since.as_secs_f64() > 0.1 {
                        // User is likely dragging.
                        self.state.is_dragging = true;
                        handler.on_drag_start(drag_element, &mut self.layout_tree);
                    }
                }
                if self.state.is_dragging {
                    let delta = pos - drag_origin_pos;
                    handler.on_drag_update(
                        &mut self.layout_tree,
                        drag_element,
                        hover_changed_to,
                        delta,
                    );
                }
            }
        }

        if self.hovered_element != hover_changed_to {
            if let Some(left_element) = self.hovered_element.take() {
                handler.on_mouse_leave(left_element, &self.state);
            }
            if let Some(entered_element) = hover_changed_to {
                handler.on_mouse_enter(entered_element, &self.state);
                self.hovered_element = Some(entered_element);
            }
        }
    }

    pub fn handle_mouse_down(&mut self, handler: &mut impl GuiHandler) {
        if let Some(element) = self.hovered_element {
            handler.on_mouse_down(element, &self.state);
        }
        self.drag_start_time = std::time::Instant::now();
        self.drag_start_pos = Some(self.state.mouse_pos);
        self.drag_start_element = self.hovered_element.clone();
    }

    pub fn handle_mouse_up(&mut self, handler: &mut impl GuiHandler) {
        if let Some(element) = self.hovered_element {
            handler.on_mouse_up(element, &self.state);
        }
        self.drag_start_pos = None;
        if let Some(element) = self.drag_start_element.take() {
            if self.state.is_dragging {
                self.state.is_dragging = false;
                handler.on_drag_end(element);
            }
        }
    }
}

pub struct GuiState {
    pub size: Vec2,
    pub mouse_pos: Vec2,
    pub is_dragging: bool,
}

struct Proxy<'a> {
    handler: &'a mut dyn GuiHandler,
}

impl<'a> LayoutHandler for Proxy<'a> {
    fn on_layout(&mut self, node: LayoutNode, placement: &Placement) {
        self.handler.on_element_layout(node, placement);
    }
}



pub trait GuiHandler {
    fn on_mouse_move(&mut self, pos: Vec2);
    fn on_mouse_enter(&mut self, element: Element, state: &GuiState);
    fn on_mouse_leave(&mut self, element: Element, state: &GuiState);
    fn on_mouse_down(&mut self, element: Element, state: &GuiState);
    fn on_mouse_up(&mut self, element: Element, state: &GuiState);
    fn on_drag_update(
        &mut self,
        tree: &mut LayoutTree,
        element: Element,
        hovered: Option<Element>,
        delta: Vec2,
    );
    fn on_drag_start(&mut self, element: Element, tree: &mut LayoutTree);
    fn on_drag_end(&mut self, element: Element);
    fn on_resize(&mut self, size: Vec2);
    fn on_element_layout(&mut self, element: Element, placement: &Placement);
}
