//! Unit map type



use core::any::{Any, TypeId};



/// A map that can store any value for any number of single types.
#[derive(Default)]
pub struct UnitMap {
    map: rustc_hash::FxHashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl UnitMap {
    /// Whether any value of the type `T` is stored in this map.
    pub fn has<T: 'static>(&self) -> bool {
        self.map.contains_key(&TypeId::of::<T>())
    }

    /// Insert the value `data` into this map.
    pub fn store<T: 'static + Send + Sync>(&mut self, data: T) {
        let _ = self.map.insert(TypeId::of::<T>(), Box::new(data));
    }

    /// Get the value associated with the type `T`, if any.
    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.map.get(&TypeId::of::<T>()).map(|pipeline| {
            pipeline
                .downcast_ref::<T>()
                .expect("value of this type does not exist in UnitMap")
        })
    }

    /// Get a mutable reference to the value associated with the type `T`, if any.
    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.map.get_mut(&TypeId::of::<T>()).map(|pipeline| {
            pipeline
                .downcast_mut::<T>()
                .expect("value of this type does not exist in UnitMap")
        })
    }
}
