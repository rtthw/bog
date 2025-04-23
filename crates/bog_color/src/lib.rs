//! Bog Color

#![no_std]



#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const NONE: Self = Self { r: 0, g: 0, b: 0, a: 0 };

    /// Pack this color into a `u32` as `0xRRGGBBAA`.
    pub fn to_u32(&self) -> u32 {
        let mut color = (self.r as u32) << 24;
        color += (self.g as u32) << 16;
        color += (self.b as u32) << 8;
        color += self.a as u32;

        color
    }

    /// Build a color from a `u32` in the form of `0xRRGGBBAA`.
    pub fn from_u32(color: u32) -> Self {
        Self {
            r: (color >> 24) as u8,
            g: (color >> 16) as u8,
            b: (color >> 8) as u8,
            a: color as u8,
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_converts_to_u32() {
        let color = Color { r: 0x11, g: 0x22, b: 0x33, a: 0x44 };
        let num = color.to_u32();

        assert_eq!(num, 0x11223344);
    }

    #[test]
    fn color_converts_from_u32() {
        let num: u32 = 0x11223344;
        let color = Color::from_u32(num);

        assert_eq!(color, Color { r: 0x11, g: 0x22, b: 0x33, a: 0x44 });
    }
}
