//! Viewport type



use bog_core::{Mat4, Rect, Vec2};



#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Viewport {
    pub physical_size: Vec2,
    pub scale_factor: f64,
    pub projection: Mat4,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            physical_size: Vec2::new(1.0, 1.0),
            scale_factor: 1.0,
            projection: Mat4::orthographic_rh_gl(
                0.0, 1.0,
                1.0, 0.0,
                -1.0, 1.0
            ),
        }
    }
}

impl Viewport {
    #[inline]
    pub const fn rect(&self) -> Rect {
        Rect::new(Vec2::ZERO, self.physical_size)
    }

    pub fn resize(&mut self, physical_size: Vec2) {
        self.physical_size = physical_size;
        self.projection = Mat4::orthographic_rh_gl(
            0.0, self.physical_size.x as f32,
            self.physical_size.y as f32, 0.0,
            -1.0, 1.0,
        );
    }
}
