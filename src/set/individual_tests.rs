//! Unit tests for individual operations.

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

    #[test]
    fn insert_remove() {
        const INDEX: u8 = 42;

        let mut victim = Victim::new();

        for _ in 0..3 {
            assert!(!victim.contains(INDEX));

            assert!(victim.insert(INDEX));
            assert!(!victim.insert(INDEX));
            assert!(!victim.insert(INDEX));

            assert!(victim.remove(INDEX));
            assert!(!victim.remove(INDEX));
            assert!(!victim.remove(INDEX));
        }
    }

    #[test]
    fn extend() {
        const EMPTY: [u8; 0] = [];
        const SOME: [u8; 7] = [1, 2, 3, 5, 7, 11, 13];

        {
            let mut victim = Victim::new();

            victim.extend(EMPTY);

            assert!(victim.is_empty());
            assert!(!victim.contains(0));
            assert!(!victim.contains(1));
        }

        {
            let mut victim = Victim::new();

            victim.extend(SOME);

            assert!(!victim.is_empty());
            assert_eq!(SOME.len(), victim.len());

            assert!(!victim.contains(0));

            for index in SOME {
                assert!(victim.contains(index));
            }
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

    #[test]
    fn insert_remove() {
        const INDEX: u8 = 42;

        let mut victim = Victim::new();

        for _ in 0..3 {
            assert!(!victim.contains(INDEX));

            assert!(victim.insert(INDEX));
            assert!(!victim.insert(INDEX));
            assert!(!victim.insert(INDEX));

            assert!(victim.remove(INDEX));
            assert!(!victim.remove(INDEX));
            assert!(!victim.remove(INDEX));
        }
    }

    #[test]
    fn extend() {
        const EMPTY: [u8; 0] = [];
        const SOME: [u8; 7] = [1, 2, 3, 5, 7, 11, 13];

        {
            let mut victim = Victim::new();

            victim.extend(EMPTY);

            assert!(victim.is_empty());
            assert!(!victim.contains(0));
            assert!(!victim.contains(1));
        }

        {
            let mut victim = Victim::new();

            victim.extend(SOME);

            assert!(!victim.is_empty());
            assert_eq!(SOME.len(), victim.len());

            assert!(!victim.contains(0));

            for index in SOME {
                assert!(victim.contains(index));
            }
        }
    }
} // mod index_ord_set
