


use bog::{prelude::*, view::{Element, Model, Object, View}};



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
    run_app(App {
        drag_indicator: None,
    })?;

    Ok(())
}



struct App {
    drag_indicator: Option<Quad>,
}

impl View for App {
    fn build(&mut self, layout_map: &mut LayoutMap) -> Model<App> {
        let draggable_buttons = (0..=7).map(|_n| {
            Element::new()
                .object(DraggableButton {
                    bg_color: GRAY_4,
                    border_color: GRAY_6,
                    known_rect: Rect::NONE,
                })
                .layout(Layout::default()
                    .width(50.0)
                    .height(50.0))
        });

        let root = Element::new()
            .layout(Layout::default()
                .width(1280.0)
                .height(720.0)
                .gap_x(11.0)
                .padding(11.0))
            .child(Element::new()
                .object(LeftPanel { color: GRAY_2 })
                .layout(Layout::default()
                    .flex_initial()
                    .width(300.0)
                    .padding(7.0)))
            .child(Element::new()
                .object(RightPanel { color: GRAY_3 })
                .layout(Layout::default()
                    .flex_auto()
                    .flex_wrap()
                    .gap_x(11.0)
                    .padding(7.0)
                    .align_items_center()
                    .justify_content_center())
                .children(draggable_buttons));

        Model::new(root, layout_map)
    }
}

impl AppHandler for App {
    fn window_desc(&self) -> WindowDescriptor {
        WindowDescriptor {
            title: "Bog Showcase",
            inner_size: Vec2::new(1280.0, 720.0),
            ..Default::default()
        }
    }
}



struct LeftPanel {
    color: Color,
}

impl Object for LeftPanel {
    type View = App;

    fn render(&mut self, cx: bog::view::RenderContext<Self::View>) {
        cx.renderer.fill_quad(Quad {
            bounds: cx.placement.rect(),
            bg_color: self.color,
            ..Default::default()
        });
    }
}

struct RightPanel {
    color: Color,
}

impl Object for RightPanel {
    type View = App;

    fn render(&mut self, cx: bog::view::RenderContext<Self::View>) {
        cx.renderer.fill_quad(Quad {
            bounds: cx.placement.rect(),
            bg_color: self.color,
            ..Default::default()
        });
    }

    fn post_render(&mut self, cx: RenderContext<Self::View>) {
        if let Some(drag_indicator) = &cx.view.drag_indicator {
            cx.renderer.fill_quad(*drag_indicator);
        }
    }
}



struct DraggableButton {
    bg_color: Color,
    border_color: Color,
    known_rect: Rect,
}

impl Object for DraggableButton {
    type View = App;

    fn render(&mut self, cx: bog::view::RenderContext<Self::View>) {
        self.known_rect = cx.placement.rect();
        cx.renderer.fill_quad(Quad {
            bounds: self.known_rect,
            border: Border {
                color: self.border_color,
                width: 1.0,
                radius: [3.0; 4],
            },
            bg_color: self.bg_color,
            shadow: Shadow::new(GRAY_0, vec2(2.0, 3.0), 2.0),
        });
    }

    fn on_mouse_enter(&mut self, _cx: bog::view::EventContext<Self::View>) {
        self.bg_color = GRAY_5;
    }

    fn on_mouse_leave(&mut self, _cx: bog::view::EventContext<Self::View>) {
        self.bg_color = GRAY_4;
    }

    fn on_drag_move(&mut self, cx: EventContext<Self::View>) {
        cx.view.drag_indicator = Some(Quad {
            bounds: self.known_rect + (cx.model.mouse_position() - self.known_rect.position()),
            border: Border {
                width: 3.0,
                color: GRAY_8,
                radius: [3.0; 4],
            },
            bg_color: GRAY_5.with_alpha(155),
            ..Default::default()
        });
    }

    fn on_drag_start(&mut self, cx: EventContext<Self::View>) {
        cx.view.drag_indicator = Some(Quad {
            bounds: self.known_rect,
            border: Border {
                width: 3.0,
                color: GRAY_8,
                radius: [3.0; 4],
            },
            bg_color: GRAY_5.with_alpha(155),
            ..Default::default()
        })
    }

    fn on_drag_end(&mut self, cx: EventContext<Self::View>) {
        cx.view.drag_indicator = None;
    }
}
