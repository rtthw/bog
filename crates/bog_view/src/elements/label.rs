//! Paragraph elements



use core::marker::PhantomData;

use bog_math::{Rect, Vec2};
use bog_render::{Render as _, Renderer, Text};
use bog_style::ResolvedStyle;
use bog_window::CursorIcon;

use crate::{Element, EventContext, Object, RenderContext, View};



/// A label with text that doesn't change.
pub fn static_label<V: View + 'static>(text: &'static str) -> Element<V> {
    Element::new()
        .object(StaticParagraph {
            text,
            _data: PhantomData,
        })
}

struct StaticParagraph<V: View> {
    text: &'static str,
    _data: PhantomData<V>,
}

impl<V: View> Object for StaticParagraph<V> {
    type View = V;

    fn measure(&self, available_space: Vec2, renderer: &mut Renderer, style: ResolvedStyle) -> Vec2 {
        renderer.measure_text(&Text::styled(
            self.text,
            Rect::new(Vec2::ZERO, available_space),
            &style,
        ))
    }

    fn render(&mut self, cx: RenderContext<Self::View>) {
        cx.layer_stack.fill_text(Text::styled(self.text, cx.placement.rect(), cx.style));
    }

    fn on_mouse_enter(&mut self, cx: EventContext<Self::View>) {
        cx.window.map(|w| w.set_cursor(CursorIcon::Text));
    }

    fn on_mouse_leave(&mut self, cx: EventContext<Self::View>) {
        cx.window.map(|w| w.set_cursor(CursorIcon::Default));
    }
}
