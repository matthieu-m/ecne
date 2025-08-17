//! Test suite for the `IndexForward` trait.

use core::{marker::PhantomData, num::NonZeroUsize};

use crate::index::{IndexForward, IndexOrdered};

use super::IndexTester;

/// Tests that the `$victim` correctly implements the `IndexForward` trait.
#[macro_export]
macro_rules! test_index_forward {
    ($tester:ident) => {
        mod test_index_forward {
            use super::$tester;

            type TestSuite = $crate::test::TestIndexForward<$tester>;

            #[test]
            fn validate() {
                TestSuite::validate();
            }

            #[test]
            fn first_empty() {
                TestSuite::first_empty();
            }

            #[test]
            fn first_non_empty() {
                TestSuite::first_non_empty();
            }

            #[test]
            fn next_after() {
                TestSuite::next_after();
            }

            #[test]
            fn nth_after() {
                TestSuite::nth_after();
            }

            #[cfg(feature = "nightly")]
            #[test]
            fn try_fold_after_all() {
                TestSuite::try_fold_after_all();
            }

            #[cfg(feature = "nightly")]
            #[test]
            fn try_fold_after_fail() {
                TestSuite::try_fold_after_fail();
            }
        } // mod test_index_forward
    };
}

/// Test suite for the `IndexForward` trait.
pub struct TestIndexForward<T>(PhantomData<T>);

impl<T> TestIndexForward<T>
where
    T: IndexTester<Victim: IndexForward + IndexOrdered>,
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

    /// Checks that an empty victim contains no first.
    pub fn first_empty() {
        let victim = T::victim(&[]);

        assert!(victim.first().is_none());
    }

    /// Checks that a non-empty victim returns the first index.
    pub fn first_non_empty() {
        for i in 0..=T::upper_bound() {
            let victim = T::victim(&[i]);

            assert!(Some(T::index(i)) == victim.first(), "{i}");
        }
    }

    /// Checks that a non-empty victim returns the indexes in order.
    pub fn next_after() {
        const INDEXES: [u8; 5] = [1, 2, 3, 5, 7];

        let indexes = [
            T::index(INDEXES[0]),
            T::index(INDEXES[1]),
            T::index(INDEXES[2]),
            T::index(INDEXES[3]),
            T::index(INDEXES[4]),
        ];

        let victim = T::victim(&INDEXES);

        assert!(Some(indexes[0]) == victim.first());

        for i in 0..(indexes.len() - 1) {
            assert!(Some(indexes[i + 1]) == victim.next_after(indexes[i]));
        }

        assert!(victim.next_after(indexes[indexes.len() - 1]).is_none());
    }

    /// Checks that a non-empty victim returns the n-th index as appropriate.
    pub fn nth_after() {
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

        let victim = T::victim(&INDEXES);

        assert!(Ok(indexes[1]) == victim.nth_after(0, indexes[0]));
        assert!(Ok(indexes[2]) == victim.nth_after(1, indexes[0]));
        assert!(Ok(indexes[3]) == victim.nth_after(2, indexes[0]));
        assert!(Ok(indexes[4]) == victim.nth_after(3, indexes[0]));
        assert!(Err(non_zero(1)) == victim.nth_after(4, indexes[0]));
        assert!(Err(non_zero(2)) == victim.nth_after(5, indexes[0]));

        assert!(Ok(indexes[3]) == victim.nth_after(0, indexes[2]));
        assert!(Ok(indexes[4]) == victim.nth_after(1, indexes[2]));
        assert!(Err(non_zero(1)) == victim.nth_after(2, indexes[2]));
        assert!(Err(non_zero(2)) == victim.nth_after(3, indexes[2]));

        assert!(Err(non_zero(1)) == victim.nth_after(0, indexes[4]));
    }

    /// Checks that a non-empty victim folds all the items in order.
    #[cfg(feature = "nightly")]
    pub fn try_fold_after_all() {
        const INDEXES: [u8; 5] = [1, 2, 3, 5, 7];

        let indexes = [
            T::index(INDEXES[0]),
            T::index(INDEXES[1]),
            T::index(INDEXES[2]),
            T::index(INDEXES[3]),
            T::index(INDEXES[4]),
        ];

        let victim = T::victim(&INDEXES);

        let result = victim
            .try_fold_after(indexes[0], Vec::new(), |mut acc, i| {
                assert!(acc.len() < INDEXES.len());

                acc.push(i);

                Some(acc)
            })
            .expect("no error");

        assert!(indexes[1..] == result);
    }

    /// Checks that a non-empty victim stops folding the items when instructed so.
    #[cfg(feature = "nightly")]
    pub fn try_fold_after_fail() {
        const INDEXES: [u8; 5] = [1, 2, 3, 5, 7];

        let indexes = [
            T::index(INDEXES[0]),
            T::index(INDEXES[1]),
            T::index(INDEXES[2]),
            T::index(INDEXES[3]),
            T::index(INDEXES[4]),
        ];

        let victim = T::victim(&INDEXES);

        for fail in 1..indexes.len() {
            let mut visited = Vec::new();

            let result = victim
                .try_fold_after(indexes[0], (), |_, i| {
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
