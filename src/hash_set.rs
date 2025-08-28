use core::{
    hash::{BuildHasher, Hash, Hasher},
    mem,
};

use crate::hasher::BuildDefaultHasher;

#[derive(Debug, PartialEq, Eq)]
pub enum HashSetEntry<T>
where
    T: Hash + Eq,
{
    Occupied(T),
    Empty,
    /// This HashSet type uses open addressing, there needs to be a marker for entries that have
    /// been deleted to support this.
    Deleted,
}

impl<T> HashSetEntry<T>
where
    T: Hash + Eq,
{
    pub fn take(&mut self) -> Self {
        mem::replace(self, HashSetEntry::Deleted)
    }

    pub fn as_ref(&self) -> HashSetEntry<&T> {
        match self {
            HashSetEntry::Occupied(elem) => HashSetEntry::Occupied(&elem),
            HashSetEntry::Empty => HashSetEntry::Empty,
            HashSetEntry::Deleted => HashSetEntry::Deleted,
        }
    }

    pub fn as_mut(&mut self) -> HashSetEntry<&mut T> {
        match self {
            HashSetEntry::Occupied(elem) => HashSetEntry::Occupied(elem),
            HashSetEntry::Empty => HashSetEntry::Empty,
            HashSetEntry::Deleted => HashSetEntry::Deleted,
        }
    }
}

impl<T> From<HashSetEntry<T>> for Option<T>
where
    T: Hash + Eq,
{
    fn from(value: HashSetEntry<T>) -> Self {
        match value {
            HashSetEntry::Occupied(elem) => Some(elem),
            _ => None,
        }
    }
}

pub struct HashSet<T, const N: usize, H = BuildDefaultHasher>
where
    T: Hash + Eq,
    H: BuildHasher,
{
    arr: [HashSetEntry<T>; N],
    len: usize,
    hasher: H,
}

impl<T, const N: usize> HashSet<T, N>
where
    T: Hash + Eq,
{
    pub const fn new() -> Self {
        Self {
            arr: [const { HashSetEntry::Empty }; N],
            len: 0,
            hasher: BuildDefaultHasher {},
        }
    }
}

impl<T, const N: usize, H> HashSet<T, N, H>
where
    T: Hash + Eq,
    H: BuildHasher,
{
    pub fn new_with_hasher(hasher: H) -> Self {
        Self {
            arr: [const { HashSetEntry::Empty }; N],
            len: 0,
            hasher: hasher,
        }
    }

    pub fn insert(&mut self, elem: T) -> bool {
        if let Some(spot) = self.probe_for_available_spot(&elem) {
            self.arr[spot] = HashSetEntry::Occupied(elem);
            self.len += 1;
            true
        } else {
            false
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn remove(&mut self, elem: &T) -> Option<T> {
        let spot = self.probe_for_existing_spot(elem)?;

        self.len -= 1;
        self.arr[spot].take().into()
    }

    pub fn contains(&self, elem: &T) -> bool {
        self.probe_for_existing_spot(elem).is_some()
    }

    pub fn get(&self, elem: &'_ T) -> Option<&T> {
        let spot = self.probe_for_existing_spot(elem)?;

        self.arr[spot].as_ref().into()
    }

    fn hash_element(&self, elem: &T) -> u64 {
        let mut hasher = self.hasher.build_hasher();
        elem.hash(&mut hasher);

        hasher.finish()
    }

    fn probe_for_available_spot(&self, elem: &T) -> Option<usize> {
        if self.len >= N {
            return None;
        }

        let hash = self.hash_element(elem);
        let mut spot = hash as usize % N;
        let original_spot = spot;

        loop {
            match &self.arr[spot] {
                HashSetEntry::Empty | HashSetEntry::Deleted => {
                    return Some(spot);
                }
                HashSetEntry::Occupied(el) => {
                    if el == elem {
                        return None;
                    }
                    spot = (spot + 1) % N;
                }
            }

            if spot == original_spot {
                panic!("Unable to find free spot in HashSet with len < N")
            }
        }
    }

    fn probe_for_existing_spot(&self, elem: &T) -> Option<usize> {
        if self.len == 0 {
            return None;
        }

        let hash = self.hash_element(elem);
        let mut spot = hash as usize % N;
        let original_spot = spot;

        loop {
            match &self.arr[spot] {
                HashSetEntry::Empty => {
                    return None;
                }
                HashSetEntry::Deleted => {
                    spot = (spot + 1) % N;
                }
                HashSetEntry::Occupied(el) => {
                    if el == elem {
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
macro_rules! set {
    [$($elem:expr),*] => {{
        #[allow(unused_mut)]
        let mut set = $crate::HashSet::new();
        $(set.insert($elem);)*
        set
    }};
}

#[cfg(test)]
mod tests {
    use core::hash::{BuildHasher, Hasher};

    use crate::{HashSet, hash_set::HashSetEntry};

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
    fn test_insert() {
        let mut set = HashSet::<u32, 50>::new();

        assert_eq!(set.len, 0);

        assert!(set.insert(10));
        assert!(set.contains(&10));
        assert_eq!(set.get(&10), Some(&10));
        assert_eq!(set.len, 1);

        assert!(set.insert(20));
        assert!(set.contains(&20));
        assert_eq!(set.get(&20), Some(&20));
        assert_eq!(set.len, 2);

        assert!(!set.insert(10));
        assert!(set.contains(&10));
        assert_eq!(set.get(&10), Some(&10));
        assert_eq!(set.len, 2);
    }

    #[test]
    fn test_set_macro() {
        let set: HashSet<u32, 20> = set!(1, 2, 3);

        assert_eq!(set.len, 3);
        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(set.contains(&3));

        assert_eq!(set.get(&1), Some(&1));
        assert_eq!(set.get(&2), Some(&2));
        assert_eq!(set.get(&3), Some(&3));
    }

    #[test]
    fn test_remove() {
        let mut set: HashSet<u32, 20> = set!(1, 2, 3);

        assert_eq!(set.len, 3);
        assert_eq!(set.remove(&4), None);
        assert_eq!(set.len, 3);

        assert_eq!(set.remove(&3), Some(3));
        assert!(!set.contains(&3));
        assert_eq!(set.get(&3), None);
        assert_eq!(set.len, 2);

        assert_eq!(set.remove(&2), Some(2));
        assert!(!set.contains(&2));
        assert_eq!(set.get(&2), None);
        assert_eq!(set.len, 1);

        assert_eq!(set.remove(&1), Some(1));
        assert!(!set.contains(&1));
        assert_eq!(set.get(&1), None);
        assert_eq!(set.len, 0);
    }

    #[test]
    fn test_collisions() {
        let bh = IntCollBuildHasher {};
        let mut map: HashSet<_, 50, _> = HashSet::new_with_hasher(bh);

        map.insert(1);
        map.insert(2);
        map.insert(3);
        map.insert(4);

        assert_eq!(map.arr[0], HashSetEntry::Occupied(1));
        assert_eq!(map.arr[1], HashSetEntry::Occupied(2));
        assert_eq!(map.arr[2], HashSetEntry::Occupied(3));
        assert_eq!(map.arr[3], HashSetEntry::Occupied(4));

        assert!(map.contains(&1));
        assert!(map.contains(&2));
        assert!(map.contains(&3));
        assert!(map.contains(&4));

        assert_eq!(map.len, 4);

        assert_eq!(map.get(&1), Some(&1));
        assert_eq!(map.get(&2), Some(&2));
        assert_eq!(map.get(&3), Some(&3));
        assert_eq!(map.get(&4), Some(&4));

        assert_eq!(map.remove(&2), Some(2));

        assert_eq!(map.arr[0], HashSetEntry::Occupied(1));
        assert_eq!(map.arr[1], HashSetEntry::Deleted);
        assert_eq!(map.arr[2], HashSetEntry::Occupied(3));
        assert_eq!(map.arr[3], HashSetEntry::Occupied(4));

        assert!(map.contains(&1));
        assert!(!map.contains(&2));
        assert!(map.contains(&3));
        assert!(map.contains(&4));

        assert_eq!(map.len, 3);

        assert_eq!(map.get(&1), Some(&1));
        assert_eq!(map.get(&2), None);
        assert_eq!(map.get(&3), Some(&3));
        assert_eq!(map.get(&4), Some(&4));

        assert_eq!(map.remove(&1), Some(1));

        assert_eq!(map.arr[0], HashSetEntry::Deleted);
        assert_eq!(map.arr[1], HashSetEntry::Deleted);
        assert_eq!(map.arr[2], HashSetEntry::Occupied(3));
        assert_eq!(map.arr[3], HashSetEntry::Occupied(4));

        assert!(!map.contains(&1));
        assert!(!map.contains(&2));
        assert!(map.contains(&3));
        assert!(map.contains(&4));

        assert_eq!(map.len, 2);

        assert_eq!(map.get(&1), None);
        assert_eq!(map.get(&2), None);
        assert_eq!(map.get(&3), Some(&3));
        assert_eq!(map.get(&4), Some(&4));

        assert!(map.insert(5));

        assert_eq!(map.arr[0], HashSetEntry::Occupied(5));
        assert_eq!(map.arr[1], HashSetEntry::Deleted);
        assert_eq!(map.arr[2], HashSetEntry::Occupied(3));
        assert_eq!(map.arr[3], HashSetEntry::Occupied(4));

        assert!(!map.contains(&1));
        assert!(!map.contains(&2));
        assert!(map.contains(&3));
        assert!(map.contains(&4));
        assert!(map.contains(&5));

        assert_eq!(map.len, 3);

        assert_eq!(map.get(&1), None);
        assert_eq!(map.get(&2), None);
        assert_eq!(map.get(&3), Some(&3));
        assert_eq!(map.get(&4), Some(&4));
        assert_eq!(map.get(&5), Some(&5));
    }
}
