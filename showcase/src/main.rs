


use std::collections::HashMap;

use app::{run_app, AppContext, AppHandler};
use bog::*;
use color::*;
use gui::*;
use layout::*;
use math::*;
use render::*;
use window::*;



pub const GRAY_0: Color = Color::new(13, 13, 23, 255); // 0d0d17
pub const GRAY_1: Color = Color::new(29, 29, 39, 255); // 1d1d27
pub const GRAY_2: Color = Color::new(43, 43, 53, 255); // 2b2b35
pub const GRAY_3: Color = Color::new(59, 59, 67, 255); // 3b3b43
pub const GRAY_4: Color = Color::new(73, 73, 83, 255); // 494953
pub const GRAY_5: Color = Color::new(89, 89, 109, 255); // 59596d
pub const GRAY_6: Color = Color::new(113, 113, 127, 255); // 71717f
pub const GRAY_7: Color = Color::new(139, 139, 149, 255); // 8b8b95
pub const GRAY_8: Color = Color::new(163, 163, 173, 255); // a3a3ad
pub const GRAY_9: Color = Color::new(191, 191, 197, 255); // bfbfc5



fn main() -> Result<()> {
    run_app(Showcase {
        elements: HashMap::with_capacity(7),
        drag_indicator: None,
    })?;

    Ok(())
}



struct Showcase {
    elements: HashMap<Element, Button>,
    drag_indicator: Option<Quad>,
}

impl AppHandler for Showcase {
    fn render(&mut self, renderer: &mut Renderer, viewport_rect: Rect) {
        renderer.clear();
        renderer.start_layer(viewport_rect);
        renderer.fill_quad(Quad {
            bounds: viewport_rect,
            border: Border::NONE,
            shadow: Shadow::NONE,
            bg_color: GRAY_0,
        });
        renderer.end_layer();
        renderer.start_layer(viewport_rect);
        for button in self.elements.values() {
            renderer.fill_quad(button.quad);
        }
        renderer.end_layer();
        renderer.start_layer(viewport_rect);
        for button in self.elements.values() {
            renderer.fill_text(button.text.clone());
        }
        renderer.end_layer();
        if let Some(drag_indicator) = &self.drag_indicator {
        renderer.start_layer(viewport_rect);
            renderer.fill_quad(*drag_indicator);
            renderer.end_layer();
        }
    }

    fn init(&mut self, ui: &mut Gui) {
        let left_panel_layout = Layout::default()
            .width_percent(0.2)
            .fill_height();
        let right_panel_layout = Layout::default()
            .flex_row()
            .flex_wrap()
            .fill_width()
            .fill_height()
            .gap_x(11.0)
            .align_items_center()
            .justify_content_center();

        let left_panel = ui.push_element_to_root(left_panel_layout);
        let right_panel = ui.push_element_to_root(right_panel_layout);

        self.elements.insert(left_panel, Button {
            quad: Quad {
                bounds: Rect::new(Vec2::ZERO, vec2(10.0, 10.0)),
                border: Border {
                    color: GRAY_5,
                    width: 3.0,
                    radius: [7.0, 3.0, 11.0, 19.0],
                },
                shadow: Shadow {
                    color: GRAY_3,
                    offset: vec2(2.0, 5.0),
                    blur_radius: 3.0,
                },
                bg_color: GRAY_2,
            },
            text: Text {
                content: "LEFT".to_string(),
                pos: Vec2::ZERO,
                size: 50.0,
                color: GRAY_8,
                line_height: 50.0 * 1.2,
                font_family: FontFamily::Name("JetBrainsMono Nerd Font"),
                font_style: FontStyle::Italic,
                bounds: Vec2::new(100.0, 100.0),
            },
            draggable: false,
        });
        self.elements.insert(right_panel, Button {
            quad: Quad {
                bounds: Rect::new(Vec2::ZERO, vec2(10.0, 10.0)),
                border: Border {
                    color: GRAY_5,
                    width: 3.0,
                    radius: [7.0, 3.0, 11.0, 19.0],
                },
                shadow: Shadow {
                    color: GRAY_3,
                    offset: vec2(2.0, 5.0),
                    blur_radius: 3.0,
                },
                bg_color: GRAY_2,
            },
            text: Text {
                content: "LEFT".to_string(),
                pos: Vec2::ZERO,
                size: 50.0,
                color: GRAY_8,
                line_height: 50.0 * 1.2,
                font_family: FontFamily::Name("JetBrainsMono Nerd Font"),
                font_style: FontStyle::Italic,
                bounds: Vec2::new(100.0, 100.0),
            },
            draggable: false,
        });
        for layout in [
            Layout::default().width(70.0).height(50.0).padding(5.0),
            Layout::default().width(100.0).height(30.0).padding(5.0),
            Layout::default().width(50.0).height(70.0).padding(5.0),
            Layout::default().width(40.0).height(70.0).padding(5.0),
            Layout::default().width(20.0).height(40.0).padding(5.0),
        ] {
            let element = ui.push_element(right_panel, layout);
            self.elements.insert(element, Button {
                quad: Quad {
                    bounds: Rect::new(Vec2::ZERO, vec2(10.0, 10.0)),
                    border: Border {
                        color: GRAY_5,
                        width: 3.0,
                        radius: [7.0, 3.0, 11.0, 19.0],
                    },
                    shadow: Shadow {
                        color: GRAY_3,
                        offset: vec2(2.0, 5.0),
                        blur_radius: 3.0,
                    },
                    bg_color: GRAY_6,
                },
                text: Text {
                    content: "=>".to_string(),
                    pos: Vec2::ZERO,
                    size: 20.0,
                    color: GRAY_6,
                    line_height: 20.0 * 1.2,
                    font_family: FontFamily::Name("JetBrainsMono Nerd Font"),
                    font_style: FontStyle::Normal,
                    bounds: Vec2::new(100.0, 100.0),
                },
                draggable: true,
            });
        }
    }

    fn title(&self) -> &str { "Bog Showcase" }

    fn root_layout(&self) -> Layout {
        Layout::default()
            .fill_width()
            .fill_height()
            .gap_x(11.0)
            .padding(11.0)
    }

    fn on_resize(&mut self, _size: Vec2) {}

    fn on_mousemove(&mut self, _pos: Vec2) {}

    fn on_mouseover(&mut self, element: Element, cx: AppContext) {
        cx.graphics.window().request_redraw();
        if !cx.gui_cx.state.is_dragging {
            cx.graphics.window().set_cursor(CursorIcon::Pointer);
        }
        let Some(button) = self.elements.get_mut(&element) else { return; };
        if !button.draggable { return; }
        button.quad.bg_color = GRAY_7;
        button.text.size = 30.0;
    }

    fn on_mouseleave(&mut self, element: Element, cx: AppContext) {
        cx.graphics.window().request_redraw();
        if !cx.gui_cx.state.is_dragging {
            cx.graphics.window().set_cursor(CursorIcon::Default);
        }
        let Some(button) = self.elements.get_mut(&element) else { return; };
        if !button.draggable { return; }
        button.quad.bg_color = GRAY_6;
        button.quad.border.color = GRAY_5;
        button.text.size = 20.0;
    }

    fn on_mousedown(&mut self, element: Element, cx: AppContext) {
        cx.graphics.window().request_redraw();
        let Some(button) = self.elements.get_mut(&element) else { return; };
        if !button.draggable { return; }
        button.quad.bg_color = GRAY_3;
    }

    fn on_mouseup(&mut self, element: Element, cx: AppContext) {
        cx.graphics.window().request_redraw();
        let Some(button) = self.elements.get_mut(&element) else { return; };
        if !button.draggable { return; }
        button.quad.bg_color = GRAY_6;
    }

    fn on_dragmove(
        &mut self,
        element: Element,
        cx: AppContext,
        delta: Vec2,
        hovered: Option<Element>,
    ) {
        cx.graphics.window().request_redraw();
        let Some(button) = self.elements.get(&element) else { return; };
        if !button.draggable { return; }
        self.drag_indicator = Some(Quad {
            bounds: Rect::new(button.quad.bounds.position() + delta, button.quad.bounds.size()),
            ..button.quad
        });
        if let Some(button) = hovered.and_then(|e| self.elements.get_mut(&e)) {
            if !button.draggable { return; }
            button.quad.border.color = GRAY_9;
        }
    }

    fn on_dragstart(&mut self, _element: Element, cx: AppContext) {
        cx.graphics.window().request_redraw();
        cx.graphics.window().set_cursor(CursorIcon::Grab);
    }

    fn on_dragend(&mut self, _element: Element, cx: AppContext) {
        cx.graphics.window().request_redraw();
        cx.graphics.window().set_cursor(CursorIcon::Default);
        self.drag_indicator = None;
    }

    fn on_layout(&mut self, element: Element, placement: &Placement) {
        let Some(button) = self.elements.get_mut(&element) else { return; };
        button.quad.bounds = Rect::new(
            placement.position(),
            vec2(placement.layout.size.width, placement.layout.size.height),
        );
        button.text.pos = placement.content_position();
        button.text.bounds = placement.content_size();
    }
}



struct Button {
    quad: Quad,
    text: Text,
    draggable: bool,
}
