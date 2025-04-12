


use bog::*;
use graphics::*;
use wgpu::util::DeviceExt as _;
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
    let vertex_buffer = graphics.device().create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(&quad_vertices([-0.4, 0.1], [0.8, 0.1], 0x2b2b33ff)),
        usage: wgpu::BufferUsages::VERTEX,
    });
    let index_buffer = graphics.device().create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(&quad_indices()),
        usage: wgpu::BufferUsages::INDEX,
    });

    event_loop.run(move |event, control_flow| {
        match event {
            WindowManagerEvent::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    control_flow.exit();
                }
                WindowEvent::Resized(new_size) => {
                    if new_size.width > 0 && new_size.height > 0 {
                        graphics.surface_config_mut().width = new_size.width;
                        graphics.surface_config_mut().height = new_size.height;
                        graphics.surface().configure(graphics.device(), graphics.surface_config());
                    }
                }
                WindowEvent::RedrawRequested => {
                    graphics
                        .render(|mut render_pass| {
                            render_pass.use_shader(&shader);
                            render_pass.use_vertex_buffer(0, vertex_buffer.slice(..));
                            render_pass.use_index_buffer(
                                index_buffer.slice(..),
                                wgpu::IndexFormat::Uint32,
                            );
                            render_pass.draw_indexed(0..6, 0, 0..1);
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
