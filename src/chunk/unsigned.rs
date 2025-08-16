//! Unsigned chunk.

use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Bound, Not, Sub, SubAssign};

use crate::{
    Never,
    chunk::IndexChunk,
    index::{IndexBackward, IndexCollection, IndexForward, IndexOrdered, IndexStore, IndexVault, IndexView},
};

/// Simple implementation of `IndexChunk` for integrals.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct UnsignedChunk<I>(pub I);

impl<I> BitAnd for UnsignedChunk<I>
where
    I: BitAnd<Output = I>,
{
    type Output = Self;

    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}

impl<I> BitAndAssign for UnsignedChunk<I>
where
    I: BitAndAssign,
{
    fn bitand_assign(&mut self, other: Self) {
        self.0 &= other.0;
    }
}

impl<I> BitOr for UnsignedChunk<I>
where
    I: BitOr<Output = I>,
{
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

impl<I> BitOrAssign for UnsignedChunk<I>
where
    I: BitOrAssign,
{
    fn bitor_assign(&mut self, other: Self) {
        self.0 |= other.0;
    }
}

impl<I> BitXor for UnsignedChunk<I>
where
    I: BitXor<Output = I>,
{
    type Output = Self;

    fn bitxor(self, other: Self) -> Self {
        Self(self.0 ^ other.0)
    }
}

impl<I> BitXorAssign for UnsignedChunk<I>
where
    I: BitXorAssign,
{
    fn bitxor_assign(&mut self, other: Self) {
        self.0 ^= other.0;
    }
}

impl<I> Not for UnsignedChunk<I>
where
    I: Not<Output = I>,
{
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl<I> Sub for UnsignedChunk<I>
where
    I: BitAnd<Output = I> + Not<Output = I>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 & !other.0)
    }
}

impl<I> SubAssign for UnsignedChunk<I>
where
    I: BitAndAssign + Not<Output = I>,
{
    fn sub_assign(&mut self, other: Self) {
        self.0 &= !other.0;
    }
}

macro_rules! impl_indexes_chunk_for_chunk {
    ($($u:ident)*) => { $(
        impl IndexChunk for UnsignedChunk<$u> {
            const BITS: u32 = $u::BITS;
        }

        //  #   Safety
        //
        //  -   NoPhantom: the store WILL only ever return that it contains an index if the index was inserted, and was
        //      not removed since.
        unsafe impl IndexView for UnsignedChunk<$u> {
            //  Sufficient for even u128 and an hypothetical u256, it'll serve.
            type Index = u8;

            fn is_empty(&self) -> bool {
                self.0 == 0
            }

            fn len(&self) -> usize {
                self.0.count_ones() as usize
            }

            fn contains(&self, index: Self::Index) -> bool {
                let index: u32 = index.into();

                let mask = (1 << index);

                (self.0 & mask) != 0
            }
        }

        impl IndexCollection for UnsignedChunk<$u> {
            fn span() -> (Bound<Self::Index>, Bound<Self::Index>) {
                (Bound::Included(0), Bound::Excluded($u::BITS as u8))
            }

            fn new() -> Self {
                Self(0)
            }

            fn with_span(_: (Bound<Self::Index>, Bound<Self::Index>)) -> Self {
                Self::new()
            }
        }

        //  #   Safety
        //
        //  -   NoPhantom: the store WILL only ever return that it contains an index if the index was inserted, and was
        //      not removed since.
        unsafe impl IndexStore for UnsignedChunk<$u> {
            type InsertionError = Never;

            fn clear(&mut self) {
                self.0 = 0;
            }

            fn insert(&mut self, index: Self::Index) -> Result<bool, Never> {
                use core::ops::RangeBounds;

                debug_assert!(Self::span().contains(&index), "{index}");

                let mask = (1 << (index as u32));

                let existed = (self.0 & mask) != 0;

                self.0 |= (1 << (index as u32));

                Ok(!existed)
            }

            fn remove(&mut self, index: Self::Index) -> bool {
                use core::ops::RangeBounds;

                if !Self::span().contains(&index) {
                    return false;
                }

                let mask = (1 << (index as u32));

                let existed = (self.0 & mask) != 0;

                self.0 &= !(1 << (index as u32));

                existed
            }
        }

        //  #   Safety
        //
        //  -   NoTheft: the vault WILL never return that it does not contain an index if the index was inserted, and
        //      was not removed since.
        unsafe impl IndexVault for UnsignedChunk<$u> {}

        //  #   Safety
        //
        //  -   NoDuplicate: the view WILL never return the same index a second time.
        //  -   NoPhantom: the view WILL only ever return that it contains an index if the index was inserted, and was
        //      not removed since.
        //  -   NoTheft: the view WILL return all indexes.
        unsafe impl IndexForward for UnsignedChunk<$u> {
            fn first(&self) -> Option<Self::Index> {
                let zeros = self.0.trailing_zeros();

                (zeros < $u::BITS).then_some(zeros as u8)
            }

            fn next_after(&self, index: Self::Index) -> Option<Self::Index> {
                let index: u32 = index.into();

                let next_index = index + 1;

                if next_index == $u::BITS {
                    return None;
                }

                let masked = self.0 & !((1 << next_index) - 1);

                Self(masked).first()
            }
        }

        //  #   Safety
        //
        //  -   Reverse: the view WILL return indexes in the exact opposite sequence than `IndexForward` does.
        unsafe impl IndexBackward for UnsignedChunk<$u> {
            fn last(&self) -> Option<Self::Index> {
                let zeros = self.0.leading_zeros();

                $u::BITS.checked_sub(zeros + 1).map(|n| n as u8)
            }

            fn next_before(&self, index: Self::Index) -> Option<Self::Index> {
                let index: u32 = index.into();

                let masked = self.0 & ((1 << index) - 1);

                Self(masked).last()
            }
        }

        //  #   Safety
        //
        //  -   Ordered: the `IndexForward` implementation WILL return indexes in strictly increasing order.
        unsafe impl IndexOrdered for UnsignedChunk<$u> {}
    )* };
}

impl_indexes_chunk_for_chunk!(u8 u16 u32 u64 u128 usize);

#[cfg(test)]
mod tests {
    macro_rules! test_unsigned_chunk {
        ($($u:ident)*) => { $(
            mod $u {
                use crate::chunk::UnsignedChunk;

                struct Tester;

                impl crate::test::IndexTester for Tester {
                    type Index = u8;
                    type Victim = UnsignedChunk<$u>;

                    fn upper_bound() -> u8 { 7 }

                    fn victim(indexes: &[u8]) -> Self::Victim {
                        UnsignedChunk(indexes.iter().fold(0, |acc, i| acc | (1 << (*i as u32))))
                    }

                    fn index(i: u8) -> Self::Index { i }
                }

                crate::test_index_view!(Tester);
                crate::test_index_collection!(Tester);
                crate::test_index_store!(Tester);
                crate::test_index_forward!(Tester);
                crate::test_index_backward!(Tester);
            }
       )* };
    }

    test_unsigned_chunk!(u8 u16 u32 u64 u128 usize);
} // mod tests
