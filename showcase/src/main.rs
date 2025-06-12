


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
        mouse_pos: Vec2::ZERO,
    })
}



struct App {
    mouse_pos: Vec2,
}

impl AppHandler for App {
    fn render<'pass>(&'pass mut self, cx: AppContext, pass: &mut RenderPass<'pass>) {
        let area = cx.renderer.viewport_rect();
        pass.start_layer(area);
        pass.fill_quad(Quad::new_colored(area, GRAY_1));
        pass.end_layer();
        pass.start_layer(area);

        let (_side_area, main_area) = area.split_portion_h(0.2);
        pass.fill_quad(Quad::new_colored(main_area, GRAY_2));

        pass.end_layer();
    }

    fn input(&mut self, cx: AppContext, input: InputEvent) {
        cx.window.request_redraw();
        match input {
            InputEvent::MouseMove { x, y } => {
                self.mouse_pos = vec2(x, y);
            }
            _ => {}
        }
    }

    fn window_desc(&self) -> WindowDescriptor {
        WindowDescriptor {
            title: "Bog - Showcase",
            ..Default::default()
        }
    }
}
