//! Primitive types for custom shaders



use std::any::{Any, TypeId};

use bog_math::Rect;



pub trait Primitive: std::fmt::Debug + Send + Sync + 'static {
    fn prepare(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        storage: &mut PrimitiveStorage,
        bounds: &Rect,
        // TODO: Setup some sort of viewport management system.
    );

    fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        storage: &PrimitiveStorage,
        target: &wgpu::TextureView,
        clip_bounds: &Rect<u32>,
    );
}

#[derive(Default)]
pub struct PrimitiveStorage {
    map: rustc_hash::FxHashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl PrimitiveStorage {
    pub fn has<T: 'static>(&self) -> bool {
        self.map.contains_key(&TypeId::of::<T>())
    }

    pub fn store<T: 'static + Send + Sync>(&mut self, data: T) {
        let _ = self.map.insert(TypeId::of::<T>(), Box::new(data));
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.map.get(&TypeId::of::<T>()).map(|pipeline| {
            pipeline
                .downcast_ref::<T>()
                .expect("value of this type does not exist in PrimitiveStorage")
        })
    }

    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.map.get_mut(&TypeId::of::<T>()).map(|pipeline| {
            pipeline
                .downcast_mut::<T>()
                .expect("value of this type does not exist in PrimitiveStorage")
        })
    }
}
