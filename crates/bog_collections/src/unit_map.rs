//! Unit map type



use std::any::{Any, TypeId};



#[derive(Default)]
pub struct UnitMap {
    map: rustc_hash::FxHashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl UnitMap {
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
                .expect("value of this type does not exist in UnitMap")
        })
    }

    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.map.get_mut(&TypeId::of::<T>()).map(|pipeline| {
            pipeline
                .downcast_mut::<T>()
                .expect("value of this type does not exist in UnitMap")
        })
    }
}
