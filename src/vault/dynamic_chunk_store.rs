//! A dynamically-sized chunk-based store.

use core::{cmp, mem, ops::Bound};

#[cfg(feature = "nightly")]
use core::hint;

use crate::{
    Never,
    chunk::IndexChunk,
    index::{
        IndexBackward, IndexBackwardChunked, IndexCollection, IndexForward, IndexForwardChunked, IndexOrdered,
        IndexOrderedChunked, IndexStore, IndexStoreChunked, IndexVault, IndexView, IndexViewChunked,
    },
    not::{
        IndexBackwardChunkedNot, IndexBackwardNot, IndexForwardChunkedNot, IndexForwardNot, IndexOrderedChunkedNot,
        IndexOrderedNot, IndexViewNot,
    },
};

/// A dynamically-sized chunk-based store.
#[derive(Debug)]
pub struct DynamicChunkStore<C> {
    count: usize,
    chunks: Box<[C]>,
}

//  #   Safety
//
//  -   NoPhantom: the store will only ever return indexes that have been inserted and have not been removed since.
unsafe impl<C> IndexView for DynamicChunkStore<C>
where
    C: IndexChunk<Index = u16> + IndexView,
{
    //  Fixed-width, for easier conversions.
    type Index = u64;

    fn is_empty(&self) -> bool {
        self.count == 0
    }

    fn len(&self) -> usize {
        self.count
    }

    fn contains(&self, index: Self::Index) -> bool {
        let (outer, inner) = Self::split(index);

        self.chunks.get(outer).is_some_and(|c| c.contains(inner))
    }
}

//  Safety:
//
//  -   NoPhantom: the store will only ever return that it contains an index if the index was inserted, and was not
//      removed since.
unsafe impl<C> IndexViewNot for DynamicChunkStore<C>
where
    C: IndexChunk<Index = u16> + IndexViewNot,
{
    fn len_not(&self) -> usize {
        //  Well, it's unreachable in practice, for obvious reasons...

        usize::MAX - self.count
    }
}

impl<C> IndexCollection for DynamicChunkStore<C>
where
    C: IndexChunk<Index = u16> + IndexCollection,
{
    fn span() -> (Bound<Self::Index>, Bound<Self::Index>) {
        (Bound::Included(0), Bound::Unbounded)
    }

    fn new() -> Self {
        let chunks = Box::new([]);
        let count = 0;

        Self { count, chunks }
    }

    fn with_span(range: (Bound<Self::Index>, Bound<Self::Index>)) -> Self {
        let mut this = Self::new();

        let n = match range.1 {
            Bound::Included(n) => n,
            Bound::Excluded(0) => return this,
            Bound::Excluded(n) => n - 1,
            Bound::Unbounded => return this,
        };

        let (upto, _) = Self::split(n);

        this.reserve(upto + 1);

        this
    }
}

//  #   Safety
//
//  -   NoPhantom: the store will only ever return indexes that have been inserted and have not been removed since.
unsafe impl<C> IndexStore for DynamicChunkStore<C>
where
    C: IndexChunk<Index = u16> + IndexStore,
{
    type InsertionError = Never;

    fn clear(&mut self) {
        #[inline(never)]
        fn do_clear<C>(chunks: &mut [C])
        where
            C: IndexStore,
        {
            chunks.iter_mut().for_each(|c| c.clear());
        }

        if hint::likely(self.count == 0) {
            return;
        }

        self.count = 0;
        do_clear(&mut self.chunks);
    }

    fn insert(&mut self, index: Self::Index) -> Result<bool, Self::InsertionError> {
        let (outer, inner) = Self::split(index);

        if hint::unlikely(outer >= self.chunks.len()) {
            self.grow(outer + 1);
        }

        //  Safety:
        //  -   InBounds: `self.grow(outer + 1)` guarantees that `self.chunks.len() >= outer + 1`.
        let chunk = unsafe { self.chunks.get_unchecked_mut(outer) };

        //  C should never return Err for an in-bounds index, and `Self::split` ensure `inner` is in-bounds for C,
        //  hence there are only two cases to consider: Ok(true) & Ok(false).
        //
        //  Still, since Err(_) has the same semantics (no inserted) than Ok(false), might as well fold them together,
        //  just in case.
        let inserted = chunk.insert(inner).is_ok_and(|r| r);

        if inserted {
            self.count += 1;
        }

        Ok(inserted)
    }

    fn remove(&mut self, index: Self::Index) -> bool {
        let (outer, inner) = Self::split(index);

        //  If out-of-bounds, then there's nothing to remove.
        let Some(chunk) = self.chunks.get_mut(outer) else {
            return false;
        };

        let removed = chunk.remove(inner);

        if removed {
            self.count -= 1;
        }

        removed
    }
}

//  #   Safety
//
//  -   NoTheft: the vault will never return that it does not contain an index if the index was inserted, and was
//      not removed since.
unsafe impl<C> IndexVault for DynamicChunkStore<C> where C: IndexChunk<Index = u16> + IndexVault {}

//  #   Safety
//
//  -   NoDuplicate: the view SHALL never return the same index a second time.
//  -   NoPhantom: the view SHALL only ever return that it contains an index if the index was inserted, and was not
//      removed since.
//  -   NoTheft: if `Self` implements `IndexVault`, the view shall return all indexes.
unsafe impl<C> IndexForward for DynamicChunkStore<C>
where
    C: IndexChunk<Index = u16> + IndexForward,
{
    fn first(&self) -> Option<Self::Index> {
        let (outer, inner) = self
            .chunks
            .iter()
            .enumerate()
            .find_map(|(i, c)| c.first().map(|r| (i, r)))?;

        Some(Self::fuse(outer, inner))
    }

    fn next_after(&self, current: Self::Index) -> Option<Self::Index> {
        let (outer, inner) = Self::split(current);

        if let Some(inner) = self.chunks.get(outer).and_then(|chunk| chunk.next_after(inner)) {
            return Some(Self::fuse(outer, inner));
        }

        let (outer, inner) = self
            .chunks
            .iter()
            .enumerate()
            .skip(outer + 1)
            .find_map(|(i, c)| c.first().map(|r| (i, r)))?;

        Some(Self::fuse(outer, inner))
    }
}

//  Safety:
//
//  -   NoDuplicate: the view will never return the same index a second time.
//  -   NoPhantom: the view will only ever return that it contains an index if the index was inserted, and was not
//      removed since.
//  -   NoTheft: the view will return all indexes.
unsafe impl<C> IndexForwardNot for DynamicChunkStore<C>
where
    C: IndexChunk<Index = u16> + IndexForwardNot,
{
    fn first_not(&self) -> Option<Self::Index> {
        let (outer, inner) = self
            .chunks
            .iter()
            .enumerate()
            .find_map(|(i, c)| c.first_not().map(|r| (i, r)))?;

        Some(Self::fuse(outer, inner))
    }

    fn next_after_not(&self, current: Self::Index) -> Option<Self::Index> {
        let (outer, inner) = Self::split(current);

        if let Some(inner) = self.chunks.get(outer).and_then(|chunk| chunk.next_after_not(inner)) {
            return Some(Self::fuse(outer, inner));
        }

        let (outer, inner) = self
            .chunks
            .iter()
            .enumerate()
            .skip(outer + 1)
            .find_map(|(i, c)| c.first_not().map(|r| (i, r)))?;

        Some(Self::fuse(outer, inner))
    }
}

//  #   Safety
//
//  -   Reverse: the view WILL return indexes in the exact opposite sequence than `IndexForward` does.
unsafe impl<C> IndexBackward for DynamicChunkStore<C>
where
    C: IndexChunk<Index = u16> + IndexBackward,
{
    fn last(&self) -> Option<Self::Index> {
        let (outer, inner) = self
            .chunks
            .iter()
            .enumerate()
            .rev()
            .find_map(|(i, c)| c.last().map(|r| (i, r)))?;

        Some(Self::fuse(outer, inner))
    }

    fn next_before(&self, current: Self::Index) -> Option<Self::Index> {
        let (outer, inner) = Self::split(current);

        if let Some(inner) = self.chunks.get(outer).and_then(|chunk| chunk.next_before(inner)) {
            return Some(Self::fuse(outer, inner));
        }

        let limit = outer.min(self.chunks.len());

        let (outer, inner) = self
            .chunks
            .get(..limit)?
            .iter()
            .enumerate()
            .rev()
            .find_map(|(i, c)| c.last().map(|r| (i, r)))?;

        Some(Self::fuse(outer, inner))
    }
}

//  Safety:
//
//  -   Reverse: the view will return indexes in the exact opposite sequence than `IndexForward` does.
unsafe impl<C> IndexBackwardNot for DynamicChunkStore<C>
where
    C: IndexChunk<Index = u16> + IndexBackwardNot,
{
    fn last_not(&self) -> Option<Self::Index> {
        let (outer, inner) = self
            .chunks
            .iter()
            .enumerate()
            .rev()
            .find_map(|(i, c)| c.last_not().map(|r| (i, r)))?;

        Some(Self::fuse(outer, inner))
    }

    fn next_before_not(&self, current: Self::Index) -> Option<Self::Index> {
        let (outer, inner) = Self::split(current);

        if let Some(inner) = self.chunks.get(outer).and_then(|chunk| chunk.next_before_not(inner)) {
            return Some(Self::fuse(outer, inner));
        }

        let limit = outer.min(self.chunks.len());

        let (outer, inner) = self
            .chunks
            .get(..limit)?
            .iter()
            .enumerate()
            .rev()
            .find_map(|(i, c)| c.last_not().map(|r| (i, r)))?;

        Some(Self::fuse(outer, inner))
    }
}

//  Safety:
//
//  -   Ordered: the `IndexForward` implementation will return indexes in strictly increasing order.
unsafe impl<C> IndexOrdered for DynamicChunkStore<C> where C: IndexChunk<Index = u16> + IndexOrdered {}

//  Safety:
//
//  -   Ordered: the `IndexForward` implementation will return indexes in strictly increasing order.
unsafe impl<C> IndexOrderedNot for DynamicChunkStore<C> where C: IndexChunk<Index = u16> + IndexForwardNot + IndexOrdered
{}

//  Safety:
//
//  -   NoPhantom: the view will only ever return that it contains an index if the index was inserted, and was not
//      removed since.
//  -   SplitFuse: `split` and `fuse` are one another inverse.
//  -   TwoLevels: `split` and `fuse` are consistent with `IndexView`.
unsafe impl<C> IndexViewChunked for DynamicChunkStore<C>
where
    C: IndexChunk<Index = u16>,
{
    type ChunkIndex = usize;
    type Chunk = C;

    fn fuse(outer: Self::ChunkIndex, inner: C::Index) -> Self::Index {
        const {
            assert!(core::mem::size_of::<usize>() <= core::mem::size_of::<u64>());
        };

        let bits: u64 = C::BITS.into();

        let outer = outer as u64;
        let inner: u64 = inner.into();

        outer * bits + inner
    }

    fn split(index: Self::Index) -> (Self::ChunkIndex, C::Index) {
        const {
            assert!(C::BITS <= (u16::MAX as u32 + 1));
        };

        let bits: u64 = C::BITS.into();

        let (outer, inner) = (index / bits, index % bits);

        (outer as usize, inner as u16)
    }

    fn get_chunk(&self, index: Self::ChunkIndex) -> Option<Self::Chunk> {
        self.chunks.get(index).copied()
    }
}

//  #   Safety
//
//  -   NoPhantom: the store will only ever return indexes that have been inserted and have not been removed since.
unsafe impl<C> IndexStoreChunked for DynamicChunkStore<C>
where
    C: IndexChunk<Index = u16> + IndexView,
{
    type SetError = Never;

    fn set_chunk(&mut self, index: Self::ChunkIndex, chunk: Self::Chunk) -> Result<(), Self::SetError> {
        if hint::unlikely(index >= self.chunks.len()) {
            self.grow(index + 1);
        }

        //  Safety:
        //  -   InBounds: `self.grow(index + 1)` guarantees that `self.chunks.len() >= index + 1`.
        let current = unsafe { self.chunks.get_unchecked_mut(index) };

        let before = current.len();
        let after = chunk.len();

        *current = chunk;

        self.count -= before;
        self.count += after;

        Ok(())
    }
}

//  #   Safety
//
//  -   NoDuplicate: the view will never return the same index a second time.
//  -   NoPhantom: the view will only ever return that it contains an index if the index was inserted, and was not
//      removed since.
//  -   NoTheft: the view will return all indexes.
unsafe impl<C> IndexForwardChunked for DynamicChunkStore<C>
where
    C: IndexChunk<Index = u16>,
{
    fn first_chunk(&self) -> Option<Self::ChunkIndex> {
        (!self.chunks.is_empty()).then_some(0)
    }

    fn next_chunk_after(&self, current: Self::ChunkIndex) -> Option<Self::ChunkIndex> {
        (current + 1 < self.chunks.len()).then_some(current + 1)
    }
}

//  #   Safety
//
//  -   NoDuplicate: the view will never return the same index a second time.
//  -   NoPhantom: the view will only ever return that it contains an index if the index was inserted, and was not
//      removed since.
//  -   NoTheft: the view will return all indexes.
unsafe impl<C> IndexForwardChunkedNot for DynamicChunkStore<C>
where
    C: IndexChunk<Index = u16> + IndexViewNot,
{
    #[inline(always)]
    fn first_chunk_not(&self) -> Option<Self::ChunkIndex> {
        self.first_chunk()
    }

    #[inline(always)]
    fn next_chunk_after_not(&self, current: Self::ChunkIndex) -> Option<Self::ChunkIndex> {
        self.next_chunk_after(current)
    }
}

//  Safety:
//
//  -   Reverse: the view will return indexes in the exact opposite sequence than `IndexForwardChunked` does.
unsafe impl<C> IndexBackwardChunked for DynamicChunkStore<C>
where
    C: IndexChunk<Index = u16>,
{
    fn last_chunk(&self) -> Option<Self::ChunkIndex> {
        self.chunks.len().checked_sub(1)
    }

    fn next_chunk_before(&self, current: Self::ChunkIndex) -> Option<Self::ChunkIndex> {
        current.checked_sub(1)
    }
}

//  Safety:
//
//  -   Reverse: the view will return indexes in the exact opposite sequence than `IndexForwardChunked` does.
unsafe impl<C> IndexBackwardChunkedNot for DynamicChunkStore<C>
where
    C: IndexChunk<Index = u16> + IndexViewNot,
{
    #[inline(always)]
    fn last_chunk_not(&self) -> Option<Self::ChunkIndex> {
        self.last_chunk()
    }

    #[inline(always)]
    fn next_chunk_before_not(&self, current: Self::ChunkIndex) -> Option<Self::ChunkIndex> {
        self.next_chunk_before(current)
    }
}

//  #   Safety
//
//  -   Ordered: the view will return indexes in strictly increasing order.
unsafe impl<C> IndexOrderedChunked for DynamicChunkStore<C> where C: IndexChunk<Index = u16> {}

//  #   Safety
//
//  -   Ordered: the view will return indexes in strictly increasing order.
unsafe impl<C> IndexOrderedChunkedNot for DynamicChunkStore<C> where C: IndexChunk<Index = u16> + IndexViewNot {}

//
//  Implementation (memory)
//

impl<C> DynamicChunkStore<C>
where
    C: IndexChunk,
{
    //  #   Safety
    //
    //  -   Growth: after execution, `self.chunks.len() >= minimal`.
    #[inline(never)]
    fn grow(&mut self, minimal: usize) {
        debug_assert!(minimal > self.chunks.len(), "{minimal} <= {}", self.chunks.len());

        let target = cmp::max(self.chunks.len() * 2, minimal);

        let additional = target - self.chunks.len();

        self.reserve(additional);
    }

    #[inline(never)]
    fn reserve(&mut self, additional: usize) {
        let chunks = mem::replace(&mut self.chunks, Box::new([]));

        let mut chunks: Vec<_> = chunks.into();

        chunks.reserve(additional);

        let capacity = chunks.capacity();

        chunks.resize(capacity, C::default());

        self.chunks = chunks.into_boxed_slice();
    }
}

#[cfg(not(feature = "nightly"))]
mod hint {
    #[inline(always)]
    pub(super) fn likely(b: bool) -> bool {
        b
    }

    #[inline(always)]
    pub(super) fn unlikely(b: bool) -> bool {
        b
    }
} // mod hint
