//! Rendering types
//!
//! This is a collection of types to be used with the renderer.



use bog_color::Color;
use bog_math::{Rect, Vec2};



#[derive(Clone, Debug)]
pub struct Text {
    pub content: String,
    pub pos: Vec2,
    pub size: f32,
    pub color: Color,
    pub line_height: f32,
    pub font_family: FontFamily<'static>,
    pub font_style: FontStyle,
    pub bounds: Vec2,
}

impl Default for Text {
    fn default() -> Self {
        Self {
            content: String::new(),
            pos: Vec2::ZERO,
            size: 20.0,
            color: Color::default(),
            line_height: 20.0 * 1.2,
            font_family: FontFamily::SansSerif,
            font_style: FontStyle::Normal,
            bounds: Vec2::INFINITY,
        }
    }
}

pub type FontFamily<'a> = glyphon::Family<'a>;
pub type FontStyle = glyphon::Style;
pub type FontWeight = glyphon::Weight;
pub type FontWidth = glyphon::Stretch;



#[derive(Clone, Copy, Debug)]
pub struct Quad {
    pub bounds: Rect,
    pub border: Border,
    pub shadow: Shadow,
    pub bg_color: Color,
}



#[derive(Clone, Copy, Debug)]
pub struct Border {
    pub color: Color,
    pub width: f32,
    pub radius: [f32; 4],
}

impl Border {
    pub const NONE: Self = Self {
        color: Color::NONE,
        width: 0.0,
        radius: [0.0; 4],
    };

    pub const fn new(color: Color, width: f32, radius: f32) -> Self {
        Self {
            color,
            width,
            radius: [radius; 4],
        }
    }
}



#[derive(Clone, Copy, Debug)]
pub struct Shadow {
    pub color: Color,
    pub offset: Vec2,
    pub blur_radius: f32,
}

impl Shadow {
    pub const NONE: Self = Self {
        color: Color::NONE,
        offset: Vec2::ZERO,
        blur_radius: 0.0,
    };

    pub const fn new(color: Color, offset: Vec2, blur_radius: f32) -> Self {
        Self {
            color,
            offset,
            blur_radius,
        }
    }
}
