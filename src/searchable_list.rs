use core::{cmp::Ordering, ops::Index};

/// This module provides a list type that can be searched and indexed efficiently (O(1)). It
/// potentially involves restructuring the backing array when a

pub struct SearchableList<T, const N: usize>
where
    T: Ord,
{
    // holds a sorted list of elements. For index j < self.len, backing[j] = Some((i, elem)). i is
    // the index such that indices[i] = Some(j)
    backing: [Option<(usize, T)>; N],
    // for index i, indices[i] is the value j such that backing[j] = SearchableList[i]
    indices: [Option<usize>; N],
    len: usize,
}

impl<T, const N: usize> SearchableList<T, N>
where
    T: Ord,
{
    pub const N: usize = N;

    pub const fn new() -> Self {
        Self {
            backing: [const { None }; N],
            indices: [const { None }; N],
            len: 0,
        }
    }

    pub const fn max_len(&self) -> usize {
        Self::N
    }

    /// Get the length of the list (the number of actual elements, not the size of the backing
    /// array. The size of the backing array is accessible by SearchableList::N)
    pub fn len(&self) -> usize {
        self.len
    }

    /// Push an element to the **back** of the list
    pub fn push(&mut self, elem: T) {
        if self.len >= N {
            panic!("Tried to add element to full list")
        }

        let spot = self.search_for_new_spot(&elem, 0, self.len);
        if spot != self.len {
            for j in (spot..self.len).rev() {
                let el = self.backing[j].take().unwrap_or_else(|| {
                    panic!(
                        "Unexpected None at index {} in backing array with len {}",
                        j, self.len
                    )
                });
                if let Some(j2) = self.indices[el.0].as_mut() {
                    if *j2 != j {
                        panic!("Mismatched j's, j = {}, j2 = {}", j, j2)
                    } else {
                        *j2 = j + 1
                    }
                }

                self.backing[j + 1] = Some(el)
            }
        }

        self.backing[spot] = Some((self.len, elem));
        self.indices[self.len] = Some(spot);
        self.len += 1;
    }

    /// Pop an element from the **back** of the list
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else if let Some(j) = self.indices[self.len - 1] {
            if let Some((i, elem)) = self.backing[j].take() {
                if i != self.len - 1 {
                    panic!(
                        "Mismatched indices, backing array contains {} but expected {}",
                        i,
                        self.len - 1
                    );
                } else {
                    for j2 in j..self.len - 1 {
                        if let Some((i2, elem)) = self.backing[j2 + 1].take()
                            && let Some(index_entry) = self.indices[i2].as_mut()
                        {
                            *index_entry = *index_entry - 1;
                            self.backing[j2] = Some((i2, elem));
                        }
                    }

                    self.len -= 1;
                    self.indices[self.len] = None;
                    self.backing[self.len] = None;

                    Some(elem)
                }
            } else {
                panic!(
                    "Unexpected None at index {} of SList backing arr, with len = {}",
                    j, self.len
                );
            }
        } else {
            panic!(
                "Unexpected None at index {} of SList indices arr, with len = {}",
                self.len - 1,
                self.len
            );
        }
    }

    pub fn find(&self, elem: &T) -> Option<usize> {
        self.search_for_existing_spot_by(|el| el.cmp(elem), 0, self.len)
    }

    fn search_for_existing_spot_by<F>(&self, f: F, start_j: usize, end_j: usize) -> Option<usize>
    where
        F: Fn(&T) -> Ordering,
    {
        let diff = end_j - start_j;

        if diff == 0 {
            None
        } else if diff == 1 {
            if let Some((start_i, se)) = &self.backing[start_j]
                && let Some((end_i, ee)) = &self.backing[end_j]
            {
                if f(se).is_eq() {
                    Some(*start_i)
                } else if f(ee).is_eq() {
                    Some(*end_i)
                } else {
                    None
                }
            } else {
                panic!(
                    "Unexpected none at in index {} or {} in backing array",
                    start_j, end_j
                )
            }
        } else {
            let midpoint = start_j + (diff / 2);

            if let Some((midpoint_i, elem)) = &self.backing[midpoint] {
                match f(elem) {
                    Ordering::Equal => Some(*midpoint_i),
                    Ordering::Less => self.search_for_existing_spot_by(f, midpoint, end_j),
                    Ordering::Greater => self.search_for_existing_spot_by(f, start_j, midpoint),
                }
            } else {
                panic!(
                    "Unexpected None at index {} of backing array for searchable list with len {}",
                    midpoint, self.len
                );
            }
        }
    }

    fn search_for_new_spot(&self, elem: &T, start_j: usize, end_j: usize) -> usize {
        let diff = end_j - start_j;

        if diff == 0 {
            // Should only happen if self.len == 0
            if self.len != 0 {
                panic!("Unexpected diff of 0 when searching for element spot")
            } else {
                0
            }
        } else if diff == 1 {
            if let Some(el) = &self.backing[start_j] {
                match el.1.cmp(elem) {
                    Ordering::Equal | Ordering::Less => end_j,
                    Ordering::Greater => {
                        if start_j == 0 {
                            // If start_j is 0, e.g this is a new smallest element,
                            start_j
                        } else {
                            panic!(
                                "Reached invalid state during binary search, expected to find spot after index {} but didn't",
                                start_j
                            )
                        }
                    }
                }
            } else {
                panic!(
                    "Unexpected None at index {} of backing array for searchable list with len {}",
                    start_j, self.len
                );
            }
        } else {
            let midpoint = start_j + (diff / 2);

            if let Some(el) = &self.backing[midpoint] {
                match el.1.cmp(elem) {
                    Ordering::Less | Ordering::Equal => {
                        // element at midpoint <= target, search second half
                        self.search_for_new_spot(elem, midpoint, end_j)
                    }
                    Ordering::Greater => {
                        // element at midpoint > target, search first half
                        self.search_for_new_spot(elem, start_j, midpoint)
                    }
                }
            } else {
                panic!(
                    "Unexpected None at index {} of backing array for searchable list with len {}",
                    midpoint, self.len
                );
            }
        }
    }

    #[cfg(test)]
    fn verify_invariates(&self) {
        if self.len > N {
            panic!("SList len {} is greater than max {}", self.len, N);
        }

        // Verify length of backing array, and that indices match between arrays
        for index in 0..self.len {
            let be = &self.backing[index];
            let ie = &self.indices[index];

            match (be, ie) {
                (Some((i, _)), Some(_)) => {
                    let mj2 = self.indices[*i];

                    if let Some(j2) = mj2 {
                        if j2 != index {
                            panic!(
                                "Mismatched indices: backing array at index {} contains {} but indices[{}] contains {}",
                                index, i, i, j2
                            )
                        }
                    }
                }
                (None, Some(_)) => panic!(
                    "Unexpected None in backing array at index {} for len {}",
                    index, self.len
                ),
                (Some(_), None) => panic!(
                    "Unexpected None in indices array at index {} for len {}",
                    index, self.len
                ),
                (None, None) => panic!(
                    "Unexpected None in backing and indices arrays at index {} for len {}",
                    index, self.len
                ),
            }
        }

        if self.len > 0 {
            // Verify that the backing array is sorted in order
            for j in 0..self.len - 1 {
                let el1 = &self.backing[j].as_ref().unwrap().1;
                let el2 = &self.backing[j + 1].as_ref().unwrap().1;

                assert!(matches!(el1.cmp(el2), Ordering::Less | Ordering::Equal));
            }
        }
    }
}

impl<T, const N: usize> Index<usize> for SearchableList<T, N>
where
    T: Ord,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len {
            panic!(
                "Attempted to access index {} of SList with len {}",
                index, self.len
            )
        } else if let Some(j) = self.indices[index]
            && let Some((_, elem)) = &self.backing[j]
        {
            elem
        } else {
            panic!("Issue in index method")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_back() {
        let mut slist = SearchableList::<u32, 10>::new();
        let mut exp_backing = [None; 10];
        let mut exp_indices = [None; 10];
        let mut exp_len = 0;

        assert_eq!(slist.len(), exp_len);
        assert_eq!(&slist.backing, &exp_backing);
        assert_eq!(&slist.indices, &exp_indices);
        slist.verify_invariates();

        slist.push(1);
        exp_backing[0] = Some((0, 1));
        exp_indices[0] = Some(0);
        exp_len = 1;

        assert_eq!(slist.len(), exp_len);
        assert_eq!(&slist.backing, &exp_backing);
        assert_eq!(&slist.indices, &exp_indices);
        slist.verify_invariates();

        slist.push(3);
        exp_backing[1] = Some((1, 3));
        exp_indices[1] = Some(1);
        exp_len = 2;

        assert_eq!(slist.len(), exp_len);
        assert_eq!(&slist.backing, &exp_backing);
        assert_eq!(&slist.indices, &exp_indices);
        slist.verify_invariates();

        slist.push(2);
        exp_backing[1] = Some((2, 2));
        exp_backing[2] = Some((1, 3));
        exp_indices[1] = Some(2);
        exp_indices[2] = Some(1);
        exp_len = 3;

        assert_eq!(slist.len(), exp_len);
        assert_eq!(&slist.backing, &exp_backing);
        assert_eq!(&slist.indices, &exp_indices);
        slist.verify_invariates();

        slist.push(0);
        exp_backing[0] = Some((3, 0));
        exp_backing[1] = Some((0, 1));
        exp_backing[2] = Some((2, 2));
        exp_backing[3] = Some((1, 3));
        exp_indices[0] = Some(1);
        exp_indices[1] = Some(3);
        exp_indices[2] = Some(2);
        exp_indices[3] = Some(0);
        exp_len = 4;

        assert_eq!(slist.len(), exp_len);
        assert_eq!(&slist.backing, &exp_backing);
        assert_eq!(&slist.indices, &exp_indices);
        slist.verify_invariates();
    }

    #[test]
    fn test_pop() {
        let mut slist = SearchableList::<u32, 10>::new();
        let mut exp_backing = [None; 10];
        let mut exp_indices = [None; 10];

        exp_backing[0] = Some((3, 0));
        exp_backing[1] = Some((0, 1));
        exp_backing[2] = Some((2, 2));
        exp_backing[3] = Some((1, 3));
        exp_indices[0] = Some(1);
        exp_indices[1] = Some(3);
        exp_indices[2] = Some(2);
        exp_indices[3] = Some(0);
        let mut exp_len = 4;

        slist.push(1);
        slist.push(3);
        slist.push(2);
        slist.push(0);

        assert_eq!(slist.len(), exp_len);
        assert_eq!(&slist.backing, &exp_backing);
        assert_eq!(&slist.indices, &exp_indices);
        slist.verify_invariates();

        assert_eq!(slist.pop(), Some(0));

        exp_indices[0] = Some(0);
        exp_indices[1] = Some(2);
        exp_indices[2] = Some(1);
        exp_indices[3] = None;
        exp_backing[0] = Some((0, 1));
        exp_backing[1] = Some((2, 2));
        exp_backing[2] = Some((1, 3));
        exp_backing[3] = None;
        exp_len = 3;

        assert_eq!(slist.len(), exp_len);
        assert_eq!(&slist.backing, &exp_backing);
        assert_eq!(&slist.indices, &exp_indices);
        slist.verify_invariates();

        assert_eq!(slist.pop(), Some(2));

        exp_indices[0] = Some(0);
        exp_indices[1] = Some(1);
        exp_indices[2] = None;
        exp_backing[0] = Some((0, 1));
        exp_backing[1] = Some((1, 3));
        exp_backing[2] = None;
        exp_len = 2;

        assert_eq!(slist.len(), exp_len);
        assert_eq!(&slist.backing, &exp_backing);
        assert_eq!(&slist.indices, &exp_indices);
        slist.verify_invariates();

        assert_eq!(slist.pop(), Some(3));

        exp_indices[0] = Some(0);
        exp_indices[1] = None;
        exp_backing[0] = Some((0, 1));
        exp_backing[1] = None;
        exp_len = 1;

        assert_eq!(slist.len(), exp_len);
        assert_eq!(&slist.backing, &exp_backing);
        assert_eq!(&slist.indices, &exp_indices);
        slist.verify_invariates();

        assert_eq!(slist.pop(), Some(1));

        exp_indices[0] = None;
        exp_backing[0] = None;
        exp_len = 0;

        assert_eq!(slist.len(), exp_len);
        assert_eq!(&slist.backing, &exp_backing);
        assert_eq!(&slist.indices, &exp_indices);
        slist.verify_invariates();

        assert_eq!(slist.pop(), None);
    }

    #[test]
    fn test_index() {
        let mut slist = SearchableList::<u32, 10>::new();
        slist.push(1);
        slist.push(3);
        slist.push(2);
        slist.push(0);

        assert_eq!(slist[0], 1);
        assert_eq!(slist[1], 3);
        assert_eq!(slist[2], 2);
        assert_eq!(slist[3], 0);
    }

    #[test]
    #[should_panic(expected = "Attempted to access index 4 of SList with len 4")]
    fn test_index_oob_panic() {
        let mut slist = SearchableList::<u32, 10>::new();
        slist.push(1);
        slist.push(3);
        slist.push(2);
        slist.push(0);

        assert_eq!(slist[0], 1);
        assert_eq!(slist[1], 3);
        assert_eq!(slist[2], 2);
        assert_eq!(slist[3], 0);

        slist[4];
    }

    #[test]
    fn test_find() {
        let mut slist = SearchableList::<u32, 10>::new();
        slist.push(1);
        slist.push(3);
        slist.push(2);
        slist.push(0);

        assert_eq!(slist.find(&0), Some(3));
        assert_eq!(slist.find(&1), Some(0));
        assert_eq!(slist.find(&2), Some(2));
        assert_eq!(slist.find(&3), Some(1));
    }
}
