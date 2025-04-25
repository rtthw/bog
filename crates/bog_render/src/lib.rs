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

use bog_color::Color;
use bog_math::{Mat4, Rect, Vec2};



pub trait Render {
    fn start_layer(&mut self, bounds: Rect);
    fn end_layer(&mut self);
    fn start_transform(&mut self, transform: Mat4);
    fn end_transform(&mut self);
    fn fill_quad(&mut self, quad: Quad);
    fn fill_text(&mut self, text: &Text);
    fn clear(&mut self);
}



pub struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    staging_belt: wgpu::util::StagingBelt,

    layers: LayerStack,

    quad_pipeline: QuadPipeline,
    quad_manager: QuadManager,

    text_pipeline: TextPipeline,
    text_manager: TextManager,
}

impl Renderer {
    pub fn new(device: wgpu::Device, queue: wgpu::Queue, format: wgpu::TextureFormat) -> Self {
        let quad_pipeline = QuadPipeline::new(&device, format);
        let text_pipeline = TextPipeline::new(&device, &queue, format);

        Self {
            device,
            queue,
            staging_belt: wgpu::util::StagingBelt::new(buffer::MAX_WRITE_SIZE as u64),

            layers: LayerStack::new(),

            quad_pipeline,
            quad_manager: QuadManager::new(),

            text_pipeline,
            text_manager: TextManager::new(),
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
            if !layer.texts.is_empty() {
                self.text_manager.prepare(
                    &mut self.text_pipeline,
                    &self.device,
                    &self.queue,
                    &layer.texts,
                    viewport.projection,
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
            let mut text_layer = 0;
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
        self.staging_belt.finish();
        let submission = self.queue.submit(std::iter::once(encoder.finish()));
        self.staging_belt.recall();

        submission
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn resize(&mut self, viewport_size: Vec2) {
        self.text_pipeline.viewport.update(&self.queue, glyphon::Resolution {
            width: viewport_size.x as u32,
            height: viewport_size.y as u32,
        });
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

    fn fill_text(&mut self, text: &Text) {
        let (layer, _transform) = self.layers.current_mut();
        let mut buffer = glyphon::Buffer::new(
            &mut self.text_pipeline.font_system,
            glyphon::Metrics { font_size: text.size, line_height: text.line_height },
        );
        buffer.set_text(
            &mut self.text_pipeline.font_system,
            &text.content,
            &glyphon::Attrs::new()
                // TODO: Setup font attrs selection system.
                .family(glyphon::Family::Monospace),
            glyphon::Shaping::Basic,
        );
        buffer.shape_until_scroll(&mut self.text_pipeline.font_system, false);
        let text = TextBuffer {
            buffer,
            bounds: Rect::new(text.pos, text.bounds),
            color: text.color,
        };

        layer.texts.push(text);
    }

    fn clear(&mut self) {
        self.layers.clear();
    }
}



pub struct Text {
    pub content: String,
    pub pos: Vec2,
    pub size: f32,
    pub color: Color,
    pub line_height: f32,
    pub bounds: Vec2,
}



// ---



#[derive(Debug)]
pub struct TextBuffer {
    buffer: glyphon::Buffer,
    bounds: Rect,
    color: Color,
}

struct TextManager {
    layers: Vec<TextLayer>,
    prepare_layer: usize,
}

impl TextManager {
    fn new() -> Self {
        Self {
            layers: Vec::new(),
            prepare_layer: 0,
        }
    }

    fn prepare(
        &mut self,
        pipeline: &mut TextPipeline,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        texts: &[TextBuffer],
        transform: Mat4,
    ) {
        if self.layers.len() <= self.prepare_layer {
            self.layers.push(TextLayer {
                renderer: glyphon::TextRenderer::new(
                    &mut pipeline.atlas,
                    device,
                    wgpu::MultisampleState::default(),
                    None,
                ),
            });
        }

        let layer = &mut self.layers[self.prepare_layer];
        let text_areas = texts.iter().map(|t| {
            let bounds = t.bounds * transform; // TODO: Add `* t.transform`.
            glyphon::TextArea {
                buffer: &t.buffer,
                left: bounds.x,
                top: bounds.y,
                scale: 1.0, // TODO: Scaling?
                bounds: glyphon::TextBounds {
                    left: bounds.x as i32,
                    top: bounds.y as i32,
                    right: (bounds.x + bounds.w) as i32,
                    bottom: (bounds.y + bounds.h) as i32,
                },
                default_color: glyphon::Color(t.color.to_u32()),
                custom_glyphs: &[],
            }
        });
        layer.renderer.prepare(
            device,
            queue,
            &mut pipeline.font_system,
            &mut pipeline.atlas,
            &mut pipeline.viewport,
            text_areas,
            &mut pipeline.swash_cache,
        )
            .unwrap();
    }

    fn render<'a>(
        &'a self,
        pipeline: &'a TextPipeline,
        layer: usize,
        bounds: Rect<u32>,
        render_pass: &mut wgpu::RenderPass<'a>,
    ) {
        if let Some(layer) = self.layers.get(layer) {
            render_pass.set_scissor_rect(bounds.x, bounds.y, bounds.w, bounds.h);
            layer.renderer.render(&pipeline.atlas, &pipeline.viewport, render_pass)
                .unwrap();
        }
    }
}

struct TextLayer {
    renderer: glyphon::TextRenderer,
}

struct TextPipeline {
    font_system: glyphon::FontSystem,
    atlas: glyphon::TextAtlas,
    viewport: glyphon::Viewport,
    cache: glyphon::Cache,
    swash_cache: glyphon::SwashCache,
}

impl TextPipeline {
    fn new(device: &wgpu::Device, queue: &wgpu::Queue, format: wgpu::TextureFormat) -> Self {
        let cache = glyphon::Cache::new(device);

        Self {
            font_system: glyphon::FontSystem::new(),
            atlas: glyphon::TextAtlas::new(device, queue, &cache, format),
            viewport: glyphon::Viewport::new(device, &cache),
            cache,
            swash_cache: glyphon::SwashCache::new(),
        }
    }
}
