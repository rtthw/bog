


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

        let mut theme = Theme::new(
            Style {
                text: TextStyle {
                    family: FontFamily::SansSerif,
                    slant: TextSlant::Normal,
                    weight: LineWeight::NORMAL,
                    height: Unit::Em(1.0),
                },
                border: BorderStyle {
                    color: GRAY_5,
                    width: Unit::Px(2.0),
                    radius: BorderRadius::Uniform(5.0),
                },
                shadow: ShadowStyle {
                    color: GRAY_0,
                    offset_x: Unit::Px(3.0),
                    offset_y: Unit::Px(3.0),
                    spread: Unit::Px(5.0),
                },
                fg_color: GRAY_9,
                bg_color: GRAY_1,
            },
            17.0,
        );
        let left_panel_class = StyleClass::base(&mut theme, Styling {
            bg_color: Some(GRAY_1),
            border_width: Some(Unit::Px(0.0)),
            ..Default::default()
        });
        let right_panel_class = StyleClass::base(&mut theme, Styling {
            bg_color: Some(GRAY_2),
            border_width: Some(Unit::Px(0.0)),
            ..Default::default()
        });
        let large_text_class = StyleClass::base(&mut theme, Styling {
            fg_color: Some(GRAY_6),
            text_height: Some(Unit::Em(1.7)),
            ..Default::default()
        });
        let small_text_class = StyleClass::base(&mut theme, Styling {
            fg_color: Some(GRAY_7),
            text_height: Some(Unit::Em(0.7)),
            text_slant: Some(TextSlant::Italic),
            ..Default::default()
        });
        let hrule_class = StyleClass::base(&mut theme, Styling {
            bg_color: Some(GRAY_7.with_alpha(155)),
            border_width: Some(Unit::Px(0.0)),
            border_radius: Some(BorderRadius::Uniform(3.0)),
            ..Default::default()
        });
        hrule_class.with_hover(&mut theme, Styling {
            bg_color: Some(GRAY_8),
            ..Default::default()
        });

        let root = Element::new()
            .layout(Layout::default().gap_x(11.0).padding(11.0))
            .child(panel()
                .style(left_panel_class)
                .layout(Layout::default()
                    .flex_initial()
                    .flex_column()
                    .width_percent(0.2)
                    .gap_y(7.0)
                    .padding(7.0))
                .child(static_paragraph("Bog")
                    .style(large_text_class)
                    .layout(Layout::default().fill_width()))
                .child(horizontal_rule(5.0).style(hrule_class))
                .child(Button::new(static_paragraph("Click Me").style(small_text_class))
                    .on_click(|_cx| {
                        println!("Button clicked!");
                    }))
                .child(Scrollable::new()
                    .children([
                        test_paragraph(),
                        test_paragraph(),
                        test_button(),
                        test_paragraph(),
                        test_paragraph(),
                        test_paragraph(),
                        test_paragraph(),
                        test_paragraph(),
                        test_paragraph(),
                        test_paragraph(),
                        test_paragraph(),
                        test_button(),
                        test_button(),
                        test_paragraph(),
                        test_paragraph(),
                        test_paragraph(),
                        test_paragraph(),
                        test_paragraph(),
                        test_paragraph(),
                        test_paragraph(),
                        test_paragraph(),
                        test_paragraph(),
                        test_paragraph(),
                    ])))
            .child(panel()
                .style(right_panel_class)
                .layout(Layout::default()
                    .flex_auto()
                    .flex_wrap()
                    .gap_x(11.0)
                    .padding(7.0)
                    .align_items_center()
                    .justify_content_center())
                .children(draggable_buttons));

        Model::new(root, layout_map, theme)
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



struct DraggableButton {
    bg_color: Color,
    border_color: Color,
    known_rect: Rect,
}

impl Object for DraggableButton {
    type View = App;

    fn render(&mut self, cx: RenderContext<Self::View>) {
        self.known_rect = cx.placement.rect();
        cx.layer_stack.fill_quad(Quad {
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

    fn on_mouse_enter(&mut self, _cx: EventContext<Self::View>) {
        self.bg_color = GRAY_5;
    }

    fn on_mouse_leave(&mut self, _cx: EventContext<Self::View>) {
        self.bg_color = GRAY_4;
    }

    fn on_drag_move(&mut self, cx: EventContext<Self::View>) {
        cx.view.drag_indicator = Some(Quad {
            bounds: self.known_rect + cx.model.drag_delta(),
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



fn test_button<V: View + 'static>() -> Element<V> {
    Button::new(static_paragraph("Button"))
        .on_click(|_cx| {
            println!("Test button clicked!");
        })
        .into()
}

fn test_paragraph<V: View + 'static>() -> Element<V> {
    static_paragraph("This is a test paragraph that may span a few lines or so.")
}
