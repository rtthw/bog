


use bog::*;
use graphics::*;
use window::*;



fn main() -> Result<()> {
    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new()
        .with_title("Bog Testing")
        .with_inner_size(dpi::LogicalSize::new(1200.0, 800.0))
        .build(&event_loop)?;
    let graphics = futures::executor::block_on(async {
        WindowGraphics::from_window(&window).await
    })?;

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
