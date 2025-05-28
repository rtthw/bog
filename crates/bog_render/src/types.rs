//! Rendering types
//!
//! This is a collection of types to be used with the renderer.



use bog_color::Color;
use bog_math::{Rect, Vec2};
use bog_style::{FontFamily, TextSlant};



/// A renderable piece of text.
#[derive(Clone, Debug)]
pub struct Text<'a> {
    pub content: &'a str,
    pub pos: Vec2,
    pub size: f32,
    pub color: Color,
    /// Set this to `0.0` if you just want to use the font's default line height.
    pub line_height: f32,
    pub font_family: FontFamily<'static>,
    pub text_slant: TextSlant,
    pub bounds: Vec2,
}

impl Default for Text<'_> {
    fn default() -> Self {
        Self {
            content: "",
            pos: Vec2::ZERO,
            size: 20.0,
            color: Color::default(),
            line_height: 20.0 * 1.2,
            font_family: FontFamily::SansSerif,
            text_slant: TextSlant::Normal,
            bounds: Vec2::INFINITY,
        }
    }
}



/// A renderable rectangle that can have a [`Border`] and [`Shadow`].
#[derive(Clone, Copy, Debug)]
pub struct Quad {
    /// The size and position of the quad.
    pub bounds: Rect,
    /// The [`Border`] applied around the quad.
    pub border: Border,
    /// The [`Shadow`]] applied under the quad.
    pub shadow: Shadow,
    /// The color used to fill in the quad.
    pub bg_color: Color,
}

impl Default for Quad {
    fn default() -> Self {
        Self {
            bounds: Rect::default(),
            border: Border::default(),
            shadow: Shadow::default(),
            bg_color: Color::default(),
        }
    }
}

impl Quad {
    /// Create a new quad with the given bounds and [`Color`], but no border or shadow.
    pub fn new_colored(bounds: Rect, bg_color: Color) -> Self {
        Self {
            bounds,
            bg_color,
            ..Default::default()
        }
    }
}



/// The border of a [`Quad`].
#[derive(Clone, Copy, Debug)]
pub struct Border {
    /// The color of the border.
    pub color: Color,
    /// The border's width, in pixels.
    pub width: f32,
    /// The radius of the border in `pqdb` order (top-left, top-right, bottom-right, bottom-left).
    pub radius: [f32; 4],
}

impl Default for Border {
    fn default() -> Self {
        Self::NONE
    }
}

impl Border {
    /// No border.
    pub const NONE: Self = Self {
        color: Color::NONE,
        width: 0.0,
        radius: [0.0; 4],
    };

    /// Create a new border with the given color, width, and radius on all 4 corners.
    pub const fn new(color: Color, width: f32, radius: f32) -> Self {
        Self {
            color,
            width,
            radius: [radius; 4],
        }
    }
}



/// The border of a [`Quad`].
#[derive(Clone, Copy, Debug)]
pub struct Shadow {
    /// The color of the shading.
    pub color: Color,
    /// The offset of the shadow, in pixels.
    pub offset: Vec2,
    /// The "spread" for the blurring effect of the shadow.
    pub blur_radius: f32,
}

impl Default for Shadow {
    fn default() -> Self {
        Self::NONE
    }
}

impl Shadow {
    /// No shadow.
    pub const NONE: Self = Self {
        color: Color::NONE,
        offset: Vec2::ZERO,
        blur_radius: 0.0,
    };

    /// Create a new shadow with the given color, offset, and blur radius.
    pub const fn new(color: Color, offset: Vec2, blur_radius: f32) -> Self {
        Self {
            color,
            offset,
            blur_radius,
        }
    }
}
