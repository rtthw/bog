



use app::*;
use bog::*;
use collections::NoHashMap;
use color::*;
use ui::*;
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
        elements: NoHashMap::with_capacity(32),
        drag_indicator: None,
    })?;

    Ok(())
}



struct Showcase {
    elements: NoHashMap<Node, Box<dyn Element>>,
    drag_indicator: Option<Quad>,
}

impl AppHandler for Showcase {
    fn render(
        &mut self,
        renderer: &mut Renderer,
        tree: &mut LayoutTree,
        viewport_rect: Rect,
    ) {
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
            // tree.iter_placements(&mut |node, placement| {
            //     let Some(element) = self.elements.get(&node) else { return; };
            //     element.render(renderer, placement, viewport_rect);
            // });
            renderer.end_layer();
        }

        // Overlay layer.
        if let Some(drag_indicator) = &self.drag_indicator {
            renderer.start_layer(viewport_rect);
            renderer.fill_quad(*drag_indicator);
            renderer.end_layer();
        }
    }

    fn view(&mut self) -> View {
        View::new(Element::new()
            .layout(Layout::default()
                .width(1280.0)
                .height(720.0)
                .gap_x(11.0)
                .padding(11.0))
            .child(Element::new()
                .layout(Layout::default()
                    .flex_initial()
                    .width(300.0)
                    .padding(7.0)))
            .child(Element::new()
                .layout(Layout::default()
                    .flex_auto()
                    .flex_wrap()
                    .gap_x(11.0)
                    .padding(7.0)
                    .align_items_center()
                    .justify_content_center())))

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
            Layout::default().width(40.0).height(40.0).padding(7.0),
            Layout::default().width(45.0).height(40.0).padding(7.0),
            Layout::default().width(55.0).height(50.0).padding(7.0),
            Layout::default().width(50.0).height(40.0).padding(7.0),
            Layout::default().width(45.0).height(45.0).padding(7.0),
            Layout::default().width(50.0).height(45.0).padding(7.0),
            Layout::default().width(50.0).height(55.0).padding(7.0),
        ]
            .into_iter().enumerate()
        {
            let node = ui.push_node(right_panel, layout);
            self.elements.insert(node, Box::new(draggable_button(&format!("{}", index + 1))));
        }
    }

    fn window_desc(&self) -> WindowDescriptor {
        WindowDescriptor {
            title: "Bog Showcase",
            inner_size: Vec2::new(1280.0, 720.0),
            ..Default::default()
        }
    }
}

impl LayoutHandler for Showcase {
    fn on_layout(&mut self, node: Node, placement: &Placement) {
        AppHandler::on_layout(self, node, placement);
    }
}
