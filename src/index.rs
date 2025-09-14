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

/// A store of indexes.
///
/// #   Safety
///
/// -   NoPhantom: the store will only ever return indexes that have been inserted and have not been removed since.
pub unsafe trait IndexStoreChunked: IndexViewChunked {
    /// Error on `set_chunk`.
    type SetError: fmt::Debug;

    /// Replaces the chunk at the given index.
    ///
    /// Returns an error if the chunk could not be set.
    ///
    /// Implementers are encouraged to make the operation atomic.
    fn set_chunk(&mut self, index: Self::ChunkIndex, chunk: Self::Chunk) -> Result<(), Self::SetError>;
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

//
//  Implementations for references.
//

//  #   Safety
//
//  -   As per T.
unsafe impl<T> IndexView for &T
where
    T: IndexView,
{
    type Index = T::Index;

    #[inline(always)]
    fn is_empty(&self) -> bool {
        (**self).is_empty()
    }

    #[inline(always)]
    fn len(&self) -> usize {
        (**self).len()
    }

    #[inline(always)]
    fn contains(&self, index: Self::Index) -> bool {
        (**self).contains(index)
    }
}

//  #   Safety
//
//  -   As per T.
unsafe impl<T> IndexView for &mut T
where
    T: IndexView,
{
    type Index = T::Index;

    #[inline(always)]
    fn is_empty(&self) -> bool {
        (**self).is_empty()
    }

    #[inline(always)]
    fn len(&self) -> usize {
        (**self).len()
    }

    #[inline(always)]
    fn contains(&self, index: Self::Index) -> bool {
        (**self).contains(index)
    }
}

//  #   Safety
//
//  -   As per T.
unsafe impl<T> IndexStore for &mut T
where
    T: IndexStore,
{
    type InsertionError = T::InsertionError;

    #[inline(always)]
    fn clear(&mut self) {
        (**self).clear();
    }

    #[inline(always)]
    fn insert(&mut self, index: Self::Index) -> Result<bool, Self::InsertionError> {
        (**self).insert(index)
    }

    #[inline(always)]
    fn remove(&mut self, index: Self::Index) -> bool {
        (**self).remove(index)
    }
}

//  #   Safety
//
//  -   As per T.
unsafe impl<T> IndexVault for &T where T: IndexVault {}

//  #   Safety
//
//  -   As per T.
unsafe impl<T> IndexVault for &mut T where T: IndexVault {}

//  #   Safety
//
//  -   As per T.
unsafe impl<T> IndexForward for &T
where
    T: IndexForward,
{
    #[inline(always)]
    fn first(&self) -> Option<Self::Index> {
        (**self).first()
    }

    #[inline(always)]
    fn next_after(&self, current: Self::Index) -> Option<Self::Index> {
        (**self).next_after(current)
    }

    #[inline(always)]
    fn nth_after(&self, n: usize, current: Self::Index) -> Result<Self::Index, NonZeroUsize> {
        (**self).nth_after(n, current)
    }

    #[cfg(feature = "nightly")]
    #[inline(always)]
    fn try_fold_after<B, F, R>(&self, current: Self::Index, accumulator: B, f: F) -> R
    where
        F: FnMut(B, Self::Index) -> R,
        R: Try<Output = B>,
    {
        (**self).try_fold_after(current, accumulator, f)
    }
}

//  #   Safety
//
//  -   As per T.
unsafe impl<T> IndexForward for &mut T
where
    T: IndexForward,
{
    #[inline(always)]
    fn first(&self) -> Option<Self::Index> {
        (**self).first()
    }

    #[inline(always)]
    fn next_after(&self, current: Self::Index) -> Option<Self::Index> {
        (**self).next_after(current)
    }

    #[inline(always)]
    fn nth_after(&self, n: usize, current: Self::Index) -> Result<Self::Index, NonZeroUsize> {
        (**self).nth_after(n, current)
    }

    #[cfg(feature = "nightly")]
    #[inline(always)]
    fn try_fold_after<B, F, R>(&self, current: Self::Index, accumulator: B, f: F) -> R
    where
        F: FnMut(B, Self::Index) -> R,
        R: Try<Output = B>,
    {
        (**self).try_fold_after(current, accumulator, f)
    }
}

//  #   Safety
//
//  -   As per T.
unsafe impl<T> IndexBackward for &T
where
    T: IndexBackward,
{
    #[inline(always)]
    fn last(&self) -> Option<Self::Index> {
        (**self).last()
    }

    #[inline(always)]
    fn next_before(&self, current: Self::Index) -> Option<Self::Index> {
        (**self).next_before(current)
    }

    #[inline(always)]
    fn nth_before(&self, n: usize, current: Self::Index) -> Result<Self::Index, NonZeroUsize> {
        (**self).nth_before(n, current)
    }

    #[cfg(feature = "nightly")]
    #[inline(always)]
    fn try_fold_before<B, F, R>(&self, current: Self::Index, accumulator: B, f: F) -> R
    where
        F: FnMut(B, Self::Index) -> R,
        R: Try<Output = B>,
    {
        (**self).try_fold_before(current, accumulator, f)
    }
}

//  #   Safety
//
//  -   As per T.
unsafe impl<T> IndexBackward for &mut T
where
    T: IndexBackward,
{
    #[inline(always)]
    fn last(&self) -> Option<Self::Index> {
        (**self).last()
    }

    #[inline(always)]
    fn next_before(&self, current: Self::Index) -> Option<Self::Index> {
        (**self).next_before(current)
    }

    #[inline(always)]
    fn nth_before(&self, n: usize, current: Self::Index) -> Result<Self::Index, NonZeroUsize> {
        (**self).nth_before(n, current)
    }

    #[cfg(feature = "nightly")]
    #[inline(always)]
    fn try_fold_before<B, F, R>(&self, current: Self::Index, accumulator: B, f: F) -> R
    where
        F: FnMut(B, Self::Index) -> R,
        R: Try<Output = B>,
    {
        (**self).try_fold_before(current, accumulator, f)
    }
}

//  #   Safety
//
//  -   As per T.
unsafe impl<T> IndexOrdered for &T where T: IndexOrdered {}

//  #   Safety
//
//  -   As per T.
unsafe impl<T> IndexOrdered for &mut T where T: IndexOrdered {}

//  #   Safety
//
//  -   As per T.
unsafe impl<T> IndexViewChunked for &T
where
    T: IndexViewChunked,
{
    type ChunkIndex = T::ChunkIndex;

    type Chunk = T::Chunk;

    #[inline(always)]
    fn fuse(outer: Self::ChunkIndex, inner: <Self::Chunk as IndexView>::Index) -> Self::Index {
        T::fuse(outer, inner)
    }

    #[inline(always)]
    fn split(index: Self::Index) -> (Self::ChunkIndex, <Self::Chunk as IndexView>::Index) {
        T::split(index)
    }

    #[inline(always)]
    fn get_chunk(&self, index: Self::ChunkIndex) -> Option<Self::Chunk> {
        (**self).get_chunk(index)
    }
}

//  #   Safety
//
//  -   As per T.
unsafe impl<T> IndexViewChunked for &mut T
where
    T: IndexViewChunked,
{
    type ChunkIndex = T::ChunkIndex;

    type Chunk = T::Chunk;

    #[inline(always)]
    fn fuse(outer: Self::ChunkIndex, inner: <Self::Chunk as IndexView>::Index) -> Self::Index {
        T::fuse(outer, inner)
    }

    #[inline(always)]
    fn split(index: Self::Index) -> (Self::ChunkIndex, <Self::Chunk as IndexView>::Index) {
        T::split(index)
    }

    #[inline(always)]
    fn get_chunk(&self, index: Self::ChunkIndex) -> Option<Self::Chunk> {
        (**self).get_chunk(index)
    }
}

//  #   Safety
//
//  -   As per T.
unsafe impl<T> IndexStoreChunked for &mut T
where
    T: IndexStoreChunked,
{
    type SetError = T::SetError;

    #[inline(always)]
    fn set_chunk(&mut self, index: Self::ChunkIndex, chunk: Self::Chunk) -> Result<(), Self::SetError> {
        (**self).set_chunk(index, chunk)
    }
}

//  #   Safety
//
//  -   As per T.
unsafe impl<T> IndexForwardChunked for &T
where
    T: IndexForwardChunked,
{
    #[inline(always)]
    fn first_chunk(&self) -> Option<Self::ChunkIndex> {
        (**self).first_chunk()
    }

    #[inline(always)]
    fn next_chunk_after(&self, current: Self::ChunkIndex) -> Option<Self::ChunkIndex> {
        (**self).next_chunk_after(current)
    }
}

//  #   Safety
//
//  -   As per T.
unsafe impl<T> IndexForwardChunked for &mut T
where
    T: IndexForwardChunked,
{
    #[inline(always)]
    fn first_chunk(&self) -> Option<Self::ChunkIndex> {
        (**self).first_chunk()
    }

    #[inline(always)]
    fn next_chunk_after(&self, current: Self::ChunkIndex) -> Option<Self::ChunkIndex> {
        (**self).next_chunk_after(current)
    }
}

//  #   Safety
//
//  -   As per T.
unsafe impl<T> IndexBackwardChunked for &T
where
    T: IndexBackwardChunked,
{
    #[inline(always)]
    fn last_chunk(&self) -> Option<Self::ChunkIndex> {
        (**self).last_chunk()
    }

    #[inline(always)]
    fn next_chunk_before(&self, index: Self::ChunkIndex) -> Option<Self::ChunkIndex> {
        (**self).next_chunk_before(index)
    }
}

//  #   Safety
//
//  -   As per T.
unsafe impl<T> IndexBackwardChunked for &mut T
where
    T: IndexBackwardChunked,
{
    #[inline(always)]
    fn last_chunk(&self) -> Option<Self::ChunkIndex> {
        (**self).last_chunk()
    }

    #[inline(always)]
    fn next_chunk_before(&self, index: Self::ChunkIndex) -> Option<Self::ChunkIndex> {
        (**self).next_chunk_before(index)
    }
}

//  #   Safety
//
//  -   As per T.
unsafe impl<T> IndexOrderedChunked for &T where T: IndexOrderedChunked {}

//  #   Safety
//
//  -   As per T.
unsafe impl<T> IndexOrderedChunked for &mut T where T: IndexOrderedChunked {}

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
