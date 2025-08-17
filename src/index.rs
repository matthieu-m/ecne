//! A collection of traits for index-based vaults.

use core::{fmt, num::NonZeroUsize, ops::Bound};

#[cfg(feature = "nightly")]
use core::ops::Try;

use crate::chunk::IndexChunk;

/// A view of indexes.
///
/// #   Safety
///
/// -   NoPhantom: the view SHALL only ever return that it contains an index if the index was inserted, and was not
///     removed since.
pub unsafe trait IndexView {
    /// The type of the index.
    ///
    /// There is no guarantee that the index is stored verbatim, and it may, in fact, be de-materialized on storing it,
    /// and re-materialized on retrieving it.
    type Index: Copy + Eq + Ord;

    /// Returns whether the collection is empty, or not.
    fn is_empty(&self) -> bool;

    /// Returns the number of indexes in the collection.
    fn len(&self) -> usize;

    /// Returns whether the given index is contained in the store.
    fn contains(&self, index: Self::Index) -> bool;
}

/// A collection of indexes.
pub trait IndexCollection: IndexView {
    /// Returns the span of index values which MAY be inserted.
    ///
    /// Attempts to insert values outside this span WILL fail, possibly via panicking or aborting.
    fn span() -> (Bound<Self::Index>, Bound<Self::Index>);

    /// Constructs a new, empty, collection.
    ///
    /// Implementers should attempt to avoid any expensive operation, such as memory allocation, if possible.
    fn new() -> Self;

    /// Constructs a new collection, with the appropriate capacity for storing indexes within the given span.
    ///
    /// Implementers should attempt to pre-reserve the necessary space for the given span, if possible.
    fn with_span(range: (Bound<Self::Index>, Bound<Self::Index>)) -> Self;
}

/// A store of indexes.
///
/// #   Safety
///
/// -   NoPhantom: the store will only ever return indexes that have been inserted and have not been removed since.
pub unsafe trait IndexStore: IndexView {
    /// Error on insertion.
    type InsertionError: fmt::Debug;

    /// Removes all the indexes from the store.
    fn clear(&mut self);

    /// Inserts the index in the store, returns whether it is newly inserted.
    ///
    /// May return an error if the insertion fails, or _panic_ or _abort_. Check the implementation documentation.
    fn insert(&mut self, index: Self::Index) -> Result<bool, Self::InsertionError>;

    /// Removes the index from the store, returns whether it was in the store prior to removal.
    fn remove(&mut self, index: Self::Index) -> bool;
}

/// A trustworthy vault of indexes.
///
/// #   Safety
///
/// -   NoTheft: the vault SHALL never return that it does not contain an index if the index was inserted, and was not
///     removed since.
pub unsafe trait IndexVault: IndexView {}

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
pub unsafe trait IndexForward: IndexView {
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
    fn try_fold_after<B, F, R>(&self, mut current: Self::Index, mut accumulator: B, mut f: F) -> R
    where
        F: FnMut(B, Self::Index) -> R,
        R: Try<Output = B>,
    {
        accumulator = f(accumulator, current)?;

        loop {
            let Some(n) = self.next_after(current) else {
                return R::from_output(accumulator);
            };

            current = n;

            accumulator = f(accumulator, current)?;
        }
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
    fn try_fold_before<B, F, R>(&self, mut current: Self::Index, mut accumulator: B, mut f: F) -> R
    where
        F: FnMut(B, Self::Index) -> R,
        R: Try<Output = B>,
    {
        accumulator = f(accumulator, current)?;

        loop {
            let Some(n) = self.next_before(current) else {
                return R::from_output(accumulator);
            };

            current = n;

            accumulator = f(accumulator, current)?;
        }
    }
}

/// An _ordered_ view of the indexes in the store.
///
/// #   Safety
///
/// -   Ordered: the `IndexForward` implementation SHALL return indexes in strictly increasing order.
pub unsafe trait IndexOrdered: IndexForward {}

/// A chunked _view_ of the indexes in the store.
///
/// #   Safety
///
/// -   NoPhantom: the store SHALL only ever return that it contains an index if the index was inserted, and was not
///     removed since.
/// -   SplitFuse: `index` == `Self::fuse(Self::split(index))`.
/// -   TwoLevels: `self.contains(index)` if and only if
///     `self.get_chunk(Self::split(index).0).is_some_and(|c| c.contains(Self::split(index).1))`.
pub unsafe trait IndexViewChunked: IndexView {
    /// Index of the chunks.
    type ChunkIndex: Copy + Eq + Ord;

    /// Type of the chunk.
    type Chunk: IndexChunk;

    /// Fuses a tuple (chunk index, index-in-chunk) into a single index.
    ///
    /// Only defined if `outer`/`inner` refer to an index contained in an instance of `Self`.
    fn fuse(outer: Self::ChunkIndex, inner: <Self::Chunk as IndexView>::Index) -> Self::Index;

    /// Splits an index into a tuple (chunk index, index-in-chunk).
    ///
    /// Only defined if `index` refers to an index contained in an instance of `Self`.
    fn split(index: Self::Index) -> (Self::ChunkIndex, <Self::Chunk as IndexView>::Index);

    /// Returns the given chunk, if any.
    ///
    /// The chunk is _purposefully_ returned by value to allow implementations to materialize it on the fly.
    fn get_chunk(&self, index: Self::ChunkIndex) -> Option<Self::Chunk>;
}

/// An iterable _chunked_ view of the indexes in the store.
///
/// For backward iteration -- whatever that means -- see `IndexBackwardChunked`.
///
/// #   Safety
///
/// -   NoDuplicate: the view SHALL never return the same index a second time.
/// -   NoPhantom: the view SHALL only ever return that it contains an index if the index was inserted, and was not
///     removed since.
/// -   NoTheft: if `Self` implements `IndexVault`, the view SHALL return all indexes.
pub unsafe trait IndexForwardChunked: IndexViewChunked {
    /// Returns one index of a chunk.
    ///
    /// If `Self` implements `IndexVault`, no prior chunk SHALL contain any set index.
    fn first_chunk(&self) -> Option<Self::ChunkIndex>;

    /// Returns the next index after the provided one, if any.
    fn next_chunk_after(&self, current: Self::ChunkIndex) -> Option<Self::ChunkIndex>;
}

/// An iterable _chunked_ view of the indexes in the store.
///
/// For forward iteration -- whatever that means -- see `IndexForwardChunked`.
///
/// #   Safety
///
/// -   Reverse: the view SHALL return indexes in the exact opposite sequence than `IndexForwardChunked` does.
pub unsafe trait IndexBackwardChunked: IndexViewChunked {
    /// Returns one index of a chunk.
    ///
    /// If `Self` implements `IndexVault`, not later chunk SHALL contain any set index.
    fn last_chunk(&self) -> Option<Self::ChunkIndex>;

    /// Returns the next index before the provided one, if any.
    fn next_chunk_before(&self, index: Self::ChunkIndex) -> Option<Self::ChunkIndex>;
}

/// An _ordered_ view of the indexes in the store.
///
/// #   Safety
///
/// -   Ordered: the `IndexForwardChunked` implementation SHALL return indexes in strictly increasing order.
pub unsafe trait IndexOrderedChunked: IndexForwardChunked {}

#[cfg(test)]
mod tests {
    use core::ops::Bound;

    use alloc::collections::BTreeSet;

    use crate::Never;

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

    unsafe impl IndexView for Victim {
        type Index = usize;

        fn is_empty(&self) -> bool {
            self.0.is_empty()
        }

        fn len(&self) -> usize {
            self.0.len()
        }

        fn contains(&self, index: Self::Index) -> bool {
            self.0.contains(&index)
        }
    }

    impl IndexCollection for Victim {
        fn span() -> (Bound<Self::Index>, Bound<Self::Index>) {
            (Bound::Included(usize::MIN), Bound::Included(usize::MAX))
        }

        fn new() -> Self {
            Self::default()
        }

        fn with_span(_range: (Bound<Self::Index>, Bound<Self::Index>)) -> Self {
            Self::default()
        }
    }

    //  Safety:
    //  -   NoPhantom: the store WILL ever return that it contains an index if the index was inserted, and was not
    //      removed since.
    unsafe impl IndexStore for Victim {
        type InsertionError = Never;

        fn clear(&mut self) {
            self.0.clear();
        }

        fn insert(&mut self, index: Self::Index) -> Result<bool, Self::InsertionError> {
            Ok(self.0.insert(index))
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
