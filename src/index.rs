//! A collection of traits for index-based vaults.

use core::num::NonZeroUsize;

#[cfg(feature = "nightly")]
use core::ops::Try;

/// A collection indexes.
pub trait IndexCollection {
    /// Constructs a new collection, with the appropriate capacity for storing up to `n` indexes.
    fn with_capacity(n: usize) -> Self;

    /// Returns whether the collection is empty, or not.
    fn is_empty(&self) -> bool;

    /// Returns the number of indexes in the collection.
    fn len(&self) -> usize;

    /// Returns the capacity of the collection.
    fn capacity(&self) -> usize;

    /// Removes all the indexes from the collection.
    fn clear(&mut self);
}

/// A store of indexes.
///
/// #   Safety
///
/// -   NoPhantom: the store SHALL only ever return that it contains an index if the index was inserted, and was not
///     removed since.
pub unsafe trait IndexStore: IndexCollection {
    /// The type of the index.
    ///
    /// There is no guarantee that the index is stored verbatim, and it may, in fact, be de-materialized on storing it,
    /// and re-materialized on retrieving it.
    type Index: Copy + Eq + Ord;

    /// Returns whether the given index is contained in the store.
    fn contains(&self, index: Self::Index) -> bool;

    /// Inserts the index in the store, returns whether it is newly inserted.
    fn insert(&mut self, index: Self::Index) -> bool;

    /// Removes the index from the store, returns whether it was in the store prior to removal.
    fn remove(&mut self, index: Self::Index) -> bool;
}

/// A trustworthy vault of indexes.
///
/// #   Safety
///
/// -   NoTheft: the vault SHALL never return that it does not contain an index if the index was inserted, and was not
///     removed since.
pub unsafe trait IndexVault: IndexStore {}

/// An iterable view of the indexes in the store.
///
/// For backward iteration -- whatever that means -- see `IndexBackward`.
///
/// #   Safety
///
/// -   NoDuplicate: the view SHALL never return the same index a second time.
/// -   NoPhantom: the view SHALL only ever return that it contains an index if the index was inserted, and was not
///     removed since.
/// -   NoTheft: if `Self` implements `IndexVault`, the view SHALL return all indexes.
pub unsafe trait IndexForward: IndexStore {
    //  Why not an iterator?
    //
    //  Implementing functionality such as `drain`, `extract_if`, or `retain`, requires mutable access in-between steps.

    /// Returns one index contained by the store, if any.
    fn first(&self) -> Option<Self::Index>;

    /// Returns the next index after the provided one, if any.
    fn next_after(&self, current: Self::Index) -> Option<Self::Index>;

    /// Returns the n-th index after the provided one, or the remainder of `n`.
    ///
    /// #   Note to Implementors
    ///
    /// Try to implement this method if it can be implemented to skip ahead, rather than advance one at a time.
    fn nth_after(&self, n: usize, mut current: Self::Index) -> Result<Self::Index, NonZeroUsize> {
        for i in 0..n {
            //  Safety:
            //  -   NonZero: i < n.
            let remainder = unsafe { NonZeroUsize::new_unchecked(n - i + 1) };

            current = self.next_after(current).ok_or(remainder)?;
        }

        self.next_after(current).ok_or(NonZeroUsize::MIN)
    }

    /// Applies the function `f` as long as it returns successfully, producing a single, final value.
    ///
    /// #   Note to Implementors
    ///
    /// Try to implement this method if internal iteration can be optimized.
    #[cfg(feature = "nightly")]
    fn try_fold_after<B, F, R>(&self, current: Self::Index, mut accumulator: B, mut f: F) -> R
    where
        F: FnMut(B, Self::Index) -> R,
        R: Try<Output = B>,
    {
        accumulator = f(accumulator, current)?;

        while let Some(current) = self.next_after(current) {
            accumulator = f(accumulator, current)?;
        }

        R::from_output(accumulator)
    }
}

/// An iterable view of the indexes in the store.
///
/// For forward iteration -- whatever that means -- see `IndexForward`.
///
/// #   Safety
///
/// -   Reverse: the view SHALL return indexes in the exact opposite sequence than `IndexForward` does.
pub unsafe trait IndexBackward: IndexForward {
    /// Returns one index contained by the store, if any.
    fn last(&self) -> Option<Self::Index>;

    /// Returns the next index before the provided one, if any.
    fn next_before(&self, current: Self::Index) -> Option<Self::Index>;

    /// Returns the n-th index before the provided one, or the remainder of `n`.
    ///
    /// #   Note to Implementors
    ///
    /// Try to implement this method if it can be implemented to skip ahead, rather than advance one at a time.
    fn nth_before(&self, n: usize, mut current: Self::Index) -> Result<Self::Index, NonZeroUsize> {
        for i in 0..n {
            //  Safety:
            //  -   NonZero: i < n.
            let remainder = unsafe { NonZeroUsize::new_unchecked(n - i + 1) };

            current = self.next_before(current).ok_or(remainder)?;
        }

        self.next_before(current).ok_or(NonZeroUsize::MIN)
    }

    /// Applies the function `f` as long as it returns successfully, producing a single, final value.
    ///
    /// #   Note to Implementors
    ///
    /// Try to implement this method if internal iteration can be optimized.
    #[cfg(feature = "nightly")]
    fn try_fold_before<B, F, R>(&self, current: Self::Index, mut accumulator: B, mut f: F) -> R
    where
        F: FnMut(B, Self::Index) -> R,
        R: Try<Output = B>,
    {
        accumulator = f(accumulator, current)?;

        while let Some(current) = self.next_before(current) {
            accumulator = f(accumulator, current)?;
        }

        R::from_output(accumulator)
    }
}

/// An _ordered_ view of the indexes in the store.
///
/// #   Safety
///
/// -   Ordered: the `IndexForward` implementation SHALL return indexes in strictly increasing order.
pub unsafe trait IndexOrdered: IndexForward {}

#[cfg(test)]
mod tests {
    use core::ops::Bound;

    use alloc::collections::BTreeSet;

    use super::*;

    #[derive(Clone, Default)]
    struct Victim(BTreeSet<usize>);

    #[test]
    fn nth_after() {
        let victim = Victim(BTreeSet::from_iter([1, 2, 3, 5]));

        assert_eq!(Ok(2), victim.nth_after(0, 1));
        assert_eq!(Ok(3), victim.nth_after(0, 2));
        assert_eq!(Ok(5), victim.nth_after(0, 3));
        assert_eq!(Err(1), victim.nth_after(0, 5).map_err(|e| e.get()));

        assert_eq!(Ok(3), victim.nth_after(1, 1));
        assert_eq!(Ok(5), victim.nth_after(1, 2));
        assert_eq!(Err(1), victim.nth_after(1, 3).map_err(|e| e.get()));
        assert_eq!(Err(2), victim.nth_after(1, 5).map_err(|e| e.get()));

        assert_eq!(Ok(5), victim.nth_after(2, 1));
        assert_eq!(Err(1), victim.nth_after(2, 2).map_err(|e| e.get()));
        assert_eq!(Err(2), victim.nth_after(2, 3).map_err(|e| e.get()));
        assert_eq!(Err(3), victim.nth_after(2, 5).map_err(|e| e.get()));

        assert_eq!(Err(1), victim.nth_after(3, 1).map_err(|e| e.get()));
        assert_eq!(Err(2), victim.nth_after(3, 2).map_err(|e| e.get()));
        assert_eq!(Err(3), victim.nth_after(3, 3).map_err(|e| e.get()));
        assert_eq!(Err(4), victim.nth_after(3, 5).map_err(|e| e.get()));

        assert_eq!(Err(5), victim.nth_after(7, 1).map_err(|e| e.get()));
        assert_eq!(Err(6), victim.nth_after(7, 2).map_err(|e| e.get()));
        assert_eq!(Err(7), victim.nth_after(7, 3).map_err(|e| e.get()));
        assert_eq!(Err(8), victim.nth_after(7, 5).map_err(|e| e.get()));
    }

    #[test]
    fn nth_before() {
        let victim = Victim(BTreeSet::from_iter([1, 2, 3, 5]));

        assert_eq!(Ok(3), victim.nth_before(0, 5));
        assert_eq!(Ok(2), victim.nth_before(0, 3));
        assert_eq!(Ok(1), victim.nth_before(0, 2));
        assert_eq!(Err(1), victim.nth_before(0, 1).map_err(|e| e.get()));

        assert_eq!(Ok(2), victim.nth_before(1, 5));
        assert_eq!(Ok(1), victim.nth_before(1, 3));
        assert_eq!(Err(1), victim.nth_before(1, 2).map_err(|e| e.get()));
        assert_eq!(Err(2), victim.nth_before(1, 1).map_err(|e| e.get()));

        assert_eq!(Ok(1), victim.nth_before(2, 5));
        assert_eq!(Err(1), victim.nth_before(2, 3).map_err(|e| e.get()));
        assert_eq!(Err(2), victim.nth_before(2, 2).map_err(|e| e.get()));
        assert_eq!(Err(3), victim.nth_before(2, 1).map_err(|e| e.get()));

        assert_eq!(Err(1), victim.nth_before(3, 5).map_err(|e| e.get()));
        assert_eq!(Err(2), victim.nth_before(3, 3).map_err(|e| e.get()));
        assert_eq!(Err(3), victim.nth_before(3, 2).map_err(|e| e.get()));
        assert_eq!(Err(4), victim.nth_before(3, 1).map_err(|e| e.get()));

        assert_eq!(Err(5), victim.nth_before(7, 5).map_err(|e| e.get()));
        assert_eq!(Err(6), victim.nth_before(7, 3).map_err(|e| e.get()));
        assert_eq!(Err(7), victim.nth_before(7, 2).map_err(|e| e.get()));
        assert_eq!(Err(8), victim.nth_before(7, 1).map_err(|e| e.get()));
    }

    impl IndexCollection for Victim {
        fn with_capacity(_n: usize) -> Self {
            Self::default()
        }

        fn is_empty(&self) -> bool {
            self.0.is_empty()
        }

        fn len(&self) -> usize {
            self.0.len()
        }

        fn capacity(&self) -> usize {
            self.0.len()
        }

        fn clear(&mut self) {
            self.0.clear();
        }
    }

    //  Safety:
    //  -   NoPhantom: the store WILL ever return that it contains an index if the index was inserted, and was not
    //      removed since.
    unsafe impl IndexStore for Victim {
        type Index = usize;

        fn contains(&self, index: Self::Index) -> bool {
            self.0.contains(&index)
        }

        fn insert(&mut self, index: Self::Index) -> bool {
            self.0.insert(index)
        }

        fn remove(&mut self, index: Self::Index) -> bool {
            self.0.remove(&index)
        }
    }

    //  Safety:
    //  -   NoTheft: the vault WILL never return that it does not contain an index if the index was inserted, and was not
    //      removed since.
    unsafe impl IndexVault for Victim {}

    //  #   Safety
    //
    //  -   NoDuplicate: the view WILL never return the same index a second time.
    //  -   NoPhantom: the view WILL only ever return that it contains an index if the index was inserted, and was not
    //      removed since.
    //  -   NoTheft: if `Self` implements `IndexVault`, the view WILL return all indexes.
    unsafe impl IndexForward for Victim {
        fn first(&self) -> Option<Self::Index> {
            self.0.first().copied()
        }

        fn next_after(&self, current: Self::Index) -> Option<Self::Index> {
            self.0.range(forward_range(current)).next().copied()
        }
    }

    //  #   Safety
    //
    //  -   Reverse: the view WILL return indexes in the exact opposite sequence than `IndexForward` does.
    unsafe impl IndexBackward for Victim {
        fn last(&self) -> Option<Self::Index> {
            self.0.last().copied()
        }

        fn next_before(&self, current: Self::Index) -> Option<Self::Index> {
            self.0.range(backward_range(current)).next_back().copied()
        }
    }

    fn backward_range<I>(current: I) -> (Bound<I>, Bound<I>) {
        (Bound::Unbounded, Bound::Excluded(current))
    }

    fn forward_range<I>(current: I) -> (Bound<I>, Bound<I>) {
        (Bound::Excluded(current), Bound::Unbounded)
    }
} // mod tests
