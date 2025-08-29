use core::{
    hash::{BuildHasher, Hash, Hasher},
    mem,
};

use crate::hasher::BuildDefaultHasher;

#[derive(Debug)]
enum HashMapEntry<K, V>
where
    K: Hash + Eq,
{
    Occupied(K, V),
    Empty,
    Deleted,
}

impl<K, V> PartialEq for HashMapEntry<K, V>
where
    K: Hash + Eq,
    V: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Occupied(l0, l1), Self::Occupied(r0, r1)) => l0 == r0 && l1 == r1,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}
impl<K, V> Eq for HashMapEntry<K, V>
where
    K: Hash + Eq,
    V: Eq,
{
}

impl<K, V> HashMapEntry<K, V>
where
    K: Hash + Eq,
{
    pub fn take(&mut self) -> Self {
        mem::replace(self, HashMapEntry::Deleted)
    }

    pub fn as_ref(&self) -> HashMapEntry<&K, &V> {
        match &self {
            HashMapEntry::Occupied(k, v) => HashMapEntry::Occupied(k, v),
            HashMapEntry::Empty => HashMapEntry::Empty,
            HashMapEntry::Deleted => HashMapEntry::Deleted,
        }
    }

    pub fn as_mut_val(&mut self) -> Option<&mut V> {
        match self {
            HashMapEntry::Occupied(_, v) => Some(v),
            HashMapEntry::Empty => None,
            HashMapEntry::Deleted => None,
        }
    }
}

impl<K, V> From<HashMapEntry<K, V>> for Option<V>
where
    K: Hash + Eq,
{
    fn from(value: HashMapEntry<K, V>) -> Self {
        match value {
            HashMapEntry::Occupied(_, v) => Some(v),
            _ => None,
        }
    }
}

impl<K, V> From<HashMapEntry<K, V>> for Option<(K, V)>
where
    K: Hash + Eq,
{
    fn from(value: HashMapEntry<K, V>) -> Self {
        match value {
            HashMapEntry::Occupied(k, v) => Some((k, v)),
            _ => None,
        }
    }
}

pub struct HashMap<K, V, const N: usize, H = BuildDefaultHasher>
where
    K: Hash + Eq,
    H: BuildHasher,
{
    build_hasher: H,
    entries: [HashMapEntry<K, V>; N],
    len: usize,
}

impl<K, V, const N: usize> HashMap<K, V, N>
where
    K: Hash + Eq,
{
    pub fn new() -> Self {
        Self {
            entries: [const { HashMapEntry::Empty }; N],
            len: 0,
            build_hasher: BuildDefaultHasher {},
        }
    }
}

impl<K, V, const N: usize, H> HashMap<K, V, N, H>
where
    K: Hash + Eq,
    H: BuildHasher,
{
    pub fn new_with_hasher(hasher: H) -> Self {
        Self {
            entries: [const { HashMapEntry::Empty }; N],
            len: 0,
            build_hasher: hasher,
        }
    }

    pub fn insert(&mut self, key: K, val: V) -> bool {
        if let Some(spot) = self.probe_for_available_spot(&key) {
            self.entries[spot] = HashMapEntry::Occupied(key, val);
            self.len += 1;
            true
        } else {
            false
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let spot = self.probe_for_existing_spot(key)?;

        self.len -= 1;
        self.entries[spot].take().into()
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.probe_for_existing_spot(key).is_some()
    }

    pub fn get(&self, key: &'_ K) -> Option<&V> {
        let spot = self.probe_for_existing_spot(key)?;

        self.entries[spot].as_ref().into()
    }

    pub fn get_mut(&mut self, key: &'_ K) -> Option<&mut V> {
        let spot = self.probe_for_existing_spot(key)?;

        self.entries[spot].as_mut_val()
    }

    fn hash_key(&self, key: &K) -> u64 {
        let mut hasher = self.build_hasher.build_hasher();
        key.hash(&mut hasher);

        hasher.finish()
    }

    fn probe_for_available_spot(&self, key: &K) -> Option<usize> {
        if self.len >= N {
            return None;
        }

        let hash = self.hash_key(key);
        let mut spot = hash as usize % N;
        let original_spot = spot;

        loop {
            match &self.entries[spot] {
                HashMapEntry::Empty | HashMapEntry::Deleted => {
                    return Some(spot);
                }
                HashMapEntry::Occupied(k, _) => {
                    if k == key {
                        return None;
                    }
                    spot = (spot + 1) % N;
                }
            }

            if spot == original_spot {
                panic!("Unable to find free spot in HashMap with len < N")
            }
        }
    }

    fn probe_for_existing_spot(&self, key: &K) -> Option<usize> {
        if self.len == 0 {
            return None;
        }

        let hash = self.hash_key(key);
        let mut spot = hash as usize % N;
        let original_spot = spot;

        loop {
            match &self.entries[spot] {
                HashMapEntry::Empty => {
                    return None;
                }
                HashMapEntry::Deleted => {
                    spot = (spot + 1) % N;
                }
                HashMapEntry::Occupied(k, _) => {
                    if k == key {
                        return Some(spot);
                    } else {
                        spot = (spot + 1) % N
                    }
                }
            }

            if spot == original_spot {
                return None;
            }
        }
    }
}

#[macro_export]
macro_rules! map {
    [$(($key:expr, $value:expr)),*] => {{
        #[allow(unused_mut)]
        let mut map = $crate::HashMap::new();
        $(map.insert($key, $value);)*
        map
    }};
}

#[cfg(test)]
mod tests {

    use super::*;

    // A type that always returns a hash of zero, to allow both testing hash collision logic and to
    // directly test the contents of the backing structure in a reproducible way
    struct IntCollHasher {}
    impl Hasher for IntCollHasher {
        fn finish(&self) -> u64 {
            0
        }

        fn write(&mut self, _bytes: &[u8]) {}
    }
    struct IntCollBuildHasher {}
    impl BuildHasher for IntCollBuildHasher {
        type Hasher = IntCollHasher;

        fn build_hasher(&self) -> Self::Hasher {
            IntCollHasher {}
        }
    }

    #[test]
    fn test_insert_contains() {
        let mut map: HashMap<u32, f64, 50> = HashMap::new();

        map.insert(1, 1.0);
        map.insert(2, 2.0);
        map.insert(3, 3.0);
        map.insert(4, 4.0);

        assert_eq!(map.get(&1), Some(&1.0));
        assert_eq!(map.get(&2), Some(&2.0));
        assert_eq!(map.get(&3), Some(&3.0));
        assert_eq!(map.get(&4), Some(&4.0));

        assert!(map.contains_key(&1));
        assert!(map.contains_key(&2));
        assert!(map.contains_key(&3));
        assert!(map.contains_key(&4));

        assert!(!map.contains_key(&5));

        assert_eq!(map.len, 4);
    }

    #[test]
    fn test_map_macro() {
        let map: HashMap<_, _, 50> = map!((1, 1.0), (2, 2.0), (3, 3.0), (4, 4.0));

        assert_eq!(map.get(&1), Some(&1.0));
        assert_eq!(map.get(&2), Some(&2.0));
        assert_eq!(map.get(&3), Some(&3.0));
        assert_eq!(map.get(&4), Some(&4.0));

        assert!(map.contains_key(&1));
        assert!(map.contains_key(&2));
        assert!(map.contains_key(&3));
        assert!(map.contains_key(&4));

        assert!(!map.contains_key(&5));

        assert_eq!(map.len, 4);
    }

    #[test]
    fn test_remove() {
        let mut map: HashMap<_, _, 50> = map!((1, 1.0), (2, 2.0), (3, 3.0), (4, 4.0));

        assert_eq!(map.remove(&1), Some(1.0));
        assert_eq!(map.len, 3);
        assert!(!map.contains_key(&1));
        assert_eq!(map.get(&1), None);
        assert_eq!(map.remove(&1), None);

        assert_eq!(map.remove(&2), Some(2.0));
        assert_eq!(map.len, 2);
        assert!(!map.contains_key(&2));
        assert_eq!(map.get(&2), None);
        assert_eq!(map.remove(&2), None);

        assert_eq!(map.remove(&3), Some(3.0));
        assert_eq!(map.len, 1);
        assert!(!map.contains_key(&3));
        assert_eq!(map.get(&3), None);
        assert_eq!(map.remove(&3), None);

        assert_eq!(map.remove(&4), Some(4.0));
        assert_eq!(map.len, 0);
        assert!(!map.contains_key(&4));
        assert_eq!(map.get(&4), None);
        assert_eq!(map.remove(&4), None);
    }

    #[test]
    fn test_collisions() {
        let bh = IntCollBuildHasher {};
        let mut map: HashMap<_, _, 50, _> = HashMap::new_with_hasher(bh);

        map.insert(1, 1.0);
        map.insert(2, 2.0);
        map.insert(3, 3.0);
        map.insert(4, 4.0);

        assert_eq!(map.entries[0], HashMapEntry::Occupied(1, 1.0));
        assert_eq!(map.entries[1], HashMapEntry::Occupied(2, 2.0));
        assert_eq!(map.entries[2], HashMapEntry::Occupied(3, 3.0));
        assert_eq!(map.entries[3], HashMapEntry::Occupied(4, 4.0));

        assert!(map.contains_key(&1));
        assert!(map.contains_key(&2));
        assert!(map.contains_key(&3));
        assert!(map.contains_key(&4));

        assert_eq!(map.len, 4);

        assert_eq!(map.get(&1), Some(&1.0));
        assert_eq!(map.get(&2), Some(&2.0));
        assert_eq!(map.get(&3), Some(&3.0));
        assert_eq!(map.get(&4), Some(&4.0));

        assert_eq!(map.remove(&2), Some(2.0));

        assert_eq!(map.entries[0], HashMapEntry::Occupied(1, 1.0));
        assert_eq!(map.entries[1], HashMapEntry::Deleted);
        assert_eq!(map.entries[2], HashMapEntry::Occupied(3, 3.0));
        assert_eq!(map.entries[3], HashMapEntry::Occupied(4, 4.0));

        assert!(map.contains_key(&1));
        assert!(!map.contains_key(&2));
        assert!(map.contains_key(&3));
        assert!(map.contains_key(&4));

        assert_eq!(map.len, 3);

        assert_eq!(map.get(&1), Some(&1.0));
        assert_eq!(map.get(&2), None);
        assert_eq!(map.get(&3), Some(&3.0));
        assert_eq!(map.get(&4), Some(&4.0));

        assert_eq!(map.remove(&1), Some(1.0));

        assert_eq!(map.entries[0], HashMapEntry::Deleted);
        assert_eq!(map.entries[1], HashMapEntry::Deleted);
        assert_eq!(map.entries[2], HashMapEntry::Occupied(3, 3.0));
        assert_eq!(map.entries[3], HashMapEntry::Occupied(4, 4.0));

        assert!(!map.contains_key(&1));
        assert!(!map.contains_key(&2));
        assert!(map.contains_key(&3));
        assert!(map.contains_key(&4));

        assert_eq!(map.len, 2);

        assert_eq!(map.get(&1), None);
        assert_eq!(map.get(&2), None);
        assert_eq!(map.get(&3), Some(&3.0));
        assert_eq!(map.get(&4), Some(&4.0));

        assert!(map.insert(5, 5.0));

        assert_eq!(map.entries[0], HashMapEntry::Occupied(5, 5.0));
        assert_eq!(map.entries[1], HashMapEntry::Deleted);
        assert_eq!(map.entries[2], HashMapEntry::Occupied(3, 3.0));
        assert_eq!(map.entries[3], HashMapEntry::Occupied(4, 4.0));

        assert!(!map.contains_key(&1));
        assert!(!map.contains_key(&2));
        assert!(map.contains_key(&3));
        assert!(map.contains_key(&4));
        assert!(map.contains_key(&5));

        assert_eq!(map.len, 3);

        assert_eq!(map.get(&1), None);
        assert_eq!(map.get(&2), None);
        assert_eq!(map.get(&3), Some(&3.0));
        assert_eq!(map.get(&4), Some(&4.0));
        assert_eq!(map.get(&5), Some(&5.0));
    }
}
