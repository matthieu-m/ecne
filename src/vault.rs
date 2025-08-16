//! A collection of index vaults for common needs.

#[cfg(any(feature = "alloc", test))]
mod btree_set {
    use core::ops::Bound;

    #[cfg(feature = "nightly")]
    use core::{num::NonZeroUsize, ops::Try};

    use alloc::collections::BTreeSet;

    use crate::{
        Never,
        index::{IndexBackward, IndexCollection, IndexForward, IndexOrdered, IndexStore, IndexVault, IndexView},
    };

    //  #   Safety
    //
    //  -   NoPhantom: the store will only ever return indexes that have been inserted and have not been removed since.
    unsafe impl<I> IndexView for BTreeSet<I>
    where
        I: Copy + Eq + Ord,
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

    impl<I> IndexCollection for BTreeSet<I>
    where
        I: Copy + Eq + Ord,
    {
        fn span() -> (Bound<Self::Index>, Bound<Self::Index>) {
            (Bound::Unbounded, Bound::Unbounded)
        }

        fn new() -> Self {
            Self::new()
        }

        fn with_span(_range: (Bound<Self::Index>, Bound<Self::Index>)) -> Self {
            Self::new()
        }
    }

    //  #   Safety
    //
    //  -   NoPhantom: the store will only ever return indexes that have been inserted and have not been removed since.
    unsafe impl<I> IndexStore for BTreeSet<I>
    where
        I: Copy + Eq + Ord,
    {
        type InsertionError = Never;

        fn clear(&mut self) {
            self.clear()
        }

        fn insert(&mut self, index: Self::Index) -> Result<bool, Never> {
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
    unsafe impl<I> IndexVault for BTreeSet<I> where I: Copy + Eq + Ord {}

    //  #   Safety
    //
    //  -   NoDuplicate: the view SHALL never return the same index a second time.
    //  -   NoPhantom: the view SHALL only ever return that it contains an index if the index was inserted, and was not
    //      removed since.
    //  -   NoTheft: if `Self` implements `IndexVault`, the view shall return all indexes.
    unsafe impl<I> IndexForward for BTreeSet<I>
    where
        I: Copy + Eq + Ord,
    {
        fn first(&self) -> Option<Self::Index> {
            self.first().copied()
        }

        fn next_after(&self, current: Self::Index) -> Option<Self::Index> {
            self.range(forward_range(current)).next().copied()
        }

        #[cfg(feature = "nightly")]
        fn nth_after(&self, n: usize, current: Self::Index) -> Result<Self::Index, NonZeroUsize> {
            let mut iterator = self.range(forward_range(current));

            if let Some(n) = n.checked_sub(1) {
                iterator.advance_by(n).map_err(|err| err.saturating_add(1))?;
            }

            iterator.next().copied().ok_or(NonZeroUsize::MIN)
        }

        #[cfg(feature = "nightly")]
        fn try_fold_after<B, F, R>(&self, current: Self::Index, accumulator: B, f: F) -> R
        where
            F: FnMut(B, Self::Index) -> R,
            R: Try<Output = B>,
        {
            self.range(forward_range(current)).copied().try_fold(accumulator, f)
        }
    }

    //  #   Safety
    //
    //  -   Reverse: the view WILL return indexes in the exact opposite sequence than `IndexForward` does.
    unsafe impl<I> IndexBackward for BTreeSet<I>
    where
        I: Copy + Eq + Ord,
    {
        fn last(&self) -> Option<Self::Index> {
            self.last().copied()
        }

        fn next_before(&self, current: Self::Index) -> Option<Self::Index> {
            self.range(backward_range(current)).next_back().copied()
        }

        #[cfg(feature = "nightly")]
        fn nth_before(&self, n: usize, current: Self::Index) -> Result<Self::Index, NonZeroUsize> {
            let mut iterator = self.range(backward_range(current));

            if let Some(n) = n.checked_sub(1) {
                iterator.advance_back_by(n).map_err(|err| err.saturating_add(1))?;
            }

            iterator.next_back().copied().ok_or(NonZeroUsize::MIN)
        }

        #[cfg(feature = "nightly")]
        fn try_fold_before<B, F, R>(&self, current: Self::Index, accumulator: B, f: F) -> R
        where
            F: FnMut(B, Self::Index) -> R,
            R: Try<Output = B>,
        {
            self.range(backward_range(current)).copied().try_rfold(accumulator, f)
        }
    }

    //  #   Safety
    //
    //  -   Ordered: the `IndexForward` implementation WILL return indexes in strictly increasing order.
    unsafe impl<I> IndexOrdered for BTreeSet<I> where I: Copy + Eq + Ord {}

    fn backward_range<I>(current: I) -> (Bound<I>, Bound<I>) {
        (Bound::Unbounded, Bound::Excluded(current))
    }

    fn forward_range<I>(current: I) -> (Bound<I>, Bound<I>) {
        (Bound::Excluded(current), Bound::Unbounded)
    }
} // mod btree_set

#[cfg(any(feature = "std", test))]
mod hash_set {
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
} // mod hash_set
