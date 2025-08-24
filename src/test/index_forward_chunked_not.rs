//! Test suite for the `IndexForwardChunkedNot` trait.

use core::{marker::PhantomData, ops::Not};

use crate::{
    index::{IndexView, IndexViewChunked},
    not::{IndexForwardChunkedNot, IndexForwardNot, IndexOrderedChunkedNot, IndexOrderedNot},
};

use super::IndexTesterNot;

/// Tests that the `$victim` correctly implements the `IndexForwardChunkedNot` trait.
#[macro_export]
macro_rules! test_index_forward_chunked_not {
    ($tester:ident) => {
        mod test_index_forward_chunked_not {
            use super::$tester;

            type TestSuite = $crate::test::TestIndexForwardChunkedNot<$tester>;

            #[test]
            fn validate() {
                TestSuite::validate();
            }

            #[test]
            fn first_chunk_full() {
                TestSuite::first_chunk_full();
            }

            #[test]
            fn first_chunk_non_full() {
                TestSuite::first_chunk_non_full();
            }

            #[test]
            fn next_chunk_after_ascending() {
                TestSuite::next_chunk_after_ascending();
            }

            #[test]
            fn next_chunk_after_consistent() {
                TestSuite::next_chunk_after_consistent();
            }
        } // mod test_index_forward_chunked_not
    };
}

/// Test suite for the `IndexForward` trait.
pub struct TestIndexForwardChunkedNot<T>(PhantomData<T>);

impl<T> TestIndexForwardChunkedNot<T>
where
    T: IndexTesterNot<
        Victim: IndexForwardChunkedNot<Chunk: IndexForwardNot + IndexOrderedNot> + IndexOrderedChunkedNot,
    >,
{
    const MINIMUM_UPPER_BOUND: u8 = 13;

    /// Validates `T` itself.
    pub fn validate() {
        assert!(
            T::upper_bound() >= Self::MINIMUM_UPPER_BOUND,
            "{} < {}",
            T::upper_bound(),
            Self::MINIMUM_UPPER_BOUND
        );
    }

    /// Checks that a full victim contains no element.
    pub fn first_chunk_full() {
        //  This does not mean that `first_chunk` MUST return 0, however, it could return the index of an empty chunk.

        let victim = T::victim_not(&[]);

        assert!(
            victim
                .first_chunk_not()
                .is_none_or(|c| victim.get_chunk(c).is_none_or(|c| c.not().is_empty()))
        );
    }

    /// Checks that a non-full victim returns the first index.
    pub fn first_chunk_non_full() {
        //  This does not mean that `first_chunk_not` MUST return the index of the first non-empty chunk, however,
        //  merely that it must return an index that is prior to it.

        for i in 0..=T::upper_bound() {
            let index = T::index(i);
            let (outer, _) = T::Victim::split(index);

            let victim = T::victim_not(&[i]);

            assert!(victim.first_chunk_not().is_some_and(|n| n <= outer), "{i}");
        }
    }

    /// Checks that a non-empty victim returns the chunk indexes in order.
    pub fn next_chunk_after_ascending() {
        const INDEXES: [u8; 7] = [1, 2, 3, 5, 7, 11, 13];

        let victim = T::victim_not(&INDEXES);

        let mut outer = victim.first_chunk_not().expect("non empty");

        while let Some(next) = victim.next_chunk_after_not(outer) {
            assert!(next > outer);

            outer = next;
        }
    }

    /// Checks that a non-empty victim returns all the chunks with indexes.
    pub fn next_chunk_after_consistent() {
        const INDEXES: [u8; 7] = [1, 2, 3, 5, 7, 11, 13];

        let expected = [
            T::index(INDEXES[0]),
            T::index(INDEXES[1]),
            T::index(INDEXES[2]),
            T::index(INDEXES[3]),
            T::index(INDEXES[4]),
            T::index(INDEXES[5]),
            T::index(INDEXES[6]),
        ];

        let mut next = 0;

        let victim = T::victim_not(&INDEXES);

        let mut outer = victim.first_chunk_not().expect("non empty");

        loop {
            if let Some(chunk) = victim.get_chunk(outer)
                && let Some(mut inner) = chunk.first_not()
            {
                loop {
                    assert!(expected[next] == T::Victim::fuse(outer, inner));

                    next += 1;

                    let Some(i) = chunk.next_after_not(inner) else { break };

                    inner = i;
                }
            }

            let Some(o) = victim.next_chunk_after_not(outer) else {
                break;
            };

            outer = o;
        }

        assert_eq!(expected.len(), next);
    }
}
