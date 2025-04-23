//! Rendering types
//!
//! This is a collection of types to be used with the renderer.



use bog_color::Color;
use bog_math::{Rect, Vec2};



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
