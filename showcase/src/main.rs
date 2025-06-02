


use bog::prelude::*;
use cosmic_text::Edit as _;



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
    let mut font_system = cosmic_text::FontSystem::new();
    let syntax_system = cosmic_text::SyntaxSystem::new();
    let editor = cosmic_text::SyntaxEditor::new(
        cosmic_text::Buffer::new(
            &mut font_system,
            cosmic_text::Metrics::new(19.0, 23.0),
        ),
        &syntax_system,
        "base16-ocean.dark",
    ).unwrap();

    run_app(App {
        editor,
        swash_cache: cosmic_text::SwashCache::new(),
        display_scale: 1.0,
    })?;

    Ok(())
}



struct App<'a, 'b> {
    editor: cosmic_text::SyntaxEditor<'a, 'b>,
    swash_cache: cosmic_text::SwashCache,
    display_scale: f32,
}

impl<'a, 'b> AppHandler for App<'a, 'b> {
    fn startup(&mut self, cx: AppContext) {
        self.editor.load_text(
            &mut cx.renderer.text_pipeline().font_system,
            "main.rs",
            cosmic_text::Attrs::new().family(cosmic_text::Family::Monospace),
        ).unwrap();
    }

    fn render(&mut self, cx: AppContext, layers: &mut LayerStack) {
        layers.start_layer(cx.renderer.viewport_rect());
        layers.fill_quad(Quad {
            bounds: cx.renderer.viewport_rect(),
            bg_color: GRAY_1,
            ..Default::default()
        });

        let rect = cx.renderer.viewport_rect();
        self.editor.with_buffer_mut(|buffer| {
            buffer.set_size(
                &mut cx.renderer.text_pipeline().font_system,
                Some(rect.w - 13.0 * self.display_scale),
                Some(rect.h),
            )
        });

        // NOTE: This will obviously change to actually render the text, but the borrow checker
        //       gets mad due to the closure. The renderer is actually still fast enough to render
        //       this, though. A `Quad` is being drawn for each pixel at the moment.
        self.editor.draw(
            &mut cx.renderer.text_pipeline().font_system,
            &mut self.swash_cache,
            |x, y, w, h, c| {
                layers.fill_quad(Quad {
                    bounds: Rect::new(vec2(x as _, y as _), vec2(w as _, h as _)),
                    bg_color: Color {
                        r: c.r(),
                        g: c.g(),
                        b: c.b(),
                        a: c.a(),
                    },
                    ..Default::default()
                });
                // layers.fill_text(Text {
                //     content: run.text,
                //     pos: vec2(0.0, run.line_top),
                //     bounds: vec2(run.line_w, run.line_height),
                //     size: 19.0,
                //     color: GRAY_8,
                //     line_height: run.line_height,
                //     font_family: FontFamily::Monospace,
                //     ..Default::default()
                // });
            });

        layers.end_layer();
    }

    fn on_wheel_movement(&mut self, cx: AppContext, movement: WheelMovement) {
        match movement {
            WheelMovement::Lines { y, .. } => {
                self.editor.action(
                    &mut cx.renderer.text_pipeline().font_system,
                    cosmic_text::Action::Scroll { lines: -y as _ },
                );
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
