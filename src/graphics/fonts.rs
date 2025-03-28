//! Font management



use std::{collections::HashMap, sync::Arc};

use three_d::CpuMesh;



#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("ttf parsing error")]
    TtfParsingError(#[from] ttf_parser::FaceParsingError),
    #[error("glyh outline error")]
    GlyphOutlineError(#[from] lyon::tessellation::TessellationError),
}



#[derive(Clone, Default)]
pub struct Fonts {
    map: HashMap<String, Font>,
}

impl Fonts {
    pub fn load_font(&mut self, name: &str, bytes: Vec<u8>, size: f32) -> Result<(), Error> {
        let font = Font::load(bytes, size)?;
        let _ = self.map.insert(name.to_string(), font);
        Ok(())
    }
}



#[derive(Clone)]
pub struct Font {
    data: Arc<dyn AsRef<[u8]> + Send + Sync>,
    swash_key: swash::CacheKey,
    index: u32,
    glyph_map: HashMap<u16, CpuMesh>,
    size: f32,
    row_height: f32,
}

impl Font {
    pub fn load(bytes: Vec<u8>, size: f32) -> Result<Self, Error> {
        let mut glyph_map = HashMap::with_capacity(100);

        let font_count = ttf_parser::fonts_in_collection(&bytes).unwrap_or(0);
        if font_count != 0 {
            // Multiple fonts found.
            todo!("Support font collections")
        } else {
            let index = 0;
            let swash_ref = swash::FontRef::from_index(&bytes, index as usize).unwrap();
            let face = ttf_parser::Face::parse(&bytes, index)?;
            // print_face_info(&face);
            let mut row_height: f32 = 0.0;
            let upe = face.units_per_em() as f32 / size;
            // TODO: Maybe this shouldn't be pre-outlining every glyph in the font.
            for glyph_id in 0..face.number_of_glyphs() {
                let ttf_id = ttf_parser::GlyphId(glyph_id);
                let _glyph_name = face.glyph_name(ttf_id);
                let mut outliner = GlyphOutliner::new(upe);
                if let Some(bbox) = face.outline_glyph(ttf_id, &mut outliner) {
                    row_height = row_height.max(bbox.height() as f32 / upe);
                    let path = outliner.finish();
                    let mesh = glyph_outline_path_to_mesh(path)
                        .unwrap(); // TODO: Handle individual glyph errors.
                    let _ = glyph_map.insert(glyph_id, mesh);
                }
            }

            let swash_key = swash_ref.key;
            row_height += face.line_gap() as f32;

            Ok(Self {
                data: Arc::new(bytes),
                swash_key,
                index,
                glyph_map,
                size,
                row_height,
            })
        }
    }
}



// --- Private



struct GlyphOutliner {
    upe: f32,
    builder: lyon::path::Builder,
}

impl ttf_parser::OutlineBuilder for GlyphOutliner {
    fn move_to(&mut self, x: f32, y: f32) {
        self.builder.begin(lyon::math::Point::new(x / self.upe, y / self.upe));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.builder.line_to(lyon::math::Point::new(x / self.upe, y / self.upe));
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.builder.quadratic_bezier_to(
            lyon::math::Point::new(x1 / self.upe, y1 / self.upe),
            lyon::math::Point::new(x / self.upe, y / self.upe),
        );
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.builder.cubic_bezier_to(
            lyon::math::Point::new(x1 / self.upe, y1 / self.upe),
            lyon::math::Point::new(x2 / self.upe, y2 / self.upe),
            lyon::math::Point::new(x / self.upe, y / self.upe),
        );
    }

    fn close(&mut self) {
        self.builder.close();
    }
}

impl GlyphOutliner {
    fn new(upe: f32) -> Self {
        Self {
            upe,
            builder: lyon::path::Path::builder(),
        }
    }

    fn finish(self) -> lyon::path::Path {
        self.builder.build()
    }
}

fn glyph_outline_path_to_mesh(path: lyon::path::Path) -> Result<CpuMesh, Error> {
    let mut tessellator = lyon::tessellation::FillTessellator::new();
    let mut geometry: lyon::tessellation::VertexBuffers<three_d::Vec3, u32>
        = lyon::tessellation::VertexBuffers::new();
    let options = lyon::tessellation::FillOptions::default();
    tessellator.tessellate_path(
        &path,
        &options,
        &mut lyon::tessellation::BuffersBuilder::new(
            &mut geometry,
            |vertex: lyon::tessellation::FillVertex| {
                three_d::vec3(vertex.position().x, vertex.position().y, 0.0)
            },
        ),
    )?;

    Ok(CpuMesh {
        positions: three_d::Positions::F32(geometry.vertices),
        indices: three_d::Indices::U32(geometry.indices),
        ..Default::default()
    })
}
