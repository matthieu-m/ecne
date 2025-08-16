//! Test suite for the `IndexView` trait.

use core::marker::PhantomData;

use crate::index::IndexView;

use super::IndexTester;

/// Tests that the `$victim` correctly implements the `IndexView` trait.
#[macro_export]
macro_rules! test_index_view {
    ($tester:ident) => {
        mod test_index_view {
            use super::$tester;

            type TestSuite = $crate::test::TestIndexView<$tester>;

            #[test]
            fn validate() {
                TestSuite::validate();
            }

            #[test]
            fn empty() {
                TestSuite::empty();
            }

            #[test]
            fn non_empty() {
                TestSuite::non_empty();
            }
        } // mod test_index_view
    };
}

/// Test suite for the `IndexView` trait.
pub struct TestIndexView<T>(PhantomData<T>);

impl<T> TestIndexView<T>
where
    T: IndexTester,
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

    /// Checks that an empty `victim` will appropriately be empty, have a length of 0, and not contain any element.
    pub fn empty() {
        let victim = T::victim(&[]);

        assert!(victim.is_empty());
        assert_eq!(0, victim.len());

        for i in 0..=T::upper_bound() {
            assert!(!victim.contains(T::index(i)), "{i}");
        }
    }

    /// Checks that a non-empty `victim` will appropriately not be empty, will have the expected length, and will
    /// not contain any unexpected elements.
    pub fn non_empty() {
        const INDEXES: [u8; 3] = [0, 3, 6];

        let victim = T::victim(&INDEXES);

        assert!(!victim.is_empty());
        assert_eq!(3, victim.len());

        for i in 0..=T::upper_bound() {
            if INDEXES.contains(&i) {
                continue;
            }

            assert!(!victim.contains(T::index(i)), "{i}");
        }
    }
}
