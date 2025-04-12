//! Painter abstraction



use std::ops::Range;

use crate::math::{vec2, Vec2};

use super::{RenderPass, Shader, ShaderDescriptor, Vertex, WindowGraphics};



// TODO: It might be better to create a custom `PaintVertex` type.
pub struct Painter {
    shader: Shader,
    vertex_buffer: PaintBuffer,
    index_buffer: PaintBuffer,
}

impl Painter {
    pub fn new(graphics: &WindowGraphics) -> Self {
        let shader = Shader::new(graphics.device(), ShaderDescriptor {
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(
                include_str!("painter.wgsl"),
            )),
            label: Some("Painter Shader"),
            pipeline_label: Some("Painter Render Pipeline"),
            pipeline_layout_label: Some("Painter Render Pipeline Layout"),
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

        const VBUF_CAP: wgpu::BufferAddress = (std::mem::size_of::<Vertex>() * 1024) as _;
        const IBUF_CAP: wgpu::BufferAddress = (std::mem::size_of::<u32>() * 1024 * 3) as _;

        let vertex_buffer_inner = graphics.device().create_buffer(&wgpu::BufferDescriptor {
            label: Some("Painter Vertex Buffer"),
            size: VBUF_CAP,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let index_buffer_inner = graphics.device().create_buffer(&wgpu::BufferDescriptor {
            label: Some("Painter Index Buffer"),
            size: IBUF_CAP,
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            shader,
            vertex_buffer: PaintBuffer {
                inner: vertex_buffer_inner,
                slices: Vec::with_capacity(64),
                capacity: VBUF_CAP,
            },
            index_buffer: PaintBuffer {
                inner: index_buffer_inner,
                slices: Vec::with_capacity(64),
                capacity: IBUF_CAP,
            },
        }
    }

    pub fn prepare(&mut self, graphics: &WindowGraphics, paints: &[PaintMesh]) {
        let (num_vertices, num_indices) = paints.iter().fold((0, 0), |acc, paint| {
            (acc.0 + paint.vertices.len(), acc.1 + paint.indices.len())
        });

        if num_indices > 0 {
            self.index_buffer.slices.clear();

            let required_buffer_size = (std::mem::size_of::<u32>() * num_indices) as u64;
            if self.index_buffer.capacity < required_buffer_size {
                // Resize index buffer, if needed.
                self.index_buffer.capacity = (self.index_buffer.capacity * 2)
                    .min(required_buffer_size);
                self.index_buffer.inner = graphics.device().create_buffer(&wgpu::BufferDescriptor {
                    label: Some("Painter Index Buffer"),
                    usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
                    size: self.index_buffer.capacity,
                    mapped_at_creation: false,
                });
            }

            let index_buffer_staging = graphics.queue().write_buffer_with(
                &self.index_buffer.inner,
                0,
                std::num::NonZeroU64::new(required_buffer_size).unwrap(),
            );

            let Some(mut index_buffer_staging) = index_buffer_staging else {
                panic!(
                    "Failed to create staging buffer for index data. Index count: {num_indices}. Required index buffer size: {required_buffer_size}. Actual size {} and capacity: {} (bytes)",
                    self.index_buffer.inner.size(),
                    self.index_buffer.capacity,
                );
            };

            let mut index_offset = 0;
            for paint in paints {
                let size = paint.indices.len() * std::mem::size_of::<u32>();
                let slice = index_offset..(size + index_offset);
                index_buffer_staging[slice.clone()]
                    .copy_from_slice(bytemuck::cast_slice(&paint.indices));
                self.index_buffer.slices.push(slice);
                index_offset += size;
            }
        }

        if num_vertices > 0 {
            self.vertex_buffer.slices.clear();

            let required_buffer_size = (std::mem::size_of::<Vertex>() * num_vertices) as u64;
            if self.vertex_buffer.capacity < required_buffer_size {
                // Resize vertex buffer if needed.
                self.vertex_buffer.capacity = (self.vertex_buffer.capacity * 2)
                    .min(required_buffer_size);
                self.vertex_buffer.inner = graphics.device().create_buffer(&wgpu::BufferDescriptor {
                    label: Some("Painter Vertex Buffer"),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    size: self.vertex_buffer.capacity,
                    mapped_at_creation: false,
                });
            }

            let vertex_buffer_staging = graphics.queue().write_buffer_with(
                &self.vertex_buffer.inner,
                0,
                std::num::NonZeroU64::new(required_buffer_size).unwrap(),
            );

            let Some(mut vertex_buffer_staging) = vertex_buffer_staging else {
                panic!(
                    "Failed to create staging buffer for vertex data. Vertex count: {num_vertices}. Required vertex buffer size: {required_buffer_size}. Actual size {} and capacity: {} (bytes)",
                    self.vertex_buffer.inner.size(),
                    self.vertex_buffer.capacity,
                );
            };

            let mut vertex_offset = 0;
            for paint in paints {
                let size = paint.vertices.len() * std::mem::size_of::<Vertex>();
                let slice = vertex_offset..(size + vertex_offset);
                vertex_buffer_staging[slice.clone()]
                    .copy_from_slice(bytemuck::cast_slice(&paint.vertices));
                self.vertex_buffer.slices.push(slice);
                vertex_offset += size;
            }
        }
    }

    pub fn render(&self, mut pass: RenderPass, paints: &[PaintMesh]) {
        pass.use_shader(&self.shader);

        let mut index_buffer_slices = self.index_buffer.slices.iter();
        let mut vertex_buffer_slices = self.vertex_buffer.slices.iter();

        for paint in paints {
            let index_buffer_slice = index_buffer_slices.next().unwrap();
            let vertex_buffer_slice = vertex_buffer_slices.next().unwrap();

            pass.use_index_buffer(
                self.index_buffer.inner.slice(
                    index_buffer_slice.start as u64..index_buffer_slice.end as u64,
                ),
                wgpu::IndexFormat::Uint32,
            );
            pass.use_vertex_buffer(
                0,
                self.vertex_buffer.inner.slice(
                    vertex_buffer_slice.start as u64..vertex_buffer_slice.end as u64,
                ),
            );
            pass.draw_indexed(0..paint.indices.len() as u32, 0, 0..1);
        }
    }
}

pub struct PaintMesh {
    pub indices: Vec<u32>,
    pub vertices: Vec<Vertex>,
}

impl PaintMesh {
    pub fn quad(pos: Vec2, size: Vec2, color: u32) -> Self {
        Self {
            indices: [0, 1, 2, 2, 1, 3].to_vec(),
            vertices: vec![
                Vertex { pos: pos.into(), color },
                Vertex { pos: [pos.x + size.x, pos.y], color },
                Vertex { pos: [pos.x, pos.y + size.y], color },
                Vertex { pos: [pos.x + size.x, pos.y + size.y], color },
            ],
        }
    }
}

pub struct PaintBuffer {
    inner: wgpu::Buffer,
    slices: Vec<Range<usize>>,
    capacity: wgpu::BufferAddress,
}



pub struct Rectangle {
    pub pos: Vec2,
    pub size: Vec2,
    pub color: u32,
    pub corner_radii: [f32; 4], // pqbd
}

impl Rectangle {
    pub fn to_mesh(self) -> PaintMesh {
        let mut geometry: lyon::tessellation::VertexBuffers<Vertex, u32>
            = lyon::tessellation::VertexBuffers::new();
        let options = lyon::tessellation::FillOptions::default();
        let constructor = |vertex: lyon::tessellation::FillVertex| {
            Vertex {
                pos: vertex.position().into(),
                color: self.color,
            }
        };
        let mut geometry_builder
            = lyon::tessellation::BuffersBuilder::new(&mut geometry, constructor);
        let mut tessellator = lyon::tessellation::FillTessellator::new();

        let mut builder = tessellator.builder(
            &options,
            &mut geometry_builder,
        );

        builder.add_rounded_rectangle(
            &lyon::math::Box2D {
                min: lyon::math::point(self.pos.x, self.pos.y),
                max: lyon::math::point(self.pos.x + self.size.x, self.pos.y + self.size.y),
            },
            &lyon::path::builder::BorderRadii {
                top_left: self.corner_radii[0],
                top_right: self.corner_radii[1],
                bottom_left: self.corner_radii[2],
                bottom_right: self.corner_radii[3],
            },
            lyon::path::Winding::Positive,
        );

        builder.build().unwrap();

        PaintMesh {
            indices: geometry.indices,
            vertices: geometry.vertices,
        }
    }
}
