//! Test suite for the `IndexCollection` trait.

use core::{marker::PhantomData, ops::Bound};

use crate::index::{IndexCollection, IndexView};

use super::IndexTester;

/// Tests that the `$victim` correctly implements the `IndexCollection` trait.
#[macro_export]
macro_rules! test_index_collection {
    ($tester:ident) => {
        mod test_index_collection {
            use super::$tester;

            type TestSuite = $crate::test::TestIndexCollection<$tester>;

            #[test]
            fn validate() {
                TestSuite::validate();
            }

            #[test]
            fn new() {
                TestSuite::new();
            }

            #[test]
            fn with_span() {
                TestSuite::with_span();
            }
        } // mod test_index_collection
    };
}

/// Test suite for the `IndexCollection` trait.
pub struct TestIndexCollection<T>(PhantomData<T>);

impl<T> TestIndexCollection<T>
where
    T: IndexTester<Victim: IndexCollection>,
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

    /// Checks that a `victim` created by `new` will appropriately be empty, have a length of 0, and not contain any
    /// element.
    #[allow(clippy::new_ret_no_self)]
    pub fn new() {
        let victim = T::Victim::new();

        assert!(victim.is_empty());
        assert_eq!(0, victim.len());

        for i in 0..=T::upper_bound() {
            assert!(!victim.contains(T::index(i)), "{i}");
        }
    }

    /// Checks that a `victim` created by `with_span` will appropriately be empty, have a length of 0, and not contain
    /// any element.
    pub fn with_span() {
        let start = Bound::Included(T::index(0));
        let end = Bound::Included(T::index(Self::MINIMUM_UPPER_BOUND));

        let victim = T::Victim::with_span((start, end));

        assert!(victim.is_empty());
        assert_eq!(0, victim.len());

        for i in 0..=T::upper_bound() {
            assert!(!victim.contains(T::index(i)), "{i}");
        }
    }
}
