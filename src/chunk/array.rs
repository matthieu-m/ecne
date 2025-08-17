//! Array chunk.

use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Bound, Not, Sub, SubAssign};

use crate::{
    Never,
    chunk::IndexChunk,
    index::{
        IndexBackward, IndexBackwardChunked, IndexCollection, IndexForward, IndexForwardChunked, IndexOrdered,
        IndexOrderedChunked, IndexStore, IndexStoreChunked, IndexVault, IndexView, IndexViewChunked,
    },
};

/// Simple implementation of `IndexChunk` for arrays of chunks.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ArrayChunk<C, const N: usize>(pub [C; N]);

impl<C, const N: usize> ArrayChunk<C, N>
where
    C: IndexChunk,
{
    /// Creates a new, empty, instance.
    pub fn new() -> Self {
        Self([C::new(); N])
    }
}

impl<C, const N: usize> ArrayChunk<C, N>
where
    C: IndexChunk,
{
    fn apply<F>(&mut self, other: Self, fun: F)
    where
        F: Fn(&mut C, C),
    {
        self.0
            .iter_mut()
            .zip(other.0)
            .for_each(|(this, other)| fun(this, other));
    }

    fn map<F>(self, fun: F) -> Self
    where
        F: Fn(C) -> C,
    {
        let mut result = [C::new(); N];

        result.iter_mut().zip(self.0).for_each(|(r, s)| *r = fun(s));

        Self(result)
    }

    fn map_with<F>(self, other: Self, fun: F) -> Self
    where
        F: Fn(C, C) -> C,
    {
        let mut result = [C::new(); N];

        result
            .iter_mut()
            .zip(self.0)
            .zip(other.0)
            .for_each(|((r, s), o)| *r = fun(s, o));

        Self(result)
    }
}

impl<C, const N: usize> Default for ArrayChunk<C, N>
where
    C: IndexChunk,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<C, const N: usize> BitAnd for ArrayChunk<C, N>
where
    C: IndexChunk,
{
    type Output = Self;

    fn bitand(self, other: Self) -> Self {
        self.map_with(other, |this, other| this & other)
    }
}

impl<C, const N: usize> BitAndAssign for ArrayChunk<C, N>
where
    C: IndexChunk,
{
    fn bitand_assign(&mut self, other: Self) {
        self.apply(other, |this, other| *this &= other);
    }
}

impl<C, const N: usize> BitOr for ArrayChunk<C, N>
where
    C: IndexChunk,
{
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        self.map_with(other, |this, other| this | other)
    }
}

impl<C, const N: usize> BitOrAssign for ArrayChunk<C, N>
where
    C: IndexChunk,
{
    fn bitor_assign(&mut self, other: Self) {
        self.apply(other, |this, other| *this |= other);
    }
}

impl<C, const N: usize> BitXor for ArrayChunk<C, N>
where
    C: IndexChunk,
{
    type Output = Self;

    fn bitxor(self, other: Self) -> Self {
        self.map_with(other, |this, other| this ^ other)
    }
}

impl<C, const N: usize> BitXorAssign for ArrayChunk<C, N>
where
    C: IndexChunk,
{
    fn bitxor_assign(&mut self, other: Self) {
        self.apply(other, |this, other| *this ^= other);
    }
}

impl<C, const N: usize> Not for ArrayChunk<C, N>
where
    C: IndexChunk,
{
    type Output = Self;

    fn not(self) -> Self::Output {
        self.map(|this| !this)
    }
}

impl<C, const N: usize> Sub for ArrayChunk<C, N>
where
    C: IndexChunk,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self.map_with(other, |this, other| this & !other)
    }
}

impl<C, const N: usize> SubAssign for ArrayChunk<C, N>
where
    C: IndexChunk,
{
    fn sub_assign(&mut self, other: Self) {
        self.apply(other, |this, other| *this &= !other);
    }
}

impl<C, const N: usize> IndexChunk for ArrayChunk<C, N>
where
    C: IndexChunk<Index = u8>,
{
    const BITS: u32 = C::BITS * (N as u32);
}

//  Safety:
//
//  -   NoPhantom: the store will only ever return that it contains an index if the index was inserted, and was not
//      removed since.
unsafe impl<C, const N: usize> IndexView for ArrayChunk<C, N>
where
    C: IndexChunk<Index = u8>,
{
    type Index = u16;

    fn is_empty(&self) -> bool {
        self.0.iter().all(|u| u.is_empty())
    }

    fn len(&self) -> usize {
        self.0.iter().map(|u| u.len()).sum()
    }

    fn contains(&self, index: Self::Index) -> bool {
        let (outer, inner) = Self::split(index);

        let outer: usize = outer.into();

        self.0.get(outer).is_some_and(|chunk| chunk.contains(inner))
    }
}

impl<C, const N: usize> IndexCollection for ArrayChunk<C, N>
where
    C: IndexChunk<Index = u8>,
{
    fn span() -> (Bound<Self::Index>, Bound<Self::Index>) {
        const {
            assert!((Self::BITS - 1) <= (Self::Index::MAX as u32));
        };

        let upper = if Self::BITS == 0 {
            Bound::Excluded(0)
        } else {
            Bound::Included((Self::BITS - 1) as Self::Index)
        };

        (Bound::Included(0), upper)
    }

    fn new() -> Self {
        Self::new()
    }

    fn with_span(_: (Bound<Self::Index>, Bound<Self::Index>)) -> Self {
        Self::new()
    }
}

//  Safety:
//
//  -   NoPhantom: the store will only ever return that it contains an index if the index was inserted, and was not
//      removed since.
unsafe impl<C, const N: usize> IndexStore for ArrayChunk<C, N>
where
    C: IndexChunk<Index = u8>,
{
    type InsertionError = C::InsertionError;

    fn clear(&mut self) {
        self.0.iter_mut().for_each(|c| c.clear());
    }

    fn insert(&mut self, index: Self::Index) -> Result<bool, Self::InsertionError> {
        let (outer, inner) = Self::split(index);

        let outer: usize = outer.into();

        //  The user should always specify an in-bounds index. If they don't... that's their problem.
        let Some(chunk) = self.0.get_mut(outer) else {
            return Ok(false);
        };

        chunk.insert(inner)
    }

    fn remove(&mut self, index: Self::Index) -> bool {
        let (outer, inner) = Self::split(index);

        let outer: usize = outer.into();

        self.0.get_mut(outer).is_some_and(|chunk| chunk.remove(inner))
    }
}

//  Safety:
//
//  -   NoTheft: the vault will never return that it does not contain an index if the index was inserted, and was not
//      removed since.
unsafe impl<C, const N: usize> IndexVault for ArrayChunk<C, N> where C: IndexChunk<Index = u8> + IndexVault {}

//  Safety:
//
//  -   NoDuplicate: the view will never return the same index a second time.
//  -   NoPhantom: the view will only ever return that it contains an index if the index was inserted, and was not
//      removed since.
//  -   NoTheft: the view will return all indexes.
unsafe impl<C, const N: usize> IndexForward for ArrayChunk<C, N>
where
    C: IndexChunk<Index = u8> + IndexForward,
{
    fn first(&self) -> Option<Self::Index> {
        let (outer, inner) = self.0.iter().enumerate().find_map(|(i, c)| c.first().map(|r| (i, r)))?;

        Some(Self::fuse(outer as u16, inner))
    }

    fn next_after(&self, current: Self::Index) -> Option<Self::Index> {
        let (outer, inner) = Self::split(current);

        let outer: usize = outer.into();

        if let Some(inner) = self.0.get(outer).and_then(|chunk| chunk.next_after(inner)) {
            return Some(Self::fuse(outer as u16, inner));
        }

        let (outer, inner) = self
            .0
            .iter()
            .enumerate()
            .skip(outer + 1)
            .find_map(|(i, c)| c.first().map(|r| (i, r)))?;

        Some(Self::fuse(outer as u16, inner))
    }
}

//  Safety:
//
//  -   Reverse: the view will return indexes in the exact opposite sequence than `IndexForward` does.
unsafe impl<C, const N: usize> IndexBackward for ArrayChunk<C, N>
where
    C: IndexChunk<Index = u8> + IndexBackward,
{
    fn last(&self) -> Option<Self::Index> {
        let (outer, inner) = self
            .0
            .iter()
            .enumerate()
            .rev()
            .find_map(|(i, c)| c.last().map(|r| (i, r)))?;

        Some(Self::fuse(outer as u16, inner))
    }

    fn next_before(&self, current: Self::Index) -> Option<Self::Index> {
        let (outer, inner) = Self::split(current);

        let outer: usize = outer.into();

        if let Some(inner) = self.0.get(outer).and_then(|chunk| chunk.next_before(inner)) {
            return Some(Self::fuse(outer as u16, inner));
        }

        let limit = outer.min(self.0.len());

        let (outer, inner) = self
            .0
            .get(..limit)?
            .iter()
            .enumerate()
            .rev()
            .find_map(|(i, c)| c.last().map(|r| (i, r)))?;

        Some(Self::fuse(outer as u16, inner))
    }
}

//  Safety:
//
//  -   Ordered: the `IndexForward` implementation will return indexes in strictly increasing order.
unsafe impl<C, const N: usize> IndexOrdered for ArrayChunk<C, N> where C: IndexChunk<Index = u8> + IndexForward {}

//  Safety:
//
//  -   NoPhantom: the view will only ever return that it contains an index if the index was inserted, and was not
//      removed since.
//  -   SplitFuse: `split` and `fuse` are one another inverse.
//  -   TwoLevels: `split` and `fuse` are consistent with `IndexView`.
unsafe impl<C, const N: usize> IndexViewChunked for ArrayChunk<C, N>
where
    C: IndexChunk<Index = u8>,
{
    type ChunkIndex = u16;
    type Chunk = C;

    fn fuse(outer: Self::ChunkIndex, inner: C::Index) -> Self::Index {
        //  Will never overflow, because all indexes retrieved _were once inserted_, and they could only be inserted
        //  by being `u16` in the first place.

        let bits = C::BITS as u16;

        let inner: u16 = inner.into();

        outer * bits + inner
    }

    fn split(index: Self::Index) -> (Self::ChunkIndex, C::Index) {
        //  C is indexed by u8, ergo C::BITS is small enough that `index % bits` fits in u8.

        let bits = C::BITS as u16;

        let (outer, inner) = (index / bits, index % bits);

        (outer, inner as u8)
    }

    fn get_chunk(&self, index: Self::ChunkIndex) -> Option<Self::Chunk> {
        self.0.get(index as usize).copied()
    }
}

/// A store of indexes.
///
/// #   Safety
///
/// -   NoPhantom: the store will only ever return indexes that have been inserted and have not been removed since.
unsafe impl<C, const N: usize> IndexStoreChunked for ArrayChunk<C, N>
where
    C: IndexChunk<Index = u8>,
{
    type SetError = Never;

    /// #   Panics
    ///
    /// If `index >= N`.
    fn set_chunk(&mut self, index: Self::ChunkIndex, chunk: Self::Chunk) -> Result<(), Self::SetError> {
        self.0[index as usize] = chunk;

        Ok(())
    }
}

//  #   Safety
//
//  -   NoDuplicate: the view will never return the same index a second time.
//  -   NoPhantom: the view will only ever return that it contains an index if the index was inserted, and was not
//      removed since.
//  -   NoTheft: the view will return all indexes.
unsafe impl<C, const N: usize> IndexForwardChunked for ArrayChunk<C, N>
where
    C: IndexChunk<Index = u8>,
{
    fn first_chunk(&self) -> Option<Self::ChunkIndex> {
        (N > 0).then_some(0)
    }

    fn next_chunk_after(&self, current: Self::ChunkIndex) -> Option<Self::ChunkIndex> {
        let i: usize = current.into();

        (i + 1 < N).then(|| current + 1)
    }
}

//  Safety:
//
//  -   Reverse: the view will return indexes in the exact opposite sequence than `IndexForwardChunked` does.
unsafe impl<C, const N: usize> IndexBackwardChunked for ArrayChunk<C, N>
where
    C: IndexChunk<Index = u8>,
{
    fn last_chunk(&self) -> Option<Self::ChunkIndex> {
        (N > 0).then(|| (N - 1) as u16)
    }

    fn next_chunk_before(&self, current: Self::ChunkIndex) -> Option<Self::ChunkIndex> {
        (current > 0).then(|| current - 1)
    }
}

//  #   Safety
//
//  -   Ordered: the view will return indexes in strictly increasing order.
unsafe impl<C, const N: usize> IndexOrderedChunked for ArrayChunk<C, N> where C: IndexChunk<Index = u8> {}

#[cfg(test)]
mod tests {
    use crate::{chunk::UnsignedChunk, test::IndexTester};

    use super::*;

    struct Tester;

    impl IndexTester for Tester {
        type Index = u16;
        type Victim = ArrayChunk<UnsignedChunk<u8>, 4>;

        fn upper_bound() -> u8 {
            8 * 4 - 1
        }

        fn victim(indexes: &[u8]) -> Self::Victim {
            let mut array: Self::Victim = ArrayChunk::new();

            for &index in indexes {
                let _ = array.insert(index.into());
            }

            array
        }

        fn index(i: u8) -> Self::Index {
            i.into()
        }
    }

    crate::test_index_view!(Tester);
    crate::test_index_collection!(Tester);
    crate::test_index_store!(Tester);
    crate::test_index_forward!(Tester);
    crate::test_index_backward!(Tester);
    crate::test_index_view_chunked!(Tester);
    crate::test_index_forward_chunked!(Tester);
    crate::test_index_backward_chunked!(Tester);
} // mod tests
