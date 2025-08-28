use core::{
    hash::{BuildHasher, Hash, Hasher},
    mem,
};

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

pub struct HashSet<T, H, const N: usize>
where
    T: Hash + Eq,
    H: BuildHasher,
{
    arr: [HashSetEntry<T>; N],
    len: usize,
    hasher: H,
}

impl<T, H, const N: usize> HashSet<T, H, N>
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
    [$h:expr => $($elem:expr),*] => {{
        #[allow(unused_mut)]
        let mut set = $crate::HashSet::new_with_hasher($h);
        $(set.insert($elem);)*
        set
    }};
}

#[cfg(test)]
mod tests {
    use core::hash::{BuildHasher, Hasher};

    use crate::{HashSet, hash_set::HashSetEntry};

    struct IntHasher {
        val: u64,
    }

    impl Hasher for IntHasher {
        fn finish(&self) -> u64 {
            self.val
        }

        fn write(&mut self, bytes: &[u8]) {
            for byte in bytes {
                self.val += *byte as u64;
            }
        }
    }

    struct IntBuildHasher {}

    impl BuildHasher for IntBuildHasher {
        type Hasher = IntHasher;

        fn build_hasher(&self) -> Self::Hasher {
            IntHasher { val: 0 }
        }
    }

    #[test]
    fn test_insert() {
        let bh = IntBuildHasher {};
        let mut set = HashSet::<u32, _, 50>::new_with_hasher(bh);

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
        let bh = IntBuildHasher {};
        let set: HashSet<u32, _, 20> = set!(bh => 1, 2, 3);

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
        let bh = IntBuildHasher {};
        let mut set: HashSet<u32, _, 20> = set!(bh => 1, 2, 3);

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
}
