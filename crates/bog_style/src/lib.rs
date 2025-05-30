//! Bog Styling

#![no_std]



use bog_color::Color;
use slotmap::Key as _;



#[derive(Clone, Copy, Debug)]
pub struct Style {
    pub text: TextStyle,
    pub border: BorderStyle,
    pub shadow: ShadowStyle,
    pub fg_color: Color,
    pub bg_color: Color,
}

#[derive(Clone, Copy, Debug)]
pub struct BorderStyle {
    pub color: Color,
    pub width: Unit,
    pub radius: BorderRadius,
}

#[derive(Clone, Copy, Debug)]
pub enum BorderRadius {
    Uniform(f32),
    Corners {
        top_left_bottom_right: f32,
        top_right_bottom_left: f32,
    },
    Discrete {
        top_left: f32,
        top_right: f32,
        bottom_right: f32,
        bottom_left: f32,
    },
}

impl BorderRadius {
    pub fn to_absolute(&self) -> [f32; 4] {
        match self {
            Self::Uniform(n) => [*n; 4],
            Self::Corners { top_left_bottom_right, top_right_bottom_left } => [
                *top_left_bottom_right,
                *top_right_bottom_left,
                *top_left_bottom_right,
                *top_right_bottom_left,
            ],
            Self::Discrete { top_left, top_right, bottom_right, bottom_left } => [
                *top_left,
                *top_right,
                *bottom_right,
                *bottom_left,
            ],
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ShadowStyle {
    pub color: Color,
    pub offset_x: Unit,
    pub offset_y: Unit,
    pub spread: Unit,
}

#[derive(Clone, Copy, Debug)]
pub struct TextStyle {
    pub family: FontFamily<'static>,
    pub slant: TextSlant,
    pub weight: LineWeight,
    pub height: Unit,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FontFamily<'a> {
    Named(&'a str),
    Serif,
    SansSerif,
    Monospace,
    Cursive,
    Fantasy,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TextSlant {
    Normal,
    Italic,
    Oblique,
}

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug)]
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



#[derive(Clone, Copy, Debug)]
pub enum Unit {
    /// Absolute pixels.
    Px(f32),
    /// Relative to the current `em` size.
    Em(f32),
    /// Relative to the root `em` size.
    Rem(f32),
}

impl Unit {
    pub fn to_absolute(&self, rem: f32, em: f32) -> f32 {
        match self {
            Unit::Px(n) => *n,
            Unit::Em(n) => n * em,
            Unit::Rem(n) => n * rem,
        }
    }
}



/// A [`Style`] that has been resolved to absolute units.
pub struct ResolvedStyle {
    pub em: f32,

    pub fg_color: Color,
    pub bg_color: Color,

    pub border_color: Color,
    pub border_width: f32,
    pub border_radius: [f32; 4],

    pub shadow_color: Color,
    pub shadow_offset_x: f32,
    pub shadow_offset_y: f32,
    pub shadow_spread: f32,

    pub font_family: FontFamily<'static>,
    pub text_slant: TextSlant,
}



pub struct Theme {
    pub base_style: Style,
    pub root_em: f32,

    pub class_defaults: slotmap::SlotMap<slotmap::DefaultKey, Styling>,
    pub hover_classes: slotmap::SecondaryMap<slotmap::DefaultKey, Styling>,
    pub focus_classes: slotmap::SecondaryMap<slotmap::DefaultKey, Styling>,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            base_style: Style {
                text: TextStyle {
                    family: FontFamily::SansSerif,
                    slant: TextSlant::Normal,
                    weight: LineWeight::NORMAL,
                    height: Unit::Em(1.0),
                },
                border: BorderStyle {
                    color: Color::new(89, 89, 109, 255),
                    width: Unit::Px(2.0),
                    radius: BorderRadius::Uniform(5.0),
                },
                shadow: ShadowStyle {
                    color: Color::new(13, 13, 23, 255),
                    offset_x: Unit::Px(3.0),
                    offset_y: Unit::Px(3.0),
                    spread: Unit::Px(5.0),
                },
                fg_color: Color::new(191, 191, 197, 255),
                bg_color: Color::new(29, 29, 39, 255),
            },
            root_em: 17.0,

            class_defaults: slotmap::SlotMap::with_capacity(8),
            hover_classes: slotmap::SecondaryMap::with_capacity(8),
            focus_classes: slotmap::SecondaryMap::with_capacity(8),
        }
    }
}

impl Theme {
    pub fn new(base_style: Style, root_font_size: f32) -> Self {
        Self {
            base_style,
            root_em: root_font_size,

            class_defaults: slotmap::SlotMap::with_capacity(8),
            hover_classes: slotmap::SecondaryMap::with_capacity(8),
            focus_classes: slotmap::SecondaryMap::with_capacity(8),
        }
    }

    #[inline]
    pub const fn root_em(&self) -> f32 {
        self.root_em
    }

    pub fn resolve(&self, class: StyleClass, parent_em: f32, hovered: bool) -> ResolvedStyle {
        let mut style = self.class_defaults.get(class.0)
            .and_then(|styling| Some(self.base_style + styling))
            .unwrap_or(self.base_style);

        if hovered {
            if let Some(styling) = self.hover_classes.get(class.0) {
                style = styling.apply(style);
            }
        }

        // TODO: Actually calculate an accurate `em` size here.
        let em = style.text.height.to_absolute(self.root_em, parent_em);

        ResolvedStyle {
            em,

            fg_color: style.fg_color,
            bg_color: style.bg_color,

            border_color: style.border.color,
            border_width: style.border.width.to_absolute(self.root_em, em),
            border_radius: style.border.radius.to_absolute(),

            shadow_color: style.shadow.color,
            shadow_offset_x: style.shadow.offset_x.to_absolute(self.root_em, em),
            shadow_offset_y: style.shadow.offset_y.to_absolute(self.root_em, em),
            shadow_spread: style.shadow.spread.to_absolute(self.root_em, em),

            font_family: style.text.family,
            text_slant: style.text.slant,
        }
    }

    pub fn resolve_root(&self) -> ResolvedStyle {
        let em = self.root_em;

        ResolvedStyle {
            em,

            fg_color: self.base_style.fg_color,
            bg_color: self.base_style.bg_color,

            border_color: self.base_style.border.color,
            border_width: self.base_style.border.width.to_absolute(self.root_em, em),
            border_radius: self.base_style.border.radius.to_absolute(),

            shadow_color: self.base_style.shadow.color,
            shadow_offset_x: self.base_style.shadow.offset_x.to_absolute(self.root_em, em),
            shadow_offset_y: self.base_style.shadow.offset_y.to_absolute(self.root_em, em),
            shadow_spread: self.base_style.shadow.spread.to_absolute(self.root_em, em),

            font_family: self.base_style.text.family,
            text_slant: self.base_style.text.slant,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StyleClass(slotmap::DefaultKey);

// For more efficient usage with `bog_collections::NoHashMap`.
impl core::hash::Hash for StyleClass {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.0.data().as_ffi());
    }
}

impl StyleClass {
    /// The default styling to use by objects with this class.
    pub fn base(theme: &mut Theme, styling: Styling) -> Self {
        Self(theme.class_defaults.insert(styling))
    }

    /// The styling to use when the object identified by this class is hovered.
    pub fn with_hover(&self, theme: &mut Theme, styling: Styling) {
        theme.hover_classes.insert(self.0, styling);
    }

    /// The styling to use when the object identified by this class is in focus.
    pub fn with_focus(&self, theme: &mut Theme, styling: Styling) {
        theme.hover_classes.insert(self.0, styling);
    }

    /// The null style class. This is useful if you want to enforce the [`Theme`]'s defaults.
    #[inline]
    pub fn null() -> Self {
        Self(slotmap::DefaultKey::null())
    }
}

#[derive(Debug, Default)]
pub struct Styling {
    pub fg_color: Option<Color>,
    pub bg_color: Option<Color>,
    pub border_color: Option<Color>,
    pub border_width: Option<Unit>,
    pub border_radius: Option<BorderRadius>,
    pub shadow_color: Option<Color>,
    pub shadow_offset_x: Option<Unit>,
    pub shadow_offset_y: Option<Unit>,
    pub font_family: Option<FontFamily<'static>>,
    pub text_slant: Option<TextSlant>,
    pub text_weight: Option<LineWeight>,
    pub text_height: Option<Unit>,
}

impl Styling {
    /// Apply this set of changes to the given [`Style`].
    pub fn apply(&self, style: Style) -> Style {
        Style {
            text: TextStyle {
                slant: self.text_slant.unwrap_or(style.text.slant),
                weight: self.text_weight.unwrap_or(style.text.weight),
                height: self.text_height.unwrap_or(style.text.height),
                family: self.font_family.unwrap_or(style.text.family),
                // ..style.text
            },
            border: BorderStyle {
                color: self.border_color.unwrap_or(style.border.color),
                width: self.border_width.unwrap_or(style.border.width),
                radius: self.border_radius.unwrap_or(style.border.radius),
            },
            shadow: ShadowStyle {
                color: self.shadow_color.unwrap_or(style.shadow.color),
                offset_x: self.shadow_offset_x.unwrap_or(style.shadow.offset_x),
                offset_y: self.shadow_offset_y.unwrap_or(style.shadow.offset_y),
                spread: style.shadow.spread,
            },
            fg_color: self.fg_color.unwrap_or(style.fg_color),
            bg_color: self.bg_color.unwrap_or(style.bg_color),
        }
    }
}

impl core::ops::Add<&Styling> for Style {
    type Output = Self;

    fn add(self, rhs: &Styling) -> Self::Output {
        rhs.apply(self)
    }
}
