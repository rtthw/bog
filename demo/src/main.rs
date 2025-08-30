


use bog::prelude::*;



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
    let root = Element::new(Widget::Null)
        .style(Style::new()
            .horizontal()
            .background_color(GRAY_1))
        .children(vec![
            Element::new(Widget::SidePanel)
                .style(Style::new().width(Length::Portion(0.2)))
                .event_mask(EventMask::CLICK | EventMask::FOCUS),
            Element::new(Widget::MainPanel)
                .style(Style::new().background_color(GRAY_2))
                .event_mask(EventMask::CLICK | EventMask::FOCUS),
        ]);

    run_simple_app(None, App {
        ui: UserInterface::new(root, Rect::new(vec2(0.0, 0.0), vec2(1200.0, 800.0))),
    })
}



struct App {
    ui: UserInterface<Widget>,
}

impl SimpleApp for App {
    type CustomEvent = ();

    fn render<'pass>(&'pass mut self, cx: AppContext, pass: &mut RenderPass<'pass>) {
        let area = cx.renderer.viewport_rect();
        pass.start_layer(area);
        self.ui.crawl(|ui, node| {
            let bounds = ui.bounds(node);
            let style = ui.style(node);

            pass.fill_quad(Quad {
                bounds,
                border: Border {
                    color: style.border_color,
                    width: style.border_width,
                    ..Default::default()
                },
                bg_color: style.background_color,
                ..Default::default()
            });
        });
        pass.end_layer();
    }

    fn input(&mut self, cx: AppContext, event: InputEvent) {
        cx.window.request_redraw();
        self.ui.handle_input(event);
        while let Some(event) = self.ui.next_event() {
            match event {
                Event::Focus { old, new } => {
                    if let Some(old) = old {
                        self.ui.update_style(old, |style| {
                            style.border_width = 0.0;
                        });
                    }
                    self.ui.update_style(new, |style| {
                        style.border_color = GRAY_6;
                        style.border_width = 2.0;
                    });
                }
                _ => {}
            }
        }
    }

    fn window_desc(&self) -> WindowDescriptor {
        WindowDescriptor {
            title: "Bog - Demo App",
            inner_size: vec2(1200.0, 800.0),
            ..Default::default()
        }
    }
}

enum Widget {
    Null,

    SidePanel,
    MainPanel,
}
