//! Type map



use core::{any::TypeId, hash::{BuildHasherDefault, Hasher}};



/// A map that can store homogenous values for any number of types.
#[derive(Default)]
pub struct TypeMap<V> {
    map: hashbrown::HashMap<TypeId, V, BuildHasherDefault<TypeIdHasher>>,
}

impl<V> TypeMap<V> {
    /// Create an empty type map.
    pub const fn new() -> Self {
        Self {
            map: hashbrown::HashMap::with_hasher(BuildHasherDefault::new()),
        }
    }

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



/// A hashless hasher designed specifically with [`TypeId`]s in mind.
#[derive(Default)]
pub struct TypeIdHasher {
    hash: u64,
}

impl Hasher for TypeIdHasher {
    fn write_u64(&mut self, n: u64) {
        // Only a single value can be hashed, so the old hash should be zero.
        debug_assert_eq!(self.hash, 0);
        self.hash = n;
    }

    fn write_u128(&mut self, n: u128) {
        debug_assert_eq!(self.hash, 0);
        self.hash = n as u64;
    }

    fn write(&mut self, _bytes: &[u8]) {
        panic!("Type ID is the wrong type!")
    }

    fn finish(&self) -> u64 {
        self.hash
    }
}
