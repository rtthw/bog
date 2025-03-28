


use bog::*;
use graphics::*;
use layout::{Layout, Ui};
use scene::Scene;
use three_d::Geometry as _;



fn main() -> Result<()> {
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_title("Bog Testing")
        .with_inner_size(winit::dpi::LogicalSize::new(1200, 800))
        .build(&event_loop)
        .unwrap();
    let mut graphics = WindowGraphics::from_winit_window(&window, GraphicsConfig::new(1200, 800))?;

    graphics.renderer_mut().load_font(
        "mono",
        include_bytes!("../data/JetBrainsMonoNerdFont_Regular.ttf").to_vec(),
        20.0,
    )?;

    let mut scene = Scene::default();
    let mut ui = Ui::new(Layout::default()
        .flex_row()
        .flex_wrap()
        .padding(15.0)
        .gap_x(15.0)
        .fill_width()
        .fill_height());

    for word in ["This", "is", "a", "test", "for", "text", "rendering", "...", "***", "=>>"] {
        let mesh = graphics.renderer().mesh_for_text("mono", word, None).unwrap();
        let width = mesh.aabb().size().x;
        let row_height = graphics.renderer().get_font("mono").unwrap().row_height();
        let material = three_d::ColorMaterial {
            color: three_d::Srgba::BLACK,
            ..Default::default()
        };

        let id = scene.append(mesh, material);

        ui.push_to_root(
            Layout::default()
                .width(width)
                // NOTE: The bounding box could be smaller than the row height, like with lowercase
                //       letters for example.
                .height(row_height),
            id,
        );
    }

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        match event {
            winit::event::Event::WindowEvent {
                event: winit::event::WindowEvent::CloseRequested,
                ..
            } => {
                control_flow.set_exit();
            }
            winit::event::Event::WindowEvent {
                event: winit::event::WindowEvent::Resized(new_size),
                ..
            } => {
                let (width, height): (u32, u32) = new_size.into();
                ui.resize(&mut scene, width as f32, height as f32);
                graphics.resize(new_size);
                window.request_redraw();
            }
            winit::event::Event::RedrawRequested(_) => {
                let (width, height) = window.inner_size().into();
                let viewport = three_d::Viewport::new_at_origo(width, height);
                three_d::RenderTarget::screen(graphics.renderer(), width, height)
                    .clear(three_d::ClearState::color_and_depth(0.7, 0.7, 0.7, 1.0, 1.0))
                    .render(&three_d::Camera::new_2d(viewport), scene.objects(), &[]);
                graphics.swap_buffers().unwrap();
            }
            _ => {}
        }
    });
}
