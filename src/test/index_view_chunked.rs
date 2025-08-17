//! Test suite for the `IndexViewChunked` trait.

use core::marker::PhantomData;

use crate::index::{IndexView, IndexViewChunked};

use super::IndexTester;

/// Tests that the `$victim` correctly implements the `IndexViewChunked` trait.
#[macro_export]
macro_rules! test_index_view_chunked {
    ($tester:ident) => {
        mod test_index_view_chunked {
            use super::$tester;

            type TestSuite = $crate::test::TestIndexViewChunked<$tester>;

            #[test]
            fn validate() {
                TestSuite::validate();
            }

            #[test]
            fn split_fuse_round_trip() {
                TestSuite::split_fuse_round_trip();
            }

            #[test]
            fn get_chunk() {
                TestSuite::get_chunk();
            }
        } // mod test_index_view_chunked
    };
}

/// Test suite for the `IndexViewChunked` trait.
pub struct TestIndexViewChunked<T>(PhantomData<T>);

impl<T> TestIndexViewChunked<T>
where
    T: IndexTester<Victim: IndexViewChunked>,
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

    /// Checks that fuse and split actually round-trip.
    pub fn split_fuse_round_trip() {
        for i in 0..=T::upper_bound() {
            let index = T::index(i);

            let (outer, inner) = T::Victim::split(index);

            let recovered = T::Victim::fuse(outer, inner);

            assert!(index == recovered);
        }
    }

    /// Checks that chunks offer a consistent view of the indexes.
    pub fn get_chunk() {
        const INDEXES: [u8; 7] = [1, 2, 3, 5, 7, 11, 13];

        let victim = T::victim(&INDEXES);

        for i in 0..=T::upper_bound() {
            let is_contained = INDEXES.contains(&i);

            let index = T::index(i);
            let (outer, inner) = T::Victim::split(index);

            assert_eq!(is_contained, victim.contains(index), "{i}");
            assert_eq!(
                is_contained,
                victim.get_chunk(outer).is_some_and(|c| c.contains(inner)),
                "{i}"
            );
        }
    }
}
