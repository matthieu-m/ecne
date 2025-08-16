//! Chunked iteration.

mod array;
mod unsigned;

use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Sub, SubAssign};

use crate::index::{IndexCollection, IndexStore};

pub use array::ArrayChunk;
pub use unsigned::UnsignedChunk;

/// A chunk of indexes.
pub trait IndexChunk:
    Copy
    + BitAnd<Output = Self>
    + BitAndAssign
    + BitOr<Output = Self>
    + BitOrAssign
    + BitXor<Output = Self>
    + BitXorAssign
    + Not<Output = Self>
    + Sub<Output = Self>
    + SubAssign
    + IndexCollection
    + IndexStore
{
    /// Number of bits in this chunk.
    const BITS: u32;
}
