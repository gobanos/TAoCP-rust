mod sequential_allocation;

use std::cmp::Ordering;

#[derive(Debug,PartialEq)]
pub enum LinearListError {
    OutOfRange,
    MemoryOverflow,
}

pub type LinearListResult<T> = Result<T, LinearListError>;

trait LinearList {
    type Item;

    fn length(&self) -> usize;

    fn get(&self, pos: usize) -> Option<&Self::Item>;
    fn get_mut(&mut self, pos: usize) -> Option<&mut Self::Item>;

    fn insert_before(&mut self, pos: usize, item: Self::Item) -> LinearListResult<()>;

    fn delete(&mut self, pos: usize) -> LinearListResult<Self::Item>;

    fn insert_after(&mut self, pos: usize, item: Self::Item) -> LinearListResult<()> {
        self.insert_before(pos + 1, item)
    }

    fn combine<T>(&mut self, mut other: T) where T: LinearList<Item=Self::Item> {
        while let Ok(item) = other.delete(0) {  // Take the first item in other
            let len = self.length();            // And push it back in self

            self.insert_before(len, item).unwrap();
        }
    }

    fn combine_all<T>(&mut self, others: Vec<T>) where T: LinearList<Item=Self::Item>, T: Sized {
        for list in others {
            self.combine(list);
        }
    }

    fn clone(&self) -> Self where Self::Item: Clone, Self: Default {
        let mut new_list = Self::default();

        for i in 0..self.length() {
            new_list.insert_before(i, self.get(i).unwrap().clone()).unwrap();
        }

        new_list
    }

    fn clone_combine<T>(&mut self, other: &T) where T: LinearList<Item=Self::Item>, Self::Item: Clone {
        for i in 0..other.length() {
            let item = other.get(i).unwrap();   // Take the current item in other
            let len = self.length();            // And push it back in self

            self.insert_before(len, item.clone()).unwrap();
        }
    }

    fn clone_combine_all<T>(&mut self, others: &[T]) where T: LinearList<Item=Self::Item>, T: Sized, Self::Item: Clone {
        for list in others {
            self.clone_combine(list);
        }
    }

    fn sort(&mut self) where Self::Item: Ord {
        let mut sorted = false;
        while !sorted {
            sorted = true;

            let i_iter = 0..self.length();
            let j_iter = i_iter.clone().skip(1);

            for (i, j) in i_iter.zip(j_iter) {
                let a : *mut _ = self.get_mut(i).unwrap();
                let b : *mut _ = self.get_mut(j).unwrap();

                if unsafe { *a > *b } {
                    sorted = false;
                    unsafe {
                        ::std::ptr::swap(a, b);
                    }
                }
            }
        }
    }

    fn sort_by<F>(&mut self, compare: F) where F: Fn(&Self::Item, &Self::Item) -> Ordering {
        let mut sorted = false;
        while !sorted {
            sorted = true;

            let i_iter = 0..self.length();
            let j_iter = i_iter.clone().skip(1);

            for (i, j) in i_iter.zip(j_iter) {
                let a : *mut _ = self.get_mut(i).unwrap();
                let b : *mut _ = self.get_mut(j).unwrap();

                match compare(unsafe { &*a }, unsafe { &*b }) {
                    Ordering::Greater => {
                        sorted = false;
                        unsafe {
                            ::std::ptr::swap(a, b);
                        }
                    },
                    _ => ()
                }
            }
        }
    }

    fn search_by<P>(&self, predicate: P) -> Option<&Self::Item> where P: Fn(&Self::Item) -> bool {
        for i in 0..self.length() {
            let item = self.get(i).unwrap();
            if predicate(item) {
                return Some(item);
            }
        }
        None
    }
}