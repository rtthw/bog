


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

    let bg_color = three_d::Srgba::new_opaque(43, 43, 53);

    let mut scene = Scene::default();
    let mut ui = Ui::new(Layout::default()
        .flex_row()
        .flex_wrap()
        .gap_x(19.0)
        .padding(11.0)
        .fill_width()
        .fill_height());

    for word in ["This", "is", "@_ |>", "test", "for", "text", "#_(o)", "...", "***", "=>>"] {
        let text_mesh = graphics.renderer().mesh_for_text("mono", word, None).unwrap();
        let text_material = three_d::ColorMaterial {
            color: three_d::Srgba::new_opaque(163, 163, 173),
            ..Default::default()
        };

        let mut mesh = CpuMesh::square();
        mesh.transform(three_d::Mat4::from_scale(0.5)).unwrap();
        let pane_mesh = Mesh::new(graphics.renderer(), &mesh);
        let pane_material = three_d::ColorMaterial {
            color: three_d::Srgba::new_opaque(29, 29, 39),
            ..Default::default()
        };

        let width = text_mesh.aabb().size().x;
        let height = text_mesh.aabb().size().y;
        let row_height = graphics.renderer().get_font("mono").unwrap().row_height();

        let pane_id = scene.append(pane_mesh, pane_material);
        let text_id = scene.append(text_mesh, text_material);

        let pane_node = ui.push_to_root(
            Layout::default()
                .align_content_center()
                .align_items_center()
                .width(width)
                .height(row_height),
            pane_id,
            true,
        );
        let _text_node = ui.push_to(
            Layout::default()
                .width(width)
                .height(height),
            pane_node,
            text_id,
            false,
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
                let [r, g, b, a] = bg_color.into();
                three_d::RenderTarget::screen(graphics.renderer(), width, height)
                    .clear(three_d::ClearState::color_and_depth(r, g, b, a, 1.0))
                    .render(&three_d::Camera::new_2d(viewport), scene.objects(), &[]);
                graphics.swap_buffers().unwrap();
            }
            _ => {}
        }
    });
}
