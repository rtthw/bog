


use animation::*;
use bog::*;
use graphics::*;
use layout::Layout;
use math::{vec2, Mat4};
use new_renderer::{Mesh2D, Painter2D, Shape, Tessellator};
use ui::Ui;



fn main() -> Result<()> {
    let (mut screen_width, mut screen_height) = (1200.0, 800.0);
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_title("Bog Testing")
        .with_inner_size(winit::dpi::LogicalSize::new(screen_width, screen_height))
        .build(&event_loop)
        .unwrap();
    let mut graphics = WindowGraphics::from_winit_window(&window, GraphicsConfig::new(1200, 800))?;

    graphics.renderer_mut().load_font(
        "mono",
        include_bytes!("../data/JetBrainsMonoNerdFont_Regular.ttf").to_vec(),
        20.0,
    )?;

    let bg_color = Srgba::new_opaque(43, 43, 53);

    let mut ui = Ui::new(Layout::default()
        .flex_row()
        .flex_wrap()
        .gap_x(19.0)
        .padding(11.0)
        .fill_width()
        .fill_height());

    for word in ["This", "is", "@_ |>", "test", "for", "text", "#_(o)", "...", "***", "=>>"] {
        let mut text_mesh = graphics.renderer()
            .mesh_for_text("mono", word, Srgba::new_opaque(163, 163, 173), None)
            .unwrap();
        let text_size = text_mesh.compute_size();
        let row_height = graphics.renderer().get_font("mono").unwrap().row_height();

        let mut pane_mesh = Mesh2D::new();
        Tessellator.tessellate_shape(Shape::Rect {
            pos: vec2(0.0, 0.0),
            size: vec2(text_size[0], text_size[1]),
            color: Srgba::new_opaque(23, 23, 29),
        }, &mut pane_mesh);

        let pane_node = ui.push_to_root(
            Layout::default()
                .align_content_center()
                .align_items_center()
                .width(text_size[0])
                .height(row_height),
            pane_mesh,
        );
        let _text_node = ui.push_to(
            Layout::default()
                .width(text_size[0])
                .height(text_size[1]),
            pane_node,
            text_mesh,
        );
    }

    let mut painter = Painter2D::new(graphics.renderer().gl());
    let mut main_mesh = Mesh2D::new();
    let mut tesselator = Tessellator;
    tesselator.tessellate_shape(Shape::Rect {
        pos: vec2(0.0, 0.0),
        size: vec2(200.0, 200.0),
        color: Srgba::new_opaque(163, 163, 173),
    }, &mut main_mesh);
    println!(
        "MESH: {:?}",
        main_mesh,
    );

    let start_time = std::time::Instant::now();
    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        // if animate {
        //     let seconds_since_start = std::time::Instant::now()
        //         .duration_since(start_time)
        //         .as_secs_f32();
        //     ui.handle_animations(seconds_since_start);
        //     window.request_redraw();
        // }

        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    control_flow.set_exit();
                }
                winit::event::WindowEvent::Resized(new_size) => {
                    (screen_width, screen_height) = new_size.into();
                    ui.resized(screen_width, screen_height);
                    graphics.resize(new_size);
                    window.request_redraw();
                }
                // winit::event::WindowEvent::CursorMoved { position, .. } => {
                //     let (x, y): (f32, f32) = position.into();
                //     ui.handle_cursor_moved(x, screen_height - y);
                // }
                // winit::event::WindowEvent::MouseInput { state, button, .. } => {
                //     ui.handle_mouse_down(..);
                // }
                _ => {}
            }
            winit::event::Event::RedrawRequested(_) => {
                let (width, height) = window.inner_size().into();
                let viewport = Viewport::new_at_origo(width, height);
                let [r, g, b, a] = bg_color.into();
                RenderTarget::screen(graphics.renderer(), width, height)
                    .clear(ClearState::color_and_depth(r, g, b, a, 1.0))
                    // .render(&Camera::new_2d(viewport), ui.objects(), &[])
                    .write(|| -> Result<()> {
                        painter.render(viewport, &main_mesh);
                        Ok(())
                    })
                    .unwrap();
                graphics.swap_buffers().unwrap();
            }
            _ => {}
        }
    });
}
