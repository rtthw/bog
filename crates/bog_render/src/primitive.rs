//! Primitive types for custom shaders



use bog_collections::UnitMap;
use bog_math::Rect;

use crate::Viewport;



pub trait Primitive: core::fmt::Debug + Send + Sync + 'static {
    fn prepare(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        storage: &mut UnitMap,
        bounds: &Rect,
        viewport: &Viewport,
    );

    fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        storage: &UnitMap,
        target: &wgpu::TextureView,
        clip_bounds: &Rect<u32>,
    );
}
