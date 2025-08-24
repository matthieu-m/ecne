//! Adapter over an IndexView.

use core::{num::NonZeroUsize, ops::Not};

use crate::index::{
    IndexBackward, IndexBackwardChunked, IndexForward, IndexForwardChunked, IndexOrdered, IndexOrderedChunked,
    IndexVault, IndexView, IndexViewChunked,
};

#[cfg(feature = "nightly")]
use core::ops::Try;

/// Adapts an `IndexView` so as to negate it.
///
/// Returns not-contained for elements in the view, and contained for elements not in the view.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct NotView<S>(S);

//
//  Construction
//

impl<S> NotView<S> {
    /// Creates a new instance.
    #[inline(always)]
    pub fn new(view: S) -> Self {
        Self(view)
    }
}

//
//  Deconstruction
//

impl<S> NotView<S> {
    /// Returns a reference to the view.
    #[inline(always)]
    pub fn as_view(&self) -> &S {
        &self.0
    }

    /// Returns a mutable reference to the view.
    #[inline(always)]
    pub fn as_view_mut(&mut self) -> &mut S {
        &mut self.0
    }

    /// Returns the view.
    #[inline(always)]
    pub fn into_view(self) -> S {
        self.0
    }
}

//
//  Traits required.
//

/// A negated view of indexes.
///
/// #   Safety
///
/// -   NoPhantom: the view SHALL only ever return that it contains an index if the index was inserted, and was not
///     removed since.
pub unsafe trait IndexViewNot: IndexView {
    /// Returns the number of indexes NOT contained in the view.
    ///
    /// Note: not only does this require a _finite_ number of indexes, it also requires a number of indexes which can
    /// necessarily be represented by `usize` (or a panic/abort).
    fn len_not(&self) -> usize;
}

/// A negated iterable view of the indexes.
///
/// For backward iteration -- whatever that means -- see `IndexBackwardNot`.
///
/// #   Safety
///
/// -   NoDuplicate: the view SHALL never return the same index a second time.
/// -   NoPhantom: the view SHALL only ever return that it contains an index if the index was inserted, and was not
///     removed since.
/// -   NoTheft: if `Self` implements `IndexVault`, the view SHALL return all indexes.
pub unsafe trait IndexForwardNot: IndexForward + IndexViewNot {
    /// Returns one index NOT contained by the view, if any.
    fn first_not(&self) -> Option<Self::Index>;

    /// Returns the next index after `current` NOT contained by the view, if any.
    fn next_after_not(&self, current: Self::Index) -> Option<Self::Index>;

    /// Returns the n-th index after `current` NOT contained by the view, or the remainder of `n`.
    ///
    /// #   Note to Implementors
    ///
    /// Try to implement this method if it can be implemented to skip ahead, rather than advance one at a time.
    fn nth_after_not(&self, n: usize, mut current: Self::Index) -> Result<Self::Index, NonZeroUsize> {
        for i in 0..n {
            //  Safety:
            //  -   NonZero: i < n.
            let remainder = unsafe { NonZeroUsize::new_unchecked(n - i + 1) };

            current = self.next_after_not(current).ok_or(remainder)?;
        }

        self.next_after_not(current).ok_or(NonZeroUsize::MIN)
    }

    /// Applies the function `f` as long as it returns successfully, producing a single, final value.
    ///
    /// #   Note to Implementors
    ///
    /// Try to implement this method if internal iteration can be optimized.
    #[cfg(feature = "nightly")]
    fn try_fold_after_not<B, F, R>(&self, mut current: Self::Index, mut accumulator: B, mut f: F) -> R
    where
        F: FnMut(B, Self::Index) -> R,
        R: Try<Output = B>,
    {
        loop {
            let Some(n) = self.next_after_not(current) else {
                return R::from_output(accumulator);
            };

            current = n;

            accumulator = f(accumulator, current)?;
        }
    }
}

/// A negated iterable view of the indexes in the store.
///
/// For forward iteration -- whatever that means -- see `IndexForwardNot`.
///
/// #   Safety
///
/// -   Reverse: the view SHALL return indexes in the exact opposite sequence than `IndexForwardNot` does.
pub unsafe trait IndexBackwardNot: IndexBackward + IndexViewNot {
    /// Returns one index NOT contained by the view, if any.
    fn last_not(&self) -> Option<Self::Index>;

    /// Returns the next index before `current` that is NOT contained by the view, if any.
    fn next_before_not(&self, current: Self::Index) -> Option<Self::Index>;

    /// Returns the n-th index before `current` that is NOT contained by the view, or the remainder of `n`.
    ///
    /// #   Note to Implementors
    ///
    /// Try to implement this method if it can be implemented to skip ahead, rather than advance one at a time.
    fn nth_before_not(&self, n: usize, mut current: Self::Index) -> Result<Self::Index, NonZeroUsize> {
        for i in 0..n {
            //  Safety:
            //  -   NonZero: i < n.
            let remainder = unsafe { NonZeroUsize::new_unchecked(n - i + 1) };

            current = self.next_before_not(current).ok_or(remainder)?;
        }

        self.next_before_not(current).ok_or(NonZeroUsize::MIN)
    }

    /// Applies the function `f` as long as it returns successfully, producing a single, final value.
    ///
    /// #   Note to Implementors
    ///
    /// Try to implement this method if internal iteration can be optimized.
    #[cfg(feature = "nightly")]
    fn try_fold_before_not<B, F, R>(&self, mut current: Self::Index, mut accumulator: B, mut f: F) -> R
    where
        F: FnMut(B, Self::Index) -> R,
        R: Try<Output = B>,
    {
        loop {
            let Some(n) = self.next_before_not(current) else {
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
/// -   Ordered: the `IndexForwardNot` implementation SHALL return indexes in strictly increasing order.
pub unsafe trait IndexOrderedNot: IndexForwardNot + IndexOrdered {}

/// A negated iterable _chunked_ view of the indexes in the store.
///
/// For backward iteration -- whatever that means -- see `IndexBackwardChunkedNot`.
///
/// #   Safety
///
/// -   NoDuplicate: the view SHALL never return the same index a second time.
/// -   NoPhantom: the view SHALL only ever return that it contains an index if the index was inserted, and was not
///     removed since.
/// -   NoTheft: if `Self` implements `IndexVault`, the view SHALL return all indexes.
pub unsafe trait IndexForwardChunkedNot: IndexForwardChunked + IndexViewNot {
    /// Returns one index of a chunk.
    ///
    /// If `Self` implements `IndexVault`, no prior chunk SHALL contain any NON-set index.
    fn first_chunk_not(&self) -> Option<Self::ChunkIndex>;

    /// Returns the next index `current`, if any.
    fn next_chunk_after_not(&self, current: Self::ChunkIndex) -> Option<Self::ChunkIndex> {
        self.next_chunk_after(current)
    }
}

/// A negated iterable _chunked_ view of the indexes in the store.
///
/// For forward iteration -- whatever that means -- see `IndexForwardChunkedNot`.
///
/// #   Safety
///
/// -   Reverse: the view SHALL return indexes in the exact opposite sequence than `IndexForwardChunkedNot` does.
pub unsafe trait IndexBackwardChunkedNot: IndexBackwardChunked + IndexViewNot {
    /// Returns one index of a chunk.
    ///
    /// If `Self` implements `IndexVault`, not later chunk SHALL contain any NON-set index.
    fn last_chunk_not(&self) -> Option<Self::ChunkIndex>;

    /// Returns the next index `current`, if any.
    fn next_chunk_before_not(&self, index: Self::ChunkIndex) -> Option<Self::ChunkIndex> {
        self.next_chunk_before(index)
    }
}

/// An _ordered_ view of the indexes in the store.
///
/// #   Safety
///
/// -   Ordered: the `IndexForwardChunkedNot` implementation SHALL return indexes in strictly increasing order.
pub unsafe trait IndexOrderedChunkedNot: IndexForwardChunkedNot + IndexOrderedChunked {}

//
//  Index trait implementations
//

//  Safety:
//  -   NoPhantom: inherited.
unsafe impl<S> IndexView for NotView<S>
where
    S: IndexViewNot,
{
    type Index = S::Index;

    #[inline(always)]
    fn is_empty(&self) -> bool {
        !self.0.is_empty()
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.0.len_not()
    }

    #[inline(always)]
    fn contains(&self, index: Self::Index) -> bool {
        !self.0.contains(index)
    }
}

//  Safety:
//  -   NoTheft: inherited.
unsafe impl<S> IndexVault for NotView<S> where S: IndexVault + IndexViewNot {}

//  Safety:
//  -   NoDuplicate: inherited.
//  -   NoPhantom: inherited.
//  -   NoTheft: inherited.
unsafe impl<S> IndexForward for NotView<S>
where
    S: IndexForwardNot,
{
    #[inline(always)]
    fn first(&self) -> Option<Self::Index> {
        self.0.first_not()
    }

    #[inline(always)]
    fn next_after(&self, current: Self::Index) -> Option<Self::Index> {
        self.0.next_after_not(current)
    }

    #[inline(always)]
    fn nth_after(&self, n: usize, current: Self::Index) -> Result<Self::Index, NonZeroUsize> {
        self.0.nth_after_not(n, current)
    }

    #[cfg(feature = "nightly")]
    #[inline(always)]
    fn try_fold_after<B, F, R>(&self, current: Self::Index, accumulator: B, f: F) -> R
    where
        F: FnMut(B, Self::Index) -> R,
        R: Try<Output = B>,
    {
        self.0.try_fold_after_not(current, accumulator, f)
    }
}

//  #   Safety
//
//  -   Reverse: inherited.
unsafe impl<S> IndexBackward for NotView<S>
where
    S: IndexBackwardNot + IndexForwardNot,
{
    #[inline(always)]
    fn last(&self) -> Option<Self::Index> {
        self.0.last_not()
    }

    #[inline(always)]
    fn next_before(&self, current: Self::Index) -> Option<Self::Index> {
        self.0.next_before_not(current)
    }

    #[inline(always)]
    fn nth_before(&self, n: usize, current: Self::Index) -> Result<Self::Index, NonZeroUsize> {
        self.0.nth_before_not(n, current)
    }

    #[cfg(feature = "nightly")]
    #[inline(always)]
    fn try_fold_before<B, F, R>(&self, current: Self::Index, accumulator: B, f: F) -> R
    where
        F: FnMut(B, Self::Index) -> R,
        R: Try<Output = B>,
    {
        self.0.try_fold_before_not(current, accumulator, f)
    }
}

//  #   Safety
//
//  -   Ordered: inherited.
unsafe impl<S> IndexOrdered for NotView<S> where S: IndexOrderedNot {}

//  #   Safety
//
//  -   NoPhantom: inherited.
//  -   SplitFuse: inherited`.
//  -   TwoLevels: inherited.
unsafe impl<S> IndexViewChunked for NotView<S>
where
    S: IndexViewChunked + IndexViewNot,
{
    type ChunkIndex = S::ChunkIndex;
    type Chunk = S::Chunk;

    #[inline(always)]
    fn fuse(outer: Self::ChunkIndex, inner: <Self::Chunk as IndexView>::Index) -> Self::Index {
        S::fuse(outer, inner)
    }

    #[inline(always)]
    fn split(index: Self::Index) -> (Self::ChunkIndex, <Self::Chunk as IndexView>::Index) {
        S::split(index)
    }

    #[inline(always)]
    fn get_chunk(&self, index: Self::ChunkIndex) -> Option<Self::Chunk> {
        self.0.get_chunk(index).map(|c| c.not())
    }
}

//  #   Safety
//
//  -   NoDuplicate: inherited.
//  -   NoPhantom: inherited.
//  -   NoTheft: inherited.
unsafe impl<S> IndexForwardChunked for NotView<S>
where
    S: IndexForwardChunkedNot,
{
    fn first_chunk(&self) -> Option<Self::ChunkIndex> {
        self.0.first_chunk_not()
    }

    fn next_chunk_after(&self, current: Self::ChunkIndex) -> Option<Self::ChunkIndex> {
        self.0.next_chunk_after_not(current)
    }
}

//  #   Safety
//
//  -   Reverse: inherited.
unsafe impl<S> IndexBackwardChunked for NotView<S>
where
    S: IndexBackwardChunkedNot,
{
    fn last_chunk(&self) -> Option<Self::ChunkIndex> {
        self.0.last_chunk_not()
    }

    fn next_chunk_before(&self, index: Self::ChunkIndex) -> Option<Self::ChunkIndex> {
        self.0.next_chunk_before_not(index)
    }
}

//  #   Safety
//
//  -   Ordered: inherited.
unsafe impl<S> IndexOrderedChunked for NotView<S> where S: IndexOrderedChunkedNot {}
