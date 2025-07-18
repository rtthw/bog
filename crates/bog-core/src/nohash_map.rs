//! No Hash Map



use core::{hash::BuildHasherDefault, marker::PhantomData};
use hashbrown::HashMap;



/// A [`HashMap`] that doesn't hash.
pub struct NoHashMap<K, V> {
    map: HashMap<K, V, BuildHasherDefault<NoHashHasher<K>>>,
}

impl<K, V> Default for NoHashMap<K, V> {
    fn default() -> Self {
        Self {
            map: HashMap::default()
        }
    }
}

impl<K: Default + Eq + core::hash::Hash + Into<u64>, V> NoHashMap<K, V> {
    /// Create an empty map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a map with space pre-allocated for `capacity` key-value pairs.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            map: HashMap::with_capacity_and_hasher(capacity, BuildHasherDefault::new()),
        }
    }

    /// Get the value associated with the given key.
    pub fn get(&self, k: &K) -> Option<&V> {
        self.map.get(k)
    }

    /// Get a mutable reference to the value associated with the given key.
    pub fn get_mut(&mut self, k: &K) -> Option<&mut V> {
        self.map.get_mut(k)
    }

    /// Insert the given key-value pair into the map.
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.map.insert(k, v)
    }
}



// ---



#[derive(Clone, Copy, Debug, Default)]
struct NoHashHasher<T>(u64, PhantomData<T>);

impl<T: Into<u64>> core::hash::Hasher for NoHashHasher<T> {
    fn write(&mut self, _: &[u8]) { panic!("Invalid use of NoHashHasher") }

    fn write_u8(&mut self, n: u8)       { self.0 = u64::from(n) }
    fn write_u16(&mut self, n: u16)     { self.0 = u64::from(n) }
    fn write_u32(&mut self, n: u32)     { self.0 = u64::from(n) }
    fn write_u64(&mut self, n: u64)     { self.0 = n }
    fn write_usize(&mut self, n: usize) { self.0 = n as u64 }

    fn write_i8(&mut self, n: i8)       { self.0 = n as u64 }
    fn write_i16(&mut self, n: i16)     { self.0 = n as u64 }
    fn write_i32(&mut self, n: i32)     { self.0 = n as u64 }
    fn write_i64(&mut self, n: i64)     { self.0 = n as u64 }
    fn write_isize(&mut self, n: isize) { self.0 = n as u64 }

    fn finish(&self) -> u64 { self.0 }
}
