//! Panel element



use core::marker::PhantomData;

use bog_render::Render;

use crate::{Element, Object, View};



/// A simple container [`Element`] with nothing but an [`Object`] that will render a styled
/// [`Quad`](bog_render::Quad).
pub fn panel<V: View + 'static>() -> Element<V> {
    Element::new()
        .object(Panel {
            _data: PhantomData,
        })
}

struct Panel<V: View> {
    _data: PhantomData<V>,
}

impl<V: View> Object for Panel<V> {
    type View = V;

    fn render(&mut self, cx: crate::RenderContext<Self::View>) {
        cx.layer_stack.fill_styled_quad(cx.placement.rect(), cx.style);
    }
}
