//! Text rendering functionality



use core::hash::Hash as _;

use bog_core::{Rect, Vec2};

use crate::{FontFamily, Text, TextSlant};



pub struct TextManager {
    pub(crate) cache: TextCache,
    layers: Vec<TextLayer>,
    prepare_layer: usize,
}

impl TextManager {
    pub fn new() -> Self {
        Self {
            cache: TextCache::default(),
            layers: Vec::new(),
            prepare_layer: 0,
        }
    }

    pub fn prepare(
        &mut self,
        pipeline: &mut TextPipeline,
        device: &gpu::Device,
        queue: &gpu::Queue,
        texts: &[Text],
        // transform: Mat4,
    ) {
        if self.layers.len() <= self.prepare_layer {
            self.layers.push(TextLayer {
                renderer: glyphon::TextRenderer::new(
                    &mut pipeline.atlas,
                    device,
                    gpu::MultisampleState::default(),
                    None,
                ),
            });
        }

        let layer = &mut self.layers[self.prepare_layer];
        let keys = texts.iter()
            .map(|t| {
                let key = TextCacheKey::from(t);
                let (hash, _entry) = self.cache.allocate(&mut pipeline.font_system, key);

                hash
            })
            .collect::<Vec<_>>();
        let text_areas = texts.iter().zip(keys.iter()).map(|(t, key)| {
            let entry = self.cache.get(&key).unwrap();
            let (r, g, b, a) = (t.color.r, t.color.g, t.color.b, t.color.a);

            glyphon::TextArea {
                buffer: &entry.buffer,
                left: t.bounds.x,
                top: t.bounds.y,
                scale: 1.0, // TODO: Scaling?
                bounds: glyphon::TextBounds {
                    left: t.bounds.x as i32,
                    top: t.bounds.y as i32,
                    right: (t.bounds.x + entry.min_bounds.x) as i32,
                    bottom: (t.bounds.y + entry.min_bounds.y) as i32,
                },
                default_color: glyphon::Color::rgba(r, g, b, a),
                custom_glyphs: &[],
            }
        });

        if layer.renderer.prepare(
            device,
            queue,
            &mut pipeline.font_system,
            &mut pipeline.atlas,
            &mut pipeline.viewport,
            text_areas,
            &mut pipeline.swash_cache,
        ).is_ok() {
            self.prepare_layer += 1;
        }
    }

    pub fn render<'a>(
        &'a self,
        pipeline: &'a TextPipeline,
        layer: usize,
        bounds: Rect<u32>,
        render_pass: &mut gpu::RenderPass<'a>,
    ) {
        if let Some(layer) = self.layers.get(layer) {
            render_pass.set_scissor_rect(bounds.x, bounds.y, bounds.w, bounds.h);
            layer.renderer.render(&pipeline.atlas, &pipeline.viewport, render_pass)
                .unwrap();
        }
    }

    pub fn cleanup(&mut self) {
        self.cache.trim();

        self.prepare_layer = 0;
    }
}

struct TextLayer {
    renderer: glyphon::TextRenderer,
}

pub struct TextPipeline {
    pub font_system: glyphon::FontSystem,
    atlas: glyphon::TextAtlas,
    pub viewport: glyphon::Viewport,
    swash_cache: glyphon::SwashCache,
}

impl TextPipeline {
    pub fn new(device: &gpu::Device, queue: &gpu::Queue, format: gpu::TextureFormat) -> Self {
        let cache = glyphon::Cache::new(device);

        Self {
            font_system: glyphon::FontSystem::new(),
            atlas: glyphon::TextAtlas::new(device, queue, &cache, format),
            viewport: glyphon::Viewport::new(device, &cache),
            swash_cache: glyphon::SwashCache::new(),
        }
    }
}



// --- Caching



#[derive(Clone, Copy, Debug)]
pub(crate) struct TextCacheKey<'a> {
    content: &'a str,
    size: f32,
    line_height: f32, // 0.0 == "use font line height"
    font_family: FontFamily<'a>,
    text_slant: TextSlant,
    // NOTE: If the position of a piece of text changes, but it's size doesn't, then the rendering
    //       remains unaffected. That's why we don't store position here.
    bounds: Vec2,
}

impl TextCacheKey<'_> {
    fn hash<H: core::hash::Hasher>(self, mut hasher: H) -> u64 {
        self.content.hash(&mut hasher);
        self.size.to_bits().hash(&mut hasher);
        self.line_height.to_bits().hash(&mut hasher);
        self.font_family.hash(&mut hasher);
        self.text_slant.hash(&mut hasher);
        self.bounds.x.to_bits().hash(&mut hasher);
        self.bounds.y.to_bits().hash(&mut hasher);

        hasher.finish()
    }
}

impl<'a> From<&'a Text<'a>> for TextCacheKey<'a> {
    fn from(value: &'a Text) -> Self {
        Self {
            content: &value.content,
            size: value.size,
            line_height: value.line_height,
            font_family: value.font_family,
            text_slant: value.text_slant,
            bounds: value.bounds.size(),
        }
    }
}

pub(crate) struct TextCacheEntry {
    pub(crate) buffer: glyphon::Buffer,
    pub(crate) min_bounds: Vec2,
}

#[derive(Default)]
pub(crate) struct TextCache {
    entries: rustc_hash::FxHashMap<u64, TextCacheEntry>,
    aliases: rustc_hash::FxHashMap<u64, u64>,
    recently_used: rustc_hash::FxHashSet<u64>,
}

impl TextCache {
    fn get(&self, key: &u64) -> Option<&TextCacheEntry> {
        self.entries.get(key)
    }

    pub(crate) fn allocate<'a>(
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
                if key.line_height == 0.0 {
                    (key.size * 1.4142_f32).max(f32::MIN_POSITIVE)
                } else {
                    key.line_height.max(f32::MIN_POSITIVE)
                },
            );
            let mut buffer = glyphon::Buffer::new(font_system, metrics);

            buffer.set_size(
                font_system,
                Some(key.bounds.x),
                if key.line_height == 0.0 {
                    None
                } else {
                    Some(key.bounds.y.max(key.line_height))
                },
            );
            buffer.set_text(
                font_system,
                key.content,
                &glyphon::Attrs::new()
                    .family(family_to_glyphon(key.font_family))
                    .style(slant_to_glyphon(key.text_slant)),
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

const fn family_to_glyphon(family: FontFamily) -> glyphon::Family {
    match family {
        FontFamily::Named(name) => glyphon::Family::Name(name),
        FontFamily::Serif => glyphon::Family::Serif,
        FontFamily::SansSerif => glyphon::Family::SansSerif,
        FontFamily::Monospace => glyphon::Family::Monospace,
        FontFamily::Cursive => glyphon::Family::Cursive,
        FontFamily::Fantasy => glyphon::Family::Fantasy,
    }
}

const fn slant_to_glyphon(slant: TextSlant) -> glyphon::Style {
    match slant {
        TextSlant::Normal => glyphon::Style::Normal,
        TextSlant::Italic => glyphon::Style::Italic,
        TextSlant::Oblique => glyphon::Style::Oblique,
    }
}
