use super::{ LinearList, LinearListResult, LinearListError };

const MEMORY_SIZE: usize = 10;

struct Stack<T> {
    length: usize,
    memory: [T; MEMORY_SIZE],
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack {
            length: 0,
            memory: unsafe { ::std::mem::zeroed() },
        }
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Stack::new()
    }
}

impl<T> LinearList for Stack<T> {
    type Item = T;

    fn length(&self) -> usize {
        self.length
    }

    fn get(&self, pos: usize) -> Option<&Self::Item> {
        if pos < self.length {
            Some(&self.memory[pos])
        } else {
            None
        }
    }

    fn get_mut(&mut self, pos: usize) -> Option<&mut Self::Item> {
        if pos < self.length {
            Some(&mut self.memory[pos])
        } else {
            None
        }
    }

    fn insert_before(&mut self, pos: usize, item: Self::Item) -> LinearListResult<()> {
        if pos > self.length {
            Err(LinearListError::OutOfRange)
        } else if self.length == MEMORY_SIZE {
            Err(LinearListError::MemoryOverflow)
        } else {
            for i in (pos..self.length).rev() {
                self.memory.swap(i, i + 1);
            }
            self.memory[pos] = item;
            self.length += 1;
            Ok(())
        }
    }

    fn delete(&mut self, pos: usize) -> LinearListResult<Self::Item> {
        if pos < self.length {
            for i in pos..self.length - 1 {
                self.memory.swap(i, i + 1);
            }

            let item = unsafe {
                let mut item = ::std::mem::zeroed();
                ::std::mem::swap(&mut self.memory[self.length - 1], &mut item);
                item
            };

            self.length -= 1;
            Ok(item)
        } else {
            Err(LinearListError::OutOfRange)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::*;

    #[test]
    fn default() {
        let stack: Stack<usize> = Stack::default();

        assert_eq!(stack.length(), 0);
    }

    #[test]
    fn insert_get() {
        let mut stack = Stack::default();

        assert_eq!(stack.get(0), None);

        stack.insert_before(0, 42).unwrap();

        assert_eq!(stack.get(0), Some(&42));
        assert_eq!(stack.get(1), None);
    }

    #[test]
    fn insert_get_mut() {
        let mut stack = Stack::default();

        assert_eq!(stack.get_mut(0), None);

        stack.insert_before(0, 42).unwrap();

        assert_eq!(stack.get_mut(0), Some(&mut 42));
        assert_eq!(stack.get_mut(1), None);
    }

    #[test]
    fn delete() {
        let mut stack = Stack::default();

        assert_eq!(stack.get(0), None);

        stack.insert_before(0, 42).unwrap();

        assert_eq!(stack.get(0), Some(&42));

        assert_eq!(stack.delete(0).unwrap(), 42);
        assert_eq!(stack.get(0), None);
    }

    #[test]
    fn out_of_range() {
        let mut stack = Stack::default();

        assert_eq!(stack.insert_before(1, 42), Err(LinearListError::OutOfRange));
    }

    #[test]
    fn memory_overflow() {
        let mut stack = Stack::default();

        for i in 0..MEMORY_SIZE {
            assert_eq!(stack.insert_before(i, 42), Ok(()));
        }
        assert_eq!(stack.insert_before(MEMORY_SIZE, 42), Err(LinearListError::MemoryOverflow));
    }

    #[test]
    fn memory_order1() {
        let mut stack = Stack::default();

        for i in 0..MEMORY_SIZE {
            assert_eq!(stack.insert_before(i, i), Ok(()));
        }

        for i in (0..MEMORY_SIZE).rev() {
            assert_eq!(stack.delete(i), Ok(i));
        }
    }

    #[test]
    fn memory_order2() {
        let mut stack = Stack::default();

        for i in 0..MEMORY_SIZE {
            assert_eq!(stack.insert_before(i, i), Ok(()));
        }

        for i in 0..MEMORY_SIZE {
            assert_eq!(stack.delete(0), Ok(i));
        }
    }

    #[test]
    fn memory_order3() {
        let mut stack = Stack::default();

        for i in 0..MEMORY_SIZE {
            assert_eq!(stack.insert_before(0, i), Ok(()));
        }

        for i in (0..MEMORY_SIZE).rev() {
            assert_eq!(stack.delete(0), Ok(i));
        }
    }

    #[test]
    fn memory_sort() {
        let mut stack = Stack::default();

        for i in 0..MEMORY_SIZE {
            assert_eq!(stack.insert_before(0, i), Ok(()));
        }

        stack.sort();

        for i in 0..MEMORY_SIZE {
            assert_eq!(stack.delete(0), Ok(i));
        }
    }

    #[test]
    fn sort() {
        let mut stack = Stack::default();

        for i in 0..MEMORY_SIZE {
            assert_eq!(stack.insert_before(0, i), Ok(()));
        }

        assert_eq!(stack.get(0), Some(&(MEMORY_SIZE-1)));

        stack.sort();

        assert_eq!(stack.get(0), Some(&0));
    }
}