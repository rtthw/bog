


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
        scroll_offset: 0.0,
    })?;

    Ok(())
}



struct App {
    scroll_offset: f32,
}

impl AppHandler for App {
    fn render(&mut self, renderer: &mut Renderer, layers: &mut LayerStack) {
        layers.start_layer(renderer.viewport_rect());
        layers.fill_quad(Quad {
            bounds: renderer.viewport_rect(),
            bg_color: GRAY_1,
            ..Default::default()
        });
        layers.end_layer();
    }

    fn on_wheel_movement(&mut self, movement: WheelMovement) {
        match movement {
            WheelMovement::Lines { y, .. } => {
                self.scroll_offset += y * 20.0;
            }
            WheelMovement::Pixels { y, .. } => {
                self.scroll_offset += y;
            }
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
