//! Sizing types



#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Length {
    Px(f32),
    Em(f32),
    Lh(f32),
    Rem(f32),
    Rlh(f32),
}

#[inline]
pub const fn px(value: f32) -> Length {
    Length::Px(value)
}

#[inline]
pub const fn em(value: f32) -> Length {
    Length::Em(value)
}

#[inline]
pub const fn lh(value: f32) -> Length {
    Length::Lh(value)
}

impl Length {
    pub fn to_absolute(self, sizing: SizingContext) -> f32 {
        match self {
            Length::Px(val) => val,
            Length::Em(val) => val * sizing.em(),
            Length::Lh(val) => val * sizing.lh(),
            Length::Rem(val) => val * sizing.rem(),
            Length::Rlh(val) => val * sizing.rlh(),
        }
    }
}



pub struct SizingContext {
    pub font: FontMetrics,
    pub root_font: FontMetrics,
    pub scale: f32,
    pub global_scale: f32,
}

impl SizingContext {
    #[inline]
    pub const fn em(&self) -> f32 {
        self.font.em
    }

    #[inline]
    pub const fn lh(&self) -> f32 {
        self.font.lh
    }

    #[inline]
    pub const fn rem(&self) -> f32 {
        self.root_font.em
    }

    #[inline]
    pub const fn rlh(&self) -> f32 {
        self.root_font.lh
    }
}



pub struct FontMetrics {
    pub em: f32,
    pub lh: f32,
}
