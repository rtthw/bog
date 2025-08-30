


use bog::prelude::*;



fn main() -> Result<()> {
    run_simple_app(None, MyApp)
}

struct MyApp;

impl SimpleApp for MyApp {
    type CustomEvent = ();

    fn render<'a: 'pass, 'pass>(
        &'a mut self,
        cx: AppContext<'pass>,
        pass: &'pass mut RenderPass<'a>,
    ) {
        pass.start_layer(cx.renderer.viewport_rect());
        pass.fill_quad(Quad {
            bounds: cx.renderer.viewport_rect(),
            bg_color: Color::new(43, 43, 53, 255),
            ..Default::default()
        });
        pass.end_layer();
    }

    fn input(&mut self, cx: AppContext, _event: InputEvent) {
        cx.window.request_redraw();
    }

    fn window_desc(&self) -> WindowDescriptor {
        WindowDescriptor {
            title: "Bog - Quickstart Example",
            ..Default::default()
        }
    }
}
