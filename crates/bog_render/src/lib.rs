//! Bog Render



pub mod buffer;
mod layer;
pub mod primitive;
mod quad;
mod types;
mod viewport;

pub use layer::*;
use quad::*;
pub use types::*;
pub use viewport::*;

use bog_math::{Mat4, Rect};



pub trait Render {
    fn start_layer(&mut self, bounds: Rect);
    fn end_layer(&mut self);
    fn start_transform(&mut self, transform: Mat4);
    fn end_transform(&mut self);
    fn fill_quad(&mut self, quad: Quad);
    fn clear(&mut self);
}



pub struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    format: wgpu::TextureFormat,
    staging_belt: wgpu::util::StagingBelt,

    layers: LayerStack,

    quad_pipeline: QuadPipeline,
    quad_manager: QuadManager,
}

impl Renderer {
    pub fn new(device: wgpu::Device, queue: wgpu::Queue, format: wgpu::TextureFormat) -> Self {
        let quad_pipeline = QuadPipeline::new(&device, format);

        Self {
            device,
            queue,
            format,
            staging_belt: wgpu::util::StagingBelt::new(buffer::MAX_WRITE_SIZE as u64),

            layers: LayerStack::new(),

            quad_pipeline,
            quad_manager: QuadManager::new(),
        }
    }

    pub fn render(
        &mut self,
        target: &wgpu::TextureView,
        viewport: &Viewport,
    ) -> wgpu::SubmissionIndex {
        // 1. Prepare.
        let scale_factor = viewport.scale_factor as f32;
        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("bog::encoder"),
            },
        );
        for layer in self.layers.iter_mut() {
            if !layer.quads.is_empty() {
                self.quad_manager.prepare(
                    &self.quad_pipeline,
                    &self.device,
                    &mut self.staging_belt,
                    &mut encoder,
                    &layer.quads,
                    viewport.projection,
                    scale_factor,
                );
            }
        }

        // 2. Render.
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("bog::render_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: target,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            let physical_bounds = Rect {
                x: 0.0,
                y: 0.0,
                w: viewport.physical_size.x,
                h: viewport.physical_size.y,
            };
            let mut quad_layer = 0;
            for layer in self.layers.iter() {
                let Some(physical_bounds) =
                    physical_bounds.intersection(&(layer.bounds * scale_factor))
                else {
                    continue;
                };

                let Some(scissor_rect) = physical_bounds.snap_to_u32() else {
                    continue;
                };

                if !layer.quads.is_empty() {
                    self.quad_manager.render(
                        &self.quad_pipeline,
                        quad_layer,
                        scissor_rect,
                        &layer.quads,
                        &mut render_pass,
                    );

                    quad_layer += 1;
                }
            }
        }

        // 3. Finalize.
        self.quad_manager.cleanup();
        self.staging_belt.finish();
        let submission = self.queue.submit(std::iter::once(encoder.finish()));
        self.staging_belt.recall();

        submission
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }
}

impl Render for Renderer {
    fn start_layer(&mut self, bounds: Rect) {
        self.layers.push_clip(bounds);
    }

    fn end_layer(&mut self) {
        self.layers.pop_clip();
    }

    fn start_transform(&mut self, transform: Mat4) {
        self.layers.push_transformation(transform);
    }

    fn end_transform(&mut self) {
        self.layers.pop_transformation();
    }

    fn fill_quad(&mut self, quad: Quad) {
        let (layer, transform) = self.layers.current_mut();
        let bounds = quad.bounds * transform;
        let color = quad.bg_color.to_u32();
        let quad = QuadPrimitive {
            position: [bounds.x, bounds.y],
            size: [bounds.w, bounds.h],
            border_color: quad.border.color.to_u32(),
            border_radius: quad.border.radius.into(),
            border_width: quad.border.width,
            shadow_color: quad.shadow.color.to_u32(),
            shadow_offset: quad.shadow.offset.into(),
            shadow_blur_radius: quad.shadow.blur_radius,
        };

        layer.quads.push(QuadSolid { color, quad });
    }

    fn clear(&mut self) {
        self.layers.clear();
    }
}
