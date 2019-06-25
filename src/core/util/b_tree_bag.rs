//! This module defines a `BTreeBag` collection of elements that implement `Ord`.
//!
//! The implementation has the following goals:
//! 1) Insert and remove elements in a time that is similar to O(log(n)).
//! 2) Allow for duplicate values.
//! 3) Provide an iterator that visits the elements in sorted order.
//! 4) Implement Hash, PartialOrd.

use std::collections::btree_map::BTreeMap;

pub struct BTreeBag<T> {
    inner: BTreeMap<T, usize>
}

struct DuplicationState<'a, T> {
    element: &'a T,
    total_count: usize,
    current_count: usize,
}

impl<'a, T> From<(&'a T, &'a usize)> for DuplicationState<'a, T> {
    fn from(kv: (&'a T, &'a usize)) -> DuplicationState<'a, T> {
        DuplicationState {
            element: kv.0,
            total_count: *kv.1,
            current_count: 0,
        }
    }
}

impl<'a, T> Iterator for DuplicationState<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.current_count < self.total_count {
            self.current_count += 1;
            Some(self.element)
        } else {
            None
        }
    }
}

impl<T> BTreeBag<T>
    where T: Ord {
    pub fn new() -> BTreeBag<T> {
        BTreeBag {
            inner: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, t: T) {
        self.inner
            .entry(t)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item=&'a T> {
        self.inner.iter().flat_map(|kv| {
            let i: DuplicationState<'a, T> = kv.into();
            i
        })
    }
}
