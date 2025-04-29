



use app::{run_app, AppContext, AppHandler};
use bog::*;
use collections::NoHashMap;
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
        elements: NoHashMap::with_capacity(7),
        drag_indicator: None,
    })?;

    Ok(())
}



struct Showcase {
    elements: NoHashMap<Node, Button>,
    drag_indicator: Option<Quad>,
}

impl AppHandler for Showcase {
    fn render(&mut self, renderer: &mut Renderer, tree: &mut LayoutTree, viewport_rect: Rect) {
        renderer.clear();

        { // Background layer.
            renderer.start_layer(viewport_rect);
            renderer.fill_quad(Quad {
                bounds: viewport_rect,
                border: Border::NONE,
                shadow: Shadow::NONE,
                bg_color: GRAY_0,
            });
            renderer.end_layer();
        }

        { // Main layer.
            renderer.start_layer(viewport_rect);
            // The `iter_placements` call will iterate bottom-up, so rendering each element through
            // this method is ideal.
            tree.iter_placements(&mut |node, _placement| {
                let Some(element) = self.elements.get(&node) else { return; };
                renderer.fill_quad(element.quad);
                renderer.fill_text(element.text.clone());
            });
            renderer.end_layer();
        }

        // Overlay layer.
        if let Some(drag_indicator) = &self.drag_indicator {
            renderer.start_layer(viewport_rect);
            renderer.fill_quad(*drag_indicator);
            renderer.end_layer();
        }
    }

    fn init(&mut self, ui: &mut Gui) {
        let left_panel_layout = Layout::default()
            .width_percent(0.2)
            .fill_height()
            .padding(11.0);
        let right_panel_layout = Layout::default()
            .flex_row()
            .flex_wrap()
            .fill_width()
            .fill_height()
            .gap_x(11.0)
            .padding(11.0)
            .align_items_center()
            .justify_content_center();

        let left_panel = ui.push_node_to_root(left_panel_layout);
        let right_panel = ui.push_node_to_root(right_panel_layout);

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
                color: GRAY_7,
                line_height: 50.0 * 1.2,
                font_family: FontFamily::Name("JetBrainsMono Nerd Font"),
                font_style: FontStyle::Normal,
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
                content: "RIGHT".to_string(),
                pos: Vec2::ZERO,
                size: 50.0,
                color: GRAY_7,
                line_height: 50.0 * 1.2,
                font_family: FontFamily::Name("JetBrainsMono Nerd Font"),
                font_style: FontStyle::Normal,
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
            let node = ui.push_node(right_panel, layout);
            self.elements.insert(node, simple_button("<>\n<-"));
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

    fn on_mouseover(&mut self, node: Node, cx: AppContext) {
        cx.graphics.window().request_redraw();
        let Some(button) = self.elements.get_mut(&node) else { return; };
        if !button.draggable { return; }
        if !cx.gui_cx.state.is_dragging {
            cx.graphics.window().set_cursor(CursorIcon::Pointer);
        }
        button.quad.bg_color = GRAY_7;
        button.text.size = 30.0;
    }

    fn on_mouseleave(&mut self, node: Node, cx: AppContext) {
        cx.graphics.window().request_redraw();
        let Some(button) = self.elements.get_mut(&node) else { return; };
        if !button.draggable { return; }
        if !cx.gui_cx.state.is_dragging {
            cx.graphics.window().set_cursor(CursorIcon::Default);
        }
        button.quad.bg_color = GRAY_6;
        button.quad.border.color = GRAY_5;
        button.text.size = 20.0;
    }

    fn on_mousedown(&mut self, node: Node, cx: AppContext) {
        cx.graphics.window().request_redraw();
        let Some(button) = self.elements.get_mut(&node) else { return; };
        if !button.draggable { return; }
        button.quad.bg_color = GRAY_3;
    }

    fn on_mouseup(&mut self, node: Node, cx: AppContext) {
        cx.graphics.window().request_redraw();
        let Some(button) = self.elements.get_mut(&node) else { return; };
        if !button.draggable { return; }
        button.quad.bg_color = GRAY_6;
    }

    fn on_dragmove(&mut self, node: Node, cx: AppContext, delta: Vec2, over: Option<Node>) {
        let Some(button) = self.elements.get(&node) else { return; };
        if !button.draggable { return; }
        cx.graphics.window().request_redraw();
        self.drag_indicator = Some(Quad {
            bounds: Rect::new(button.quad.bounds.position() + delta, button.quad.bounds.size()),
            ..button.quad
        });
        if let Some(button) = over.and_then(|e| self.elements.get_mut(&e)) {
            if !button.draggable { return; }
            button.quad.border.color = GRAY_9;
        }
    }

    fn on_dragstart(&mut self, node: Node, cx: AppContext) {
        let Some(button) = self.elements.get_mut(&node) else { return; };
        if !button.draggable { return; }
        cx.graphics.window().request_redraw();
        cx.graphics.window().set_cursor(CursorIcon::Grab);
    }

    fn on_dragend(&mut self, node: Node, cx: AppContext, over: Option<Node>) {
        self.drag_indicator = None;
        if let Some(over_node) = over {
            cx.gui_cx.tree.try_swap_nodes(node, over_node);
            // See: `impl LayoutHandler for Showcase`.
            cx.gui_cx.tree.do_layout(self);
        }
        cx.graphics.window().set_cursor(CursorIcon::Default);
        cx.graphics.window().request_redraw();
    }

    fn on_layout(&mut self, node: Node, placement: &Placement) {
        let Some(button) = self.elements.get_mut(&node) else { return; };
        button.quad.bounds = Rect::new(
            placement.position(),
            vec2(placement.layout.size.width, placement.layout.size.height),
        );
        button.text.pos = placement.content_position();
        button.text.bounds = placement.content_size();
    }
}

impl LayoutHandler for Showcase {
    fn on_layout(&mut self, node: Node, placement: &Placement) {
        AppHandler::on_layout(self, node, placement);
    }
}



struct Button {
    quad: Quad,
    text: Text,
    draggable: bool,
}

fn simple_button(text: &str) -> Button {
    Button {
        quad: Quad {
            // The bounds will be determined by the layout engine. You can put anything here.
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
            content: text.to_string(),
            pos: Vec2::ZERO,
            size: 20.0,
            color: GRAY_8,
            line_height: 20.0 * 1.2,
            font_family: FontFamily::Name("JetBrainsMono Nerd Font"),
            font_style: FontStyle::Normal,
            bounds: Vec2::new(100.0, 100.0),
        },
        draggable: true,
    }
}
