//! GUI



use crate::{layout::*, math::Vec2};



pub struct Gui {
    layout_tree: LayoutTree,
}

impl Gui {
    pub fn new(layout_tree: LayoutTree) -> Self {
        Self {
            layout_tree,
        }
    }

    pub fn handle_mouse_move(&mut self, handler: &mut impl GuiHandler, pos: Vec2) {
        handler.on_mouse_move(pos);
    }
}



pub trait GuiHandler {
    fn on_mouse_move(&mut self, pos: Vec2);
}
