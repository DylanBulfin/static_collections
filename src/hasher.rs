use core::hash::{BuildHasher, Hasher};

pub struct DefaultHasher {
    val: u64,
}
impl Hasher for DefaultHasher {
    fn finish(&self) -> u64 {
        self.val
    }

    fn write(&mut self, bytes: &[u8]) {
        let sum = bytes.iter().map(|b| *b as u64).sum();
        self.val = self.val.wrapping_add(sum);
    }
}

pub struct BuildDefaultHasher<const SEED: u64 = 0> {}
impl<const SEED: u64> BuildHasher for BuildDefaultHasher<SEED> {
    type Hasher = DefaultHasher;

    fn build_hasher(&self) -> Self::Hasher {
        Self::Hasher { val: SEED }
    }
}
