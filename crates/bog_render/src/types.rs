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

#[derive(Clone, Copy, Debug)]
pub struct Shadow {
    pub color: Color,
    pub offset: Vec2,
    pub blur_radius: f32,
}
