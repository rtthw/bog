//! Font management



use std::sync::Arc;

use crate::math::{vec2, Vec2};



type Result<T> = std::result::Result<T, FontError>;

#[derive(thiserror::Error, Debug)]
pub enum FontError {
    #[error("attempted to load invalid font")]
    InvalidFont,
    #[error("glyph #{0} doesn't exist")]
    GlyphDoesntExist(u16),
    #[error("face parsing error")]
    ParsingError(#[from] ttf_parser::FaceParsingError),
    #[error("glyph outlining error")]
    OutliningError(#[from] lyon::tessellation::TessellationError),
}



pub fn load_font_face(bytes: &[u8]) -> Result<FontFace> {
    let font_count = ttf_parser::fonts_in_collection(bytes)
        .ok_or(FontError::InvalidFont)?;

    // TODO: Support font collections?
    if font_count != 0 {
        return Err(FontError::InvalidFont);
    }

    let face = FontFace {
        data: Arc::new(bytes.to_vec()),
        index: font_count,
    };

    Ok(face)
}



pub struct FontFace {
    data: Arc<dyn AsRef<[u8]> + Send + Sync>,
    index: u32,
}

impl FontFace {
    pub fn parse(&self) -> Result<ParsedFontFace> {
        let inner = ttf_parser::Face::parse((*self.data).as_ref(), self.index)?;

        Ok(ParsedFontFace {
            inner,
        })
    }
}



pub struct ParsedFontFace<'a> {
    inner: ttf_parser::Face<'a>,
}

// Cheap getters.
impl<'a> ParsedFontFace<'a> {
    pub fn units_per_em(&self) -> u16 {
        self.inner.units_per_em()
    }

    pub fn global_bbox(&self) -> ttf_parser::Rect { // TODO: Custom rect type.
        self.inner.global_bounding_box()
    }
}

impl<'a> ParsedFontFace<'a> {
    pub fn line_gap(&self) -> i16 {
        self.inner.line_gap()
    }

    pub fn glyph_mesh(&self, id: u16, size: f32) -> Result<GlyphMesh> {
        let scale = self.units_per_em() as f32 / size;
        let mut outliner = GlyphOutliner {
            scale,
            builder: lyon::path::Path::builder(),
        };
        let _bbox = self.inner.outline_glyph(ttf_parser::GlyphId(id), &mut outliner)
            .ok_or(FontError::GlyphDoesntExist(id));

        glyph_outline_path_to_mesh(outliner.builder.build())
    }
}



pub struct GlyphMesh {
    pub vertices: Vec<Vec2>,
    pub indices: Vec<u32>,
}



// --- Private



struct GlyphOutliner {
    scale: f32,
    builder: lyon::path::Builder,
}

impl ttf_parser::OutlineBuilder for GlyphOutliner {
    fn move_to(&mut self, x: f32, y: f32) {
        self.builder.begin(lyon::math::Point::new(x / self.scale, y / self.scale));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.builder.line_to(lyon::math::Point::new(x / self.scale, y / self.scale));
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.builder.quadratic_bezier_to(
            lyon::math::Point::new(x1 / self.scale, y1 / self.scale),
            lyon::math::Point::new(x / self.scale, y / self.scale),
        );
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.builder.cubic_bezier_to(
            lyon::math::Point::new(x1 / self.scale, y1 / self.scale),
            lyon::math::Point::new(x2 / self.scale, y2 / self.scale),
            lyon::math::Point::new(x / self.scale, y / self.scale),
        );
    }

    fn close(&mut self) {
        self.builder.close();
    }
}

fn glyph_outline_path_to_mesh(path: lyon::path::Path) -> Result<GlyphMesh> {
    let mut tessellator = lyon::tessellation::FillTessellator::new();
    let mut geometry: lyon::tessellation::VertexBuffers<Vec2, u32>
        = lyon::tessellation::VertexBuffers::new();
    let options = lyon::tessellation::FillOptions::default();
    tessellator.tessellate_path(
        &path,
        &options,
        &mut lyon::tessellation::BuffersBuilder::new(
            &mut geometry,
            |vertex: lyon::tessellation::FillVertex| {
                vec2(vertex.position().x, vertex.position().y)
            },
        ),
    )?;

    Ok(GlyphMesh {
        indices: geometry.indices,
        vertices: geometry.vertices,
    })
}
