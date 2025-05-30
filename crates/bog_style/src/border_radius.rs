//! Border radius type



/// The radii for the corners of a rectangle.
#[derive(Clone, Copy, Debug)]
pub enum BorderRadius {
    /// Uniform radii.
    Uniform(f32),
    /// Radii defined by opposing diagonals.
    Corners {
        top_left_bottom_right: f32,
        top_right_bottom_left: f32,
    },
    /// Discrete radii.
    Discrete {
        top_left: f32,
        top_right: f32,
        bottom_right: f32,
        bottom_left: f32,
    },
}

impl BorderRadius {
    /// No corner rounding.
    pub const NONE: Self = Self::Uniform(0.0);

    /// Convert this border radius into a set of pixel values.
    pub const fn to_absolute(&self) -> [f32; 4] {
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

impl From<f32> for BorderRadius {
    fn from(value: f32) -> Self {
        Self::Uniform(value)
    }
}

impl From<[f32; 2]> for BorderRadius {
    fn from(value: [f32; 2]) -> Self {
        Self::Corners { top_left_bottom_right: value[0], top_right_bottom_left: value[1] }
    }
}

impl From<[f32; 4]> for BorderRadius {
    fn from(value: [f32; 4]) -> Self {
        Self::Discrete {
            top_left: value[0],
            top_right: value[1],
            bottom_right: value[2],
            bottom_left: value[3],
        }
    }
}

impl From<(f32, f32)> for BorderRadius {
    fn from(value: (f32, f32)) -> Self {
        Self::Corners { top_left_bottom_right: value.0, top_right_bottom_left: value.1 }
    }
}

impl From<(f32, f32, f32, f32)> for BorderRadius {
    fn from(value: (f32, f32, f32, f32)) -> Self {
        Self::Discrete {
            top_left: value.0,
            top_right: value.1,
            bottom_right: value.2,
            bottom_left: value.3,
        }
    }
}

impl Into<[f32; 4]> for BorderRadius {
    fn into(self) -> [f32; 4] {
        self.to_absolute()
    }
}
