//! Color type



use three_d::{Srgba, Vec4};



#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Color(Srgba);

impl Color {
    pub fn to_linear_srgb(&self) -> Vec4 {
        self.0.to_linear_srgb()
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self(Srgba::new_opaque(r, g, b))
    }

    pub fn from_hsl(h: f32, s: f32, l: f32) -> Self {
        // Clamp input values to valid ranges.
        let h = h.clamp(0.0, 360.0);
        let s = s.clamp(0.0, 100.0);
        let l = l.clamp(0.0, 100.0);

        hsl_to_rgb(h / 360.0, s / 100.0, l / 100.0)
    }
}

impl Into<[u8; 4]> for Color {
    fn into(self) -> [u8; 4] {
        self.0.into()
    }
}

impl Into<[f32; 4]> for Color {
    fn into(self) -> [f32; 4] {
        self.0.into()
    }
}



// ---



fn hsl_to_rgb(hue: f32, saturation: f32, lightness: f32) -> Color {
    let red: f32;
    let green: f32;
    let blue: f32;

    // Check if the color is achromatic (grayscale).
    if saturation == 0.0 {
        red = lightness;
        green = lightness;
        blue = lightness;
    } else {
        // Calculate RGB components for colored cases.
        let q = if lightness < 0.5 {
            lightness * (1.0 + saturation)
        } else {
            lightness + saturation - lightness * saturation
        };
        let p = 2.0 * lightness - q;
        red = hue_to_rgb(p, q, hue + 1.0 / 3.0);
        green = hue_to_rgb(p, q, hue);
        blue = hue_to_rgb(p, q, hue - 1.0 / 3.0);
    }

    Color(Srgba::new(
        (red * 255.0).round() as u8,
        (green * 255.0).round() as u8,
        (blue * 255.0).round() as u8,
        255,
    ))
}

fn hue_to_rgb(p: f32, q: f32, t: f32) -> f32 {
    // Adjust the hue value to be within the valid range [0, 1].
    let mut t = t;
    if t < 0.0 {
        t += 1.0;
    }
    if t > 1.0 {
        t -= 1.0;
    }

    // Calculate the RGB component based on the hue value.
    if t < 1.0 / 6.0 {
        p + (q - p) * 6.0 * t
    } else if t < 1.0 / 2.0 {
        q
    } else if t < 2.0 / 3.0 {
        p + (q - p) * (2.0 / 3.0 - t) * 6.0
    } else {
        p
    }
}
