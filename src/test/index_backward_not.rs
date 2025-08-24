//! Test suite for the `IndexBackwardNot` trait.

use core::{marker::PhantomData, num::NonZeroUsize};

use crate::not::{IndexBackwardNot, IndexOrderedNot};

use super::IndexTesterNot;

/// Tests that the `$victim` correctly implements the `IndexBackwardNot` trait.
#[macro_export]
macro_rules! test_index_backward_not {
    ($tester:ident) => {
        mod test_index_backward_not {
            use super::$tester;

            type TestSuite = $crate::test::TestIndexBackwardNot<$tester>;

            #[test]
            fn validate() {
                TestSuite::validate();
            }

            #[test]
            fn last_full() {
                TestSuite::last_full();
            }

            #[test]
            fn last_non_full() {
                TestSuite::last_non_full();
            }

            #[test]
            fn next_before_not() {
                TestSuite::next_before_not();
            }

            #[test]
            fn nth_before_not() {
                TestSuite::nth_before_not();
            }

            #[cfg(feature = "nightly")]
            #[test]
            fn try_fold_before_not_all() {
                TestSuite::try_fold_before_not_all();
            }

            #[cfg(feature = "nightly")]
            #[test]
            fn try_fold_before_not_fail() {
                TestSuite::try_fold_before_not_fail();
            }
        } // mod test_index_backward_not
    };
}

/// Test suite for the `IndexBackwardNot` trait.
pub struct TestIndexBackwardNot<T>(PhantomData<T>);

impl<T> TestIndexBackwardNot<T>
where
    T: IndexTesterNot<Victim: IndexBackwardNot + IndexOrderedNot>,
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

    /// Checks that a full victim contains no last.
    pub fn last_full() {
        let victim = T::victim_not(&[]);

        assert!(victim.last_not().is_none());
    }

    /// Checks that a non-empty victim returns the last index.
    pub fn last_non_full() {
        for i in 0..=T::upper_bound() {
            let victim = T::victim_not(&[i]);

            assert!(Some(T::index(i)) == victim.last_not(), "{i}");
        }
    }

    /// Checks that a non-empty victim returns the indexes in order.
    pub fn next_before_not() {
        const INDEXES: [u8; 5] = [1, 2, 3, 5, 7];

        let indexes = [
            T::index(INDEXES[0]),
            T::index(INDEXES[1]),
            T::index(INDEXES[2]),
            T::index(INDEXES[3]),
            T::index(INDEXES[4]),
        ];

        let victim = T::victim_not(&INDEXES);

        assert!(Some(indexes[indexes.len() - 1]) == victim.last_not());

        for i in 1..indexes.len() {
            assert!(Some(indexes[i - 1]) == victim.next_before_not(indexes[i]));
        }

        assert!(victim.next_before_not(indexes[0]).is_none());
    }

    /// Checks that a non-empty victim returns the n-th index as appropriate.
    pub fn nth_before_not() {
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

        assert!(Ok(indexes[3]) == victim.nth_before_not(0, indexes[4]));
        assert!(Ok(indexes[2]) == victim.nth_before_not(1, indexes[4]));
        assert!(Ok(indexes[1]) == victim.nth_before_not(2, indexes[4]));
        assert!(Ok(indexes[0]) == victim.nth_before_not(3, indexes[4]));
        assert!(Err(non_zero(1)) == victim.nth_before_not(4, indexes[4]));
        assert!(Err(non_zero(2)) == victim.nth_before_not(5, indexes[4]));

        assert!(Ok(indexes[1]) == victim.nth_before_not(0, indexes[2]));
        assert!(Ok(indexes[0]) == victim.nth_before_not(1, indexes[2]));
        assert!(Err(non_zero(1)) == victim.nth_before_not(2, indexes[2]));
        assert!(Err(non_zero(2)) == victim.nth_before_not(3, indexes[2]));

        assert!(Err(non_zero(1)) == victim.nth_before_not(0, indexes[0]));
    }

    /// Checks that a non-empty victim folds all the items in order.
    #[cfg(feature = "nightly")]
    pub fn try_fold_before_not_all() {
        const INDEXES: [u8; 5] = [1, 2, 3, 5, 7];

        let indexes = [
            T::index(INDEXES[4]),
            T::index(INDEXES[3]),
            T::index(INDEXES[2]),
            T::index(INDEXES[1]),
            T::index(INDEXES[0]),
        ];

        let victim = T::victim_not(&INDEXES);

        let result = victim
            .try_fold_before_not(indexes[0], Vec::new(), |mut acc, i| {
                assert!(acc.len() < INDEXES.len());

                acc.push(i);

                Some(acc)
            })
            .expect("no error");

        assert!(indexes[1..] == result);
    }

    /// Checks that a non-empty victim stops folding the items when instructed so.
    #[cfg(feature = "nightly")]
    pub fn try_fold_before_not_fail() {
        const INDEXES: [u8; 5] = [1, 2, 3, 5, 7];

        let indexes = [
            T::index(INDEXES[4]),
            T::index(INDEXES[3]),
            T::index(INDEXES[2]),
            T::index(INDEXES[1]),
            T::index(INDEXES[0]),
        ];

        let victim = T::victim_not(&INDEXES);

        for fail in 1..indexes.len() {
            let mut visited = Vec::new();

            let result = victim
                .try_fold_before_not(indexes[0], (), |_, i| {
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
