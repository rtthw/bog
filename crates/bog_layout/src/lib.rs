//! Bog Layout

// TODO: #![no_std]



mod layout;
pub mod tree;

pub use layout::Layout;
pub use tree::*;



pub trait LayoutHandler {
    fn on_layout(&mut self, node: u64, placement: &Placement);
}
