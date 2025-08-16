//! Test suite for the `IndexBackward` trait.

use core::{marker::PhantomData, num::NonZeroUsize};

use crate::index::{IndexBackward, IndexOrdered};

use super::IndexTester;

/// Tests that the `$victim` correctly implements the `IndexBackward` trait.
#[macro_export]
macro_rules! test_index_backward {
    ($tester:ident) => {
        mod test_index_backward {
            use super::$tester;

            type TestSuite = $crate::test::TestIndexBackward<$tester>;

            #[test]
            fn validate() {
                TestSuite::validate();
            }

            #[test]
            fn last_empty() {
                TestSuite::last_empty();
            }

            #[test]
            fn last_non_empty() {
                TestSuite::last_non_empty();
            }

            #[test]
            fn next_before() {
                TestSuite::next_before();
            }

            #[test]
            fn nth_before() {
                TestSuite::nth_before();
            }

            #[cfg(feature = "nightly")]
            #[test]
            fn try_fold_before_all() {
                TestSuite::try_fold_before_all();
            }

            #[cfg(feature = "nightly")]
            #[test]
            fn try_fold_before_fail() {
                TestSuite::try_fold_before_fail();
            }
        } // mod test_index_backward
    };
}

/// Test suite for the `IndexBackward` trait.
pub struct TestIndexBackward<T>(PhantomData<T>);

impl<T> TestIndexBackward<T>
where
    T: IndexTester<Victim: IndexBackward + IndexOrdered>,
{
    const MINIMUM_UPPER_BOUND: u8 = 6;

    /// Validates `T` itself.
    pub fn validate() {
        assert!(
            T::upper_bound() > Self::MINIMUM_UPPER_BOUND,
            "{} <= {}",
            T::upper_bound(),
            Self::MINIMUM_UPPER_BOUND
        );
    }

    /// Checks that an empty victim contains no last.
    pub fn last_empty() {
        let victim = T::victim(&[]);

        assert!(victim.last().is_none());
    }

    /// Checks that a non-empty victim returns the last index.
    pub fn last_non_empty() {
        for i in 0..=T::upper_bound() {
            let victim = T::victim(&[i]);

            assert!(Some(T::index(i)) == victim.last(), "{i}");
        }
    }

    /// Checks that a non-empty victim returns the indexes in order.
    pub fn next_before() {
        const INDEXES: [u8; 5] = [1, 2, 3, 5, 7];

        let indexes = [
            T::index(INDEXES[0]),
            T::index(INDEXES[1]),
            T::index(INDEXES[2]),
            T::index(INDEXES[3]),
            T::index(INDEXES[4]),
        ];

        let victim = T::victim(&INDEXES);

        assert!(Some(indexes[indexes.len() - 1]) == victim.last());

        for i in 1..indexes.len() {
            assert!(Some(indexes[i - 1]) == victim.next_before(indexes[i]));
        }

        assert!(victim.next_before(indexes[0]).is_none());
    }

    /// Checks that a non-empty victim returns the n-th index as appropriate.
    pub fn nth_before() {
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

        assert!(Ok(indexes[3]) == victim.nth_before(0, indexes[4]));
        assert!(Ok(indexes[2]) == victim.nth_before(1, indexes[4]));
        assert!(Ok(indexes[1]) == victim.nth_before(2, indexes[4]));
        assert!(Ok(indexes[0]) == victim.nth_before(3, indexes[4]));
        assert!(Err(non_zero(1)) == victim.nth_before(4, indexes[4]));
        assert!(Err(non_zero(2)) == victim.nth_before(5, indexes[4]));

        assert!(Ok(indexes[1]) == victim.nth_before(0, indexes[2]));
        assert!(Ok(indexes[0]) == victim.nth_before(1, indexes[2]));
        assert!(Err(non_zero(1)) == victim.nth_before(2, indexes[2]));
        assert!(Err(non_zero(2)) == victim.nth_before(3, indexes[2]));

        assert!(Err(non_zero(1)) == victim.nth_before(0, indexes[0]));
    }

    /// Checks that a non-empty victim folds all the items in order.
    #[cfg(feature = "nightly")]
    pub fn try_fold_before_all() {
        const INDEXES: [u8; 5] = [1, 2, 3, 5, 7];

        let indexes = [
            T::index(INDEXES[4]),
            T::index(INDEXES[3]),
            T::index(INDEXES[2]),
            T::index(INDEXES[1]),
            T::index(INDEXES[0]),
        ];

        let victim = T::victim(&INDEXES);

        let result = victim
            .try_fold_before(indexes[0], Vec::new(), |mut acc, i| {
                assert!(acc.len() < INDEXES.len());

                acc.push(i);

                Some(acc)
            })
            .expect("no error");

        assert!(indexes[..] == result);
    }

    /// Checks that a non-empty victim stops folding the items when instructed so.
    #[cfg(feature = "nightly")]
    pub fn try_fold_before_fail() {
        const INDEXES: [u8; 5] = [1, 2, 3, 5, 7];

        let indexes = [
            T::index(INDEXES[4]),
            T::index(INDEXES[3]),
            T::index(INDEXES[2]),
            T::index(INDEXES[1]),
            T::index(INDEXES[0]),
        ];

        let victim = T::victim(&INDEXES);

        for fail in 1..indexes.len() {
            let mut visited = Vec::new();

            let result = victim
                .try_fold_before(indexes[0], (), |_, i| {
                    assert!(visited.len() < INDEXES.len());

                    if i == indexes[fail] {
                        return Err(i);
                    }

                    visited.push(i);

                    Ok(())
                })
                .expect_err("error");

            assert!(indexes[..fail] == visited, "{fail}");
            assert!(indexes[fail] == result, "{fail}");
        }
    }
}
