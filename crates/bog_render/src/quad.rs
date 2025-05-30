//! Quad rendering



use bog_math::{Mat4, Rect};



pub struct QuadManager {
    pub layers: Vec<QuadLayer>,
    pub prepare_layer: usize,
}

impl QuadManager {
    pub fn new() -> Self {
        Self {
            layers: Vec::with_capacity(3),
            prepare_layer: 0,
        }
    }

    pub fn prepare(
        &mut self,
        pipeline: &QuadPipeline,
        device: &wgpu::Device,
        belt: &mut wgpu::util::StagingBelt,
        encoder: &mut wgpu::CommandEncoder,
        quads: &[QuadSolid],
        transform: Mat4,
        scale: f32,
    ) {
        debug_assert!(!quads.is_empty());

        if self.layers.len() <= self.prepare_layer {
            self.layers.push(QuadLayer::new(device, &pipeline.constants_layout));
        }

        let layer = &mut self.layers[self.prepare_layer];
        layer.prepare(device, encoder, belt, quads, transform, scale);

        self.prepare_layer += 1;
    }

    pub fn render<'a>(
        &'a self,
        pipeline: &'a QuadPipeline,
        layer: usize,
        bounds: Rect<u32>,
        quads: &[QuadSolid],
        render_pass: &mut wgpu::RenderPass<'a>,
    ) {
        if let Some(layer) = self.layers.get(layer) {
            render_pass.set_scissor_rect(bounds.x, bounds.y, bounds.w, bounds.h);
            pipeline.render(render_pass, &layer.constants, layer, 0..quads.len());
        }
    }

    pub fn cleanup(&mut self) {
        self.prepare_layer = 0;
    }
}

pub struct QuadLayer {
    constants: wgpu::BindGroup,
    constants_buffer: wgpu::Buffer,
    instance_buffer: crate::buffer::Buffer<QuadSolid>,
    instance_count: usize,
}

impl QuadLayer {
    pub fn new(device: &wgpu::Device, constant_layout: &wgpu::BindGroupLayout) -> Self {
        let constants_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("bog::uniforms_buffer::quad"),
            size: core::mem::size_of::<Uniforms>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let constants = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("bog::uniforms_bind_group::quad"),
            layout: constant_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: constants_buffer.as_entire_binding(),
            }],
        });
        let instance_buffer = crate::buffer::Buffer::new(
            device,
            "bog::buffer::quad",
            2000,
            wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        );

        Self {
            constants,
            constants_buffer,
            instance_buffer,
            instance_count: 2000,
        }
    }

    pub fn prepare(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        belt: &mut wgpu::util::StagingBelt,
        quads: &[QuadSolid],
        transform: Mat4,
        scale: f32,
    ) {
        let uniforms = Uniforms::new(transform, scale);
        let bytes = bytemuck::bytes_of(&uniforms);

        belt.write_buffer(
            encoder,
            &self.constants_buffer,
            0,
            (bytes.len() as u64).try_into().expect("sized uniforms"),
            device,
        ).copy_from_slice(bytes);

        let _ = self.instance_buffer.resize(device, quads.len());
        let _ = self.instance_buffer.write(device, encoder, belt, 0, quads);

        self.instance_count = quads.len();
    }
}



#[derive(Clone, Copy, Debug)]
#[derive(bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
struct Uniforms {
    transform: [f32; 16],
    scale: f32,
    _padding: [f32; 3], // Align to `mat4x4<f32>`.
}

impl Uniforms {
    fn new(transform: Mat4, scale: f32) -> Self {
        Self {
            transform: *transform.as_ref(),
            scale,
            _padding: [0.0, 0.0, 0.0],
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[derive(bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct QuadPrimitive {
    pub position: [f32; 2],
    pub size: [f32; 2],
    pub border_color: u32,
    pub border_radius: [f32; 4], // pqdb ordering
    pub border_width: f32,
    pub shadow_color: u32,
    pub shadow_offset: [f32; 2],
    pub shadow_blur_radius: f32,
}

#[derive(Clone, Copy, Debug)]
#[derive(bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct QuadSolid {
    pub color: u32,
    pub quad: QuadPrimitive,
}

pub struct QuadPipeline {
    pipeline: wgpu::RenderPipeline,
    constants_layout: wgpu::BindGroupLayout,
}

impl QuadPipeline {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let constants_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("bog::uniforms_layout::quad"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(
                        core::mem::size_of::<Uniforms>() as wgpu::BufferAddress,
                    ),
                },
                count: None,
            }],
        });

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("bog::pipeline_layout::quad"),
            push_constant_ranges: &[],
            bind_group_layouts: &[&constants_layout],
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("bog::shader::quad"),
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(
                include_str!("shaders/quad.wgsl"),
            )),
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("bog::pipeline::quad"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: core::mem::size_of::<QuadSolid>() as u64,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes: &wgpu::vertex_attr_array!(
                        // Color.
                        0 => Uint32,
                        // Position.
                        1 => Float32x2,
                        // Size.
                        2 => Float32x2,
                        // Border color.
                        3 => Uint32,
                        // Border radius.
                        4 => Float32x4,
                        // Border width.
                        5 => Float32,
                        // Shadow color.
                        6 => Uint32,
                        // Shadow offset.
                        7 => Float32x2,
                        // Shadow blur radius.
                        8 => Float32,
                    ),
                }],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::SrcAlpha,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add,
                        },
                        alpha: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::One,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add,
                        },
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options:
                    wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                front_face: wgpu::FrontFace::Cw,
                ..Default::default()
            },
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
            constants_layout,
        }
    }

    pub fn render<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
        constants: &'a wgpu::BindGroup,
        layer: &'a QuadLayer,
        range: core::ops::Range<usize>,
    ) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, constants, &[]);
        render_pass.set_vertex_buffer(0, layer.instance_buffer.slice(..));

        render_pass.draw(0..6, range.start as u32..range.end as u32);
    }
}
