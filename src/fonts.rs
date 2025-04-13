//! Font management



use std::sync::Arc;



type Result<T> = std::result::Result<T, FontError>;

#[derive(thiserror::Error, Debug)]
pub enum FontError {
    #[error("attempted to load invalid font")]
    InvalidFont,
    #[error("face parsing error")]
    ParsingError(#[from] ttf_parser::FaceParsingError),
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

impl<'a> ParsedFontFace<'a> {
    pub fn line_gap(&self) -> i16 {
        self.inner.line_gap()
    }
}
