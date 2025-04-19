//! Rectangle type

use crate::Vec2;



pub struct Rect<T = f32> {
    pub x: T,
    pub y: T,
    pub w: T,
    pub h: T,
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
}
