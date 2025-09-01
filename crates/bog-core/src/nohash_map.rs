//! No Hash Map



use core::hash::{BuildHasherDefault, Hash};
use hashbrown::{Equivalent, HashMap};



/// A [`HashMap`] that doesn't hash.
#[derive(Clone, Debug)]
pub struct NoHashMap<K, V> {
    map: HashMap<K, V, BuildHasherDefault<NoHashHasher>>,
}

impl<K, V> Default for NoHashMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> NoHashMap<K, V> {
    /// Create an empty map.
    pub const fn new() -> Self {
        Self {
            map: HashMap::with_hasher(BuildHasherDefault::new()),
        }
    }
}

impl<K: Eq + Hash, V> NoHashMap<K, V> {
    /// Create a map with space pre-allocated for `capacity` key-value pairs.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            map: HashMap::with_capacity_and_hasher(capacity, BuildHasherDefault::new()),
        }
    }

    /// Get the value associated with the given key.
    #[inline]
    pub fn get<Q>(&self, k: &Q) -> Option<&V>
    where
        Q: Hash + Equivalent<K> + ?Sized,
    {
        self.map.get(k)
    }

    /// Get a mutable reference to the value associated with the given key.
    #[inline]
    pub fn get_mut<Q>(&mut self, k: &Q) -> Option<&mut V>
    where
        Q: Hash + Equivalent<K> + ?Sized,
    {
        self.map.get_mut(k)
    }

    /// Insert the given key-value pair into the map.
    #[inline]
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.map.insert(k, v)
    }

    /// Remove the given key from the map.
    #[inline]
    pub fn remove<Q>(&mut self, k: &Q) -> Option<V>
    where
        Q: Hash + Equivalent<K> + ?Sized,
    {
        self.map.remove(k)
    }
}



// ---



#[derive(Clone, Copy, Debug, Default)]
struct NoHashHasher(u64);

impl core::hash::Hasher for NoHashHasher {
    fn write(&mut self, _: &[u8]) {
        panic!("invalid use of NoHashHasher, please ensure your NoHashMap's key type can be represented as a u64")
    }

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



#[cfg(test)]
mod tests {
    use core::hash::Hash as _;

    use super::*;

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    struct MyKeyType(u32);

    #[test]
    fn no_hash_map_works() {
        let mut map = NoHashMap::<MyKeyType, String>::default();
        let key = MyKeyType(45);

        map.insert(key, "Something".to_string());

        assert!(map.get(&key).is_some_and(|s| s == "Something"));

        map.insert(key, "Other".to_string());

        assert!(map.get(&key).is_some_and(|s| s == "Other"));
    }

    #[test]
    fn no_hash_hasher_wont_hash() {
        let mut hasher = NoHashHasher::default();

        assert!(hasher.0 == 0);

        MyKeyType(45).hash(&mut hasher);

        assert!(hasher.0 == 45);
    }
}
