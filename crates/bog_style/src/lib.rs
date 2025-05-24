//! Bog Styling



use bog_collections::NoHashMap;
use bog_color::Color;



pub struct Style {
    pub text: TextStyle,
    pub border: BorderStyle,
    pub shadow: ShadowStyle,
    pub bg_color: Color,
}

pub struct BorderStyle {
    pub color: Color,
    pub width: f32,
}

pub struct ShadowStyle {
    pub color: Color,
    pub spread: f32,
}

pub struct TextStyle {
    pub family: FontFamily<'static>,
    pub slant: TextSlant,
    pub weight: LineWeight,
}

pub enum FontFamily<'a> {
    Named(&'a str),
    Serif,
    SansSerif,
    Monospace,
    Cursive,
    Fantasy,
}

pub enum TextSlant {
    Normal,
    Italic,
    Oblique,
}

pub enum TextStretch {
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    Normal,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
}

pub struct LineWeight(pub u16);

impl LineWeight {
    /// Thin line weight (100), the thinnest value.
    pub const THIN: Self = Self(100);
    /// Extra light line weight (200).
    pub const EXTRA_LIGHT: Self = Self(200);
    /// Light line weight (300).
    pub const LIGHT: Self = Self(300);
    /// Normal line weight (400).
    pub const NORMAL: Self = Self(400);
    /// Medium line weight (500, higher than normal).
    pub const MEDIUM: Self = Self(500);
    /// Semi-bold line weight (600).
    pub const SEMIBOLD: Self = Self(600);
    /// Bold line weight (700).
    pub const BOLD: Self = Self(700);
    /// Extra-bold line weight (800).
    pub const EXTRA_BOLD: Self = Self(800);
    /// Black line weight (900), the thickest value.
    pub const BLACK: Self = Self(900);
}



pub struct Theme {
    pub base_style: Style,
    pub class_defaults: NoHashMap<u64, StyleUpdate>,
    pub hover_classes: NoHashMap<u64, StyleUpdate>,
    pub focus_classes: NoHashMap<u64, StyleUpdate>,
}

pub struct StyleUpdate {
    pub bg_color: Option<Color>,
    pub border_color: Option<Color>,
    pub shadow_color: Option<Color>,
    pub text_slant: Option<TextSlant>,
    pub text_weight: Option<LineWeight>,
}

impl StyleUpdate {
    /// Apply this set of changes to the given [`Style`].
    pub fn apply(self, style: Style) -> Style {
        Style {
            text: TextStyle {
                slant: self.text_slant.unwrap_or(style.text.slant),
                weight: self.text_weight.unwrap_or(style.text.weight),
                family: style.text.family,
                // ..style.text
            },
            border: BorderStyle {
                color: self.border_color.unwrap_or(style.border.color),
                width: style.border.width,
            },
            shadow: ShadowStyle {
                color: self.shadow_color.unwrap_or(style.shadow.color),
                spread: style.shadow.spread,
            },
            bg_color: self.bg_color.unwrap_or(style.bg_color),
        }
    }
}
