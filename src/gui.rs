//! GUI



use std::collections::HashMap;

use crate::{layout::*, math::{vec2, Vec2}};



pub struct Gui<E> {
    layout_tree: LayoutTree,
    elements: HashMap<LayoutNode, E>,

    size: Vec2,
    mouse_pos: Vec2,
    hovered_node: Option<LayoutNode>,
}

impl<E> Gui<E> {
    pub fn new() -> Self {
        Self {
            layout_tree: LayoutTree::new(Layout::default()),
            elements: HashMap::new(),
            size: vec2(0.0, 0.0),
            mouse_pos: vec2(0.0, 0.0),
            hovered_node: None,
        }
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
    fn on_resize(&mut self, size: Vec2);
    fn on_element_layout(&mut self, element: &mut Self::Element, placement: &Placement);
}
