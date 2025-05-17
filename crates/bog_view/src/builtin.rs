//! Builtin Elements



use core::marker::PhantomData;

use bog_color::Color;
use bog_layout::Layout;
use bog_math::Vec2;
use bog_render::{Border, Quad, Render as _, Renderer, Text};
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
            pos: cx.placement.position(),
            bounds: cx.placement.size(),
            ..self.text.clone()
        });
        // cx.renderer.fill_quad(Quad {
        //     bounds: cx.placement.rect(),
        //     border: Border {
        //         color: Color::from_u32(0xffffffff),
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



pub struct HorizontalRule<V: View> {
    inner: Element<V>,
    object: HorizontalRuleObject<V>,
}

impl<V: View + 'static> HorizontalRule<V> {
    pub fn new() -> Self {
        Self {
            inner: Element::new()
                .layout(Layout::default()
                    .fill_width()
                    .height(3.0)),
            object: HorizontalRuleObject {
                quad: Quad {
                    bg_color: Color::new(139, 139, 149, 255),
                    border: Border {
                        radius: [3.0; 4],
                        ..Default::default()
                    },
                    ..Default::default()
                },
                _view: PhantomData
            },
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.object.quad.bg_color = color;
        self
    }

    pub fn into(self) -> Element<V> {
        self.inner
            .object(self.object)
    }
}

struct HorizontalRuleObject<V: View> {
    quad: Quad,
    _view: PhantomData<V>,
}

impl<V: View> Object for HorizontalRuleObject<V> {
    type View = V;

    fn render(&mut self, cx: RenderContext<Self::View>) {
        cx.renderer.fill_quad(Quad {
            bounds: cx.placement.rect(),
            ..self.quad
        });
    }
}
