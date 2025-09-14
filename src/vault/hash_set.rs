//! Implementation of IndexXxx traits for HashSet.

use core::{
    hash::{BuildHasher, Hash},
    ops::Bound,
};

use std::collections::HashSet;

use crate::{
    Never,
    index::{IndexCollection, IndexStore, IndexVault, IndexView},
};

//  #   Safety
//
//  -   NoPhantom: the store will only ever return indexes that have been inserted and have not been removed since.
unsafe impl<I, S> IndexView for HashSet<I, S>
where
    I: Copy + Eq + Hash + Ord,
    S: BuildHasher,
{
    type Index = I;

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn contains(&self, index: Self::Index) -> bool {
        self.contains(&index)
    }
}

impl<I, S> IndexCollection for HashSet<I, S>
where
    I: Copy + Eq + Hash + Ord,
    S: Default + BuildHasher,
{
    fn span() -> (Bound<Self::Index>, Bound<Self::Index>) {
        (Bound::Unbounded, Bound::Unbounded)
    }

    fn new() -> Self {
        Self::with_hasher(S::default())
    }

    fn with_span(_range: (Bound<Self::Index>, Bound<Self::Index>)) -> Self {
        Self::new()
    }
}

//  #   Safety
//
//  -   NoPhantom: the store will only ever return indexes that have been inserted and have not been removed since.
unsafe impl<I, S> IndexStore for HashSet<I, S>
where
    I: Copy + Eq + Hash + Ord,
    S: Default + BuildHasher,
{
    type InsertionError = Never;

    fn clear(&mut self) {
        self.clear()
    }

    fn insert(&mut self, index: Self::Index) -> Result<bool, Self::InsertionError> {
        Ok(self.insert(index))
    }

    fn remove(&mut self, index: Self::Index) -> bool {
        self.remove(&index)
    }
}

//  #   Safety
//
//  -   NoTheft: the vault will never return that it does not contain an index if the index was inserted, and was
//      not removed since.
unsafe impl<I, S> IndexVault for HashSet<I, S>
where
    I: Copy + Eq + Hash + Ord,
    S: Default + BuildHasher,
{
}

#[cfg(test)]
mod tests {
    use crate::test::IndexTester;

    use super::*;

    struct Tester;

    impl IndexTester for Tester {
        type Index = u8;
        type Victim = HashSet<u8>;

        fn upper_bound() -> u8 {
            u8::MAX
        }

        fn victim(indexes: &[u8]) -> Self::Victim {
            indexes.iter().copied().collect()
        }

        fn index(i: u8) -> Self::Index {
            i
        }
    }

    crate::test_index_view!(Tester);
    crate::test_index_collection!(Tester);
    crate::test_index_store!(Tester);
} // mod tests
