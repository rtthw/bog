


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
    let shader = Shader::new(graphics.device(), ShaderDescriptor {
        source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(
            include_str!("shader.wgsl"),
        )),
        label: Some("shader.wgsl"),
        pipeline_label: Some("Render Pipeline"),
        pipeline_layout_label: Some("Render Pipeline Layout"),
        vertex_entry_point: Some("vs_main"),
        vertex_buffers: &[Vertex::desc()],
        fragment_entry_point: Some("fs_main"),
        fragment_targets: &[Some(wgpu::ColorTargetState {
            format: graphics.surface_config().format,
            blend: Some(wgpu::BlendState::REPLACE),
            write_mask: wgpu::ColorWrites::ALL,
        })],
        ..Default::default()
    });

    event_loop.run(move |event, control_flow| {
        match event {
            WindowManagerEvent::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    control_flow.exit();
                }
                WindowEvent::RedrawRequested => {
                    graphics
                        .render(|mut render_pass| {
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
