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
    #[inline]
    pub const fn r(&self) -> u8 {
        self.0[0]
    }

    #[inline]
    pub const fn g(&self) -> u8 {
        self.0[1]
    }

    #[inline]
    pub const fn b(&self) -> u8 {
        self.0[2]
    }

    #[inline]
    pub const fn a(&self) -> u8 {
        self.0[3]
    }
}
