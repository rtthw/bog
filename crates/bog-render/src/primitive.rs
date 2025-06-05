//! Primitive types for custom shaders



use bog_core::{Rect, UnitMap};

use crate::Viewport;



pub trait Primitive: core::fmt::Debug + Send + Sync + 'static {
    fn prepare(
        &self,
        device: &gpu::Device,
        queue: &gpu::Queue,
        format: gpu::TextureFormat,
        storage: &mut UnitMap,
        bounds: &Rect,
        viewport: &Viewport,
    );

    fn render(
        &self,
        encoder: &mut gpu::CommandEncoder,
        storage: &UnitMap,
        target: &gpu::TextureView,
        clip_bounds: &Rect<u32>,
    );
}
