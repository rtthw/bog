//! Type map



use core::any::TypeId;



/// A map that can store homogenous values for any number of types.
#[derive(Default)]
pub struct TypeMap<V> {
    map: rustc_hash::FxHashMap<TypeId, V>,
}

impl<V> TypeMap<V> {
    /// Whether the type `K` has a value stored in this map.
    pub fn has<K: 'static>(&self) -> bool {
        self.map.contains_key(&TypeId::of::<K>())
    }

    /// Insert the value `V` associated with `K` into this map.
    pub fn insert<K: 'static>(&mut self, value: V) {
        let _ = self.map.insert(TypeId::of::<K>(), value);
    }

    /// Get the associated value `V` for `K`, if any.
    pub fn get<K: 'static>(&self) -> Option<&V> {
        self.map.get(&TypeId::of::<K>())
    }

    /// Get a mutable reference to the associated value `V` for `K`, if any.
    pub fn get_mut<K: 'static>(&mut self) -> Option<&mut V> {
        self.map.get_mut(&TypeId::of::<K>())
    }

    /// Clear all values from this map.
    pub fn clear(&mut self) {
        self.map.clear();
    }
}
