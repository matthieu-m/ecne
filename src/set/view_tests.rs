//! Unit tests for overall operations.

mod index_set {
    use std::collections::HashSet;

    use crate::set::IndexSet;

    type Victim = IndexSet<HashSet<u8>>;

    #[test]
    fn contains() {
        const EMPTY: [u8; 0] = [];
        const SOME: [u8; 7] = [1, 2, 3, 5, 7, 11, 13];

        {
            let victim: Victim = EMPTY.into_iter().collect();

            assert!(!victim.contains(0));
            assert!(!victim.contains(1));
        }

        {
            let victim: Victim = SOME.into_iter().collect();

            assert!(!victim.contains(0));
            assert!(victim.contains(1));
        }
    }
} // mod index_set

mod index_ord_set {
    use alloc::collections::BTreeSet;

    use crate::set::IndexOrdSet;

    type Victim = IndexOrdSet<BTreeSet<u8>>;

    #[test]
    fn contains() {
        const EMPTY: [u8; 0] = [];
        const SOME: [u8; 7] = [1, 2, 3, 5, 7, 11, 13];

        {
            let victim: Victim = EMPTY.into_iter().collect();

            assert!(!victim.contains(0));
            assert!(!victim.contains(1));
        }

        {
            let victim: Victim = SOME.into_iter().collect();

            assert!(!victim.contains(0));
            assert!(victim.contains(1));
        }
    }
} // mod index_ord_set
