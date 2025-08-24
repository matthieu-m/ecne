//! Test suite for the `IndexForwardNot` trait.

use core::{marker::PhantomData, num::NonZeroUsize};

use crate::not::{IndexForwardNot, IndexOrderedNot};

use super::IndexTesterNot;

/// Tests that the victim correctly implements the `IndexForwardNot` trait.
#[macro_export]
macro_rules! test_index_forward_not {
    ($tester:ident) => {
        mod test_index_forward_not {
            use super::$tester;

            type TestSuite = $crate::test::TestIndexForwardNot<$tester>;

            #[test]
            fn validate() {
                TestSuite::validate();
            }

            #[test]
            fn first_full() {
                TestSuite::first_full();
            }

            #[test]
            fn first_non_full() {
                TestSuite::first_non_full();
            }

            #[test]
            fn next_after_not() {
                TestSuite::next_after_not();
            }

            #[test]
            fn nth_after_not() {
                TestSuite::nth_after_not();
            }

            #[cfg(feature = "nightly")]
            #[test]
            fn try_fold_after_not_all() {
                TestSuite::try_fold_after_not_all();
            }

            #[cfg(feature = "nightly")]
            #[test]
            fn try_fold_after_not_fail() {
                TestSuite::try_fold_after_not_fail();
            }
        } // mod test_index_forward_not
    };
}

/// Test suite for the `IndexForwardNot` trait.
pub struct TestIndexForwardNot<T>(PhantomData<T>);

impl<T> TestIndexForwardNot<T>
where
    T: IndexTesterNot<Victim: IndexForwardNot + IndexOrderedNot>,
{
    const MINIMUM_UPPER_BOUND: u8 = 6;

    /// Validates `T` itself.
    pub fn validate() {
        assert!(
            T::upper_bound() >= Self::MINIMUM_UPPER_BOUND,
            "{} < {}",
            T::upper_bound(),
            Self::MINIMUM_UPPER_BOUND
        );
    }

    /// Checks that a full victim contains no first.
    pub fn first_full() {
        let victim = T::victim_not(&[]);

        assert!(victim.first_not().is_none());
    }

    /// Checks that a non-full victim returns the first index.
    pub fn first_non_full() {
        for i in 0..=T::upper_bound() {
            let victim = T::victim_not(&[i]);

            assert!(Some(T::index(i)) == victim.first_not(), "{i}");
        }
    }

    /// Checks that a non-full victim returns the indexes in order.
    pub fn next_after_not() {
        const INDEXES: [u8; 5] = [1, 2, 3, 5, 7];

        let indexes = [
            T::index(INDEXES[0]),
            T::index(INDEXES[1]),
            T::index(INDEXES[2]),
            T::index(INDEXES[3]),
            T::index(INDEXES[4]),
        ];

        let victim = T::victim_not(&INDEXES);

        assert!(Some(indexes[0]) == victim.first_not());

        for i in 0..(indexes.len() - 1) {
            assert!(Some(indexes[i + 1]) == victim.next_after_not(indexes[i]));
        }

        assert!(victim.next_after_not(indexes[indexes.len() - 1]).is_none());
    }

    /// Checks that a non-full victim returns the n-th index as appropriate.
    pub fn nth_after_not() {
        const INDEXES: [u8; 5] = [1, 2, 3, 5, 7];

        fn non_zero(n: usize) -> NonZeroUsize {
            NonZeroUsize::new(n).unwrap()
        }

        let indexes = [
            T::index(INDEXES[0]),
            T::index(INDEXES[1]),
            T::index(INDEXES[2]),
            T::index(INDEXES[3]),
            T::index(INDEXES[4]),
        ];

        let victim = T::victim_not(&INDEXES);

        assert!(Ok(indexes[1]) == victim.nth_after_not(0, indexes[0]));
        assert!(Ok(indexes[2]) == victim.nth_after_not(1, indexes[0]));
        assert!(Ok(indexes[3]) == victim.nth_after_not(2, indexes[0]));
        assert!(Ok(indexes[4]) == victim.nth_after_not(3, indexes[0]));
        assert!(Err(non_zero(1)) == victim.nth_after_not(4, indexes[0]));
        assert!(Err(non_zero(2)) == victim.nth_after_not(5, indexes[0]));

        assert!(Ok(indexes[3]) == victim.nth_after_not(0, indexes[2]));
        assert!(Ok(indexes[4]) == victim.nth_after_not(1, indexes[2]));
        assert!(Err(non_zero(1)) == victim.nth_after_not(2, indexes[2]));
        assert!(Err(non_zero(2)) == victim.nth_after_not(3, indexes[2]));

        assert!(Err(non_zero(1)) == victim.nth_after_not(0, indexes[4]));
    }

    /// Checks that a non-empty victim folds all the items in order.
    #[cfg(feature = "nightly")]
    pub fn try_fold_after_not_all() {
        const INDEXES: [u8; 5] = [1, 2, 3, 5, 7];

        let indexes = [
            T::index(INDEXES[0]),
            T::index(INDEXES[1]),
            T::index(INDEXES[2]),
            T::index(INDEXES[3]),
            T::index(INDEXES[4]),
        ];

        let victim = T::victim_not(&INDEXES);

        let result = victim
            .try_fold_after_not(indexes[0], Vec::new(), |mut acc, i| {
                assert!(acc.len() < INDEXES.len());

                acc.push(i);

                Some(acc)
            })
            .expect("no error");

        assert!(indexes[1..] == result);
    }

    /// Checks that a non-empty victim stops folding the items when instructed so.
    #[cfg(feature = "nightly")]
    pub fn try_fold_after_not_fail() {
        const INDEXES: [u8; 5] = [1, 2, 3, 5, 7];

        let indexes = [
            T::index(INDEXES[0]),
            T::index(INDEXES[1]),
            T::index(INDEXES[2]),
            T::index(INDEXES[3]),
            T::index(INDEXES[4]),
        ];

        let victim = T::victim_not(&INDEXES);

        for fail in 1..indexes.len() {
            let mut visited = Vec::new();

            let result = victim
                .try_fold_after_not(indexes[0], (), |_, i| {
                    assert!(visited.len() < INDEXES.len());

                    if i == indexes[fail] {
                        return Err(i);
                    }

                    visited.push(i);

                    Ok(())
                })
                .expect_err("error");

            assert!(indexes[1..fail] == visited, "{fail}");
            assert!(indexes[fail] == result, "{fail}");
        }
    }
}
