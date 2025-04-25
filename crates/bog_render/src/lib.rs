//! Bog Render



pub mod buffer;
mod layer;
pub mod primitive;
mod quad;
mod types;
mod viewport;

use core::hash::Hash as _;

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
    fn fill_text(&mut self, text: Text);
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
        self.text_manager.cleanup();
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

    fn fill_text(&mut self, text: Text) {
        let (layer, _transform) = self.layers.current_mut();

        layer.texts.push(text);
    }

    fn clear(&mut self) {
        self.layers.clear();
    }
}



#[derive(Clone, Debug)]
pub struct Text {
    pub content: String,
    pub pos: Vec2,
    pub size: f32,
    pub color: Color,
    pub line_height: f32,
    pub font_family: FontFamily<'static>,
    pub font_style: FontStyle,
    pub bounds: Vec2,
}

pub type FontFamily<'a> = glyphon::Family<'a>;
pub type FontStyle = glyphon::Style;



// ---



#[derive(Clone, Copy, Debug)]
struct TextCacheKey<'a> {
    content: &'a str,
    size: f32,
    line_height: f32,
    font_family: FontFamily<'a>,
    font_style: FontStyle,
    bounds: Vec2,
}

impl TextCacheKey<'_> {
    fn hash<H: core::hash::Hasher>(self, mut hasher: H) -> u64 {
        self.content.hash(&mut hasher);
        self.size.to_bits().hash(&mut hasher);
        self.line_height.to_bits().hash(&mut hasher);
        self.font_family.hash(&mut hasher);
        self.font_style.hash(&mut hasher);
        self.bounds.x.to_bits().hash(&mut hasher);
        self.bounds.y.to_bits().hash(&mut hasher);

        hasher.finish()
    }
}

struct TextCacheEntry {
    buffer: glyphon::Buffer,
    min_bounds: Vec2,
}

#[derive(Default)]
struct TextCache {
    entries: rustc_hash::FxHashMap<u64, TextCacheEntry>,
    aliases: rustc_hash::FxHashMap<u64, u64>,
    recently_used: rustc_hash::FxHashSet<u64>,
}

impl TextCache {
    fn get(&self, key: &u64) -> Option<&TextCacheEntry> {
        self.entries.get(key)
    }

    fn allocate<'a>(
        &mut self,
        font_system: &mut glyphon::FontSystem,
        key: TextCacheKey<'a>,
    ) -> (u64, &mut TextCacheEntry)
    {
        let hash = key.hash(rustc_hash::FxHasher::default());
        if let Some(hash) = self.aliases.get(&hash) {
            let _ = self.recently_used.insert(*hash);

            return (*hash, self.entries.get_mut(hash).unwrap());
        }

        if let std::collections::hash_map::Entry::Vacant(entry) = self.entries.entry(hash) {
            let metrics = glyphon::Metrics::new(
                key.size,
                key.line_height.max(f32::MIN_POSITIVE),
            );
            let mut buffer = glyphon::Buffer::new(font_system, metrics);

            buffer.set_size(
                font_system,
                Some(key.bounds.x),
                Some(key.bounds.y.max(key.line_height)),
            );
            buffer.set_text(
                font_system,
                key.content,
                &glyphon::Attrs::new()
                    .family(key.font_family)
                    .style(key.font_style),
                glyphon::Shaping::Advanced,
            );

            let (bounds, has_rtl) = measure_glyphon_buffer(&buffer);

            if has_rtl {
                buffer.set_size(
                    font_system,
                    Some(bounds.x),
                    Some(bounds.y),
                );
            }

            let _ = entry.insert(TextCacheEntry {
                buffer,
                min_bounds: bounds,
            });

            for bounds in [
                bounds,
                Vec2 {
                    x: key.bounds.x,
                    ..bounds
                },
            ] {
                if key.bounds != bounds {
                    let _ = self.aliases.insert(
                        TextCacheKey { bounds, ..key }.hash(rustc_hash::FxHasher::default()),
                        hash,
                    );
                }
            }
        }

        let _ = self.recently_used.insert(hash);

        (hash, self.entries.get_mut(&hash).unwrap())
    }

    fn trim(&mut self) {
        self.entries.retain(|key, _| self.recently_used.contains(key));
        self.aliases.retain(|_, value| self.recently_used.contains(value));

        self.recently_used.clear();
    }
}

fn measure_glyphon_buffer(buffer: &glyphon::Buffer) -> (Vec2, bool) {
    let (width, height, has_rtl) = buffer.layout_runs().fold(
        (0.0, 0.0, false),
        |(width, height, has_rtl), run| {
            (
                run.line_w.max(width),
                height + run.line_height,
                has_rtl || run.rtl,
            )
        },
    );

    (Vec2::new(width, height), has_rtl)
}



struct TextManager {
    cache: TextCache,
    layers: Vec<TextLayer>,
    prepare_layer: usize,
}

impl TextManager {
    fn new() -> Self {
        Self {
            cache: TextCache::default(),
            layers: Vec::new(),
            prepare_layer: 0,
        }
    }

    fn prepare(
        &mut self,
        pipeline: &mut TextPipeline,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        texts: &[Text],
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
        let keys = texts.iter()
            .map(|t| {
                let key = TextCacheKey {
                    content: &t.content,
                    size: t.size,
                    line_height: t.line_height,
                    font_family: t.font_family,
                    font_style: t.font_style,
                    bounds: t.bounds,
                };
                let (hash, _entry) = self.cache.allocate(&mut pipeline.font_system, key);

                hash
            })
            .collect::<Vec<_>>();
        let text_areas = texts.iter().zip(keys.iter()).map(|(t, key)| {
            let entry = self.cache.get(&key).unwrap();
            let (r, g, b, a) = (t.color.r, t.color.g, t.color.b, t.color.a);

            glyphon::TextArea {
                buffer: &entry.buffer,
                left: t.pos.x,
                top: t.pos.y,
                scale: 1.0, // TODO: Scaling?
                bounds: glyphon::TextBounds {
                    left: t.pos.x as i32,
                    top: t.pos.y as i32,
                    right: (t.pos.x + entry.min_bounds.x) as i32,
                    bottom: (t.pos.y + entry.min_bounds.y) as i32,
                },
                default_color: glyphon::Color::rgba(r, g, b, a),
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

    fn cleanup(&mut self) {
        self.cache.trim();

        self.prepare_layer = 0;
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
