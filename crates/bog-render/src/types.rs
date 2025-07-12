//! Rendering types
//!
//! This is a collection of types to be used with the renderer.



use std::{hash::{Hash as _, Hasher as _}, path::{Path, PathBuf}};

use bog_core::{Color, Rect, Vec2};
pub use bytes::Bytes;



/// A renderable piece of text.
#[derive(Clone, Debug)]
pub struct Text<'a> {
    /// This text's content.
    pub content: TextContent<'a>,
    /// The clipping bounds for this text.
    pub bounds: Rect,
    /// This text's font size.
    pub size: f32,
    /// This text's color.
    pub color: Color,
    /// The font's line height, in pixels.
    ///
    /// Set this to `0.0` if you just want to use the font's default line height.
    pub line_height: f32,
    /// The font family selection for this text.
    pub font_family: FontFamily<'static>,
    /// The slant (normal, italic, oblique) of this text.
    pub text_slant: TextSlant,
}

impl Default for Text<'_> {
    fn default() -> Self {
        Self {
            content: "".into(),
            bounds: Rect::INFINITE,
            size: 20.0,
            color: Color::default(),
            line_height: 0.0, // 20.0 * 1.2,
            font_family: FontFamily::SansSerif,
            text_slant: TextSlant::Normal,
        }
    }
}

#[derive(Clone, Debug)]
pub enum TextContent<'a> {
    Owned(String),
    Borrowed(&'a str),
    Cow(std::borrow::Cow<'a, str>),
}

impl<'a> From<std::borrow::Cow<'a, str>> for TextContent<'a> {
    fn from(value: std::borrow::Cow<'a, str>) -> Self {
        Self::Cow(value)
    }
}

impl<'a> From<&'a str> for TextContent<'a> {
    fn from(value: &'a str) -> Self {
        Self::Borrowed(value)
    }
}

impl<'a> From<String> for TextContent<'a> {
    fn from(value: String) -> Self {
        Self::Owned(value)
    }
}

impl<'a> core::ops::Deref for TextContent<'a> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Owned(s) => &s,
            Self::Borrowed(s) => s,
            Self::Cow(s) => s.as_ref(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FontFamily<'a> {
    Named(&'a str),
    Serif,
    SansSerif,
    Monospace,
    Cursive,
    Fantasy,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TextSlant {
    Normal,
    Italic,
    Oblique,
}



/// A renderable rectangle that can have a fill [`Color`], [`Border`], and [`Shadow`].
#[derive(Clone, Copy, Debug)]
pub struct Quad {
    /// The size and position of the quad.
    pub bounds: Rect,
    /// The [`Border`] applied around the quad.
    pub border: Border,
    /// The [`Shadow`]] applied under the quad.
    pub shadow: Shadow,
    /// The color used to fill in the quad.
    pub bg_color: Color,
}

impl Default for Quad {
    fn default() -> Self {
        Self {
            bounds: Rect::default(),
            border: Border::default(),
            shadow: Shadow::default(),
            bg_color: Color::default(),
        }
    }
}

impl Quad {
    /// Create a new quad with the given bounds and [`Color`], but no border or shadow.
    pub fn new_colored(bounds: Rect, bg_color: Color) -> Self {
        Self {
            bounds,
            bg_color,
            ..Default::default()
        }
    }
}



/// The border of a [`Quad`].
#[derive(Clone, Copy, Debug)]
pub struct Border {
    /// The color of the border.
    pub color: Color,
    /// The border's width, in pixels.
    pub width: f32,
    /// The radius of the border in `pqdb` order (top-left, top-right, bottom-right, bottom-left).
    pub radius: [f32; 4],
}

impl Default for Border {
    fn default() -> Self {
        Self::NONE
    }
}

impl Border {
    /// No border.
    pub const NONE: Self = Self {
        color: Color::NONE,
        width: 0.0,
        radius: [0.0; 4],
    };

    /// Create a new border with the given [`Color`], width, and radius on all 4 corners.
    #[inline]
    pub const fn new(color: Color, width: f32, radius: f32) -> Self {
        Self {
            color,
            width,
            radius: [radius; 4],
        }
    }
}



/// The border of a [`Quad`].
#[derive(Clone, Copy, Debug)]
pub struct Shadow {
    /// The color of the shading.
    pub color: Color,
    /// The offset of the shadow, in pixels.
    pub offset: Vec2,
    /// The "spread" for the blurring effect of the shadow.
    pub blur_radius: f32,
}

impl Default for Shadow {
    fn default() -> Self {
        Self::NONE
    }
}

impl Shadow {
    /// No shadow.
    pub const NONE: Self = Self {
        color: Color::NONE,
        offset: Vec2::ZERO,
        blur_radius: 0.0,
    };

    /// Create a new shadow with the given [`Color`], offset, and blur radius.
    #[inline]
    pub const fn new(color: Color, offset: Vec2, blur_radius: f32) -> Self {
        Self {
            color,
            offset,
            blur_radius,
        }
    }
}



#[derive(Clone, Debug, PartialEq)]
pub enum Image {
    Raster(RasterImage, Rect),
    // Vector(VectorImage, Rect),
}

#[derive(Clone, Debug, PartialEq)]
pub struct RasterImage {
    pub handle: ImageHandle,
    pub filter_method: ImageFilterMethod,
    pub rotation: f32,
    pub opacity: f32,
    pub snap: bool,
}

impl From<ImageHandle> for RasterImage {
    fn from(value: ImageHandle) -> Self {
        Self {
            handle: value,
            filter_method: ImageFilterMethod::default(),
            rotation: 0.0,
            opacity: 1.0,
            snap: true,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ImageHandle {
    Path(u64, PathBuf),
    Bytes(u64, Bytes),
    // Rgba {
    //     width: u32,
    //     height: u32,
    //     pixels: Bytes,
    // },
}

impl ImageHandle {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
        let hash = {
            let mut hasher = rustc_hash::FxHasher::default();
            path.as_ref().hash(&mut hasher);

            hasher.finish()
        };

        Self::Path(hash, path.as_ref().to_path_buf())
    }

    pub fn from_bytes<B: Into<Bytes>>(bytes: B) -> Self {
        Self::Bytes(unique_image_handle_id(), bytes.into())
    }

    pub const fn id(&self) -> u64 {
        match self {
            Self::Path(id, _) => *id,
            Self::Bytes(id, _) => *id,
        }
    }
}

fn unique_image_handle_id() -> u64 {
    use std::sync::atomic::{self, AtomicU64};

    static NEXT_ID: AtomicU64 = AtomicU64::new(0);

    NEXT_ID.fetch_add(1, atomic::Ordering::Relaxed)
}

/// Image filtering strategy.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum ImageFilterMethod {
    /// Bilinear interpolation.
    #[default]
    Linear,
    /// Nearest neighbor.
    Nearest,
}
