


use bog::prelude::*;



fn main() -> Result<()> {
    let ps = syntect::parsing::SyntaxSet::load_defaults_newlines();
    let ts = syntect::highlighting::ThemeSet::load_defaults();

    let text = include_str!("source_viewer.rs");
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

    run_simple_app(None, App {
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

impl SimpleApp for App {
    type CustomEvent = ();

    fn startup(&mut self, cx: AppContext) {
        self.cell_bounds = cx.renderer.measure_text(&Text {
            content: "█".into(),
            size: 19.0,
            line_height: 0.0,
            font_family: FontFamily::Monospace,
            ..Default::default()
        });
    }

    fn render<'a: 'pass, 'pass>(
        &'a mut self,
        cx: AppContext<'pass>,
        pass: &'pass mut RenderPass<'a>,
    ) {
        pass.start_layer(cx.renderer.viewport_rect());
        // layers.fill_raster_image(
        //     ImageHandle::from_path("...").into(),
        //     cx.renderer.viewport_rect(),
        // );
        pass.fill_quad(Quad {
            bounds: cx.renderer.viewport_rect(),
            bg_color: Color::new(43, 43, 53, 255),
            ..Default::default()
        });
        pass.end_layer();

        pass.start_layer(cx.renderer.viewport_rect());
        let height = cx.renderer.viewport_rect().h / self.cell_bounds.y;
        let mut y_offset = 0.0;
        for line_ranges in self.lines.iter().skip(self.scroll_offset).take(height.ceil() as _) {
            let mut x_offset = 0.0;
            for (style, text) in line_ranges.iter() {
                let width = self.cell_bounds.x * text.chars().count() as f32;
                let rect = Rect::new(vec2(x_offset, y_offset), vec2(width, self.cell_bounds.y));

                if rect.contains(self.mouse_pos) {
                    pass.fill_quad(Quad {
                        bounds: rect,
                        bg_color: Color::new(59, 59, 67, 255),
                        ..Default::default()
                    });
                }

                pass.fill_text(Text {
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

        pass.end_layer();
    }

    fn input(&mut self, cx: AppContext, input: InputEvent) {
        cx.window.request_redraw();
        match input {
            InputEvent::MouseMove { x, y } => {
                self.mouse_pos = vec2(x, y);
            }
            InputEvent::WheelMove(WheelMovement::Lines { y, .. }) => {
                self.scroll_offset = self.scroll_offset
                    .saturating_add_signed(3 * -y.round() as isize);
            }
            _ => {}
        }
    }

    fn window_desc(&self) -> WindowDescriptor {
        WindowDescriptor {
            title: "Bog - Source Viewer Example",
            inner_size: Vec2::new(1200.0, 800.0),
            ..Default::default()
        }
    }
}
