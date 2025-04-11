//! Graphics module



use crate::window::Window;



type Result<T> = std::result::Result<T, GraphicsError>;

#[derive(thiserror::Error, Debug)]
pub enum GraphicsError {
    #[error("create surface error")]
    CreateSurfaceError(#[from] wgpu::CreateSurfaceError),
    #[error("request device error")]
    RequestDeviceError(#[from] wgpu::RequestDeviceError),
}



pub struct WindowGraphics<'w> {
    surface: wgpu::Surface<'w>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: [u32; 2],

    // NOTE: Window must be dropped after the other surface fields.
    window: &'w Window,
}

// Constructors.
impl<'w> WindowGraphics<'w> {
    pub async fn from_window(window: &'w Window) -> Result<Self> {
        let size: [u32; 2] = window.inner_size().into();

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch = "wasm32")]
            backends: wgpu::Backends::GL,
            ..Default::default()
        });

        let surface = instance.create_surface(window)?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap(); // TODO: Remove unwrap.

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    memory_hints: Default::default(),
                },
                None,
            )
            .await?;

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size[0],
            height: size[1],
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            desired_maximum_frame_latency: 2,
            view_formats: vec![],
        };

        Ok(Self {
            surface,
            device,
            queue,
            config,
            size,
            window,
        })
    }
}

// Getters.
impl<'w> WindowGraphics<'w> {
    pub fn surface(&self) -> &wgpu::Surface {
        &self.surface
    }

    pub fn surface_config(&self) -> &wgpu::SurfaceConfiguration {
        &self.config
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub fn window(&self) -> &Window {
        self.window
    }
}

impl<'w> WindowGraphics<'w> {
    pub fn render(&self, shader: &Shader) -> Result<()> {
        let output = self.surface.get_current_texture().unwrap();
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            },
        );

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.2,
                            g: 0.1,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&shader.pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}



pub struct Shader {
    pub(crate) pipeline: wgpu::RenderPipeline,
}

impl Shader {
    pub fn new(device: &wgpu::Device, desc: ShaderDescriptor) -> Self {
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: desc.label,
            source: desc.source,
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: desc.pipeline_layout_label,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: desc.pipeline_label,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &module,
                entry_point: desc.vertex_entry_point,
                buffers: desc.vertex_buffers,
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &module,
                entry_point: desc.fragment_entry_point,
                targets: desc.fragment_targets,
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: desc.primitive,
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        Self {
            pipeline,
        }
    }
}

pub struct ShaderDescriptor<'a> {
    pub source: wgpu::ShaderSource<'a>,
    pub label: Option<&'a str>,
    pub pipeline_label: Option<&'a str>,
    pub pipeline_layout_label: Option<&'a str>,
    pub vertex_entry_point: Option<&'a str>,
    pub vertex_buffers: &'a [wgpu::VertexBufferLayout<'a>],
    pub fragment_entry_point: Option<&'a str>,
    pub fragment_targets: &'a [Option<wgpu::ColorTargetState>],
    pub primitive: wgpu::PrimitiveState,
}

impl<'a> Default for ShaderDescriptor<'a> {
    fn default() -> Self {
        Self {
            source: wgpu::ShaderSource::Dummy(std::marker::PhantomData),
            label: None,
            pipeline_label: None,
            pipeline_layout_label: None,
            vertex_entry_point: None,
            vertex_buffers: &[],
            fragment_entry_point: None,
            fragment_targets: &[],
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // NOTE: Setting this to anything other than `Fill` requires
                //       `Features::NON_FILL_POLYGON_MODE`.
                polygon_mode: wgpu::PolygonMode::Fill,
                // NOTE: Requires `Features::DEPTH_CLIP_CONTROL`.
                unclipped_depth: false,
                // NOTE: Requires `Features::CONSERVATIVE_RASTERIZATION`.
                conservative: false,
            },
        }
    }
}
