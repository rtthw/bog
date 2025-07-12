//! Color type



/// A 32-bit color value.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Color {
    /// The red channel value (0-255).
    pub r: u8,
    /// The green channel value (0-255).
    pub g: u8,
    /// The blue channel value (0-255).
    pub b: u8,
    /// The alpha channel value (0-255).
    pub a: u8,
}

impl Color {
    /// A color with no values for its channels.
    pub const NONE: Self = Self::new(0, 0, 0, 0);

    /// Create a new color with the provided channel values.
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

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

    /// Alter this color's red channel value.
    pub fn with_red(self, red: u8) -> Self {
        Self {
            r: red,
            ..self
        }
    }

    /// Alter this color's green channel value.
    pub fn with_green(self, green: u8) -> Self {
        Self {
            g: green,
            ..self
        }
    }

    /// Alter this color's blue channel value.
    pub fn with_blue(self, blue: u8) -> Self {
        Self {
            b: blue,
            ..self
        }
    }

    /// Alter this color's alpha channel value.
    pub fn with_alpha(self, alpha: u8) -> Self {
        Self {
            a: alpha,
            ..self
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
