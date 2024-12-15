//! Color



/// A 4-byte RGBA color.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Color(pub [u8; 4]);

impl From<u32> for Color {
    fn from(value: u32) -> Self {
        let r = (value >> 24) as u8;
        let g = (value >> 16) as u8;
        let b = (value >> 8) as u8;
        let a = value as u8;

        Self([r, g, b, a])
    }
}

impl Color {
    /// Create a fully-opaque color from its red, green, and blue values.
    #[inline]
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self([r, g, b, 255])
    }

    /// Create a color from its red, green, blue, and alpha channels.
    #[inline]
    pub const fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self([r, g, b, a])
    }

    /// Create an opaque color with red, green, and blue channels that all have the same value.
    #[inline]
    pub const fn from_gray(v: u8) -> Self {
        Self([v, v, v, 255])
    }
}

impl Color {
    /// Red value.
    #[inline]
    pub const fn r(&self) -> u8 {
        self.0[0]
    }

    /// Green value.
    #[inline]
    pub const fn g(&self) -> u8 {
        self.0[1]
    }

    /// Blue value.
    #[inline]
    pub const fn b(&self) -> u8 {
        self.0[2]
    }

    /// Alpha value.
    #[inline]
    pub const fn a(&self) -> u8 {
        self.0[3]
    }

    /// Red, green, blue values as a tuple.
    #[inline]
    pub const fn rgb_tuple(&self) -> (u8, u8, u8) {
        (self.0[0], self.0[1], self.0[2])
    }

    /// Red, green, blue, and alpha values as a tuple.
    #[inline]
    pub const fn rgba_tuple(&self) -> (u8, u8, u8, u8) {
        (self.0[0], self.0[1], self.0[2], self.0[3])
    }
}
