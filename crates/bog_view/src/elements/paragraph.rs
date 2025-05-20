//! Paragraph elements



use core::marker::PhantomData;

use bog_layout::Layout;
use bog_math::Vec2;
use bog_render::{Render as _, Renderer, Text};
use bog_window::CursorIcon;

use crate::{Element, EventContext, Object, RenderContext, View};



pub struct Paragraph {}



pub fn static_paragraph<V: View + 'static>(text: Text, layout: Layout) -> Element<V> {
    Element::new()
        .object(StaticParagraph {
            text,
            _data: PhantomData,
        })
        .layout(layout)
}

struct StaticParagraph<V: View> {
    text: Text,
    _data: PhantomData<V>,
}

impl<V: View> Object for StaticParagraph<V> {
    type View = V;

    fn measure(&self, available_space: Vec2, renderer: &mut Renderer) -> Vec2 {
        renderer.measure_text(&Text {
            bounds: available_space,
            ..self.text.clone()
        })
    }

    fn render(&mut self, cx: RenderContext<Self::View>) {
        cx.renderer.fill_text(Text {
            pos: cx.placement.inner_position(),
            bounds: cx.placement.size(),
            ..self.text.clone()
        });
        // cx.renderer.fill_quad(bog_render::Quad {
        //     bounds: cx.placement.rect(),
        //     border: bog_render::Border {
        //         color: bog_color::Color::from_u32(0xffffffff),
        //         width: 1.0,
        //         ..Default::default()
        //     },
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
