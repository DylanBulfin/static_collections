use core::cmp::Ordering;

pub struct PriorityQueue<T, const N: usize>
where
    T: Ord,
{
    arr: [Option<T>; N],
    len: usize,
}

impl<T, const N: usize> PriorityQueue<T, N>
where
    T: Ord,
{
    pub const fn new() -> Self {
        Self {
            arr: [const { None }; N],
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn insert(&mut self, elem: T) {
        if self.len >= N {
            panic!("Attempt to add element to full priority queue");
        }

        let spot = self.search_for_new_spot(&elem, 0, self.len);

        for i in (spot..self.len).rev() {
            self.arr[i + 1] = self.arr[i].take();
        }

        self.arr[spot] = Some(elem);
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            let elem = self.arr[self.len - 1].take().unwrap_or_else(|| {
                panic!(
                    "Unexpected None at index {} when len is {}",
                    self.len - 1,
                    self.len
                )
            });

            self.len -= 1;
            Some(elem)
        }
    }

    fn search_for_new_spot(&self, elem: &T, start: usize, end: usize) -> usize {
        let diff = end - start;

        if diff == 0 {
            if self.len != 0 {
                panic!(
                    "search_for_new_spot called with end-start of 0 when len is {}",
                    self.len
                )
            }

            0
        } else if diff == 1 {
            let start_e = self.arr[start].as_ref().unwrap_or_else(|| {
                panic!("Unexpected None at index {} when len {}", start, self.len)
            });
            match start_e.cmp(elem) {
                Ordering::Greater | Ordering::Equal => end,
                Ordering::Less => start,
            }
        } else {
            let midpoint = start + (diff / 2);

            let mid_e = self.arr[midpoint].as_ref().unwrap_or_else(|| {
                panic!(
                    "Unexpected None at index {} when len {}",
                    midpoint, self.len
                )
            });
            match mid_e.cmp(elem) {
                Ordering::Greater | Ordering::Equal => {
                    self.search_for_new_spot(elem, midpoint, end)
                }
                Ordering::Less => self.search_for_new_spot(elem, start, midpoint),
            }
        }
    }
}

#[macro_export]
macro_rules! pqueue {
    [$($elem:expr),*] => {{
        #[allow(unused_mut)]
        let mut pqueue = $crate::PriorityQueue::new();
        $(pqueue.insert($elem);)*
        pqueue
    }};
}

#[cfg(test)]
mod tests {
    use crate::PriorityQueue;

    #[test]
    fn test_insert() {
        let mut pqueue = PriorityQueue::<_, 10>::new();

        pqueue.insert(3);
        pqueue.insert(2);
        pqueue.insert(4);
        pqueue.insert(0);
        pqueue.insert(1);

        let mut exp_arr = [None; 10];
        exp_arr[0] = Some(4);
        exp_arr[1] = Some(3);
        exp_arr[2] = Some(2);
        exp_arr[3] = Some(1);
        exp_arr[4] = Some(0);

        assert_eq!(pqueue.arr, exp_arr);
        assert_eq!(pqueue.len, 5);
    }

    #[test]
    fn test_pqueue_macro() {
        let pqueue: PriorityQueue<_, 10> = pqueue!(3, 2, 4, 0, 1);

        let mut exp_arr = [None; 10];
        exp_arr[0] = Some(4);
        exp_arr[1] = Some(3);
        exp_arr[2] = Some(2);
        exp_arr[3] = Some(1);
        exp_arr[4] = Some(0);

        assert_eq!(pqueue.arr, exp_arr);
        assert_eq!(pqueue.len, 5);
    }

    #[test]
    fn test_pop() {
        let mut pqueue: PriorityQueue<_, 10> = pqueue!(3, 2, 4, 0, 1);

        assert_eq!(pqueue.pop(), Some(0));

        let mut exp_arr = [None; 10];

        exp_arr[0] = Some(4);
        exp_arr[1] = Some(3);
        exp_arr[2] = Some(2);
        exp_arr[3] = Some(1);

        assert_eq!(pqueue.arr, exp_arr);
        assert_eq!(pqueue.len, 4);

        assert_eq!(pqueue.pop(), Some(1));
        exp_arr[3] = None;

        assert_eq!(pqueue.arr, exp_arr);
        assert_eq!(pqueue.len, 3);

        assert_eq!(pqueue.pop(), Some(2));
        exp_arr[2] = None;

        assert_eq!(pqueue.arr, exp_arr);
        assert_eq!(pqueue.len, 2);

        assert_eq!(pqueue.pop(), Some(3));
        exp_arr[1] = None;

        assert_eq!(pqueue.arr, exp_arr);
        assert_eq!(pqueue.len, 1);

        pqueue.insert(8);
        exp_arr[1] = exp_arr[0].take();
        exp_arr[0] = Some(8);

        assert_eq!(pqueue.arr, exp_arr);
        assert_eq!(pqueue.len, 2);
    }
}
