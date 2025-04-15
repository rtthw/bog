//! GUI



use std::collections::HashMap;

use crate::{layout::*, math::{vec2, Vec2}};



pub struct Gui {
    layout_tree: LayoutTree,
    elements: HashMap<LayoutNode, ElementContext>,

    size: Vec2,
    mouse_pos: Vec2,
}

impl Gui {
    pub fn new() -> Self {
        Self {
            layout_tree: LayoutTree::new(Layout::default()),
            elements: HashMap::new(),
            size: vec2(0.0, 0.0),
            mouse_pos: vec2(0.0, 0.0),
        }
    }

    pub fn handle_resize(&mut self, handler: &mut impl GuiHandler, size: Vec2) {
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

    pub fn handle_mouse_move(&mut self, handler: &mut impl GuiHandler, pos: Vec2) {
        if pos == self.mouse_pos {
            return;
        }
        self.mouse_pos = pos;
        handler.on_mouse_move(pos);
    }
}

struct Inner<'a> {
    handler: &'a mut dyn GuiHandler,
    elements: &'a mut HashMap<LayoutNode, ElementContext>,
}

impl<'a> LayoutHandler for Inner<'a> {
    fn on_layout(&mut self, node: LayoutNode, placement: &Placement) {
        let Some(element) = self.elements.get_mut(&node) else {
            println!("WARNING: Attempted to place element without context");
            return;
        };
    }
}



pub trait GuiHandler {
    fn on_mouse_move(&mut self, pos: Vec2);
    fn on_resize(&mut self, size: Vec2);
}



pub struct ElementContext {}
