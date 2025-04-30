



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
    elements: NoHashMap<Node, Box<dyn Element>>,
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
                element.render(renderer);
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
            .width(200.0)
            .fill_height()
            .padding(7.0);
        let spacer_layout = Layout::default()
            .width(5.0)
            .fill_height();
        let right_panel_layout = Layout::default()
            .flex_row()
            .flex_wrap()
            .fill_width()
            .fill_height()
            .gap_x(11.0)
            .padding(7.0)
            .align_items_center()
            .justify_content_center();

        let left_panel = ui.push_node_to_root(left_panel_layout);
        let spacer = ui.push_node_to_root(spacer_layout);
        let right_panel = ui.push_node_to_root(right_panel_layout);

        self.elements.insert(left_panel, Box::new(Button {
            quad: Quad {
                bounds: Rect::new(Vec2::ZERO, vec2(10.0, 10.0)),
                border: Border {
                    color: GRAY_3,
                    width: 1.0,
                    radius: [3.0; 4],
                },
                shadow: Shadow::NONE,
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
        }));
        self.elements.insert(spacer, Box::new(Spacer {
            quad: Quad::new_colored(Rect::NONE, GRAY_6),
            left_panel,
        }));
        self.elements.insert(right_panel, Box::new(Button {
            quad: Quad {
                bounds: Rect::new(Vec2::ZERO, vec2(10.0, 10.0)),
                border: Border {
                    color: GRAY_3,
                    width: 1.0,
                    radius: [3.0; 4],
                },
                shadow: Shadow::NONE,
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
        }));
        for (index, layout) in [
            Layout::default().width(40.0).height(70.0).padding(7.0),
            Layout::default().width(30.0).height(70.0).padding(7.0),
            Layout::default().width(90.0).height(70.0).padding(7.0),
            Layout::default().width(70.0).height(70.0).padding(7.0),
            Layout::default().width(50.0).height(70.0).padding(7.0),
        ]
            .into_iter().enumerate()
        {
            let node = ui.push_node(right_panel, layout);
            self.elements.insert(node, Box::new(draggable_button(&format!("{}", index + 1))));
        }
    }

    fn title(&self) -> &str {
        "Bog Showcase"
    }

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
        let Some(element) = self.elements.get_mut(&node) else { return; };
        if !element.draggable() { return; }
        if !cx.gui_cx.state.is_dragging {
            cx.graphics.window().set_cursor(CursorIcon::Pointer);
        }
        element.on_mouseover(cx.gui_cx.state.is_dragging);
    }

    fn on_mouseleave(&mut self, node: Node, cx: AppContext) {
        cx.graphics.window().request_redraw();
        let Some(element) = self.elements.get_mut(&node) else { return; };
        if !element.draggable() { return; }
        if !cx.gui_cx.state.is_dragging {
            cx.graphics.window().set_cursor(CursorIcon::Default);
        }
        element.on_mouseleave(cx.gui_cx.state.is_dragging);
    }

    fn on_mousedown(&mut self, node: Node, cx: AppContext) {
        cx.graphics.window().request_redraw();
        let Some(element) = self.elements.get_mut(&node) else { return; };
        if !element.draggable() { return; }
        element.on_mousedown();
    }

    fn on_mouseup(&mut self, node: Node, cx: AppContext) {
        cx.graphics.window().request_redraw();
        let Some(element) = self.elements.get_mut(&node) else { return; };
        if !element.draggable() { return; }
        element.on_mouseup();
    }

    fn on_dragmove(&mut self, node: Node, mut cx: AppContext, delta: Vec2, over: Option<Node>) {
        let Some(element) = self.elements.get_mut(&node) else { return; };
        if !element.draggable() { return; }
        element.on_dragmove(&mut cx, delta, &mut self.drag_indicator);
        cx.graphics.window().request_redraw();
        if let Some(button) = over.and_then(|e| self.elements.get_mut(&e)) {
            if !button.draggable() { return; }
            button.on_mouseover(true);
        }
    }

    fn on_dragstart(&mut self, node: Node, cx: AppContext) {
        let Some(element) = self.elements.get_mut(&node) else { return; };
        if !element.draggable() { return; }
        element.on_dragstart(cx);
    }

    fn on_dragend(&mut self, node: Node, mut cx: AppContext, over: Option<Node>) {
        self.drag_indicator = None;
        let Some(element) = self.elements.get_mut(&node) else { return; };
        if !element.draggable() { return; }
        if element.on_dragend(&mut cx, node, over) {
            // See: `impl LayoutHandler for Showcase`.
            cx.gui_cx.tree.do_layout(self);
        }
        cx.graphics.window().set_cursor(CursorIcon::Default);
        cx.graphics.window().request_redraw();
    }

    fn on_layout(&mut self, node: Node, placement: &Placement) {
        let Some(element) = self.elements.get_mut(&node) else { return; };
        element.on_layout(placement);
    }
}

impl LayoutHandler for Showcase {
    fn on_layout(&mut self, node: Node, placement: &Placement) {
        AppHandler::on_layout(self, node, placement);
    }
}



#[allow(unused)]
trait Element {
    fn render(&self, renderer: &mut Renderer);
    fn on_layout(&mut self, placement: &Placement);

    fn draggable(&self) -> bool { false }

    fn on_mouseover(&mut self, dragging: bool);
    fn on_mouseleave(&mut self, dragging: bool);
    fn on_mousedown(&mut self);
    fn on_mouseup(&mut self);

    fn on_dragmove(&mut self, cx: &mut AppContext, delta: Vec2, drag_indicator: &mut Option<Quad>) {}
    fn on_dragstart(&mut self, cx: AppContext) {}
    fn on_dragend(&mut self, cx: &mut AppContext, this: Node, over: Option<Node>) -> bool { false }
}



struct Button {
    quad: Quad,
    text: Text,
    draggable: bool,
}

impl Element for Button {
    fn render(&self, renderer: &mut Renderer) {
        renderer.fill_quad(self.quad);
        renderer.fill_text(self.text.clone());
    }

    fn on_layout(&mut self, placement: &Placement) {
        self.quad.bounds = Rect::new(
            placement.position(),
            vec2(placement.layout.size.width, placement.layout.size.height),
        );
        self.text.pos = placement.content_position();
        self.text.bounds = placement.content_size();
    }

    fn draggable(&self) -> bool {
        self.draggable
    }

    fn on_mouseover(&mut self, dragging: bool) {
        self.quad.bg_color = GRAY_7;
        self.text.size = 30.0;
        if dragging {
            self.quad.border.color = GRAY_9;
        }
    }

    fn on_mouseleave(&mut self, _dragging: bool) {
        self.quad.bg_color = GRAY_6;
        self.quad.border.color = GRAY_5;
        self.text.size = 20.0;
    }

    fn on_mousedown(&mut self) {
        self.quad.bg_color = GRAY_3;
    }

    fn on_mouseup(&mut self) {
        self.quad.bg_color = GRAY_6;
    }

    fn on_dragmove(&mut self, _cx: &mut AppContext, delta: Vec2, drag_indicator: &mut Option<Quad>) {
        *drag_indicator = Some(Quad {
            bounds: Rect::new(self.quad.bounds.position() + delta, self.quad.bounds.size()),
            border: Border::NONE,
            shadow: Shadow::NONE,
            bg_color: GRAY_0.with_alpha(137),
        });
    }

    fn on_dragstart(&mut self, cx: AppContext) {
        cx.graphics.window().request_redraw();
        cx.graphics.window().set_cursor(CursorIcon::Grab);
    }

    fn on_dragend(&mut self, cx: &mut AppContext, this: Node, over: Option<Node>) -> bool {
        if let Some(other) = over {
            cx.gui_cx.tree.try_swap_nodes(this, other);

            true
        } else {
            false
        }
    }
}

fn draggable_button(text: &str) -> Button {
    Button {
        quad: Quad {
            // The bounds will be determined by the layout engine. You can put anything here.
            bounds: Rect::new(Vec2::ZERO, vec2(10.0, 10.0)),
            border: Border {
                color: GRAY_5,
                width: 2.0,
                radius: [5.0, 5.0, 5.0, 5.0],
            },
            shadow: Shadow {
                color: GRAY_3,
                offset: vec2(2.0, 3.0),
                blur_radius: 5.0,
            },
            bg_color: GRAY_6,
        },
        text: Text {
            content: text.to_string(),
            pos: Vec2::ZERO,
            size: 20.0,
            color: GRAY_1,
            line_height: 20.0 * 1.2,
            font_family: FontFamily::Name("JetBrainsMono Nerd Font"),
            font_style: FontStyle::Normal,
            bounds: Vec2::new(100.0, 100.0),
        },
        draggable: true,
    }
}



struct Spacer {
    quad: Quad,
    left_panel: Node,
}

impl Element for Spacer {
    fn render(&self, renderer: &mut Renderer) {
        renderer.fill_quad(self.quad);
    }

    fn on_layout(&mut self, placement: &Placement) {
        self.quad.bounds = Rect::new(
            placement.position(),
            vec2(placement.layout.size.width, placement.layout.size.height),
        );
    }

    fn draggable(&self) -> bool { true }

    fn on_mouseover(&mut self, _dragging: bool) {
        self.quad.bg_color = GRAY_7;
    }

    fn on_mouseleave(&mut self, _dragging: bool) {
        self.quad.bg_color = GRAY_6;
    }

    fn on_mousedown(&mut self) {
        self.quad.bg_color = GRAY_3;
    }

    fn on_mouseup(&mut self) {
        self.quad.bg_color = GRAY_6;
    }

    fn on_dragmove(&mut self, _cx: &mut AppContext, delta: Vec2, drag_indicator: &mut Option<Quad>) {
        let drag_pos_x = self.quad.bounds.position().x + delta.x;
        *drag_indicator = Some(Quad {
            bounds: Rect::new(
                Vec2::new(drag_pos_x, self.quad.bounds.position().y),
                self.quad.bounds.size(),
            ),
            border: Border::NONE,
            shadow: Shadow::NONE,
            bg_color: GRAY_0.with_alpha(137),
        });
    }

    fn on_dragstart(&mut self, cx: AppContext) {
        cx.graphics.window().request_redraw();
        cx.graphics.window().set_cursor(CursorIcon::ColResize);
    }

    fn on_dragend(&mut self, cx: &mut AppContext, _this: Node, _over: Option<Node>) -> bool {
        let left_panel_layout = cx.gui_cx.tree.get_node_layout(self.left_panel);
        if let Some(width_len) = left_panel_layout.get_width() {
            cx.gui_cx.tree.set_node_layout(
                self.left_panel,
                left_panel_layout.width(width_len
                    + (cx.gui_cx.state.mouse_pos.x - self.quad.bounds.position().x)),
            );

            true
        } else {
            false
        }
    }
}
