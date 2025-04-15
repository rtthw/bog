//! GUI



use std::collections::HashMap;

use crate::{layout::*, math::{vec2, Vec2}};



pub struct Gui<E> {
    layout_tree: LayoutTree,
    elements: HashMap<LayoutNode, E>, // TODO: No hash.

    size: Vec2,
    mouse_pos: Vec2,
    hovered_node: Option<LayoutNode>,
    drag_start_pos: Option<Vec2>,
    drag_start_time: std::time::Instant,
    drag_start_node: Option<LayoutNode>,
    is_dragging: bool,
}

impl<E> Gui<E> {
    pub fn new(root_layout: Layout) -> Self {
        Self {
            layout_tree: LayoutTree::new(root_layout),
            elements: HashMap::new(),
            size: vec2(0.0, 0.0),
            mouse_pos: vec2(0.0, 0.0),
            hovered_node: None,
            drag_start_pos: None,
            drag_start_time: std::time::Instant::now(),
            drag_start_node: None,
            is_dragging: false,
        }
    }

    pub fn push_element_to_root(&mut self, element: E, layout: Layout) {
        let node = self.layout_tree.push_to_root(layout, true); // TODO: Node context?
        self.elements.insert(node, element);
    }

    pub fn handle_resize(&mut self, handler: &mut impl GuiHandler<Element = E>, size: Vec2) {
        if size == self.size {
            return;
        }
        self.size = size;
        self.layout_tree.resize(size);
        self.layout_tree.do_layout(&mut Inner {
            handler,
            elements: &mut self.elements,
        });
        handler.on_resize(size);
    }

    pub fn handle_mouse_move(&mut self, handler: &mut impl GuiHandler<Element = E>, pos: Vec2) {
        if pos == self.mouse_pos {
            return;
        }
        self.mouse_pos = pos;

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
            if let Some(drag_element) = self.drag_start_node.as_ref()
                .and_then(|n| self.elements.get_mut(n))
            {
                if !self.is_dragging {
                    let dur_since = std::time::Instant::now()
                        .duration_since(self.drag_start_time);
                    if dur_since.as_secs_f64() > 0.1 {
                        // User is likely dragging.
                        self.is_dragging = true;
                        handler.on_drag_start(drag_element);
                    }
                }
                if self.is_dragging {
                    let delta = pos - drag_origin_pos;
                    handler.on_drag_update(
                        drag_element,
                        hover_changed_to,
                        delta,
                    );
                }
            }
        }

        if self.hovered_node != hover_changed_to {
            if let Some(left_node) = self.hovered_node.take() {
                if let Some(element) = self.elements.get_mut(&left_node) {
                    handler.on_mouse_leave(element);
                }
            }
            if let Some(entered_node) = hover_changed_to {
                if let Some(element) = self.elements.get_mut(&entered_node) {
                    handler.on_mouse_enter(element);
                }
                self.hovered_node = Some(entered_node);
            }
        }
    }

    pub fn handle_mouse_down(&mut self, handler: &mut impl GuiHandler<Element = E>) {
        if let Some(node) = self.hovered_node {
            if let Some(element) = self.elements.get_mut(&node) {
                handler.on_mouse_down(element);
            }
        }
        self.drag_start_time = std::time::Instant::now();
        self.drag_start_pos = Some(self.mouse_pos);
        self.drag_start_node = self.hovered_node.clone();
    }

    pub fn handle_mouse_up(&mut self, handler: &mut impl GuiHandler<Element = E>) {
        if let Some(node) = self.hovered_node {
            if let Some(element) = self.elements.get_mut(&node) {
                handler.on_mouse_up(element);
            }
        }
        self.drag_start_pos = None;
        if let Some(element) = self.drag_start_node.take()
            .and_then(|n| self.elements.get_mut(&n))
        {
            if self.is_dragging {
                self.is_dragging = false;
                handler.on_drag_end(element);
            }
        }
    }
}

struct Inner<'a, E> {
    handler: &'a mut dyn GuiHandler<Element = E>,
    elements: &'a mut HashMap<LayoutNode, E>,
}

impl<'a, E> LayoutHandler for Inner<'a, E> {
    fn on_layout(&mut self, node: LayoutNode, placement: &Placement) {
        let Some(element) = self.elements.get_mut(&node) else {
            println!("WARNING: Attempted to place element without context");
            return;
        };
        self.handler.on_element_layout(element, placement);
    }
}



pub trait GuiHandler {
    type Element;

    fn on_mouse_move(&mut self, pos: Vec2);
    fn on_mouse_enter(&mut self, element: &mut Self::Element);
    fn on_mouse_leave(&mut self, element: &mut Self::Element);
    fn on_mouse_down(&mut self, element: &mut Self::Element);
    fn on_mouse_up(&mut self, element: &mut Self::Element);
    fn on_drag_update(&mut self, element: &mut Self::Element, hovered: Option<LayoutNode>, delta: Vec2);
    fn on_drag_start(&mut self, element: &mut Self::Element);
    fn on_drag_end(&mut self, element: &mut Self::Element);
    fn on_resize(&mut self, size: Vec2);
    fn on_element_layout(&mut self, element: &mut Self::Element, placement: &Placement);
}
