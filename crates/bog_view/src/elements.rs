//! Builtin Elements



mod button;
pub use button::*;

mod paragraph;
pub use paragraph::*;

mod scrollable;
pub use scrollable::*;



use core::marker::PhantomData;

use bog_layout::Layout;
use bog_render::Render as _;

use crate::{Element, Object, RenderContext, View};



pub struct HorizontalRule<V: View> {
    inner: Element<V>,
    object: HorizontalRuleObject<V>,
}

impl<V: View> HorizontalRule<V> {
    pub fn new() -> Self {
        Self {
            inner: Element::new()
                .layout(Layout::default()
                    .fill_width()
                    .height(3.0)),
            object: HorizontalRuleObject {
                _view: PhantomData,
            },
        }
    }
}

impl<V: View + 'static> Into<Element<V>> for HorizontalRule<V> {
    fn into(self) -> Element<V> {
        self.inner
            .object(self.object)
    }
}

struct HorizontalRuleObject<V: View> {
    _view: PhantomData<V>,
}

impl<V: View> Object for HorizontalRuleObject<V> {
    type View = V;

    fn render(&mut self, cx: RenderContext<Self::View>) {
        cx.renderer.fill_styled_quad(cx.placement.rect(), cx.style);
    }
}
