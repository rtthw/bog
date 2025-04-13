//! Font management



use std::sync::Arc;



type Result<T> = std::result::Result<T, FontError>;

#[derive(thiserror::Error, Debug)]
pub enum FontError {
    #[error("face parsing error")]
    ParsingError(#[from] ttf_parser::FaceParsingError),
}



pub fn load_font(bytes: &[u8]) -> Result<FontFace> {
    todo!()
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

impl<'a> ParsedFontFace<'a> {
    pub fn glyph_is_colored(&self, id: u16) -> bool {
        self.inner.is_color_glyph(ttf_parser::GlyphId(id))
    }
}
