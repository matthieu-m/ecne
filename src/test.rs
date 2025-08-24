//! Provide test-suites to help assert the conformance of `IndexXXX` implementations.
//!
//! An index generation trait is used to generate sequences of indexes to test for.
//!
//! #   Limitation
//!
//! For ease of implementation, only _vaults_, ie victims implementing `IndexVault`, can be tested with this test-suite.

mod index_backward;
mod index_backward_chunked;
mod index_backward_chunked_not;
mod index_backward_not;
mod index_collection;
mod index_forward;
mod index_forward_chunked;
mod index_forward_chunked_not;
mod index_forward_not;
mod index_store;
mod index_view;
mod index_view_chunked;
mod index_view_not;

use crate::index::IndexVault;

pub use index_backward::TestIndexBackward;
pub use index_backward_chunked::TestIndexBackwardChunked;
pub use index_backward_chunked_not::TestIndexBackwardChunkedNot;
pub use index_backward_not::TestIndexBackwardNot;
pub use index_collection::TestIndexCollection;
pub use index_forward::TestIndexForward;
pub use index_forward_chunked::TestIndexForwardChunked;
pub use index_forward_chunked_not::TestIndexForwardChunkedNot;
pub use index_forward_not::TestIndexForwardNot;
pub use index_store::TestIndexStore;
pub use index_view::TestIndexView;
pub use index_view_chunked::TestIndexViewChunked;
pub use index_view_not::TestIndexViewNot;

/// A trait to generate a view.
pub trait IndexTester {
    /// Index of the victim.
    type Index: Copy + Eq + Ord;

    /// Actual victim.
    type Victim: IndexVault<Index = Self::Index>;

    /// Returns the maximum value of `u8` which is guaranteed to work.
    ///
    /// This value should be at least 7.
    fn upper_bound() -> u8;

    /// Creates a victim containing the given indexes.
    ///
    /// Equivalent to `let victim: Victim = indexes.iter().map(|i| Self::map(*i)).collect()`.
    fn victim(indexes: &[u8]) -> Self::Victim;

    /// Maps a number to an index, arbitrarily.
    ///
    /// This operation should succeed for all `i` in `0..=Self::upper_bound()`, and may panic otherwise.
    ///
    /// This operation should be a bijection between input and output.
    ///
    /// This operation should preserve the _order_, that is, for any a, b such that a < b, then `Self::index(a)` <
    /// `Self::index(b)`.
    fn index(i: u8) -> Self::Index;
}

/// A trait to generate a not view.
pub trait IndexTesterNot: IndexTester {
    /// Returns the capacity.
    fn capacity() -> usize;

    /// Creates a victim containing all BUT the given indexes.
    ///
    /// Equivalent to `let victim: Victim = indexes.iter().map(|i| Self::map(*i)).collect()`.
    fn victim_not(indexes: &[u8]) -> Self::Victim;
}
