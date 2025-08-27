use core::ops::{Index, IndexMut};

pub struct Queue<T, const N: usize> {
    arr: [Option<T>; N],
    index: usize,
    len: usize,
}

impl<T, const N: usize> Queue<T, N> {
    pub fn new() -> Self {
        Self {
            arr: [const { None }; N],
            index: 0,
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    /// Push a value to the back of the queue
    pub fn push_back(&mut self, elem: T) {
        if self.len >= N {
            panic!("Attempt to add element to full queue");
        }

        let pos = (self.index + self.len) % N;
        self.arr[pos] = Some(elem);
        self.len += 1;
    }

    /// Pops a value from the front of the queue
    pub fn pop_front(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            let val = self.arr[self.index].take().unwrap_or_else(|| {
                panic!("Unexpected None in backing array at index {}", self.index)
            });
            self.len -= 1;
            self.index = (self.index + 1) % N;

            Some(val)
        }
    }

    pub fn iter(&self) -> QueueIter<'_, T, N> {
        QueueIter {
            base: self,
            index: 0,
        }
    }
}

impl<T, const N: usize> IndexMut<usize> for Queue<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let pos = (self.index + index) % N;
        self.arr[pos]
            .as_mut()
            .unwrap_or_else(|| panic!("Invalid index access: {}", index))
    }
}

impl<T, const N: usize> Index<usize> for Queue<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        let pos = (self.index + index) % N;
        self.arr[pos]
            .as_ref()
            .unwrap_or_else(|| panic!("Invalid index access: {}", index))
    }
}

pub struct QueueIter<'a, T, const N: usize> {
    base: &'a Queue<T, N>,
    index: usize,
}

impl<'a, T, const N: usize> Iterator for QueueIter<'a, T, N> {
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
macro_rules! queue {
    [$($elem:expr),*] => {{
        #[allow(unused_mut)]
        let mut queue = $crate::Queue::new();
        $(queue.push_back($elem);)*
        queue
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_back() {
        let mut queue = Queue::<u32, 10>::new();
        let mut exp_backing = [None; 10];
        let mut exp_len = 0;

        assert_eq!(queue.arr, exp_backing);
        assert_eq!(queue.len, exp_len);

        queue.push_back(1);
        exp_backing[0] = Some(1);
        exp_len = 1;

        assert_eq!(queue.arr, exp_backing);
        assert_eq!(queue.len, exp_len);

        queue.push_back(2);
        exp_backing[1] = Some(2);
        exp_len += 1;

        assert_eq!(queue.arr, exp_backing);
        assert_eq!(queue.len, exp_len);

        queue.push_back(3);
        exp_backing[2] = Some(3);
        exp_len += 1;

        assert_eq!(queue.arr, exp_backing);
        assert_eq!(queue.len, exp_len);

        queue.push_back(4);
        exp_backing[3] = Some(4);
        exp_len += 1;

        assert_eq!(queue.arr, exp_backing);
        assert_eq!(queue.len, exp_len);

        queue.push_back(5);
        exp_backing[4] = Some(5);
        exp_len += 1;

        assert_eq!(queue.arr, exp_backing);
        assert_eq!(queue.len, exp_len);

        queue.push_back(6);
        exp_backing[5] = Some(6);
        exp_len += 1;

        assert_eq!(queue.arr, exp_backing);
        assert_eq!(queue.len, exp_len);
        queue.push_back(7);
        exp_backing[6] = Some(7);
        exp_len += 1;

        assert_eq!(queue.arr, exp_backing);
        assert_eq!(queue.len, exp_len);
        queue.push_back(8);
        exp_backing[7] = Some(8);
        exp_len += 1;

        assert_eq!(queue.arr, exp_backing);
        assert_eq!(queue.len, exp_len);
        queue.push_back(9);
        exp_backing[8] = Some(9);
        exp_len += 1;

        assert_eq!(queue.arr, exp_backing);
        assert_eq!(queue.len, exp_len);
        queue.push_back(0);
        exp_backing[9] = Some(0);
        exp_len += 1;

        assert_eq!(queue.arr, exp_backing);
        assert_eq!(queue.len, exp_len);
    }

    #[test]
    fn test_queue_macro() {
        let queue: Queue<u32, 10> = queue![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
        assert_eq!(queue.len, 10);
        assert_eq!(queue.arr, [1, 2, 3, 4, 5, 6, 7, 8, 9, 0].map(Some))
    }

    #[test]
    #[should_panic(expected = "Attempt to add element to full queue")]
    fn test_push_back_full_panic() {
        let mut queue: Queue<u32, 10> = queue![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
        queue.push_back(10);
    }

    #[test]
    fn test_pop_front() {
        let mut queue: Queue<u32, 10> = queue![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
        let mut exp_arr = queue.arr.clone();
        let mut exp_len = queue.len();
        let mut exp_index = queue.index;

        assert_eq!(queue.arr, exp_arr);
        assert_eq!(queue.len, exp_len);
        assert_eq!(queue.index, exp_index);

        assert_eq!(queue.pop_front(), Some(1));
        exp_arr[0] = None;
        exp_len -= 1;
        exp_index += 1;

        assert_eq!(queue.arr, exp_arr);
        assert_eq!(queue.len, exp_len);
        assert_eq!(queue.index, exp_index);

        assert_eq!(queue.pop_front(), Some(2));
        exp_arr[1] = None;
        exp_len -= 1;
        exp_index += 1;

        assert_eq!(queue.arr, exp_arr);
        assert_eq!(queue.len, exp_len);
        assert_eq!(queue.index, exp_index);

        assert_eq!(queue.pop_front(), Some(3));
        exp_arr[2] = None;
        exp_len -= 1;
        exp_index += 1;

        assert_eq!(queue.arr, exp_arr);
        assert_eq!(queue.len, exp_len);
        assert_eq!(queue.index, exp_index);

        assert_eq!(queue.pop_front(), Some(4));
        exp_arr[3] = None;
        exp_len -= 1;
        exp_index += 1;

        assert_eq!(queue.arr, exp_arr);
        assert_eq!(queue.len, exp_len);
        assert_eq!(queue.index, exp_index);

        assert_eq!(queue.pop_front(), Some(5));
        exp_arr[4] = None;
        exp_len -= 1;
        exp_index += 1;

        assert_eq!(queue.arr, exp_arr);
        assert_eq!(queue.len, exp_len);
        assert_eq!(queue.index, exp_index);

        assert_eq!(queue.pop_front(), Some(6));
        exp_arr[5] = None;
        exp_len -= 1;
        exp_index += 1;

        assert_eq!(queue.arr, exp_arr);
        assert_eq!(queue.len, exp_len);
        assert_eq!(queue.index, exp_index);

        assert_eq!(queue.pop_front(), Some(7));
        exp_arr[6] = None;
        exp_len -= 1;
        exp_index += 1;

        assert_eq!(queue.arr, exp_arr);
        assert_eq!(queue.len, exp_len);
        assert_eq!(queue.index, exp_index);

        assert_eq!(queue.pop_front(), Some(8));
        exp_arr[7] = None;
        exp_len -= 1;
        exp_index += 1;

        assert_eq!(queue.arr, exp_arr);
        assert_eq!(queue.len, exp_len);
        assert_eq!(queue.index, exp_index);

        assert_eq!(queue.pop_front(), Some(9));
        exp_arr[8] = None;
        exp_len -= 1;
        exp_index += 1;

        assert_eq!(queue.arr, exp_arr);
        assert_eq!(queue.len, exp_len);
        assert_eq!(queue.index, exp_index);

        assert_eq!(queue.pop_front(), Some(0));
        exp_arr[9] = None;
        exp_len -= 1;
        exp_index = 0;

        assert_eq!(queue.arr, exp_arr);
        assert_eq!(queue.len, exp_len);
        assert_eq!(queue.index, exp_index);

        assert_eq!(queue.pop_front(), None);
        assert_eq!(queue.arr, exp_arr);
        assert_eq!(queue.len, exp_len);
        assert_eq!(queue.index, exp_index);
    }

    #[test]
    fn test_index() {
        let queue: Queue<u32, 10> = queue![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        for i in 0..10 {
            assert_eq!(i, queue[i] as usize);
        }
    }

    #[test]
    fn test_iter() {
        let queue: Queue<u32, 10> = queue![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        for (i, n) in queue.iter().enumerate() {
            assert_eq!(i, *n as usize);
        }
    }
}
