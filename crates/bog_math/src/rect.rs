//! Rectangle type



use glam::Vec4Swizzles as _;

use crate::{vec4, Mat4, Vec2};



#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rect<T = f32> {
    pub x: T,
    pub y: T,
    pub w: T,
    pub h: T,
}

impl Default for Rect<f32> {
    fn default() -> Self {
        Self::NONE
    }
}

impl Rect<f32> {
    /// A zero-sized rectangle located at the origin.
    pub const NONE: Self = Self::new(Vec2::ZERO, Vec2::ZERO);
    pub const INFINITE: Self = Self::new(
        Vec2::ZERO,
        Vec2::new(f32::INFINITY, f32::INFINITY),
    );

    pub const fn new(pos: Vec2, size: Vec2) -> Self {
        Self {
            x: pos.x,
            y: pos.y,
            w: size.x,
            h: size.y,
        }
    }
}

impl Rect<f32> {
    pub fn position(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    pub fn size(&self) -> Vec2 {
        Vec2::new(self.w, self.h)
    }

    pub fn contains(&self, point: Vec2) -> bool {
        self.x <= point.x
            && point.x < self.x + self.w
            && self.y <= point.y
            && point.y < self.y + self.h
    }

    pub fn intersection(&self, other: &Self) -> Option<Self> {
        let x = self.x.max(other.x);
        let y = self.y.max(other.y);

        let lower_right_x = (self.x + self.w).min(other.x + other.w);
        let lower_right_y = (self.y + self.h).min(other.y + other.h);

        let w = lower_right_x - x;
        let h = lower_right_y - y;

        if w > 0.0 && h > 0.0 {
            Some(Self { x, y, w, h })
        } else {
            None
        }
    }

    pub fn snap_to_u32(self) -> Option<Rect<u32>> {
        let w = self.w as u32;
        let h = self.h as u32;

        if w < 1 || h < 1 {
            return None;
        }

        Some(Rect {
            x: self.x as u32,
            y: self.y as u32,
            w,
            h,
        })
    }
}

impl core::ops::Add<Vec2> for Rect<f32> {
    type Output = Self;

    fn add(self, rhs: Vec2) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            w: self.w,
            h: self.h,
        }
    }
}

impl core::ops::Mul<f32> for Rect<f32> {
    type Output = Self;

    fn mul(self, scale: f32) -> Self {
        Self {
            x: self.x * scale,
            y: self.y * scale,
            w: self.w * scale,
            h: self.h * scale,
        }
    }
}

impl core::ops::Mul<Mat4> for Rect<f32> {
    type Output = Self;

    fn mul(self, transform: Mat4) -> Self {
        let pos = self.position();
        let size = self.size();

        Self::new(
            transform.mul_vec4(vec4(pos.x, pos.y, 1.0, 0.0)).xy(),
            transform.mul_vec4(vec4(size.x, size.y, 1.0, 0.0)).xy(),
        )
    }
}
