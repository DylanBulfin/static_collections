use core::ops::{Index, IndexMut};

pub struct Stack<T, const N: usize> {
    arr: [Option<T>; N],
    len: usize,
}

impl<T, const N: usize> Stack<T, N> {
    pub const fn new() -> Self {
        Self {
            arr: [const { None }; N],
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    /// Push a value to the front of the stack (the back of the backing array)
    pub fn push(&mut self, elem: T) {
        if self.len >= N {
            panic!("Attempt to add value to full stack");
        }

        self.arr[self.len] = Some(elem);
        self.len += 1;
    }

    /// Pop a value from the front of the stack (the back of the backing array)
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            self.arr[self.len].take()
        }
    }

    /// Clear the backing array entirely, destroying all elements
    pub fn clear(&mut self) {
        self.arr = [const { None }; N];
        self.len = 0;
    }

    pub fn iter(&self) -> StackIter<'_, T, N> {
        StackIter {
            base: &self,
            index: 0,
        }
    }
}

impl<T, const N: usize> Index<usize> for Stack<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len {
            panic!(
                "Attempt to access invalid index {} on Stack with length {}",
                index, self.len
            );
        }
        self.arr[self.len - index - 1].as_ref().unwrap_or_else(|| {
            panic!(
                "Unexpected None at index {} of backing array",
                self.len - index - 1
            )
        })
    }
}

impl<T, const N: usize> IndexMut<usize> for Stack<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.len {
            panic!(
                "Attempt to access invalid index {} on Stack with length {}",
                index, self.len
            );
        }
        self.arr[self.len - index - 1].as_mut().unwrap_or_else(|| {
            panic!(
                "Unexpected None at index {} of backing array",
                self.len - index - 1
            )
        })
    }
}

pub struct StackIter<'a, T, const N: usize> {
    base: &'a Stack<T, N>,
    index: usize,
}

impl<'a, T, const N: usize> Iterator for StackIter<'a, T, N> {
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
macro_rules! stack {
    [$($elem:expr),*] => {{
        #[allow(unused_mut)]
        let mut stack = $crate::Stack::new();
        $(stack.push($elem);)*
        stack
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push() {
        let mut stack = Stack::<u32, 10>::new();
        let mut exp_backing = [None; 10];
        let mut exp_len = 0;

        assert_eq!(stack.arr, exp_backing);
        assert_eq!(stack.len, exp_len);

        stack.push(1);
        exp_backing[0] = Some(1);
        exp_len = 1;

        assert_eq!(stack.arr, exp_backing);
        assert_eq!(stack.len, exp_len);

        stack.push(2);
        exp_backing[1] = Some(2);
        exp_len += 1;

        assert_eq!(stack.arr, exp_backing);
        assert_eq!(stack.len, exp_len);

        stack.push(3);
        exp_backing[2] = Some(3);
        exp_len += 1;

        assert_eq!(stack.arr, exp_backing);
        assert_eq!(stack.len, exp_len);

        stack.push(4);
        exp_backing[3] = Some(4);
        exp_len += 1;

        assert_eq!(stack.arr, exp_backing);
        assert_eq!(stack.len, exp_len);

        stack.push(5);
        exp_backing[4] = Some(5);
        exp_len += 1;

        assert_eq!(stack.arr, exp_backing);
        assert_eq!(stack.len, exp_len);

        stack.push(6);
        exp_backing[5] = Some(6);
        exp_len += 1;

        assert_eq!(stack.arr, exp_backing);
        assert_eq!(stack.len, exp_len);
        stack.push(7);
        exp_backing[6] = Some(7);
        exp_len += 1;

        assert_eq!(stack.arr, exp_backing);
        assert_eq!(stack.len, exp_len);
        stack.push(8);
        exp_backing[7] = Some(8);
        exp_len += 1;

        assert_eq!(stack.arr, exp_backing);
        assert_eq!(stack.len, exp_len);
        stack.push(9);
        exp_backing[8] = Some(9);
        exp_len += 1;

        assert_eq!(stack.arr, exp_backing);
        assert_eq!(stack.len, exp_len);
        stack.push(0);
        exp_backing[9] = Some(0);
        exp_len += 1;

        assert_eq!(stack.arr, exp_backing);
        assert_eq!(stack.len, exp_len);
    }

    #[test]
    fn test_stack_macro() {
        let stack: Stack<u32, 10> = stack![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
        assert_eq!(stack.len, 10);
        assert_eq!(stack.arr, [1, 2, 3, 4, 5, 6, 7, 8, 9, 0].map(Some))
    }

    #[test]
    #[should_panic(expected = "Attempt to add value to full stack")]
    fn test_push_full_panic() {
        let mut stack: Stack<u32, 10> = stack![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
        stack.push(10);
    }

    #[test]
    fn test_pop() {
        let mut stack: Stack<u32, 10> = stack![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
        let mut exp_arr = stack.arr.clone();
        let mut exp_len = stack.len();

        assert_eq!(stack.arr, exp_arr);
        assert_eq!(stack.len, exp_len);

        assert_eq!(stack.pop(), Some(0));
        exp_arr[9] = None;
        exp_len -= 1;

        assert_eq!(stack.arr, exp_arr);
        assert_eq!(stack.len, exp_len);

        assert_eq!(stack.pop(), Some(9));
        exp_arr[8] = None;
        exp_len -= 1;

        assert_eq!(stack.arr, exp_arr);
        assert_eq!(stack.len, exp_len);

        assert_eq!(stack.pop(), Some(8));
        exp_arr[7] = None;
        exp_len -= 1;

        assert_eq!(stack.arr, exp_arr);
        assert_eq!(stack.len, exp_len);

        assert_eq!(stack.pop(), Some(7));
        exp_arr[6] = None;
        exp_len -= 1;

        assert_eq!(stack.arr, exp_arr);
        assert_eq!(stack.len, exp_len);

        assert_eq!(stack.pop(), Some(6));
        exp_arr[5] = None;
        exp_len -= 1;

        assert_eq!(stack.arr, exp_arr);
        assert_eq!(stack.len, exp_len);

        assert_eq!(stack.pop(), Some(5));
        exp_arr[4] = None;
        exp_len -= 1;

        assert_eq!(stack.arr, exp_arr);
        assert_eq!(stack.len, exp_len);

        assert_eq!(stack.pop(), Some(4));
        exp_arr[3] = None;
        exp_len -= 1;

        assert_eq!(stack.arr, exp_arr);
        assert_eq!(stack.len, exp_len);

        assert_eq!(stack.pop(), Some(3));
        exp_arr[2] = None;
        exp_len -= 1;

        assert_eq!(stack.arr, exp_arr);
        assert_eq!(stack.len, exp_len);

        assert_eq!(stack.pop(), Some(2));
        exp_arr[1] = None;
        exp_len -= 1;

        assert_eq!(stack.arr, exp_arr);
        assert_eq!(stack.len, exp_len);

        assert_eq!(stack.pop(), Some(1));
        exp_arr[0] = None;
        exp_len -= 1;

        assert_eq!(stack.arr, exp_arr);
        assert_eq!(stack.len, exp_len);

        assert_eq!(stack.pop(), None);
        assert_eq!(stack.arr, exp_arr);
        assert_eq!(stack.len, exp_len);
    }

    #[test]
    fn test_index() {
        let stack: Stack<u32, 10> = stack![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        for i in 0..10 {
            assert_eq!(i, stack[i] as usize);
        }
    }

    #[test]
    fn test_iter() {
        let stack: Stack<u32, 10> = stack![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        for (i, n) in stack.iter().enumerate() {
            assert_eq!(i, *n as usize);
        }
    }
}

