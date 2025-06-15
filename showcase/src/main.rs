


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
    let root_area = {
        let area = Rect::new(vec2(0.0, 0.0), vec2(1200.0, 800.0));
        let (side_area, main_area) = area.split_portion_h(0.2);

        InputArea::new(area, "root")
            .with_children(vec![
                InputArea::new(side_area, "side"),
                InputArea::new(main_area, "main"),
            ])
    };

    run_simple_app(None, App {
        event_parser: EventParser::new(root_area),
    })
}



struct App {
    event_parser: EventParser,
}

impl SimpleApp for App {
    type CustomEvent = ();

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

    fn input(&mut self, cx: AppContext, event: InputEvent) {
        cx.window.request_redraw();
        for input in self.event_parser.parse_event(event) {
            match input {
                Input::Mouse(i) => match i {
                    MouseInput::Enter { area: "side" } => {
                        println!("(A) Works!");
                    }
                    MouseInput::Leave { area: "side" } => {
                        println!("(B) Works!");
                    }
                    MouseInput::Enter { area: "main" } => {
                        println!("(C) Works!");
                    }
                    _ => {}
                }
                _ => {}
            }
        }
    }

    fn window_desc(&self) -> WindowDescriptor {
        WindowDescriptor {
            title: "Bog - Showcase",
            inner_size: vec2(1200.0, 800.0),
            ..Default::default()
        }
    }
}
