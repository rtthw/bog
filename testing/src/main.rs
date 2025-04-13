


use bog::*;
use graphics::*;
use math::vec2;
use painter::*;
use window::*;



fn main() -> Result<()> {
    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new()
        .with_title("Bog Testing")
        .with_inner_size(dpi::LogicalSize::new(1200.0, 800.0))
        .build(&event_loop)?;
    let mut graphics = futures::executor::block_on(async {
        WindowGraphics::from_window(&window).await
    })?;
    let mut painter = Painter::new(&graphics);
    let paints = vec![
        Rectangle {
            pos: vec2(0.0, 0.0),
            size: vec2(100.0, 50.0),
            color: 0xaaaaabff,
            corner_radii: [7.0, 19.0, 19.0, 19.0],
        }.to_mesh(),
    ];

    event_loop.run(move |event, control_flow| {
        match event {
            WindowManagerEvent::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    control_flow.exit();
                }
                WindowEvent::Resized(new_size) => {
                    graphics.window().request_redraw();
                    if new_size.width > 0 && new_size.height > 0 {
                        graphics.surface_config_mut().width = new_size.width;
                        graphics.surface_config_mut().height = new_size.height;
                        graphics.surface().configure(graphics.device(), graphics.surface_config());
                    }
                }
                WindowEvent::RedrawRequested => {
                    graphics
                        .render(|render_pass| {
                            painter.prepare(&graphics, &paints);
                            painter.render(render_pass, &paints);
                        })
                        .unwrap();
                }
                _ => {}
            }
            _ => {}
        }
    })?;

    Ok(())
}
