//! Bog Render



pub extern crate wgpu as gpu;

pub mod buffer;
mod image;
mod layer;
pub mod primitive;
mod quad;
mod text;
mod types;
mod viewport;

pub use layer::*;
use image::*;
use quad::*;
use text::*;
pub use types::*;
pub use viewport::*;

use bog_math::{vec2, Mat4, Rect, Vec2};



pub struct Renderer {
    device: gpu::Device,
    queue: gpu::Queue,
    staging_belt: gpu::util::StagingBelt,

    quad_pipeline: QuadPipeline,
    quad_manager: QuadManager,

    text_pipeline: TextPipeline,
    text_manager: TextManager,

    image_pipeline: ImagePipeline,
    image_manager: ImageManager,
}

impl Renderer {
    pub fn new(device: gpu::Device, queue: gpu::Queue, format: gpu::TextureFormat) -> Self {
        let quad_pipeline = QuadPipeline::new(&device, format);
        let text_pipeline = TextPipeline::new(&device, &queue, format);
        let image_pipeline = ImagePipeline::new(&device, format);

        Self {
            device,
            queue,
            staging_belt: gpu::util::StagingBelt::new(buffer::MAX_WRITE_SIZE as u64),

            quad_pipeline,
            quad_manager: QuadManager::new(),

            text_pipeline,
            text_manager: TextManager::new(),

            image_pipeline,
            image_manager: ImageManager::new(),
        }
    }

    pub fn render(
        &mut self,
        layer_stack: &mut LayerStack,
        target: &gpu::TextureView,
        viewport: &Viewport,
    ) -> gpu::SubmissionIndex {
        // 1. Prepare.
        // let start = std::time::Instant::now();
        let scale_factor = viewport.scale_factor as f32;
        let mut encoder = self.device.create_command_encoder(
            &gpu::CommandEncoderDescriptor {
                label: Some("bog::encoder"),
            },
        );
        for layer in layer_stack.iter_mut() {
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
            if !layer.texts.is_empty() {
                self.text_manager.prepare(
                    &mut self.text_pipeline,
                    &self.device,
                    &self.queue,
                    &layer.texts,
                    // viewport.projection,
                );
            }
        }

        // 2. Render.
        {
            let mut render_pass = encoder.begin_render_pass(&gpu::RenderPassDescriptor {
                label: Some("bog::render_pass"),
                color_attachments: &[Some(gpu::RenderPassColorAttachment {
                    view: target,
                    resolve_target: None,
                    ops: gpu::Operations {
                        load: gpu::LoadOp::Clear(gpu::Color::BLACK),
                        store: gpu::StoreOp::Store,
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
            let mut text_layer = 0;
            for layer in layer_stack.iter() {
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
                if !layer.texts.is_empty() {
                    self.text_manager.render(
                        &self.text_pipeline,
                        text_layer,
                        scissor_rect,
                        &mut render_pass,
                    );

                    text_layer += 1;
                }
            }
        }

        // 3. Finalize.
        self.quad_manager.cleanup();
        self.text_manager.cleanup();
        self.staging_belt.finish();
        let submission = self.queue.submit(std::iter::once(encoder.finish()));
        self.staging_belt.recall();

        // println!("render : {}us", std::time::Instant::now().duration_since(start).as_micros());

        submission
    }

    pub fn device(&self) -> &gpu::Device {
        &self.device
    }

    pub fn resize(&mut self, viewport_size: Vec2) {
        self.text_pipeline.viewport.update(&self.queue, glyphon::Resolution {
            width: viewport_size.x as u32,
            height: viewport_size.y as u32,
        });
    }

    /// The viewport's current [`Rect`].
    pub fn viewport_rect(&self) -> Rect {
        Rect::new(Vec2::ZERO, vec2(
            self.text_pipeline.viewport.resolution().width as f32,
            self.text_pipeline.viewport.resolution().height as f32,
        ))
    }

    pub fn load_font(&mut self, bytes: impl Into<Vec<u8>>) {
        self.text_pipeline.font_system.db_mut().load_font_data(bytes.into());
    }

    pub fn set_serif_family<S: Into<String>>(&mut self, name: impl Into<String>) {
        self.text_pipeline.font_system.db_mut().set_serif_family(name);
    }

    pub fn set_sans_serif_family<S: Into<String>>(&mut self, name: impl Into<String>) {
        self.text_pipeline.font_system.db_mut().set_sans_serif_family(name);
    }

    pub fn set_cursive_family<S: Into<String>>(&mut self, name: impl Into<String>) {
        self.text_pipeline.font_system.db_mut().set_cursive_family(name);
    }

    pub fn set_fantasy_family<S: Into<String>>(&mut self, name: impl Into<String>) {
        self.text_pipeline.font_system.db_mut().set_fantasy_family(name);
    }

    pub fn set_monospace_family(&mut self, name: impl Into<String>) {
        self.text_pipeline.font_system.db_mut().set_monospace_family(name);
    }

    /// Returns the minimum bounds required by the given [`Text`].
    pub fn measure_text(&mut self, text: &Text) -> Vec2 {
        let key = TextCacheKey::from(text);
        let (_hash, entry) = self.text_manager.cache
            .allocate(&mut self.text_pipeline.font_system, key);

        entry.min_bounds
    }

    pub fn text_pipeline(&mut self) -> &mut TextPipeline {
        &mut self.text_pipeline
    }
}

impl<'a> LayerStack<'a> {
    pub fn start_layer(&mut self, bounds: Rect) {
        self.push_clip(bounds);
    }

    pub fn end_layer(&mut self) {
        self.pop_clip();
    }

    pub fn start_transform(&mut self, transform: Mat4) {
        self.push_transformation(transform);
    }

    pub fn end_transform(&mut self) {
        self.pop_transformation();
    }

    pub fn fill_quad(&mut self, quad: Quad) {
        let (layer, transform) = self.current_mut();
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

    pub fn fill_text(&mut self, text: Text<'a>) {
        let (layer, transform) = self.current_mut();
        let bounds = text.bounds * transform;

        layer.texts.push(Text {
            bounds,
            ..text
        });
    }
}
