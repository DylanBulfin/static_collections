use core::ops::{Index, IndexMut};

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

    pub fn len(&self) -> usize {
        self.len
    }

    /// Push a value to the back of the list
    pub fn push_back(&mut self, elem: T) {
        if self.len >= N {
            panic!("Attempt to add element to full list");
        }

        self.arr[self.len] = Some(elem);
        self.len += 1;
    }

    /// Pops a value from the back of the list
    pub fn pop_back(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            let val = self.arr[self.len].take().unwrap_or_else(|| {
                panic!("Unexpected None in backing array at index {}", self.len)
            });

            Some(val)
        }
    }

    /// Remove an element from a specific position in a list
    pub fn remove(&mut self, index: usize) -> T {
        if index >= self.len {
            panic!(
                "Attempt to remove element at invalid index: {} where len is {}",
                index, self.len
            );
        }

        let elem = self.arr[index]
            .take()
            .unwrap_or_else(|| panic!("Unexpected None in backing array at index {}", self.len));

        self.len -= 1;

        for i in index..self.len {
            self.arr[i] = self.arr[i + 1].take();
        }

        self.arr[self.len] = None;

        elem
    }

    pub fn remove_by<F>(&mut self, f: F) -> Option<T>
    where
        F: Fn(&T) -> bool,
    {
        let mut spot = None;

        for i in 0..self.len {
            if f(self.arr[i].as_ref().unwrap_or_else(|| {
                panic!("None at unexpected pos: {} when len is {}", i, self.len)
            })) {
                spot = Some(i);
            }
        }

        let index = spot?;

        let elem = self.arr[index]
            .take()
            .unwrap_or_else(|| panic!("Unexpected None in backing array at index {}", self.len));

        self.len -= 1;

        for i in index..self.len {
            self.arr[i] = self.arr[i + 1].take();
        }

        self.arr[self.len] = None;

        Some(elem)
    }

    pub fn iter(&self) -> ListIter<'_, T, N> {
        ListIter {
            base: self,
            index: 0,
        }
    }
}

impl<T, const N: usize> IndexMut<usize> for List<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.arr[index]
            .as_mut()
            .unwrap_or_else(|| panic!("Invalid index access: {}", index))
    }
}

impl<T, const N: usize> Index<usize> for List<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.arr[index]
            .as_ref()
            .unwrap_or_else(|| panic!("Invalid index access: {}", index))
    }
}

pub struct ListIter<'a, T, const N: usize> {
    base: &'a List<T, N>,
    index: usize,
}

impl<'a, T, const N: usize> Iterator for ListIter<'a, T, N> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.base.len {
            None
        } else {
            let elem = &self.base[self.index];
            self.index += 1;
            Some(elem)
        }
    }
}

#[macro_export]
macro_rules! list {
    [$($elem:expr),*] => {{
        #[allow(unused_mut)]
        let mut list = $crate::List::new();
        $(list.push_back($elem);)*
        list
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_back() {
        let mut list = List::<u32, 10>::new();
        let mut exp_backing = [None; 10];
        let mut exp_len = 0;

        assert_eq!(list.arr, exp_backing);
        assert_eq!(list.len, exp_len);

        list.push_back(1);
        exp_backing[0] = Some(1);
        exp_len = 1;

        assert_eq!(list.arr, exp_backing);
        assert_eq!(list.len, exp_len);

        list.push_back(2);
        exp_backing[1] = Some(2);
        exp_len += 1;

        assert_eq!(list.arr, exp_backing);
        assert_eq!(list.len, exp_len);

        list.push_back(3);
        exp_backing[2] = Some(3);
        exp_len += 1;

        assert_eq!(list.arr, exp_backing);
        assert_eq!(list.len, exp_len);

        list.push_back(4);
        exp_backing[3] = Some(4);
        exp_len += 1;

        assert_eq!(list.arr, exp_backing);
        assert_eq!(list.len, exp_len);

        list.push_back(5);
        exp_backing[4] = Some(5);
        exp_len += 1;

        assert_eq!(list.arr, exp_backing);
        assert_eq!(list.len, exp_len);

        list.push_back(6);
        exp_backing[5] = Some(6);
        exp_len += 1;

        assert_eq!(list.arr, exp_backing);
        assert_eq!(list.len, exp_len);
        list.push_back(7);
        exp_backing[6] = Some(7);
        exp_len += 1;

        assert_eq!(list.arr, exp_backing);
        assert_eq!(list.len, exp_len);
        list.push_back(8);
        exp_backing[7] = Some(8);
        exp_len += 1;

        assert_eq!(list.arr, exp_backing);
        assert_eq!(list.len, exp_len);
        list.push_back(9);
        exp_backing[8] = Some(9);
        exp_len += 1;

        assert_eq!(list.arr, exp_backing);
        assert_eq!(list.len, exp_len);
        list.push_back(0);
        exp_backing[9] = Some(0);
        exp_len += 1;

        assert_eq!(list.arr, exp_backing);
        assert_eq!(list.len, exp_len);
    }

    #[test]
    fn test_list_macro() {
        let list: List<u32, 10> = list![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
        assert_eq!(list.len, 10);
        assert_eq!(list.arr, [1, 2, 3, 4, 5, 6, 7, 8, 9, 0].map(Some))
    }

    #[test]
    #[should_panic(expected = "Attempt to add element to full list")]
    fn test_push_back_full_panic() {
        let mut list: List<u32, 10> = list![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
        list.push_back(10);
    }

    #[test]
    fn test_pop_back() {
        let mut list: List<u32, 10> = list![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
        let mut exp_arr = list.arr.clone();
        let mut exp_len = list.len();

        assert_eq!(list.arr, exp_arr);
        assert_eq!(list.len, exp_len);

        assert_eq!(list.pop_back(), Some(0));
        exp_arr[9] = None;
        exp_len -= 1;

        assert_eq!(list.arr, exp_arr);
        assert_eq!(list.len, exp_len);

        assert_eq!(list.pop_back(), Some(9));
        exp_arr[8] = None;
        exp_len -= 1;

        assert_eq!(list.arr, exp_arr);
        assert_eq!(list.len, exp_len);

        assert_eq!(list.pop_back(), Some(8));
        exp_arr[7] = None;
        exp_len -= 1;

        assert_eq!(list.arr, exp_arr);
        assert_eq!(list.len, exp_len);

        assert_eq!(list.remove(exp_len - 1), 7);
        exp_arr[6] = None;
        exp_len -= 1;

        assert_eq!(list.arr, exp_arr);
        assert_eq!(list.len, exp_len);

        assert_eq!(list.pop_back(), Some(6));
        exp_arr[5] = None;
        exp_len -= 1;

        assert_eq!(list.arr, exp_arr);
        assert_eq!(list.len, exp_len);

        assert_eq!(list.pop_back(), Some(5));
        exp_arr[4] = None;
        exp_len -= 1;

        assert_eq!(list.arr, exp_arr);
        assert_eq!(list.len, exp_len);

        assert_eq!(list.pop_back(), Some(4));
        exp_arr[3] = None;
        exp_len -= 1;

        assert_eq!(list.arr, exp_arr);
        assert_eq!(list.len, exp_len);

        assert_eq!(list.pop_back(), Some(3));
        exp_arr[2] = None;
        exp_len -= 1;

        assert_eq!(list.arr, exp_arr);
        assert_eq!(list.len, exp_len);

        assert_eq!(list.pop_back(), Some(2));
        exp_arr[1] = None;
        exp_len -= 1;

        assert_eq!(list.arr, exp_arr);
        assert_eq!(list.len, exp_len);

        assert_eq!(list.pop_back(), Some(1));
        exp_arr[0] = None;
        exp_len -= 1;

        assert_eq!(list.arr, exp_arr);
        assert_eq!(list.len, exp_len);

        assert_eq!(list.pop_back(), None);
        assert_eq!(list.arr, exp_arr);
        assert_eq!(list.len, exp_len);
    }

    #[test]
    fn test_remove() {
        let mut list: List<u32, 10> = list![1, 2, 3, 4, 5];
        let mut exp_arr = list.arr.clone();

        list.remove(2);
        exp_arr[2] = Some(4);
        exp_arr[3] = Some(5);
        exp_arr[4] = None;

        assert_eq!(list.len, 4);
        assert_eq!(list.arr, exp_arr);
    }

    #[test]
    fn test_remove_by() {
        let mut list: List<u32, 10> = list![1, 2, 3, 4, 5];
        let mut exp_arr = list.arr.clone();

        list.remove_by(|i| i * i == 9);
        exp_arr[2] = Some(4);
        exp_arr[3] = Some(5);
        exp_arr[4] = None;

        assert_eq!(list.len, 4);
        assert_eq!(list.arr, exp_arr);
    }

    #[test]
    fn test_index() {
        let list: List<u32, 10> = list![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        for i in 0..10 {
            assert_eq!(i, list[i] as usize);
        }
    }

    #[test]
    fn test_iter() {
        let list: List<u32, 10> = list![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        for (i, n) in list.iter().enumerate() {
            assert_eq!(i, *n as usize);
        }
    }
}
