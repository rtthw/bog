//! Primitive types for custom shaders



use bog_collections::unit_map::UnitMap;
use bog_math::Rect;



pub trait Primitive: std::fmt::Debug + Send + Sync + 'static {
    fn prepare(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        storage: &mut UnitMap,
        bounds: &Rect,
        // TODO: Setup some sort of viewport management system.
    );

    fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        storage: &UnitMap,
        target: &wgpu::TextureView,
        clip_bounds: &Rect<u32>,
    );
}
