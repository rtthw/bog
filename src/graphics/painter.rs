//! Painter abstraction



use crate::math::Vec2;

use super::{RenderPass, Shader, ShaderDescriptor, Vertex, WindowGraphics};



// TODO: It might be better to create a custom `PaintVertex` type.
pub struct Painter {
    shader: Shader,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
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
        let vertex_buffer = graphics.device().create_buffer(&wgpu::BufferDescriptor {
            label: Some("Painter Vertex Buffer"),
            size: 64,
            usage: wgpu::BufferUsages::VERTEX,
            mapped_at_creation: false,
        });
        let index_buffer = graphics.device().create_buffer(&wgpu::BufferDescriptor {
            label: Some("Painter Index Buffer"),
            size: 64,
            usage: wgpu::BufferUsages::INDEX,
            mapped_at_creation: false,
        });

        Self {
            shader,
            vertex_buffer,
            index_buffer,
            num_indices: 0,
        }
    }

    pub fn render(&self, mut pass: RenderPass, paints: &[Paint]) {
        pass.use_shader(&self.shader);
        let (num_vertices, num_indices) = paints.iter().fold((0, 0), |acc, paint| {
            match paint {
                Paint::Quad { .. } => {
                    (acc.0 + 4, acc.1 + 6)
                }
            }
        });

        for paint in paints {

        }
        pass.use_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.use_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }
}

pub enum Paint {
    Quad {
        pos: Vec2,
        size: Vec2,
    },
}
