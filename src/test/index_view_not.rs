//! Test suite for the `IndexViewNot` trait.

use core::marker::PhantomData;

use crate::not::IndexViewNot;

use super::IndexTesterNot;

/// Tests that the `$victim` correctly implements the `IndexViewNot` trait.
#[macro_export]
macro_rules! test_index_view_not {
    ($tester:ident) => {
        mod test_index_view_not {
            use super::$tester;

            type TestSuite = $crate::test::TestIndexViewNot<$tester>;

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
        } // mod test_index_view_not
    };
}

/// Test suite for the `IndexViewNot` trait.
pub struct TestIndexViewNot<T>(PhantomData<T>);

impl<T> TestIndexViewNot<T>
where
    T: IndexTesterNot<Victim: IndexViewNot>,
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

    /// Checks that an empty `victim` will appropriately be empty, have a length of 0, and not contain any element.
    pub fn empty() {
        let victim = T::victim(&[]);

        assert_eq!(T::capacity(), victim.len_not());
    }

    /// Checks that a non-empty `victim` will appropriately not be empty, will have the expected length, and will
    /// not contain any unexpected elements.
    pub fn non_empty() {
        const INDEXES: [u8; 3] = [0, 3, 6];

        let victim = T::victim(&INDEXES);

        assert_eq!(T::capacity() - 3, victim.len_not());
    }
}
