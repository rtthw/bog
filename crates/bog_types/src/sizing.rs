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

#[inline]
pub const fn rem(value: f32) -> Length {
    Length::Rem(value)
}

#[inline]
pub const fn rlh(value: f32) -> Length {
    Length::Rlh(value)
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



#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Percent(pub f32);

impl From<f32> for Percent {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

impl From<Portion> for Percent {
    fn from(value: Portion) -> Self {
        match value {
            Portion::Full => Self(1.0),
            Portion::Half => Self(0.5),
            Portion::OneThird => Self(0.33), // TODO: More specificity here?
            Portion::OneFourth => Self(0.25),
            Portion::OneFifth => Self(0.2),
            Portion::OneTenth => Self(0.1),
        }
    }
}

impl Percent {
    pub fn to_absolute(self, length_absolute: f32) -> f32 {
        self.0 * length_absolute
    }
}



#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Portion {
    Full,
    Half,
    OneThird,
    OneFourth,
    OneFifth,
    OneTenth,
}



pub struct SizingContext {
    pub font: FontMetrics,
    pub root_font: FontMetrics,
    pub scale: f32,
    pub global_scale: f32,
    pub ancestor_padding_box_width: f32,
    pub ancestor_padding_box_height: f32,
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
