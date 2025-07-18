//! Layer types



use bog_core::{Mat4, Rect};

use crate::{Image, QuadSolid, Text};



/// Rendering layer.
#[derive(Debug)]
pub struct Layer<'a> {
    /// The layer's clip rect.
    pub bounds: Rect,
    /// The set of quads to be rendered onto this layer.
    pub quads: Vec<QuadSolid>,
    /// The set of texts to be rendered onto this layer.
    pub texts: Vec<Text<'a>>,
    /// The set of images to be rendered onto this layer.
    pub images: Vec<Image>,
}

impl Default for Layer<'_> {
    fn default() -> Self {
        Self {
            bounds: Rect::INFINITE,
            quads: Vec::new(),
            texts: Vec::new(),
            images: Vec::new(),
        }
    }
}

impl<'a> Layer<'a> {
    pub fn with_bounds(bounds: Rect) -> Self {
        Self {
            bounds,
            ..Default::default()
        }
    }

    fn flush(&mut self) {}

    fn resize(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }

    fn reset(&mut self) {
        self.bounds = Rect::INFINITE;

        self.quads.clear();
        self.texts.clear();
    }
}



/// The render pass is the primary way in which things are drawn to the screen with the GPU.
pub struct RenderPass<'a> {
    layers: Vec<Layer<'a>>,
    transformations: Vec<Mat4>,
    previous: Vec<usize>,
    current: usize,
    active_count: usize,
}

impl<'a> RenderPass<'a> {
    pub fn new() -> Self {
        Self {
            layers: vec![Layer::default()],
            transformations: vec![Mat4::IDENTITY],
            previous: vec![],
            current: 0,
            active_count: 1,
        }
    }

    #[inline]
    pub fn current_mut(&mut self) -> (&mut Layer<'a>, Mat4) {
        let transformation = self.transformation();

        (&mut self.layers[self.current], transformation)
    }

    #[inline]
    pub fn transformation(&self) -> Mat4 {
        self.transformations.last().copied().unwrap()
    }

    pub fn push_clip(&mut self, bounds: Rect) {
        self.previous.push(self.current);

        self.current = self.active_count;
        self.active_count += 1;

        let bounds = bounds * self.transformation();

        if self.current == self.layers.len() {
            self.layers.push(Layer::with_bounds(bounds));
        } else {
            self.layers[self.current].resize(bounds);
        }
    }

    pub fn pop_clip(&mut self) {
        self.flush();

        self.current = self.previous.pop().unwrap();
    }

    pub fn push_transformation(&mut self, transformation: Mat4) {
        self.transformations.push(self.transformation() * transformation);
    }

    pub fn pop_transformation(&mut self) {
        let _ = self.transformations.pop();
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Layer<'a>> {
        self.flush();

        self.layers[..self.active_count].iter_mut()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Layer> {
        self.layers[..self.active_count].iter()
    }

    pub fn as_slice(&self) -> &[Layer] {
        &self.layers[..self.active_count]
    }

    pub fn flush(&mut self) {
        self.layers[self.current].flush();
    }

    pub fn clear(&mut self) {
        for layer in self.layers[..self.active_count].iter_mut() {
            layer.reset();
        }

        self.current = 0;
        self.active_count = 1;
        self.previous.clear();
    }
}
