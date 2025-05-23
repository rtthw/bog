//! Scrollable element



use core::marker::PhantomData;

use bog_color::Color;
use bog_event::WheelMovement;
use bog_layout::Layout;
use bog_math::{mat4_translation, vec2, vec3};
use bog_render::{Border, Quad, Render as _};

use crate::{Element, Object, View};



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
                    .overflow_scroll_y()
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
                v_offset: 0.0,
                content_height: 0.0,
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
            // NOTE: We need this containner child because we set its offset. The top-level
            //       scrollable element needs to remain in place for accurate interactions.
            .child(Element::new()
                .layout(Layout::default()
                    .fill_width()
                    .fill_height()
                    .overflow_scroll_y()
                    .flex_column()
                    .gap_y(7.0))
                .children(self.children))
            .object(self.object)
    }
}

struct ScrollableObject<V: View> {
    quad: Quad,
    v_offset: f32,
    content_height: f32,
    _view: PhantomData<V>,
}

impl<V: View> Object for ScrollableObject<V> {
    type View = V;

    // fn render(&mut self, cx: RenderContext<Self::View>) {
    // }

    fn on_mouse_enter(&mut self, _cx: crate::EventContext<Self::View>) {
        self.quad.border.color = Color::new(139, 139, 149, 255);
    }

    fn on_mouse_leave(&mut self, _cx: crate::EventContext<Self::View>) {
        self.quad.border.color = Color::new(113, 113, 127, 255);
    }

    fn pre_render(&mut self, cx: crate::RenderContext<Self::View>) {
        cx.renderer.fill_quad(bog_render::Quad {
            bounds: cx.placement.rect(),
            ..self.quad
        });
        // cx.renderer.fill_quad(bog_render::Quad {
        //     bounds: cx.placement.parent_rect(),
        //     border: Border::new(Color::from_u32(0xff0000ff), 1.0, 0.0),
        //     ..Default::default()
        // });
        self.content_height = cx.placement.content_size().y;
        cx.renderer.start_layer(cx.placement.inner_rect());
        cx.renderer.start_transform(mat4_translation(vec3(0.0, -self.v_offset, 0.0)));
    }

    fn post_render(&mut self, cx: crate::RenderContext<Self::View>) {
        cx.renderer.end_transform();
        cx.renderer.end_layer();
    }

    fn on_wheel(&mut self, mut cx: crate::EventContext<Self::View>) {
        cx.stop_propagation();
        if let Some(movement) = cx.model.take_wheel_movement() {
            // println!("Scrolling by {:?}", movement);
            let prev_offset = self.v_offset;
            match movement {
                WheelMovement::Lines { y, .. } => {
                    self.v_offset = (self.v_offset + (-y * 20.0))
                        .min(self.content_height)
                        .max(0.0);
                }
                WheelMovement::Pixels { y, .. } => {
                    self.v_offset = (self.v_offset + y)
                        .min(self.content_height)
                        .max(0.0);
                }
            }
            if prev_offset != self.v_offset {
                let container_node = cx.layout_map.children(cx.node)[0];
                cx.layout_map.set_offset(container_node, vec2(0.0, -self.v_offset));
                // FIXME: Don't perform this check when there is no window.
                cx.window.map(|w| w.request_redraw());
            }
        }
    }
}
