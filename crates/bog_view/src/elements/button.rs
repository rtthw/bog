//! Button element



use core::marker::PhantomData;

use bog_color::Color;
use bog_layout::Layout;
use bog_render::{Quad, Render as _};
use bog_window::CursorIcon;

use crate::{Element, EventContext, Object, View};



pub struct Button<V: View> {
    content: Element<V>,
    on_click: Option<Box<dyn Fn(EventContext<V>)>>,
}

impl<V: View + 'static> Button<V> {
    pub fn new(content: Element<V>) -> Self {
        Self {
            content,
            on_click: None,
        }
    }

    pub fn on_click(mut self, callback: impl Fn(EventContext<V>) + 'static) -> Self {
        self.on_click = Some(Box::new(callback));
        self
    }
}

impl<V: View + 'static> Into<Element<V>> for Button<V> {
    fn into(self) -> Element<V> {
        Element::new()
            .object(ButtonObject {
                on_click: self.on_click,
                bg_color: Color::new(89, 89, 109, 255),
                _view: PhantomData,
            })
            .layout(Layout::default()
                .align_items_center()
                .justify_content_center())
            .child(self.content)
    }
}

struct ButtonObject<V: View> {
    on_click: Option<Box<dyn Fn(EventContext<V>)>>,
    bg_color: Color,
    _view: PhantomData<V>,
}

impl<V: View> Object for ButtonObject<V> {
    type View = V;

    fn render(&mut self, cx: crate::RenderContext<Self::View>) {
        cx.renderer.fill_styled_quad(cx.placement.rect(), cx.style);
    }

    fn on_mouse_enter(&mut self, cx: EventContext<Self::View>) {
        self.bg_color = Color::new(113, 113, 127, 255);
        cx.window.map(|w| w.set_cursor(CursorIcon::Pointer));
    }

    fn on_mouse_leave(&mut self, cx: EventContext<Self::View>) {
        self.bg_color = Color::new(89, 89, 109, 255);
        cx.window.map(|w| w.set_cursor(CursorIcon::Default));
    }

    fn on_mouse_down(&mut self, cx: EventContext<Self::View>) {
        self.on_click.as_mut().map(|cb| (cb)(cx));
    }
}
