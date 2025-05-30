//! Builtin Elements



mod button;
pub use button::*;

mod panel;
pub use panel::*;

mod paragraph;
pub use paragraph::*;

mod scrollable;
pub use scrollable::*;



use core::marker::PhantomData;

use bog_layout::Layout;
use bog_render::Render as _;

use crate::{Element, Object, RenderContext, View};



/// Create a horizontal rule element.
pub fn horizontal_rule<V: View + 'static>(height: f32) -> Element<V> {
    Element::new()
        .layout(Layout::default()
            .fill_width()
            .height(height))
        .object(HorizontalRule {
            _view: PhantomData,
        })
}

struct HorizontalRule<V: View> {
    _view: PhantomData<V>,
}

impl<V: View> Object for HorizontalRule<V> {
    type View = V;

    fn render(&mut self, cx: RenderContext<Self::View>) {
        cx.layer_stack.fill_styled_quad(cx.placement.rect(), cx.style);
    }
}
