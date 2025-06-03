


use bog::{prelude::*, render::FontFamily};



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
    let ps = syntect::parsing::SyntaxSet::load_defaults_newlines();
    let ts = syntect::highlighting::ThemeSet::load_defaults();

    let text = include_str!("main.rs");
    let mut lines = Vec::with_capacity(text.lines().count());

    let syntax = ps.find_syntax_by_extension("rs").unwrap();
    let mut h = syntect::easy::HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
    for line in syntect::util::LinesWithEndings::from(&text) {
        let ranges: Vec<(syntect::highlighting::Style, &str)> = h.highlight_line(line, &ps)
            .unwrap();
        // let escaped = syntect::util::as_24_bit_terminal_escaped(&ranges[..], true);
        // print!("{}", escaped);
        lines.push(ranges);
    }

    run_app(App {
        lines,
        cell_bounds: Vec2::new(0.0, 1.0), // Height cannot be 0.
        scroll_offset: 0,
        mouse_pos: Vec2::ZERO,
    })?;

    Ok(())
}



struct App {
    lines: Vec<Vec<(syntect::highlighting::Style, &'static str)>>,
    cell_bounds: Vec2,
    scroll_offset: usize,
    mouse_pos: Vec2,
}

impl AppHandler for App {
    fn startup(&mut self, cx: AppContext) {
        self.cell_bounds = cx.renderer.measure_text(&Text {
            content: "█".into(),
            size: 19.0,
            line_height: 0.0,
            font_family: FontFamily::Monospace,
            ..Default::default()
        });
    }

    fn render(&mut self, cx: AppContext, layers: &mut LayerStack) {
        layers.start_layer(cx.renderer.viewport_rect());
        layers.fill_quad(Quad {
            bounds: cx.renderer.viewport_rect(),
            bg_color: GRAY_2,
            ..Default::default()
        });

        let height = cx.renderer.viewport_rect().h / self.cell_bounds.y;
        let mut y_offset = 0.0;
        for line_ranges in self.lines.iter().skip(self.scroll_offset).take(height.ceil() as _) {
            let mut x_offset = 0.0;
            for (style, text) in line_ranges.iter() {
                let width = self.cell_bounds.x * text.chars().count() as f32;
                let rect = Rect::new(vec2(x_offset, y_offset), vec2(width, self.cell_bounds.y));

                if rect.contains(self.mouse_pos) {
                    layers.fill_quad(Quad {
                        bounds: rect,
                        bg_color: GRAY_3,
                        ..Default::default()
                    });
                }

                layers.fill_text(Text {
                    content: (*text).into(),
                    bounds: rect,
                    size: 19.0,
                    color: Color {
                        r: style.foreground.r,
                        g: style.foreground.g,
                        b: style.foreground.b,
                        a: style.foreground.a,
                    },
                    line_height: self.cell_bounds.y,
                    font_family: FontFamily::Monospace,
                    ..Default::default()
                });
                x_offset += width;
            }
            y_offset += self.cell_bounds.y;
        }

        layers.end_layer();
    }

    fn on_mouse_move(&mut self, _cx: AppContext, mouse_pos: Vec2) {
        self.mouse_pos = mouse_pos;
    }

    fn on_wheel_movement(&mut self, _cx: AppContext, movement: WheelMovement) {
        match movement {
            WheelMovement::Lines { y, .. } => {
                self.scroll_offset = self.scroll_offset
                    .saturating_add_signed(3 * -y.round() as isize);
            }
            WheelMovement::Pixels { .. } => todo!(),
        }
    }

    fn window_desc(&self) -> WindowDescriptor {
        WindowDescriptor {
            title: "Bog Showcase",
            inner_size: Vec2::new(1200.0, 800.0),
            ..Default::default()
        }
    }
}
