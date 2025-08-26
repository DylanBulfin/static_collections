pub struct List<T, const N: usize> {
    arr: [Option<T>; N],
    len: usize,
}

impl<T, const N: usize> List<T, N> {
    pub const fn new() -> Self {
        Self {
            arr: [const { None }; N],
            len: 0,
        }
    }

    pub fn add(&mut self, elem: T) -> bool {
        if self.len < N {
            self.arr[self.len] = Some(elem);
            true
        } else {
            false
        }
    }
}

// impl<T, const N: usize> List<T, N>
// where
//     T: Clone,
// {
//     pub fn new_clone(t: T) -> Self {
//         Self {
//             arr: core::array::from_fn(|_| t.clone()),
//             len: 0,
//         }
//     }
// }
// impl<T, const N: usize> List<T, N>
// where
//     T: Default,
// {
//     pub fn new_default() -> Self {
//         Self {
//             arr: core::array::from_fn(|_| T::default()),
//             len: 0,
//         }
//     }
// }
// impl<T, const N: usize> List<T, N>
// where
//     T: Copy,
// {
//     pub fn new_copy(t: T) -> Self {
//         Self {
//             arr: [t; N],
//             len: 0,
//         }
//     }
// }
// impl<T, const N: usize> List<T, N>
// where
//     T: Copy + Default,
// {
//     pub fn new_copy_default() -> Self {
//         Self {
//             arr: [T::default(); N],
//             len: 0,
//         }
//     }
// }
