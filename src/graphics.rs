//! Graphics module



use std::ops::Range;

use crate::{math::Vec2, window::Window};

pub use bytemuck;
pub use wgpu;



pub const SAMPLE_COUNT: u32 = 4;



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
    config: wgpu::SurfaceConfiguration,

    depth_texture_view: Option<wgpu::TextureView>,
    multisampled_render_target: Option<wgpu::TextureView>,

    // NOTE: Window must be dropped after the other surface fields.
    window: Window,
}

// Constructors.
impl<'w> WindowGraphics<'w> {
    pub async fn from_window(
        window: Window,
    ) -> Result<(Self, wgpu::Device, wgpu::Queue, wgpu::TextureFormat)> {
        let size: [u32; 2] = window.inner_size().into();

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch = "wasm32")]
            backends: wgpu::Backends::GL,
            ..Default::default()
        });

        let surface = instance.create_surface((*window).clone())?;

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
                    trace: wgpu::Trace::Off,
                },
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

        Ok((
            Self {
                surface,
                config,
                depth_texture_view: None,
                multisampled_render_target: None,
                window,
            },
            device,
            queue,
            surface_format,
        ))
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

    pub fn surface_config_mut(&mut self) -> &mut wgpu::SurfaceConfiguration {
        &mut self.config
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn screen_size(&self) -> Vec2 {
        Vec2::new(self.surface_config().width as f32, self.surface_config().height as f32)
    }
}

impl<'w> WindowGraphics<'w> {
    pub fn get_current_texture(&self) -> wgpu::SurfaceTexture {
        self.surface.get_current_texture().unwrap()
    }

    pub fn render(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        mut func: impl FnMut(RenderPass),
    ) -> Result<()> {
        let output = self.surface.get_current_texture().unwrap();
        let frame_view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            },
        );

        {
            let color_attachment = if let Some(msaa_target) = &self.multisampled_render_target {
                wgpu::RenderPassColorAttachment {
                    view: msaa_target,
                    resolve_target: Some(&frame_view),
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                }
            } else {
                wgpu::RenderPassColorAttachment {
                    view: &frame_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                }
            };
            let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(color_attachment)],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: self.depth_texture_view.as_ref().unwrap(),
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(0.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(0),
                        store: wgpu::StoreOp::Store,
                    }),
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            func(RenderPass(render_pass));
        }

        queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn resize(&mut self, device: &wgpu::Device, new_size: Vec2) {
        self.config.width = new_size.x as _;
        self.config.height = new_size.y as _;
        self.surface.configure(device, &self.config);

        // let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
        //     label: Some("Depth texture"),
        //     size: wgpu::Extent3d {
        //         width: self.config.width,
        //         height: self.config.height,
        //         depth_or_array_layers: 1,
        //     },
        //     mip_level_count: 1,
        //     sample_count: SAMPLE_COUNT,
        //     dimension: wgpu::TextureDimension::D2,
        //     format: wgpu::TextureFormat::Depth32Float,
        //     usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        //     view_formats: &[],
        // });

        // self.depth_texture_view = Some(depth_texture
        //     .create_view(&wgpu::TextureViewDescriptor::default()));

        // self.multisampled_render_target = if SAMPLE_COUNT > 1 {
        //     let multisampled_frame_descriptor = &wgpu::TextureDescriptor {
        //         label: Some("Multisampled Frame Descriptor"),
        //         size: wgpu::Extent3d {
        //             width: self.config.width,
        //             height: self.config.height,
        //             depth_or_array_layers: 1,
        //         },
        //         mip_level_count: 1,
        //         sample_count: SAMPLE_COUNT,
        //         dimension: wgpu::TextureDimension::D2,
        //         format: self.config.format,
        //         usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        //         view_formats: &[],
        //     };
        //     Some(device
        //         .create_texture(multisampled_frame_descriptor)
        //         .create_view(&wgpu::TextureViewDescriptor::default()))
        // } else {
        //     None
        // };
    }
}



pub struct Shader {
    pub(crate) pipeline: wgpu::RenderPipeline,
}

impl Shader {
    pub fn new(device: &wgpu::Device, desc: ShaderDescriptor) -> Self {
        let depth_stencil = Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Greater,
            stencil: wgpu::StencilState {
                front: wgpu::StencilFaceState::IGNORE,
                back: wgpu::StencilFaceState::IGNORE,
                read_mask: 0,
                write_mask: 0,
            },
            bias: wgpu::DepthBiasState::default(),
        });
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: desc.label,
            source: desc.source,
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: desc.pipeline_layout_label,
            bind_group_layouts: desc.bind_group_layouts,
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
            depth_stencil,
            multisample: wgpu::MultisampleState {
                count: SAMPLE_COUNT,
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
    pub bind_group_layouts: &'a [&'a wgpu::BindGroupLayout],
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
            bind_group_layouts: &[],
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



pub struct RenderPass<'a>(wgpu::RenderPass<'a>);

impl<'a> RenderPass<'a> {
    pub fn use_shader(&mut self, shader: &Shader) {
        self.0.set_pipeline(&shader.pipeline);
    }

    pub fn use_bind_group(
        &mut self,
        index: u32,
        bind_group: &wgpu::BindGroup,
        offsets: &[wgpu::DynamicOffset],
    ) {
        self.0.set_bind_group(index, bind_group, offsets);
    }

    pub fn use_vertex_buffer(&mut self, slot: u32, buffer: wgpu::BufferSlice) {
        self.0.set_vertex_buffer(slot, buffer);
    }

    pub fn use_index_buffer(&mut self, buffer: wgpu::BufferSlice, format: wgpu::IndexFormat) {
        self.0.set_index_buffer(buffer, format);
    }

    pub fn set_scissor_rect(&mut self, x: u32, y: u32, width: u32, height: u32) {
        self.0.set_scissor_rect(x, y, width, height);
    }

    pub fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) {
        self.0.draw(vertices, instances);
    }

    pub fn draw_indexed(&mut self, indices: Range<u32>, base_vertex: i32, instances: Range<u32>) {
        self.0.draw_indexed(indices, base_vertex, instances);
    }
}



#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[derive(bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pos: [f32; 2],
    color: u32,
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Uint32,
                },
            ]
        }
    }
}
