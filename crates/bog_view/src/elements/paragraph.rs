//! Paragraph elements



use core::marker::PhantomData;

use bog_math::{Rect, Vec2};
use bog_render::{Render as _, Renderer, Text};
use bog_style::ResolvedStyle;
use bog_window::CursorIcon;

use crate::{Element, EventContext, Object, RenderContext, View};



pub struct Paragraph {}



pub fn static_paragraph<V: View + 'static>(text: &'static str) -> Element<V> {
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
        cx.renderer.fill_text(Text::styled(self.text, cx.placement.rect(), cx.style));
        // cx.renderer.fill_quad(bog_render::Quad {
        //     bounds: cx.placement.rect(),
        //     border: bog_render::Border {
        //         color: bog_color::Color::from_u32(0xffffffff),
        //         width: 1.0,
        //         ..Default::default()
        //     },
        //     ..Default::default()
        // });
        // cx.renderer.fill_quad(bog_render::Quad {
        //     bounds: cx.placement.parent_rect(),
        //     border: bog_render::Border::new(bog_color::Color::from_u32(0xff0000ff), 1.0, 0.0),
        //     ..Default::default()
        // });
    }

    fn on_mouse_enter(&mut self, cx: EventContext<Self::View>) {
        cx.window.map(|w| w.set_cursor(CursorIcon::Text));
    }

    fn on_mouse_leave(&mut self, cx: EventContext<Self::View>) {
        cx.window.map(|w| w.set_cursor(CursorIcon::Default));
    }
}
