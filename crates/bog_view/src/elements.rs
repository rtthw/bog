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

impl<V: View> HorizontalRule<V> {
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
                _view: PhantomData,
            },
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.object.quad.bg_color = color;
        self
    }
}

impl<V: View + 'static> Into<Element<V>> for HorizontalRule<V> {
    fn into(self) -> Element<V> {
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



pub struct Scrollable<V: View> {
    inner: Element<V>,
    children: Vec<Element<V>>,
    object: ScrollableObject<V>,
}

impl<V: View> Scrollable<V> {
    pub fn new() -> Self {
        Self {
            inner: Element::new()
                .layout(Layout::default()
                    // .flex_auto()
                    // .flex_grow(1.0)
                    .overflow_scroll_y()
                    .flex_column()
                    .gap_y(7.0)
                    .padding(7.0)),
            children: Vec::with_capacity(1),
            object: ScrollableObject {
                quad: Quad {
                    bg_color: Color::new(73, 73, 83, 255),
                    border: Border {
                        width: 2.0,
                        color: Color::new(113, 113, 127, 255),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                _view: PhantomData,
            },
        }
    }

    /// Add the given children to this scrollable.
    pub fn children(
        mut self,
        children: impl IntoIterator<Item = impl Into<Element<V>>>,
    ) -> Self {
        self.children.extend(children.into_iter().map(|e| e.into()));
        self
    }
}

impl<V: View + 'static> Into<Element<V>> for Scrollable<V> {
    fn into(self) -> Element<V> {
        self.inner
            .children(self.children)
            .object(self.object)
    }
}

struct ScrollableObject<V: View> {
    quad: Quad,
    _view: PhantomData<V>,
}

impl<V: View> Object for ScrollableObject<V> {
    type View = V;

    fn render(&mut self, cx: RenderContext<Self::View>) {
        cx.renderer.fill_quad(bog_render::Quad {
            bounds: cx.placement.rect(),
            ..self.quad
        });
    }

    fn pre_render(&mut self, cx: RenderContext<Self::View>) {
        cx.renderer.start_layer(cx.placement.rect());
    }

    fn post_render(&mut self, cx: RenderContext<Self::View>) {
        cx.renderer.end_layer();
    }
}
