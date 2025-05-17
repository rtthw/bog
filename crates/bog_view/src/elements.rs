//! Builtin Elements



mod button;
pub use button::*;

mod paragraph;
pub use paragraph::*;



use core::marker::PhantomData;

use bog_color::Color;
use bog_layout::Layout;
use bog_render::{Border, Quad, Render as _};

use crate::{Element, Object, RenderContext, View};



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
