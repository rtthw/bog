//! Rectangle type



use glam::Vec3Swizzles as _;

use crate::{vec3, Mat4, Vec2};



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
    /// An unsized rectangle located at the origin.
    pub const INFINITE: Self = Self::at_origin(Vec2::new(f32::INFINITY, f32::INFINITY));

    /// Create a new rectangle with the provided `position` and `size`.
    #[inline]
    pub const fn new(position: Vec2, size: Vec2) -> Self {
        Self {
            x: position.x,
            y: position.y,
            w: size.x,
            h: size.y,
        }
    }

    /// Create a new rectangle at (0, 0).
    #[inline]
    pub const fn at_origin(size: Vec2) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
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

// Shaping.
impl Rect<f32> {
    /// Shrink this rectangle by the provided margins.
    ///
    /// The resulting rectangle will be centered inside this one.
    pub const fn shrink(self, margin_x: f32, margin_y: f32) -> Self {
        let doubled_margin_horizontal = margin_x * 2.0;
        let doubled_margin_vertical = margin_y * 2.0;

        if self.w < doubled_margin_horizontal || self.h < doubled_margin_vertical {
            Self::NONE
        } else {
            Self {
                x: self.x + margin_x,
                y: self.y + margin_y,
                w: self.w - doubled_margin_horizontal,
                h: self.h - doubled_margin_vertical,
            }
        }
    }

    /// Shrink this rectangle horizontally by the provided margin.
    ///
    /// The resulting rectangle will be centered inside this one.
    pub const fn shrink_h(self, margin: f32) -> Self {
        let doubled_margin = margin * 2.0;

        if self.w < doubled_margin {
            Self::NONE
        } else {
            Self {
                x: self.x + margin,
                y: self.y,
                w: self.w - doubled_margin,
                h: self.h,
            }
        }
    }

    /// Shrink this rectangle vertically by the provided margin.
    ///
    /// The resulting rectangle will be centered inside this one.
    pub const fn shrink_v(self, margin: f32) -> Self {
        let doubled_margin = margin * 2.0;

        if self.h < doubled_margin {
            Self::NONE
        } else {
            Self {
                x: self.x,
                y: self.y + margin,
                w: self.w,
                h: self.h - doubled_margin,
            }
        }
    }

    /// Create a new rectangle centered inside this one with the provided width and height.
    pub fn inner_centered(&self, width: f32, height: f32) -> Self {
        let x = self.x + ((self.w - width).max(0.0) / 2.0);
        let y = self.y + ((self.h - height).max(0.0) / 2.0);

        Self { x, y, w: width.min(self.w), h: height.min(self.h) }
    }

    /// Split this rectangle into `count` rows with the same height.
    pub fn rows(&self, count: usize) -> Vec<Self> {
        if count == 0 {
            return vec![*self];
        }

        let row_height = self.h / count as f32;
        (0..count)
            .map(|i| {
                Self {
                    x: self.x,
                    y: row_height * i as f32,
                    w: self.w,
                    h: row_height,
                }
            })
            .collect()
    }

    /// Split this rectangle into `count` columns with the same width.
    pub fn columns(&self, count: usize) -> Vec<Self> {
        if count == 0 {
            return vec![*self];
        }

        let col_width = self.w / count as f32;
        (0..count)
            .map(|i| {
                Self {
                    x: col_width * i as f32,
                    y: self.y,
                    w: col_width,
                    h: self.h,
                }
            })
            .collect()
    }

    /// Split this rectangle horizontally at the provided length.
    pub const fn split_len_h(&self, len: f32) -> (Self, Self) {
        (
            Self { x: self.x, y: self.y, w: len, h: self.h },
            Self { x: self.x + len, y: self.y, w: self.w - len, h: self.h },
        )
    }

    /// Split this rectangle horizontally at the provided length, starting from the right side.
    pub const fn split_len_rev_h(&self, len: f32) -> (Self, Self) {
        self.split_len_h(self.w - len)
    }

    /// Split this rectangle horizontally at the provided portion of its width.
    pub const fn split_portion_h(&self, portion: f32) -> (Self, Self) {
        self.split_len_h(self.w * portion)
    }

    /// Split this rectangle vertically at the provided length.
    pub const fn split_len_v(&self, len: f32) -> (Self, Self) {
        (
            Self { x: self.x, y: self.y, w: self.w, h: len },
            Self { x: self.x, y: self.y + len, w: self.w, h: self.h - len },
        )
    }

    /// Split this rectangle vertically at the provided length, starting from the bottom side.
    pub const fn split_len_rev_v(&self, len: f32) -> (Self, Self) {
        self.split_len_v(self.h - len)
    }

    /// Split this rectangle vertically at the provided portion of its height.
    pub const fn split_portion_v(&self, portion: f32) -> (Self, Self) {
        self.split_len_v(self.h * portion)
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
        let w = self.w * scale;
        let h = self.h * scale;

        Self {
            x: self.x + ((self.w - w) / 2.0),
            y: self.y + ((self.h - h) / 2.0),
            w,
            h,
        }
    }
}

impl core::ops::Mul<Mat4> for Rect<f32> {
    type Output = Self;

    fn mul(self, transform: Mat4) -> Self {
        let pos = self.position();
        let size = self.size();

        Self::new(
            transform.transform_point3(vec3(pos.x, pos.y, 0.0)).xy().into(),
            transform.transform_vector3(vec3(size.x, size.y, 0.0)).xy().into(),
        )
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rows_and_columns() {
        let a = Rect::at_origin(Vec2::splat(1.0));
        let b = Rect::at_origin(Vec2::splat(10.0));
        let (b_1, b_2) = b.split_len_v(5.0);
        let (b_3, b_4) = b.split_len_h(5.0);
        let (b_5, b_6) = b_3.split_portion_h(0.5);
        let (b_7, b_8) = b_4.split_portion_h(0.5);

        assert_eq!(a.rows(0), vec![a]);
        assert_eq!(a.rows(5), vec![a]);
        assert_eq!(a.rows(1), vec![a]);
        assert_eq!(a.columns(0), vec![a]);
        assert_eq!(a.columns(5), vec![a]);
        assert_eq!(a.columns(1), vec![a]);

        assert_eq!(b.rows(2), vec![b_1, b_2]);
        assert_eq!(b.columns(2), vec![b_3, b_4]);
        assert_eq!(b.columns(4), vec![b_5, b_6, b_7, b_8]);
    }
}
