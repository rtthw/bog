


use bog::*;
use window::*;



fn main() -> Result<()> {
    let (screen_width, screen_height) = (1200.0, 800.0);
    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new()
        .with_title("Bog Testing")
        .with_inner_size(dpi::LogicalSize::new(screen_width, screen_height))
        .build(&event_loop)?;
    // let graphics = WindowGraphics::from_winit_window(&window, GraphicsConfig::new(1200, 800))?;

    event_loop.run(move |event, control_flow| {
        match event {
            WindowManagerEvent::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    control_flow.exit();
                }
                _ => {}
            }
            _ => {}
        }
    })?;

    Ok(())
}
