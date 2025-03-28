


use bog::*;
use window::*;



fn main() -> Result<()> {
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_title("Bog Testing")
        .with_inner_size(winit::dpi::LogicalSize::new(1200, 800))
        .build(&event_loop)
        .unwrap();
    let graphics = WindowGraphics::from_winit_window(&window, GraphicsConfig::new(1200, 800))?;

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
                graphics.resize(new_size);
                window.request_redraw();
            }
            winit::event::Event::RedrawRequested(_) => {
                let (width, height) = window.inner_size().into();
                let viewport = three_d::Viewport::new_at_origo(width, height);
                three_d::RenderTarget::screen(graphics.renderer(), width, height)
                    .clear(three_d::ClearState::color_and_depth(0.7, 0.7, 0.7, 1.0, 1.0))
                    .render(&three_d::Camera::new_2d(viewport), empty_objects(), &[]);
                graphics.swap_buffers().unwrap();
            }
            _ => {}
        }
    });
}

fn empty_objects() -> impl Iterator<Item = impl three_d::Object> {
    core::iter::empty::<three_d::Gm<three_d::Mesh, three_d::ColorMaterial>>()
}
