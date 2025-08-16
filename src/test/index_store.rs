//! Test suite for the `IndexStore` trait.

use core::marker::PhantomData;

use crate::index::{IndexStore, IndexView};

use super::IndexTester;

/// Tests that the `$victim` correctly implements the `IndexStore` trait.
#[macro_export]
macro_rules! test_index_store {
    ($tester:ident) => {
        mod test_index_store {
            use super::$tester;

            type TestSuite = $crate::test::TestIndexStore<$tester>;

            #[test]
            fn clear_empty() {
                TestSuite::clear_empty();
            }

            #[test]
            fn clear_non_empty() {
                TestSuite::clear_non_empty();
            }

            #[test]
            fn insert_remove() {
                TestSuite::insert_remove();
            }
        } // test_index_store
    };
}

/// Test suite for the `IndexStore` traits.
pub struct TestIndexStore<T>(PhantomData<T>);

impl<T> TestIndexStore<T>
where
    T: IndexTester<Victim: IndexStore>,
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

    /// Checks that `clear` keeps an empty instance empty.
    pub fn clear_empty() {
        let mut victim = T::victim(&[]);

        victim.clear();

        assert!(victim.is_empty());
        assert_eq!(0, victim.len());

        for i in 0..=T::upper_bound() {
            assert!(!victim.contains(T::index(i)), "{i}");
        }
    }

    /// Checks that `clear` makes a non-empty instance empty.
    pub fn clear_non_empty() {
        const INDEXES: [u8; 5] = [1, 2, 3, 5, 7];

        let mut victim = T::victim(&INDEXES);

        victim.clear();

        assert!(victim.is_empty());
        assert_eq!(0, victim.len());

        for i in 0..=T::upper_bound() {
            assert!(!victim.contains(T::index(i)), "{i}");
        }
    }

    /// Checks that an index is not contained by default, is contained after being inserted, and is no longer contained
    /// after being removed.
    pub fn insert_remove() {
        const INDEX: u8 = 5;

        let mut victim = T::victim(&[]);

        for i in 0..3 {
            let index = T::index(INDEX);

            assert!(!victim.contains(index), "{i}");

            assert!(victim.insert(index).unwrap(), "{i}");
            assert!(!victim.insert(index).unwrap(), "{i}");
            assert!(!victim.insert(index).unwrap(), "{i}");

            assert!(victim.contains(index), "{i}");

            assert!(victim.remove(index), "{i}");
            assert!(!victim.remove(index), "{i}");
            assert!(!victim.remove(index), "{i}");
        }
    }
}
